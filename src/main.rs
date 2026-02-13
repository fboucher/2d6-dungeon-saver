use std::io::{self, stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    cursor,
    style::{Color, SetForegroundColor, ResetColor},
};

mod dungeon;
mod explorer;
mod export;
mod renderer;
mod rng;
mod theme;

use dungeon::DungeonGenerator;
use explorer::{Explorer, Pathfinder};
use export::MapExporter;
use renderer::{SimpleRoom, MultiRoomRenderer};
use rng::SeededRng;
use theme::Theme;

const TARGET_FPS: u64 = 10;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / TARGET_FPS);

fn main() -> io::Result<()> {
    // Setup panic hook to restore terminal on crash
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = cleanup_terminal();
        original_hook(panic_info);
    }));

    // Setup signal handler for SIGTERM/SIGINT (Ctrl+C)
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        let _ = cleanup_terminal();
        r.store(false, Ordering::SeqCst);
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    // Parse CLI arguments
    let args: Vec<String> = std::env::args().collect();
    let seed = parse_seed(&args);

    // Initialize terminal
    setup_terminal()?;

    // Run main loop
    let result = run(seed, running);

    // Cleanup terminal
    cleanup_terminal()?;

    result
}

fn setup_terminal() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;
    Ok(())
}

fn cleanup_terminal() -> io::Result<()> {
    execute!(stdout(), cursor::Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn run(seed: u64, running: Arc<AtomicBool>) -> io::Result<()> {
    let theme = Theme::catppuccin_mocha();
    
    // Generate dungeon (progressive: start with entrance only)
    let mut generator = DungeonGenerator::new(seed);
    let entrance = generator.generate_entrance();
    let mut dungeon = vec![entrance];
    
    // Initialize pathfinder with dungeon layout
    let mut pathfinder = Pathfinder::new(&dungeon);
    
    // Initialize RNG for explorer behavior
    let mut rng = SeededRng::new(seed);
    
    // Create explorer at entrance room center
    let entrance = &dungeon[0];
    let start_x = entrance.x + entrance.width / 2;
    let start_y = entrance.y + entrance.height / 2;
    let mut explorer = Explorer::new(start_x, start_y);

    loop {
        // Check if we received a signal
        if !running.load(Ordering::SeqCst) {
            break;
        }
        
        let frame_start = Instant::now();
        
        // Update explorer AI
        explorer.update(&dungeon, &pathfinder, &mut rng);
        
        // Check if explorer stepped on an unexplored exit - generate new room if so
        if let Some((parent_room_id, exit_wall)) = detect_unexplored_exit(explorer.x, explorer.y, &dungeon) {
            let new_room = generator.add_room(parent_room_id, exit_wall);
            
            // Only add the room if it's not a dummy (id != usize::MAX means we haven't hit the limit)
            if new_room.id != usize::MAX {
                // Update the parent room's exit to mark it as connected
                if let Some(parent) = dungeon.iter_mut().find(|r| r.id == parent_room_id) {
                    for exit in parent.exits.iter_mut() {
                        if exit.wall == exit_wall && exit.connected_room_id.is_none() {
                            exit.connected_room_id = Some(new_room.id);
                            break;
                        }
                    }
                }
                
                dungeon.push(new_room);
                // Rebuild pathfinder with the expanded dungeon
                pathfinder = Pathfinder::new(&dungeon);
            }
        }

        // Render using MultiRoomRenderer
        render_dungeon(&dungeon, &explorer, &theme)?;

        // Handle events (non-blocking)
        if event::poll(Duration::from_millis(0))? {
            match event::read()? {
                Event::Key(key) => {
                    if should_quit(&key) {
                        break;
                    }
                }
                Event::Resize(_, _) => {
                    // Terminal resize - just continue rendering
                }
                _ => {}
            }
        }

        // Maintain target FPS
        let elapsed = frame_start.elapsed();
        if elapsed < FRAME_DURATION {
            std::thread::sleep(FRAME_DURATION - elapsed);
        }
    }

    // Export dungeon map on exit
    let exporter = MapExporter::new(seed);
    match exporter.export(&dungeon) {
        Ok(filepath) => {
            eprintln!("Map exported to: {}", filepath.display());
        }
        Err(e) => {
            eprintln!("Warning: Failed to export map: {}", e);
        }
    }

    Ok(())
}

fn render_dungeon(dungeon: &[dungeon::Room], explorer: &Explorer, theme: &Theme) -> io::Result<()> {
    let mut stdout = stdout();
    
    // Clear screen and move to top
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    
    // Build MultiRoomRenderer from dungeon
    let mut renderer = MultiRoomRenderer::new();
    
    for room in dungeon {
        let simple_room = SimpleRoom::from_dungeon_room(room);
        renderer.add_room(simple_room, room.x as usize, room.y as usize);
    }
    
    // Set explorer position
    renderer.set_explorer_pos(explorer.x as usize, explorer.y as usize);
    
    // Render to string
    let output = renderer.render();
    
    // Print with colors
    for line in output.lines() {
        for ch in line.chars() {
            let color = match ch {
                '#' => to_crossterm_color(theme.wall),
                '.' => to_crossterm_color(theme.floor),
                '-' | '|' => to_crossterm_color(theme.door),
                '@' => to_crossterm_color(theme.explorer),
                _ => Color::White,
            };
            
            if ch != ' ' {
                print!("{}{}{}", SetForegroundColor(color), ch, ResetColor);
            } else {
                print!(" ");
            }
        }
        println!();
    }
    
    // Print legend
    println!();
    println!("{}Legend:{} {}█{} wall | {} {} floor | {}─│{} door | {}@{} explorer | Press Q to quit",
        SetForegroundColor(Color::White), ResetColor,
        SetForegroundColor(to_crossterm_color(theme.wall)), ResetColor,
        SetForegroundColor(to_crossterm_color(theme.floor)), ResetColor,
        SetForegroundColor(to_crossterm_color(theme.door)), ResetColor,
        SetForegroundColor(to_crossterm_color(theme.explorer)), ResetColor);
    
    stdout.flush()?;
    Ok(())
}

fn to_crossterm_color(color: ratatui::style::Color) -> Color {
    match color {
        ratatui::style::Color::Rgb(r, g, b) => Color::Rgb { r, g, b },
        _ => Color::White,
    }
}

fn should_quit(key: &KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => true,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => true,
        _ => false,
    }
}

/// Check if explorer is on an unexplored exit and return (room_id, wall) if so
fn detect_unexplored_exit(explorer_x: u32, explorer_y: u32, dungeon: &[dungeon::Room]) -> Option<(usize, dungeon::Wall)> {
    use dungeon::Wall;
    
    for room in dungeon.iter() {
        // Check each exit in this room
        for exit in &room.exits {
            // Calculate the actual position of the exit in world coordinates
            let exit_pos = match exit.wall {
                Wall::North => (room.x + exit.position, room.y),
                Wall::South => (room.x + exit.position, room.y + room.height - 1),
                Wall::East => (room.x + room.width - 1, room.y + exit.position),
                Wall::West => (room.x, room.y + exit.position),
            };
            
            // Is the explorer standing on this exit?
            let on_exit = explorer_x == exit_pos.0 && explorer_y == exit_pos.1;
            
            // Is this exit unexplored (not connected to another room)?
            if on_exit && exit.connected_room_id.is_none() {
                return Some((room.id, exit.wall));
            }
        }
    }
    
    None
}

fn parse_seed(args: &[String]) -> u64 {
    for i in 0..args.len() {
        if args[i] == "--seed" && i + 1 < args.len() {
            if let Ok(seed) = args[i + 1].parse::<u64>() {
                return seed;
            }
        }
    }
    
    // Default to random seed based on current time
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

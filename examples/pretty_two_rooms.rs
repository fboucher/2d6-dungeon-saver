/// Pretty demo: Step 3 - Two connected rooms with Catppuccin Mocha colors and box-drawing
use crossterm::style::{Color, SetForegroundColor, ResetColor};
use dungeon_saver::theme::Theme;

fn main() {
    let theme = Theme::catppuccin_mocha();
    
    println!("{}Step 3: Pretty Two Connected Rooms{}\n", 
        SetForegroundColor(to_crossterm_color(theme.explorer)),
        ResetColor);
    
    println!("Room 1: 6×4 floor with north and east doors");
    println!("Room 2: 8×10 floor");
    println!("Connected via east door\n");
    
    render_two_rooms(&theme);
    
    println!("\n{}Legend:{}", 
        SetForegroundColor(to_crossterm_color(theme.explorer)),
        ResetColor);
    println!("  {}█{} = wall (Lavender)", 
        SetForegroundColor(to_crossterm_color(theme.wall)),
        ResetColor);
    println!("  {} {} = floor (Latte)", 
        SetForegroundColor(to_crossterm_color(theme.floor)),
        ResetColor);
    println!("  {}─{} = north door (Peach)", 
        SetForegroundColor(to_crossterm_color(theme.door)),
        ResetColor);
    println!("  {}│{} = east door connecting rooms (Peach)", 
        SetForegroundColor(to_crossterm_color(theme.door)),
        ResetColor);
}

struct Room {
    floor_width: usize,
    floor_height: usize,
    has_north_door: bool,
    has_east_door: bool,
}

impl Room {
    fn width(&self) -> usize {
        self.floor_width + 2
    }
    
    fn height(&self) -> usize {
        self.floor_height + 2
    }
}

fn render_two_rooms(theme: &Theme) {
    // Room 1: 6×4 floor with north and east doors
    let room1 = Room {
        floor_width: 6,
        floor_height: 4,
        has_north_door: true,
        has_east_door: true,
    };
    
    // Room 2: 8×10 floor
    let room2 = Room {
        floor_width: 8,
        floor_height: 10,
        has_north_door: false,
        has_east_door: false,
    };
    
    // Calculate bounding box
    let max_x = 7 + room2.width(); // room2 starts at x=7
    let max_y = room2.height().max(room1.height());
    
    // Create grid
    let mut grid: Vec<Vec<char>> = vec![vec![' '; max_x]; max_y];
    let mut colors: Vec<Vec<Color>> = vec![vec![Color::White; max_x]; max_y];
    
    // Render room1 at (0, 0)
    render_room_to_grid(&room1, 0, 0, &mut grid, &mut colors, theme);
    
    // Render room2 at (7, 0) - shares wall with room1
    render_room_to_grid(&room2, 7, 0, &mut grid, &mut colors, theme);
    
    // Print the grid
    for y in 0..max_y {
        for x in 0..max_x {
            if grid[y][x] != ' ' {
                print!("{}{}{}", 
                    SetForegroundColor(colors[y][x]),
                    grid[y][x],
                    ResetColor);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn render_room_to_grid(
    room: &Room,
    offset_x: usize,
    offset_y: usize,
    grid: &mut Vec<Vec<char>>,
    colors: &mut Vec<Vec<Color>>,
    theme: &Theme,
) {
    let total_width = room.width();
    let total_height = room.height();
    
    // Calculate door positions (centered)
    let north_door_x = total_width / 2 - 1;
    let east_door_y = total_height / 2 + 1;
    
    for y in 0..total_height {
        for x in 0..total_width {
            let grid_x = offset_x + x;
            let grid_y = offset_y + y;
            
            if grid_y >= grid.len() || grid_x >= grid[0].len() {
                continue;
            }
            
            let is_wall = y == 0 || y == total_height - 1 || x == 0 || x == total_width - 1;
            
            if is_wall {
                let (ch, color) = if room.has_north_door && y == 0 && x == north_door_x {
                    ('─', to_crossterm_color(theme.door))
                } else if room.has_east_door && x == total_width - 1 && y == east_door_y {
                    ('│', to_crossterm_color(theme.door))
                } else {
                    ('█', to_crossterm_color(theme.wall))
                };
                
                // Don't overwrite doors with walls
                if grid[grid_y][grid_x] == '─' || grid[grid_y][grid_x] == '│' {
                    continue;
                }
                
                grid[grid_y][grid_x] = ch;
                colors[grid_y][grid_x] = color;
            } else {
                if grid[grid_y][grid_x] == ' ' {
                    grid[grid_y][grid_x] = ' ';
                    colors[grid_y][grid_x] = to_crossterm_color(theme.floor);
                }
            }
        }
    }
}

fn to_crossterm_color(color: ratatui::style::Color) -> Color {
    match color {
        ratatui::style::Color::Rgb(r, g, b) => Color::Rgb { r, g, b },
        _ => Color::White,
    }
}

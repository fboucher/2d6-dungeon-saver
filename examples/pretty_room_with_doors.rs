/// Pretty demo: Step 2 - Room with doors using Catppuccin Mocha colors and box-drawing
use crossterm::style::{Color, SetForegroundColor, ResetColor};
use dungeon_saver::theme::Theme;

fn main() {
    let theme = Theme::catppuccin_mocha();
    
    println!("{}Step 2: Pretty Room with Doors{}\n", 
        SetForegroundColor(to_crossterm_color(theme.explorer)),
        ResetColor);
    
    println!("Room: 6×4 floor with doors on east and north walls\n");
    
    render_room_with_doors(&theme);
    
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
    println!("  {}│{} = east door (Peach)", 
        SetForegroundColor(to_crossterm_color(theme.door)),
        ResetColor);
}

fn render_room_with_doors(theme: &Theme) {
    let floor_width = 6;
    let floor_height = 4;
    let total_width = floor_width + 2;
    let total_height = floor_height + 2;
    
    // Calculate door positions (centered)
    let north_door_x = total_width / 2 - 1;
    let east_door_y = total_height / 2 + 1;
    
    for y in 0..total_height {
        for x in 0..total_width {
            let is_wall = y == 0 || y == total_height - 1 || x == 0 || x == total_width - 1;
            
            if is_wall {
                // Check for doors
                if y == 0 && x == north_door_x {
                    // North door
                    print!("{}─{}", 
                        SetForegroundColor(to_crossterm_color(theme.door)),
                        ResetColor);
                } else if x == total_width - 1 && y == east_door_y {
                    // East door
                    print!("{}│{}", 
                        SetForegroundColor(to_crossterm_color(theme.door)),
                        ResetColor);
                } else {
                    // Regular wall
                    print!("{}█{}", 
                        SetForegroundColor(to_crossterm_color(theme.wall)),
                        ResetColor);
                }
            } else {
                // Floor
                print!("{} {}", 
                    SetForegroundColor(to_crossterm_color(theme.floor)),
                    ResetColor);
            }
        }
        println!();
    }
}

fn to_crossterm_color(color: ratatui::style::Color) -> Color {
    match color {
        ratatui::style::Color::Rgb(r, g, b) => Color::Rgb { r, g, b },
        _ => Color::White,
    }
}

/// Pretty demo: Step 1 - Single room with Catppuccin Mocha colors and box-drawing
use crossterm::style::{Color, SetForegroundColor, ResetColor};
use dungeon_saver::theme::Theme;

fn main() {
    let theme = Theme::catppuccin_mocha();
    
    println!("{}=== Pretty Single Room Demo ==={}\n", 
        SetForegroundColor(to_crossterm_color(theme.explorer)),
        ResetColor);
    
    println!("Room: 4×5 floor with Catppuccin Mocha theme\n");
    
    render_single_room(&theme);
    
    println!("\n{}Legend:{}", 
        SetForegroundColor(to_crossterm_color(theme.explorer)),
        ResetColor);
    println!("  {}█{} = wall (Lavender)", 
        SetForegroundColor(to_crossterm_color(theme.wall)),
        ResetColor);
    println!("  {} {} = floor (Latte)", 
        SetForegroundColor(to_crossterm_color(theme.floor)),
        ResetColor);
}

fn render_single_room(theme: &Theme) {
    let floor_width = 4;
    let floor_height = 5;
    let total_width = floor_width + 2;
    let total_height = floor_height + 2;
    
    for y in 0..total_height {
        for x in 0..total_width {
            let is_wall = y == 0 || y == total_height - 1 || x == 0 || x == total_width - 1;
            
            if is_wall {
                print!("{}█{}", 
                    SetForegroundColor(to_crossterm_color(theme.wall)),
                    ResetColor);
            } else {
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

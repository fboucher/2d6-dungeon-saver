/// Color theme support — currently Catppuccin Mocha
use ratatui::style::Color;

pub struct Theme {
    pub wall: Color,
    pub floor: Color,
    /// Corridor color - reserved for future corridor rendering
    #[allow(dead_code)]
    pub corridor: Color,
    pub door: Color,
    pub explorer: Color,
    pub background: Color,
}

impl Theme {
    pub fn catppuccin_mocha() -> Self {
        Self {
            wall: Color::Rgb(88, 86, 214),      // Lavender
            floor: Color::Rgb(205, 214, 244),   // Latte
            corridor: Color::Rgb(186, 194, 222), // Subtext1
            door: Color::Rgb(250, 179, 135),    // Peach
            explorer: Color::Rgb(166, 227, 161), // Green
            background: Color::Rgb(30, 30, 46),  // Base
        }
    }
}

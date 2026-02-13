use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::Widget,
    Frame,
};
use crate::dungeon::room::{Room, Wall};
use crate::theme::Theme;
use super::Camera;

/// Canvas: Ratatui drawing and screen management
pub struct Canvas {
    theme: Theme,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            theme: Theme::catppuccin_mocha(),
        }
    }

    /// Render the current frame
    pub fn render(&self, frame: &mut Frame, dungeon: &[Room], explorer_pos: (u32, u32), camera: &Camera) {
        let area = frame.area();
        
        // Create dungeon widget with camera viewport
        let dungeon_widget = DungeonWidget {
            rooms: dungeon,
            explorer_pos,
            camera,
            theme: &self.theme,
        };
        
        frame.render_widget(dungeon_widget, area);
    }
}

struct DungeonWidget<'a> {
    rooms: &'a [Room],
    explorer_pos: (u32, u32),
    camera: &'a Camera,
    theme: &'a Theme,
}

impl Widget for DungeonWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Fill background
        for y in 0..area.height {
            for x in 0..area.width {
                buf[(area.x + x, area.y + y)]
                    .set_char(' ')
                    .set_style(Style::default().bg(self.theme.background));
            }
        }
        
        // Render each room within camera viewport
        for room in self.rooms {
            self.render_room(room, area, buf);
        }
        
        // Render explorer sprite
        self.render_explorer(area, buf);
    }
}

impl DungeonWidget<'_> {
    fn render_room(&self, room: &Room, area: Rect, buf: &mut Buffer) {
        let cam_x = self.camera.x as i32;
        let cam_y = self.camera.y as i32;
        
        // Room coordinates are 1:1 with screen coordinates
        let room_screen_x = room.x as i32 - cam_x;
        let room_screen_y = room.y as i32 - cam_y;
        
        // Skip if room is completely outside viewport
        if room_screen_x + (room.width as i32) < 0 || room_screen_y + (room.height as i32) < 0 {
            return;
        }
        if room_screen_x >= area.width as i32 || room_screen_y >= area.height as i32 {
            return;
        }
        
        // Determine wall and floor styles
        let wall_style = Style::default().fg(self.theme.wall);
        let floor_style = Style::default().fg(self.theme.floor);
        let door_style = Style::default().fg(self.theme.door);
        
        // Draw room: walls and floors (1:1 mapping)
        for dy in 0..room.height {
            for dx in 0..room.width {
                let screen_x = room_screen_x + dx as i32;
                let screen_y = room_screen_y + dy as i32;
                
                // Skip if outside viewport
                if screen_x < 0 || screen_y < 0 
                    || screen_x >= area.width as i32 
                    || screen_y >= area.height as i32 {
                    continue;
                }
                
                let is_wall = dy == 0 || dy == room.height - 1 
                    || dx == 0 || dx == room.width - 1;
                
                let (ch, style) = if is_wall {
                    ('█', wall_style)
                } else {
                    (' ', Style::default().bg(floor_style.fg.unwrap_or(self.theme.floor)))
                };
                
                buf[(area.x + screen_x as u16, area.y + screen_y as u16)]
                    .set_char(ch)
                    .set_style(style);
            }
        }
        
        // Draw exits (overwrite walls with door characters)
        for exit in &room.exits {
            let (exit_x, exit_y) = match exit.wall {
                Wall::North => (room.x + exit.position, room.y),
                Wall::South => (room.x + exit.position, room.y + room.height - 1),
                Wall::West => (room.x, room.y + exit.position),
                Wall::East => (room.x + room.width - 1, room.y + exit.position),
            };
            
            let screen_x = exit_x as i32 - cam_x;
            let screen_y = exit_y as i32 - cam_y;
            
            // Skip if outside viewport
            if screen_x < 0 || screen_y < 0 
                || screen_x >= area.width as i32 
                || screen_y >= area.height as i32 {
                continue;
            }
            
            let door_char = match exit.wall {
                Wall::North | Wall::South => '─',
                Wall::East | Wall::West => '│',
            };
            
            buf[(area.x + screen_x as u16, area.y + screen_y as u16)]
                .set_char(door_char)
                .set_style(door_style);
        }
    }
    
    fn render_explorer(&self, area: Rect, buf: &mut Buffer) {
        let cam_x = self.camera.x as i32;
        let cam_y = self.camera.y as i32;
        
        // Explorer position is 1:1 with screen coordinates
        let screen_x = self.explorer_pos.0 as i32 - cam_x;
        let screen_y = self.explorer_pos.1 as i32 - cam_y;
        
        // Only render if in viewport
        if screen_x >= 0 && screen_y >= 0 
            && screen_x < area.width as i32 
            && screen_y < area.height as i32 {
            
            let explorer_style = Style::default().fg(self.theme.explorer);
            
            buf[(area.x + screen_x as u16, area.y + screen_y as u16)]
                .set_char('@')
                .set_style(explorer_style);
        }
    }
}

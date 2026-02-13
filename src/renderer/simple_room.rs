/// Simple room renderer - renders a single hardcoded room (floor dimensions)
#[derive(Clone)]
pub struct SimpleRoom {
    floor_width: usize,
    floor_height: usize,
    has_north_door: bool,
    has_south_door: bool,
    has_east_door: bool,
    has_west_door: bool,
}

impl SimpleRoom {
    /// Convert a dungeon Room to SimpleRoom
    pub fn from_dungeon_room(room: &crate::dungeon::Room) -> Self {
        use crate::dungeon::Wall;
        
        // Dungeon rooms include walls in their dimensions, so we need to subtract 2
        let floor_width = (room.width.saturating_sub(2)) as usize;
        let floor_height = (room.height.saturating_sub(2)) as usize;
        
        // Check which walls have doors/exits
        let has_north_door = room.exits.iter().any(|e| e.wall == Wall::North);
        let has_south_door = room.exits.iter().any(|e| e.wall == Wall::South);
        let has_east_door = room.exits.iter().any(|e| e.wall == Wall::East);
        let has_west_door = room.exits.iter().any(|e| e.wall == Wall::West);
        
        Self {
            floor_width,
            floor_height,
            has_north_door,
            has_south_door,
            has_east_door,
            has_west_door,
        }
    }
}

/// Renders multiple positioned rooms together
pub struct MultiRoomRenderer {
    rooms: Vec<(SimpleRoom, usize, usize)>, // (room, x_offset, y_offset)
    explorer_pos: Option<(usize, usize)>, // Optional explorer position (x, y)
}

impl SimpleRoom {
    pub fn new() -> Self {
        Self {
            floor_width: 4,
            floor_height: 5,
            has_north_door: false,
            has_south_door: false,
            has_east_door: false,
            has_west_door: false,
        }
    }

    pub fn new_with_doors() -> Self {
        Self {
            floor_width: 6,
            floor_height: 4,
            has_north_door: true,
            has_south_door: false,
            has_east_door: true,
            has_west_door: false,
        }
    }

    pub fn new_large() -> Self {
        Self {
            floor_width: 8,
            floor_height: 10,
            has_north_door: false,
            has_south_door: false,
            has_east_door: false,
            has_west_door: false,
        }
    }

    /// Create a SimpleRoom from dimensions and door info
    pub fn from_dimensions(floor_width: usize, floor_height: usize, 
                          has_north_door: bool, has_south_door: bool, 
                          has_east_door: bool, has_west_door: bool) -> Self {
        Self {
            floor_width,
            floor_height,
            has_north_door,
            has_south_door,
            has_east_door,
            has_west_door,
        }
    }

    /// Render the room to a string
    pub fn render(&self) -> String {
        let mut lines = Vec::new();
        let total_width = self.floor_width + 2;
        let total_height = self.floor_height + 2;
        
        // Calculate door positions (centered)
        // For horizontal doors (north/south): slightly left of center
        let north_door_x = if total_width > 1 { total_width / 2 - 1 } else { 0 };
        let south_door_x = if total_width > 1 { total_width / 2 - 1 } else { 0 };
        // For vertical doors (east/west): slightly below center
        let east_door_y = if total_height > 2 { total_height / 2 + 1 } else { 1 };
        let west_door_y = if total_height > 2 { total_height / 2 + 1 } else { 1 };
        
        for y in 0..total_height {
            let mut line = String::new();
            for x in 0..total_width {
                let is_wall = y == 0 || y == total_height - 1 || x == 0 || x == total_width - 1;
                
                let ch = if is_wall {
                    // Check for doors
                    if self.has_north_door && y == 0 && x == north_door_x {
                        '-'
                    } else if self.has_south_door && y == total_height - 1 && x == south_door_x {
                        '-'
                    } else if self.has_east_door && x == total_width - 1 && y == east_door_y {
                        '|'
                    } else if self.has_west_door && x == 0 && y == west_door_y {
                        '|'
                    } else {
                        '#'
                    }
                } else {
                    '.'
                };
                line.push(ch);
            }
            lines.push(line);
        }
        
        lines.join("\n")
    }

    pub fn width(&self) -> usize {
        self.floor_width + 2
    }

    pub fn height(&self) -> usize {
        self.floor_height + 2
    }
}

impl MultiRoomRenderer {
    pub fn new() -> Self {
        Self { 
            rooms: Vec::new(),
            explorer_pos: None,
        }
    }

    pub fn add_room(&mut self, room: SimpleRoom, x: usize, y: usize) {
        self.rooms.push((room, x, y));
    }

    pub fn set_explorer_pos(&mut self, x: usize, y: usize) {
        self.explorer_pos = Some((x, y));
    }

    pub fn clear_explorer(&mut self) {
        self.explorer_pos = None;
    }

    /// Render all rooms to a single string
    pub fn render(&self) -> String {
        if self.rooms.is_empty() {
            return String::new();
        }

        // Calculate the bounding box
        let mut max_x = 0;
        let mut max_y = 0;
        
        for (room, x, y) in &self.rooms {
            max_x = max_x.max(x + room.width());
            max_y = max_y.max(y + room.height());
        }

        // Create a 2D grid filled with spaces
        let mut grid: Vec<Vec<char>> = vec![vec![' '; max_x]; max_y];

        // Render each room into the grid
        for (room, offset_x, offset_y) in &self.rooms {
            let room_output = room.render();
            for (y, line) in room_output.lines().enumerate() {
                for (x, ch) in line.chars().enumerate() {
                    let grid_y = offset_y + y;
                    let grid_x = offset_x + x;
                    if grid_y < max_y && grid_x < max_x {
                        let existing = grid[grid_y][grid_x];
                        // If placing a wall on a door or vice versa, keep the door
                        if existing == '|' || existing == '-' {
                            // Keep the door
                            continue;
                        } else if ch == '|' || ch == '-' {
                            // Place the door
                            grid[grid_y][grid_x] = ch;
                        } else if existing == ' ' {
                            // Place new content in empty space
                            grid[grid_y][grid_x] = ch;
                        }
                        // Otherwise keep existing (for overlapping walls)
                    }
                }
            }
        }

        // Place explorer if position is set
        if let Some((ex_x, ex_y)) = self.explorer_pos {
            if ex_y < max_y && ex_x < max_x {
                let current_char = grid[ex_y][ex_x];
                // Only render explorer on floor tiles or door tiles (not walls or empty space)
                if current_char == '.' || current_char == '-' || current_char == '|' {
                    grid[ex_y][ex_x] = '@';
                }
            }
        }

        // Convert grid to string
        grid.iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_dimensions() {
        let room = SimpleRoom::new();
        assert_eq!(room.width(), 6);
        assert_eq!(room.height(), 7);
    }

    #[test]
    fn test_room_render() {
        let room = SimpleRoom::new();
        let output = room.render();
        let lines: Vec<&str> = output.lines().collect();
        
        // Check number of lines (5 floor + 2 walls = 7)
        assert_eq!(lines.len(), 7);
        
        // Check each line width (4 floor + 2 walls = 6)
        for line in &lines {
            assert_eq!(line.len(), 6);
        }
        
        // Check top wall
        assert_eq!(lines[0], "######");
        
        // Check middle rows (interior)
        assert_eq!(lines[1], "#....#");
        assert_eq!(lines[2], "#....#");
        assert_eq!(lines[3], "#....#");
        assert_eq!(lines[4], "#....#");
        assert_eq!(lines[5], "#....#");
        
        // Check bottom wall
        assert_eq!(lines[6], "######");
    }
}

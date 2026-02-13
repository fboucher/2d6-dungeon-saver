use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use chrono::Local;
use crate::dungeon::Room;

pub struct MapExporter {
    seed: u64,
    maps_dir: PathBuf,
}

impl MapExporter {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            maps_dir: PathBuf::from("maps"),
        }
    }

    pub fn export(&self, dungeon: &[Room]) -> io::Result<PathBuf> {
        fs::create_dir_all(&self.maps_dir)?;
        
        let timestamp = Local::now().format("%Y-%m-%d_%H%M");
        let filename = format!("{}_seed{}.txt", timestamp, self.seed);
        let filepath = self.maps_dir.join(&filename);
        
        let mut file = File::create(&filepath)?;
        
        // Write metadata header
        writeln!(file, "# Dungeon Saver — Generated Map")?;
        writeln!(file, "# Seed: {}", self.seed)?;
        writeln!(file, "# Timestamp: {}", Local::now().format("%Y-%m-%d %H:%M:%S"))?;
        writeln!(file, "# Room Count: {}", dungeon.len())?;
        writeln!(file)?;
        
        // Calculate dungeon bounds
        let (min_x, min_y, max_x, max_y) = self.calculate_bounds(dungeon);
        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;
        
        writeln!(file, "# Dimensions: {}x{}", width, height)?;
        writeln!(file)?;
        
        // Create ASCII map grid
        let mut grid = vec![vec![' '; width]; height];
        
        // Render each room
        for (idx, room) in dungeon.iter().enumerate() {
            self.render_room_to_grid(&mut grid, room, min_x, min_y, idx);
        }
        
        // Write grid to file
        for row in grid.iter() {
            let line: String = row.iter().collect();
            writeln!(file, "{}", line)?;
        }
        
        writeln!(file)?;
        writeln!(file, "# Room Details:")?;
        for (idx, room) in dungeon.iter().enumerate() {
            writeln!(file, "# Room {}: {}x{} @ ({},{}), Type: {:?}, Exits: {}",
                idx,
                room.width, room.height,
                room.x, room.y,
                room.room_type,
                room.exits.len()
            )?;
        }
        
        Ok(filepath)
    }
    
    fn calculate_bounds(&self, dungeon: &[Room]) -> (u32, u32, u32, u32) {
        let mut min_x = u32::MAX;
        let mut min_y = u32::MAX;
        let mut max_x = u32::MIN;
        let mut max_y = u32::MIN;
        
        for room in dungeon {
            min_x = min_x.min(room.x);
            min_y = min_y.min(room.y);
            max_x = max_x.max(room.x + room.width);
            max_y = max_y.max(room.y + room.height);
        }
        
        (min_x, min_y, max_x, max_y)
    }
    
    fn render_room_to_grid(&self, grid: &mut [Vec<char>], room: &Room, offset_x: u32, offset_y: u32, room_id: usize) {
        let base_x = (room.x - offset_x) as usize;
        let base_y = (room.y - offset_y) as usize;
        
        // Draw room interior
        for dy in 0..room.height {
            for dx in 0..room.width {
                let gx = base_x + dx as usize;
                let gy = base_y + dy as usize;
                
                if gy < grid.len() && gx < grid[0].len() {
                    // Draw walls
                    if dy == 0 || dy == room.height - 1 {
                        grid[gy][gx] = '-';
                    } else if dx == 0 || dx == room.width - 1 {
                        grid[gy][gx] = '|';
                    } else {
                        grid[gy][gx] = '.';
                    }
                    
                    // Draw corners
                    if (dy == 0 && dx == 0) || 
                       (dy == 0 && dx == room.width - 1) ||
                       (dy == room.height - 1 && dx == 0) ||
                       (dy == room.height - 1 && dx == room.width - 1) {
                        grid[gy][gx] = '+';
                    }
                }
            }
        }
        
        // Mark entrance room
        if room_id == 0 {
            let center_x = base_x + (room.width / 2) as usize;
            let center_y = base_y + (room.height / 2) as usize;
            if center_y < grid.len() && center_x < grid[0].len() {
                grid[center_y][center_x] = 'E';
            }
        }
        
        // Draw exits as doors
        for exit in &room.exits {
            use crate::dungeon::room::Wall;
            let (ex, ey) = match exit.wall {
                Wall::North => (base_x + exit.position as usize, base_y),
                Wall::South => (base_x + exit.position as usize, base_y + room.height as usize - 1),
                Wall::West => (base_x, base_y + exit.position as usize),
                Wall::East => (base_x + room.width as usize - 1, base_y + exit.position as usize),
            };
            
            if ey < grid.len() && ex < grid[0].len() {
                grid[ey][ex] = '▢';
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::{DungeonGenerator, room::RoomType};
    use std::fs;
    
    #[test]
    fn test_export_creates_file() {
        let seed = 42;
        let mut generator = DungeonGenerator::new(seed);
        let dungeon = generator.generate();
        
        let exporter = MapExporter::new(seed);
        let result = exporter.export(&dungeon);
        
        assert!(result.is_ok());
        let filepath = result.unwrap();
        assert!(filepath.exists());
        
        // Cleanup
        let _ = fs::remove_file(filepath);
    }
    
    #[test]
    fn test_export_contains_metadata() {
        let seed = 12345;
        let mut generator = DungeonGenerator::new(seed);
        let dungeon = generator.generate();
        
        let exporter = MapExporter::new(seed);
        let filepath = exporter.export(&dungeon).unwrap();
        
        let content = fs::read_to_string(&filepath).unwrap();
        
        assert!(content.contains("Seed: 12345"));
        assert!(content.contains(&format!("Room Count: {}", dungeon.len())));
        assert!(content.contains("Dimensions:"));
        
        // Cleanup
        let _ = fs::remove_file(filepath);
    }
    
    #[test]
    fn test_export_contains_ascii_map() {
        let seed = 999;
        let mut generator = DungeonGenerator::new(seed);
        let dungeon = generator.generate();
        
        let exporter = MapExporter::new(seed);
        let filepath = exporter.export(&dungeon).unwrap();
        
        let content = fs::read_to_string(&filepath).unwrap();
        
        // Should contain ASCII characters for walls and floors
        assert!(content.contains('+'));
        assert!(content.contains('-'));
        assert!(content.contains('|'));
        assert!(content.contains('.'));
        
        // Cleanup
        let _ = fs::remove_file(filepath);
    }
    
    #[test]
    fn test_deterministic_export() {
        let seed = 42;
        
        // Generate dungeon twice with same seed
        let mut gen1 = DungeonGenerator::new(seed);
        let dungeon1 = gen1.generate();
        
        let mut gen2 = DungeonGenerator::new(seed);
        let dungeon2 = gen2.generate();
        
        // Both exports should have same room count and structure
        assert_eq!(dungeon1.len(), dungeon2.len());
        
        for (r1, r2) in dungeon1.iter().zip(dungeon2.iter()) {
            assert_eq!(r1.width, r2.width);
            assert_eq!(r1.height, r2.height);
            assert_eq!(r1.exits.len(), r2.exits.len());
        }
    }
}

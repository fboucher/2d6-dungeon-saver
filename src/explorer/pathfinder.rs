use crate::dungeon::{Room, room::Wall};
use pathfinding::prelude::astar;
use std::collections::HashSet;

/// Pathfinding using A* algorithm
pub struct Pathfinder {
    walkable_tiles: HashSet<(u32, u32)>,
}

impl Pathfinder {
    pub fn new(dungeon: &[Room]) -> Self {
        let mut walkable_tiles = HashSet::new();
        
        for room in dungeon {
            // Add all tiles within room bounds
            for y in room.y..room.y + room.height {
                for x in room.x..room.x + room.width {
                    walkable_tiles.insert((x, y));
                }
            }
            
            // Add corridor tiles connecting to exits
            for exit in &room.exits {
                let corridor_tiles = Self::get_exit_corridor_tiles(room, &exit.wall, exit.position);
                for tile in corridor_tiles {
                    walkable_tiles.insert(tile);
                }
            }
        }
        
        Self { walkable_tiles }
    }
    
    /// Get corridor tiles connecting an exit to adjacent rooms
    fn get_exit_corridor_tiles(room: &Room, wall: &Wall, position: u32) -> Vec<(u32, u32)> {
        let mut tiles = Vec::new();
        
        match wall {
            Wall::North => {
                // Exit on north wall - add tile above
                let x = room.x + position;
                let y = room.y.saturating_sub(1);
                tiles.push((x, y));
            }
            Wall::South => {
                // Exit on south wall - add tile below
                let x = room.x + position;
                let y = room.y + room.height;
                tiles.push((x, y));
            }
            Wall::East => {
                // Exit on east wall - add tile to the right
                let x = room.x + room.width;
                let y = room.y + position;
                tiles.push((x, y));
            }
            Wall::West => {
                // Exit on west wall - add tile to the left
                let x = room.x.saturating_sub(1);
                let y = room.y + position;
                tiles.push((x, y));
            }
        }
        
        tiles
    }
    
    /// Find path from start to goal using A* algorithm
    /// Returns Some(path) if path exists, None otherwise
    pub fn find_path(&self, start: (u32, u32), goal: (u32, u32)) -> Option<Vec<(u32, u32)>> {
        let result = astar(
            &start,
            |&(x, y)| self.successors(x, y),
            |&(x, y)| self.heuristic((x, y), goal),
            |&pos| pos == goal,
        );
        
        result.map(|(path, _cost)| path)
    }
    
    /// Get valid neighbors for A* (4-directional movement)
    fn successors(&self, x: u32, y: u32) -> Vec<((u32, u32), u32)> {
        let mut neighbors = Vec::new();
        
        // Check all 4 cardinal directions
        let directions = [
            (0, -1), // North
            (1, 0),  // East
            (0, 1),  // South
            (-1, 0), // West
        ];
        
        for (dx, dy) in directions {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            
            if new_x >= 0 && new_y >= 0 {
                let new_pos = (new_x as u32, new_y as u32);
                if self.walkable_tiles.contains(&new_pos) {
                    neighbors.push((new_pos, 1)); // Cost of 1 for each step
                }
            }
        }
        
        neighbors
    }
    
    /// Manhattan distance heuristic for A*
    fn heuristic(&self, pos: (u32, u32), goal: (u32, u32)) -> u32 {
        let dx = if pos.0 > goal.0 { pos.0 - goal.0 } else { goal.0 - pos.0 };
        let dy = if pos.1 > goal.1 { pos.1 - goal.1 } else { goal.1 - pos.1 };
        dx + dy
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::room::{Room, RoomType};
    
    #[test]
    fn test_pathfinder_straight_line() {
        let rooms = vec![
            Room {
                id: 0,
                x: 0,
                y: 0,
                width: 10,
                height: 10,
                room_type: RoomType::Entrance,
                exits: vec![],
                parent_id: None,
                parent_wall: None,
            },
        ];
        
        let pathfinder = Pathfinder::new(&rooms);
        let path = pathfinder.find_path((0, 0), (5, 5));
        
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.first(), Some(&(0, 0)));
        assert_eq!(path.last(), Some(&(5, 5)));
        assert_eq!(path.len(), 11); // Manhattan distance: 5+5+1
    }
    
    #[test]
    fn test_pathfinder_unreachable() {
        let rooms = vec![
            Room {
                id: 0,
                x: 0,
                y: 0,
                width: 5,
                height: 5,
                room_type: RoomType::Normal,
                exits: vec![],
                parent_id: None,
                parent_wall: None,
            },
            Room {
                id: 1,
                x: 10,
                y: 10,
                width: 5,
                height: 5,
                room_type: RoomType::Normal,
                exits: vec![],
                parent_id: None,
                parent_wall: None,
            },
        ];
        
        let pathfinder = Pathfinder::new(&rooms);
        let path = pathfinder.find_path((0, 0), (10, 10));
        
        assert!(path.is_none()); // No path between disconnected rooms
    }
}

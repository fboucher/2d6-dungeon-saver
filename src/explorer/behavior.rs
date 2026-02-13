use crate::dungeon::{Room, Wall};
use crate::explorer::Pathfinder;
use crate::rng::SeededRng;
use std::collections::HashSet;

/// Explorer behavior: exploring vs. wandering, room discovery, pauses
#[derive(Debug, Clone)]
pub struct Explorer {
    pub x: u32,
    pub y: u32,
    pub state: ExplorerState,
    visited_rooms: HashSet<usize>,
    current_path: Vec<(u32, u32)>,
    path_index: usize,
    move_cooldown: u32, // Ticks until next move (for smooth animation)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplorerState {
    Exploring,
    Wandering,
    Pausing { ticks_remaining: u32 },
}

impl Explorer {
    pub fn new(start_x: u32, start_y: u32) -> Self {
        Self {
            x: start_x,
            y: start_y,
            state: ExplorerState::Exploring,
            visited_rooms: HashSet::new(),
            current_path: Vec::new(),
            path_index: 0,
            move_cooldown: 0,
        }
    }
    
    /// Update explorer state and position (called each frame)
    pub fn update(&mut self, dungeon: &[Room], pathfinder: &Pathfinder, rng: &mut SeededRng) {
        // Handle pausing state
        if let ExplorerState::Pausing { ticks_remaining } = self.state {
            if ticks_remaining > 1 {
                self.state = ExplorerState::Pausing { ticks_remaining: ticks_remaining - 1 };
                return;
            } else {
                // Pause complete, transition to exploring or wandering
                if self.all_exits_explored(dungeon) {
                    self.state = ExplorerState::Wandering;
                } else {
                    self.state = ExplorerState::Exploring;
                }
            }
        }
        
        // Check if we entered a new room
        if let Some(room_id) = self.current_room(dungeon) {
            if !self.visited_rooms.contains(&room_id) {
                self.visited_rooms.insert(room_id);
                // Pause for 1-3 seconds (10-30 ticks at 10 FPS)
                let pause_ticks = rng.range(10, 31) as u32;
                self.state = ExplorerState::Pausing { ticks_remaining: pause_ticks };
                self.current_path.clear();
                return;
            }
        }
        
        // Movement cooldown for smooth animation (move every 3 ticks = ~3 tiles/sec)
        if self.move_cooldown > 0 {
            self.move_cooldown -= 1;
            return;
        }
        
        // Follow current path or create new one
        if self.path_index < self.current_path.len() {
            let next_pos = self.current_path[self.path_index];
            self.x = next_pos.0;
            self.y = next_pos.1;
            self.path_index += 1;
            self.move_cooldown = 2; // Move every 3 ticks (including this one)
        } else {
            // Need new path
            self.find_new_path(dungeon, pathfinder, rng);
        }
    }
    
    /// Find a new path based on current state
    fn find_new_path(&mut self, dungeon: &[Room], pathfinder: &Pathfinder, rng: &mut SeededRng) {
        let target = match self.state {
            ExplorerState::Exploring => {
                // Find nearest unexplored exit (with connected_room_id = None)
                self.find_nearest_unexplored_exit(dungeon, pathfinder)
            }
            ExplorerState::Wandering => {
                // Pick random room center
                if !dungeon.is_empty() {
                    let room_idx = rng.range(0, dungeon.len());
                    let room = &dungeon[room_idx];
                    Some((room.x + room.width / 2, room.y + room.height / 2))
                } else {
                    None
                }
            }
            ExplorerState::Pausing { .. } => None,
        };
        
        if let Some(target_pos) = target {
            if let Some(path) = pathfinder.find_path((self.x, self.y), target_pos) {
                self.current_path = path;
                self.path_index = 0;
            }
        }
    }
    
    /// Find the nearest unexplored exit (exits with connected_room_id = None)
    fn find_nearest_unexplored_exit(&self, dungeon: &[Room], pathfinder: &Pathfinder) -> Option<(u32, u32)> {
        let mut nearest: Option<(u32, u32, usize)> = None; // (x, y, distance)
        
        for room in dungeon {
            for exit in &room.exits {
                // Only target exits that haven't been explored yet
                if exit.connected_room_id.is_none() {
                    let exit_pos = self.calculate_exit_position(room, exit);
                    if let Some(path) = pathfinder.find_path((self.x, self.y), exit_pos) {
                        let distance = path.len();
                        if nearest.is_none() || distance < nearest.unwrap().2 {
                            nearest = Some((exit_pos.0, exit_pos.1, distance));
                        }
                    }
                }
            }
        }
        
        nearest.map(|(x, y, _)| (x, y))
    }
    
    /// Calculate the position of an exit in dungeon space
    fn calculate_exit_position(&self, room: &Room, exit: &crate::dungeon::room::Exit) -> (u32, u32) {
        match exit.wall {
            Wall::North => (room.x + exit.position, room.y),
            Wall::South => (room.x + exit.position, room.y + room.height - 1),
            Wall::West => (room.x, room.y + exit.position),
            Wall::East => (room.x + room.width - 1, room.y + exit.position),
        }
    }
    
    /// Check if explorer is currently in a room
    fn current_room(&self, dungeon: &[Room]) -> Option<usize> {
        for room in dungeon {
            if self.x >= room.x && self.x < room.x + room.width &&
               self.y >= room.y && self.y < room.y + room.height {
                return Some(room.id);
            }
        }
        None
    }
    
    /// Check if all exits in the dungeon have been explored
    fn all_exits_explored(&self, dungeon: &[Room]) -> bool {
        for room in dungeon {
            for exit in &room.exits {
                if exit.connected_room_id.is_none() {
                    return false; // Found an unexplored exit
                }
            }
        }
        true // All exits are connected
    }
    
    pub fn position(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::room::{RoomType, Exit};
    
    fn create_test_dungeon() -> Vec<Room> {
        vec![
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
            Room {
                id: 1,
                x: 5,
                y: 5,
                width: 10,
                height: 10,
                room_type: RoomType::Normal,
                exits: vec![],
                parent_id: None,
                parent_wall: None,
            },
            Room {
                id: 2,
                x: 10,
                y: 10,
                width: 5,
                height: 5,
                room_type: RoomType::Small,
                exits: vec![],
                parent_id: None,
                parent_wall: None,
            },
        ]
    }
    
    #[test]
    fn test_explorer_starts_exploring() {
        let explorer = Explorer::new(5, 5);
        assert_eq!(explorer.state, ExplorerState::Exploring);
        assert_eq!(explorer.position(), (5, 5));
    }
    
    #[test]
    fn test_explorer_discovers_room() {
        let dungeon = create_test_dungeon();
        let pathfinder = Pathfinder::new(&dungeon);
        let mut rng = SeededRng::new(42);
        let mut explorer = Explorer::new(5, 5);
        
        // First update should discover room 0
        explorer.update(&dungeon, &pathfinder, &mut rng);
        
        // Should be pausing after discovering a room
        match explorer.state {
            ExplorerState::Pausing { ticks_remaining } => {
                assert!(ticks_remaining >= 10 && ticks_remaining <= 30);
            }
            _ => panic!("Expected pausing state after room discovery"),
        }
    }
    
    #[test]
    fn test_explorer_transitions_to_wandering() {
        let dungeon = vec![
            Room {
                id: 0,
                x: 0,
                y: 0,
                width: 5,
                height: 5,
                room_type: RoomType::Entrance,
                exits: vec![],
                parent_id: None,
                parent_wall: None,
            },
        ];
        
        let pathfinder = Pathfinder::new(&dungeon);
        let mut rng = SeededRng::new(42);
        let mut explorer = Explorer::new(2, 2);
        
        // Visit the only room
        explorer.update(&dungeon, &pathfinder, &mut rng);
        
        // Wait out the pause
        for _ in 0..31 {
            explorer.update(&dungeon, &pathfinder, &mut rng);
        }
        
        // Should transition to wandering since all rooms visited
        assert_eq!(explorer.state, ExplorerState::Wandering);
    }
}

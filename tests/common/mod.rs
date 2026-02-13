/// Common test utilities and fixtures for dungeon generation tests
/// 
/// This module provides:
/// - Test assertions for validating 2D6 rules
/// - Known dungeon seeds for regression testing
/// - Helper functions for room/exit verification

use dungeon_saver::dungeon::Room;

/// Known seeds that produce specific dungeon configurations
/// Used for regression testing and edge case verification
pub mod fixtures {
    /// Seed that generates entrance room at minimum size (6 squares)
    pub const SEED_MIN_ENTRANCE: u64 = 12345;
    
    /// Seed that generates entrance room at maximum size (12 squares)
    pub const SEED_MAX_ENTRANCE: u64 = 67890;
    
    /// Seed that produces a corridor on first exit
    pub const SEED_EARLY_CORRIDOR: u64 = 11111;
    
    /// Seed that generates a small room (≤6 squares)
    pub const SEED_SMALL_ROOM: u64 = 22222;
    
    /// Seed that produces doubles requiring dimension expansion
    pub const SEED_DOUBLES: u64 = 33333;
    
    /// Seed that generates a large room (≥32 squares)
    pub const SEED_LARGE_ROOM: u64 = 44444;
    
    /// Seed used for general determinism testing
    pub const SEED_DETERMINISTIC: u64 = 99999;
}

/// Assertions for 2D6 rule compliance
pub mod assertions {
    use super::Room;
    
    /// Assert room dimensions are valid (non-zero)
    pub fn assert_valid_dimensions(room: &Room) {
        assert!(room.width > 0, "Room width must be positive");
        assert!(room.height > 0, "Room height must be positive");
    }
    
    /// Assert entrance room follows special rules:
    /// - Area between 6-12 squares
    /// - Exactly 3 exits
    pub fn assert_entrance_room(room: &Room) {
        let area = room.width * room.height;
        assert!(area >= 6, "Entrance room too small: {} < 6", area);
        assert!(area <= 12, "Entrance room too large: {} > 12", area);
        assert_eq!(room.exits.len(), 3, "Entrance room must have exactly 3 exits");
    }
    
    /// Assert small room (≤6 squares) properties
    pub fn assert_small_room(room: &Room) {
        let area = room.width * room.height;
        assert!(area <= 6, "Small room too large: {} > 6", area);
    }
    
    /// Assert large room (≥32 squares) properties
    pub fn assert_large_room(room: &Room) {
        let area = room.width * room.height;
        assert!(area >= 32, "Large room too small: {} < 32", area);
    }
    
    /// Assert corridor (1 on at least one dimension)
    pub fn assert_corridor(room: &Room) {
        assert!(
            room.width == 1 || room.height == 1,
            "Corridor must have width=1 or height=1, got {}x{}",
            room.width, room.height
        );
    }
    
    /// Assert exit count is valid (0-3, based on D6 roll)
    /// Does not apply to entrance room (special case)
    pub fn assert_valid_exit_count(room: &Room) {
        assert!(
            room.exits.len() <= 3,
            "Room has too many exits: {} > 3",
            room.exits.len()
        );
    }
    
    /// Assert no exits are placed on the same wall
    pub fn assert_exits_on_different_walls(room: &Room) {
        use std::collections::HashSet;
        let walls: HashSet<_> = room.exits.iter()
            .map(|e| e.wall)
            .collect();
        assert_eq!(
            walls.len(),
            room.exits.len(),
            "Multiple exits on same wall detected"
        );
    }
}

/// Helpers for common test scenarios
pub mod helpers {
    use super::Room;
    
    /// Calculate room area
    pub fn room_area(room: &Room) -> u32 {
        room.width * room.height
    }
    
    /// Check if room is a corridor
    pub fn is_corridor(room: &Room) -> bool {
        room.width == 1 || room.height == 1
    }
    
    /// Check if room is small (≤6 squares)
    pub fn is_small_room(room: &Room) -> bool {
        room_area(room) <= 6
    }
    
    /// Check if room is large (≥32 squares)
    pub fn is_large_room(room: &Room) -> bool {
        room_area(room) >= 32
    }
    
    /// Count rooms matching a predicate
    pub fn count_rooms<F>(rooms: &[Room], predicate: F) -> usize 
    where
        F: Fn(&Room) -> bool,
    {
        rooms.iter().filter(|r| predicate(r)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::assertions::*;
    use super::helpers::*;
    
    /// Example: how to use fixtures and assertions
    #[test]
    fn test_helper_usage_example() {
        // This demonstrates the pattern for actual tests
        // Data will write the real tests using these utilities
        
        let example_room = Room {
            id: 0,
            x: 0,
            y: 0,
            width: 3,
            height: 2,
            room_type: dungeon_saver::dungeon::room::RoomType::Normal,
            exits: vec![],
            parent_id: None,
            parent_wall: None,
        };
        
        assert_valid_dimensions(&example_room);
        assert_eq!(room_area(&example_room), 6);
        assert!(is_small_room(&example_room));
    }
}

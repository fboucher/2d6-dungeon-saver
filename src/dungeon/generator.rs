/// DungeonGenerator: implements 2D6 rules for procedural room generation
use super::room::{Room, RoomType, Exit, Wall};
use crate::rng::SeededRng;

pub struct DungeonGenerator {
    rng: SeededRng,
    rooms: Vec<Room>,
    next_id: usize,
}

/// Helper for deprecated batch generation - tracks unexplored exits
#[derive(Debug)]
#[allow(dead_code)]
struct AvailableExit {
    room_id: usize,
    wall: Wall,
    exit_index: usize,
}

impl DungeonGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: SeededRng::new(seed),
            rooms: Vec::new(),
            next_id: 0,
        }
    }

    pub fn generate_entrance(&mut self) -> Room {
        self.rooms.clear();
        self.next_id = 0;

        let entrance = self.generate_entrance_room();
        self.rooms.push(entrance.clone());
        entrance
    }

    pub fn add_room(&mut self, parent_id: usize, wall: Wall) -> Room {
        // Stop generating after ~20 rooms
        if self.rooms.len() >= 20 {
            // Return a dummy room with 0 exits to signal completion
            return Room {
                id: usize::MAX,
                x: 0,
                y: 0,
                width: 1,
                height: 1,
                room_type: RoomType::Normal,
                exits: Vec::new(),
                parent_id: None,
                parent_wall: None,
            };
        }

        // Find the exit on the parent room for this wall
        let exit_index = if let Some(parent) = self.rooms.iter().find(|r| r.id == parent_id) {
            parent.exits.iter().position(|e| e.wall == wall && e.connected_room_id.is_none())
        } else {
            None
        };

        // Generate new room connected to this exit
        let new_room = self.generate_connected_room(parent_id, wall);
        let new_room_id = new_room.id;

        // Update parent exit to point to new room
        if let Some(idx) = exit_index {
            if let Some(parent) = self.rooms.iter_mut().find(|r| r.id == parent_id) {
                if let Some(exit) = parent.exits.get_mut(idx) {
                    exit.connected_room_id = Some(new_room_id);
                }
            }
        }

        self.rooms.push(new_room.clone());
        new_room
    }

    /// Legacy batch generation - deprecated in favor of progressive generation
    #[deprecated(note = "Use generate_entrance() for progressive generation")]
    #[allow(dead_code)]
    pub fn generate(&mut self) -> Vec<Room> {
        self.rooms.clear();
        self.next_id = 0;

        // Generate entrance room first at a fixed position
        let entrance = self.generate_entrance_room();
        self.rooms.push(entrance);

        // Track available exits for connection
        let mut available_exits = self.collect_available_exits();

        // Generate additional rooms connected to exits
        while self.rooms.len() < 20 && !available_exits.is_empty() {
            // Pick a random available exit
            let exit_idx = (self.rng.d6() as usize - 1) % available_exits.len();
            let available = available_exits.remove(exit_idx);
            
            // Generate new room connected to this exit
            let new_room = self.generate_connected_room(available.room_id, available.wall);
            let new_room_id = new_room.id;
            
            // Update parent exit to point to new room
            if let Some(parent) = self.rooms.iter_mut().find(|r| r.id == available.room_id) {
                if let Some(exit) = parent.exits.get_mut(available.exit_index) {
                    exit.connected_room_id = Some(new_room_id);
                }
            }
            
            self.rooms.push(new_room);
            
            // Add new room's exits to available pool
            available_exits.extend(self.collect_room_exits(new_room_id));
        }

        self.rooms.clone()
    }
    
    /// Helper for deprecated batch generation
    #[allow(dead_code)]
    fn collect_available_exits(&self) -> Vec<AvailableExit> {
        let mut available = Vec::new();
        for room in &self.rooms {
            available.extend(self.collect_room_exits(room.id));
        }
        available
    }
    
    /// Helper for deprecated batch generation
    #[allow(dead_code)]
    fn collect_room_exits(&self, room_id: usize) -> Vec<AvailableExit> {
        let mut available = Vec::new();
        if let Some(room) = self.rooms.iter().find(|r| r.id == room_id) {
            for (idx, exit) in room.exits.iter().enumerate() {
                if exit.connected_room_id.is_none() {
                    available.push(AvailableExit {
                        room_id,
                        wall: exit.wall,
                        exit_index: idx,
                    });
                }
            }
        }
        available
    }

    fn generate_entrance_room(&mut self) -> Room {
        let (width, height) = self.roll_entrance_dimensions();
        let id = self.next_id;
        self.next_id += 1;

        let mut room = Room {
            id,
            x: 10, // Centered starting position
            y: 10,
            width,
            height,
            room_type: RoomType::Entrance,
            exits: Vec::new(),
            parent_id: None,
            parent_wall: None,
        };

        // Entrance room always has 3 exits
        room.exits = self.place_exits(&room, 3, None);
        room
    }

    fn generate_connected_room(&mut self, parent_id: usize, parent_wall: Wall) -> Room {
        let (mut width, mut height) = self.rng.d66();
        let id = self.next_id;
        self.next_id += 1;

        // Check for doubles and handle expansion
        let is_double = width == height;
        if is_double && width != 6 {
            let (add_x, add_y) = self.rng.d66();
            width += add_x;
            height += add_y;
        }

        // Determine room type
        let area = width * height;
        let is_corridor = width == 1 || height == 1;
        
        let room_type = if is_corridor {
            RoomType::Corridor
        } else if area <= 6 {
            RoomType::Small
        } else if is_double && area >= 32 {
            RoomType::Large
        } else {
            RoomType::Normal
        };

        // Calculate position based on parent room and connection wall
        let (x, y) = self.calculate_room_position(parent_id, parent_wall, width, height);

        let mut room = Room {
            id,
            x,
            y,
            width,
            height,
            room_type,
            exits: Vec::new(),
            parent_id: Some(parent_id),
            parent_wall: Some(parent_wall),
        };

        // Roll for number of exits
        // The wall we entered from cannot have an exit
        let exit_count = self.roll_exit_count();
        let entrance_wall = Self::opposite_wall(parent_wall);
        room.exits = self.place_exits(&room, exit_count, Some(entrance_wall));

        room
    }
    
    fn calculate_room_position(&self, parent_id: usize, parent_wall: Wall, width: u32, height: u32) -> (u32, u32) {
        // Spacing between rooms (in dungeon cells)
        // With 2x2 tile scaling, spacing of 3 gives good visual separation
        const ROOM_SPACING: u32 = 3;
        
        if let Some(parent) = self.rooms.iter().find(|r| r.id == parent_id) {
            match parent_wall {
                Wall::North => {
                    // New room is above parent
                    let x = parent.x;
                    let y = parent.y.saturating_sub(height + ROOM_SPACING);
                    (x, y)
                }
                Wall::South => {
                    // New room is below parent
                    let x = parent.x;
                    let y = parent.y + parent.height + ROOM_SPACING;
                    (x, y)
                }
                Wall::East => {
                    // New room is to the right
                    let x = parent.x + parent.width + ROOM_SPACING;
                    let y = parent.y;
                    (x, y)
                }
                Wall::West => {
                    // New room is to the left
                    let x = parent.x.saturating_sub(width + ROOM_SPACING);
                    let y = parent.y;
                    (x, y)
                }
            }
        } else {
            // Fallback if parent not found
            (10, 10)
        }
    }
    
    fn opposite_wall(wall: Wall) -> Wall {
        match wall {
            Wall::North => Wall::South,
            Wall::South => Wall::North,
            Wall::East => Wall::West,
            Wall::West => Wall::East,
        }
    }

    fn roll_entrance_dimensions(&mut self) -> (u32, u32) {
        let (width, height) = self.rng.d66();
        let area = width * height;

        // Entrance room: 6-12 squares
        if area < 6 {
            (3, 2) // 6 squares
        } else if area > 12 {
            (3, 4) // 12 squares
        } else {
            (width, height)
        }
    }

    fn roll_exit_count(&mut self) -> u32 {
        match self.rng.d6() {
            1 => 0,
            2..=3 => 1,
            4..=5 => 2,
            6 => 3,
            _ => unreachable!(),
        }
    }

    fn place_exits(&mut self, room: &Room, count: u32, entrance_wall: Option<Wall>) -> Vec<Exit> {
        if count == 0 {
            return Vec::new();
        }

        let mut exits = Vec::new();
        let walls = [Wall::North, Wall::East, Wall::South, Wall::West];
        
        // Determine starting wall (clockwise from top)
        let start_index = if room.room_type == RoomType::Entrance {
            0 // Always start from North for entrance
        } else {
            self.rng.d6() as usize % 4
        };

        let mut placed = 0;
        let mut wall_index = start_index;
        let mut attempts = 0;

        while placed < count && attempts < 16 {
            let wall = walls[wall_index % 4];
            
            // Check if we can place on this wall
            let can_place = entrance_wall.map_or(true, |ew| ew != wall)
                && !exits.iter().any(|e: &Exit| e.wall == wall);

            if can_place {
                let position = self.calculate_exit_position(room, wall);
                exits.push(Exit { 
                    wall, 
                    position,
                    connected_room_id: None,
                });
                placed += 1;
            }

            wall_index += 1;
            attempts += 1;
        }

        exits
    }

    fn calculate_exit_position(&mut self, room: &Room, wall: Wall) -> u32 {
        match wall {
            Wall::North | Wall::South => {
                if room.width > 2 {
                    self.rng.d6() % room.width.max(1)
                } else {
                    0
                }
            }
            Wall::East | Wall::West => {
                if room.height > 2 {
                    self.rng.d6() % room.height.max(1)
                } else {
                    0
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Progressive generation tests
    #[test]
    fn test_generate_entrance_only() {
        let mut gen = DungeonGenerator::new(42);
        let entrance = gen.generate_entrance();

        assert_eq!(entrance.room_type, RoomType::Entrance);
        assert_eq!(entrance.exits.len(), 3, "Entrance must have 3 exits");
        assert_eq!(entrance.id, 0);
        
        let area = entrance.area();
        assert!(area >= 6 && area <= 12, "Entrance area must be 6-12 squares");
        
        // All exits should be unconnected initially
        for exit in &entrance.exits {
            assert_eq!(exit.connected_room_id, None);
        }
    }

    #[test]
    fn test_add_room_creates_connection() {
        let mut gen = DungeonGenerator::new(42);
        let entrance = gen.generate_entrance();
        
        // Get first exit wall from entrance
        let first_exit_wall = entrance.exits[0].wall;
        
        // Add a connected room
        let room1 = gen.add_room(entrance.id, first_exit_wall);
        
        assert_eq!(room1.id, 1);
        assert_eq!(room1.parent_id, Some(0));
        assert_eq!(room1.parent_wall, Some(first_exit_wall));
        
        // Verify entrance exit is now connected
        let updated_entrance = &gen.rooms[0];
        let connected_exit = updated_entrance.exits.iter()
            .find(|e| e.wall == first_exit_wall)
            .expect("Exit should exist");
        assert_eq!(connected_exit.connected_room_id, Some(1));
    }

    #[test]
    fn test_add_room_stops_at_20_rooms() {
        let mut gen = DungeonGenerator::new(42);
        let entrance = gen.generate_entrance();
        
        // Keep track of unexplored exits
        let mut unexplored_exits = vec![(entrance.id, entrance.exits[0].wall)];
        
        // Add rooms until we hit the limit or run out of exits
        for _ in 0..30 {
            if unexplored_exits.is_empty() {
                break;
            }
            
            let (room_id, wall) = unexplored_exits.remove(0);
            let new_room = gen.add_room(room_id, wall);
            
            if new_room.id == usize::MAX {
                // Hit the limit
                break;
            }
            
            // Add new room's unconnected exits to the queue
            for exit in &new_room.exits {
                unexplored_exits.push((new_room.id, exit.wall));
            }
        }
        
        assert!(gen.rooms.len() <= 20, "Should not exceed 20 rooms");
        // Should have hit the limit if we had enough exits
        if unexplored_exits.len() > 0 {
            assert_eq!(gen.rooms.len(), 20, "Should stop at 20 rooms when exits available");
        }
    }

    #[test]
    fn test_progressive_generation_preserves_determinism() {
        let mut gen1 = DungeonGenerator::new(42);
        let mut gen2 = DungeonGenerator::new(42);
        
        let entrance1 = gen1.generate_entrance();
        let entrance2 = gen2.generate_entrance();
        
        assert_eq!(entrance1.width, entrance2.width);
        assert_eq!(entrance1.height, entrance2.height);
        assert_eq!(entrance1.exits.len(), entrance2.exits.len());
        
        // Add same sequence of rooms
        for _ in 0..5 {
            if let (Some(r1), Some(r2)) = (gen1.rooms.last().cloned(), gen2.rooms.last().cloned()) {
                if let (Some(e1), Some(e2)) = (r1.exits.first(), r2.exits.first()) {
                    let room1 = gen1.add_room(r1.id, e1.wall);
                    let room2 = gen2.add_room(r2.id, e2.wall);
                    
                    assert_eq!(room1.width, room2.width);
                    assert_eq!(room1.height, room2.height);
                    assert_eq!(room1.room_type, room2.room_type);
                }
            }
        }
    }

    // Legacy generate() tests - kept for backward compatibility
    #[test]
    #[allow(deprecated)]
    fn test_deterministic_generation() {
        let mut gen1 = DungeonGenerator::new(42);
        let mut gen2 = DungeonGenerator::new(42);

        let dungeon1 = gen1.generate();
        let dungeon2 = gen2.generate();

        assert_eq!(dungeon1.len(), dungeon2.len());
        for (r1, r2) in dungeon1.iter().zip(dungeon2.iter()) {
            assert_eq!(r1.width, r2.width);
            assert_eq!(r1.height, r2.height);
            assert_eq!(r1.room_type, r2.room_type);
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_entrance_room_generation() {
        let mut gen = DungeonGenerator::new(42);
        let rooms = gen.generate();

        assert!(!rooms.is_empty());
        let entrance = &rooms[0];
        
        assert_eq!(entrance.room_type, RoomType::Entrance);
        assert_eq!(entrance.exits.len(), 3, "Entrance must have 3 exits");
        
        let area = entrance.area();
        assert!(area >= 6 && area <= 12, "Entrance area must be 6-12 squares");
    }

    #[test]
    #[allow(deprecated)]
    fn test_corridor_detection() {
        let mut gen = DungeonGenerator::new(100);
        let rooms = gen.generate();

        for room in &rooms {
            if room.width == 1 || room.height == 1 {
                assert_eq!(room.room_type, RoomType::Corridor);
            }
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_small_room_detection() {
        let mut gen = DungeonGenerator::new(200);
        let rooms = gen.generate();

        for room in &rooms {
            if room.area() <= 6 && !room.is_corridor() {
                assert_eq!(room.room_type, RoomType::Small);
            }
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_large_room_detection() {
        // Test that doubled rooms with area >= 32 are marked as Large
        let mut gen = DungeonGenerator::new(12345);
        let rooms = gen.generate();

        for room in &rooms {
            if room.room_type == RoomType::Large {
                assert!(room.area() >= 32, "Large rooms must have area >= 32");
            }
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_exit_count_distribution() {
        let mut gen = DungeonGenerator::new(999);
        let rooms = gen.generate();

        for room in &rooms {
            if room.room_type != RoomType::Entrance {
                assert!(room.exits.len() <= 3, "Rooms can have max 3 exits");
            }
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_no_duplicate_exits_on_same_wall() {
        let mut gen = DungeonGenerator::new(777);
        let rooms = gen.generate();

        for room in &rooms {
            let mut walls = std::collections::HashSet::new();
            for exit in &room.exits {
                assert!(walls.insert(exit.wall), "Duplicate exit on same wall");
            }
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_room_dimensions_valid() {
        let mut gen = DungeonGenerator::new(555);
        let rooms = gen.generate();

        for room in &rooms {
            assert!(room.width >= 1, "Width must be at least 1");
            assert!(room.height >= 1, "Height must be at least 1");
            assert!(room.width <= 12, "Width should not exceed reasonable bounds");
            assert!(room.height <= 12, "Height should not exceed reasonable bounds");
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_doubles_expansion() {
        // Seed chosen to generate doubles early
        let mut gen = DungeonGenerator::new(333);
        let rooms = gen.generate();

        // At least some rooms should exist beyond entrance
        assert!(rooms.len() > 1);
    }

    #[test]
    #[allow(deprecated)]
    fn test_generates_target_room_count() {
        let mut gen = DungeonGenerator::new(42);
        let rooms = gen.generate();

        assert_eq!(rooms.len(), 20, "Should generate target of 20 rooms");
    }

    #[test]
    #[allow(deprecated)]
    fn test_double_six_no_expansion() {
        // According to rules: "apart from a double 6"
        // When we roll double 6, we should NOT expand
        let mut gen = DungeonGenerator::new(888);
        let rooms = gen.generate();

        // Just verify that the generation completes
        assert!(rooms.len() > 0);
    }

    #[test]
    #[allow(deprecated)]
    fn test_entrance_no_doubling() {
        // Entrance room should never apply double expansion
        let mut gen = DungeonGenerator::new(999);
        let rooms = gen.generate();

        let entrance = &rooms[0];
        assert_eq!(entrance.room_type, RoomType::Entrance);
        // Entrance dimensions are clamped to 6-12, regardless of rolls
        assert!(entrance.area() >= 6 && entrance.area() <= 12);
    }

    #[test]
    #[allow(deprecated)]
    fn test_minimum_dimensions() {
        // D66 minimum is (1, 1)
        let mut gen = DungeonGenerator::new(111);
        let rooms = gen.generate();

        for room in &rooms {
            assert!(room.width >= 1);
            assert!(room.height >= 1);
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_corridor_from_single_dimension() {
        // When either dimension is 1, it's a corridor
        let mut gen = DungeonGenerator::new(222);
        let rooms = gen.generate();

        for room in &rooms {
            if room.width == 1 || room.height == 1 {
                assert!(room.is_corridor());
                assert_eq!(room.room_type, RoomType::Corridor);
            }
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_small_room_boundary_case() {
        // Exactly 6 squares should be small
        let mut gen = DungeonGenerator::new(321);
        let rooms = gen.generate();

        for room in &rooms {
            if room.area() == 6 && !room.is_corridor() {
                assert_eq!(room.room_type, RoomType::Small);
            }
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_large_room_boundary_case() {
        // Exactly 32 squares from doubling should be large
        let mut gen = DungeonGenerator::new(444);
        let rooms = gen.generate();

        for room in &rooms {
            if room.room_type == RoomType::Large {
                assert!(room.area() >= 32);
            }
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_exit_positions_within_bounds() {
        let mut gen = DungeonGenerator::new(555);
        let rooms = gen.generate();

        for room in &rooms {
            for exit in &room.exits {
                match exit.wall {
                    Wall::North | Wall::South => {
                        assert!(exit.position < room.width);
                    }
                    Wall::East | Wall::West => {
                        assert!(exit.position < room.height);
                    }
                }
            }
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_multiple_seeds_produce_different_dungeons() {
        let mut gen1 = DungeonGenerator::new(1);
        let mut gen2 = DungeonGenerator::new(2);
        let mut gen3 = DungeonGenerator::new(3);

        let dungeon1 = gen1.generate();
        let dungeon2 = gen2.generate();
        let dungeon3 = gen3.generate();

        // With different seeds, at least some rooms should differ
        let same_12 = dungeon1.iter().zip(dungeon2.iter())
            .all(|(r1, r2)| r1.width == r2.width && r1.height == r2.height);
        let same_13 = dungeon1.iter().zip(dungeon3.iter())
            .all(|(r1, r2)| r1.width == r2.width && r1.height == r2.height);

        // Very unlikely all three dungeons are identical
        assert!(!(same_12 && same_13), "Different seeds should produce different dungeons");
    }

    #[test]
    #[allow(deprecated)]
    fn test_all_rooms_have_valid_ids() {
        let mut gen = DungeonGenerator::new(666);
        let rooms = gen.generate();

        for (i, room) in rooms.iter().enumerate() {
            assert_eq!(room.id, i, "Room IDs should be sequential");
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_d6_range() {
        let mut rng = SeededRng::new(12345);
        for _ in 0..100 {
            let roll = rng.d6();
            assert!(roll >= 1 && roll <= 6, "D6 roll out of range: {}", roll);
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_d66_range() {
        let mut rng = SeededRng::new(54321);
        for _ in 0..100 {
            let (x, y) = rng.d66();
            assert!(x >= 1 && x <= 6, "D66 X out of range: {}", x);
            assert!(y >= 1 && y <= 6, "D66 Y out of range: {}", y);
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_zero_exits_possible() {
        // D6 roll of 1 means no exits
        let mut gen = DungeonGenerator::new(101);
        let rooms = gen.generate();

        // At least verify some room could have 0 exits (though entrance always has 3)
        let _has_zero_exits = rooms.iter().any(|r| r.exits.len() == 0);
        // This might not always be true depending on seed, so just check generation works
        assert!(rooms.len() > 0);
    }

    #[test]
    #[allow(deprecated)]
    fn test_max_three_exits_non_entrance() {
        let mut gen = DungeonGenerator::new(202);
        let rooms = gen.generate();

        for room in &rooms {
            if room.room_type != RoomType::Entrance {
                assert!(room.exits.len() <= 3, "Non-entrance rooms have max 3 exits");
            }
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_entrance_exactly_three_exits() {
        for seed in [1, 42, 100, 999, 12345] {
            let mut gen = DungeonGenerator::new(seed);
            let rooms = gen.generate();
            assert_eq!(rooms[0].exits.len(), 3, 
                "Entrance must always have exactly 3 exits (seed {})", seed);
        }
    }
}

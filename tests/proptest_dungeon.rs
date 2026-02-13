/// Property-based tests for dungeon generation
/// Uses proptest to verify invariants hold across many random seeds

mod common;

use proptest::prelude::*;
use dungeon_saver::dungeon::DungeonGenerator;
use common::assertions::*;
use common::helpers::*;

// Property: All rooms in any generated dungeon have valid dimensions
proptest! {
    #[test]
    #[ignore] // Remove when Data implements generate()
    fn prop_all_rooms_have_valid_dimensions(seed in any::<u64>()) {
        let mut gen = DungeonGenerator::new(seed);
        let dungeon = gen.generate();
        
        for room in &dungeon {
            assert_valid_dimensions(room);
        }
    }
}

// Property: All corridors have width=1 or height=1
proptest! {
    #[test]
    #[ignore]
    fn prop_corridors_are_one_square_wide(seed in any::<u64>()) {
        let mut gen = DungeonGenerator::new(seed);
        let dungeon = gen.generate();
        
        for room in dungeon.iter().filter(|r| is_corridor(r)) {
            assert_corridor(room);
        }
    }
}

// Property: First room is always entrance room
proptest! {
    #[test]
    #[ignore]
    fn prop_first_room_is_entrance(seed in any::<u64>()) {
        let mut gen = DungeonGenerator::new(seed);
        let dungeon = gen.generate();
        
        if !dungeon.is_empty() {
            assert_entrance_room(&dungeon[0]);
        }
    }
}

// Property: No room has more than 3 exits (except entrance which has exactly 3)
proptest! {
    #[test]
    #[ignore]
    fn prop_exit_count_valid(seed in any::<u64>()) {
        let mut gen = DungeonGenerator::new(seed);
        let dungeon = gen.generate();
        
        for (i, room) in dungeon.iter().enumerate() {
            if i == 0 {
                // Entrance room: exactly 3
                assert_eq!(room.exits.len(), 3);
            } else {
                // All other rooms: 0-3
                assert_valid_exit_count(room);
            }
        }
    }
}

// Property: Exits on same room are on different walls
proptest! {
    #[test]
    #[ignore]
    fn prop_exits_on_different_walls(seed in any::<u64>()) {
        let mut gen = DungeonGenerator::new(seed);
        let dungeon = gen.generate();
        
        for room in &dungeon {
            if room.exits.len() > 1 {
                assert_exits_on_different_walls(room);
            }
        }
    }
}

// Property: Same seed produces same dungeon
proptest! {
    #[test]
    #[ignore]
    fn prop_seed_determinism(seed in any::<u64>()) {
        let mut gen1 = DungeonGenerator::new(seed);
        let mut gen2 = DungeonGenerator::new(seed);
        
        let d1 = gen1.generate();
        let d2 = gen2.generate();
        
        assert_eq!(d1.len(), d2.len(), "Same seed must produce same room count");
    }
}

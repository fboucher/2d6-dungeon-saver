/// Test Conventions for Dungeon Saver
/// 
/// This document describes testing patterns and utilities for the project.
/// All tests must validate 2D6 rule compliance from 2D6 Rules.md.

// STRUCTURE
// =========
// tests/common/mod.rs       - Shared utilities, fixtures, assertions
// tests/dungeon_generation.rs - Integration tests for dungeon generation
// tests/proptest_dungeon.rs - Property-based tests using proptest
// 
// Unit tests live alongside implementation code in src/ using #[cfg(test)]

// FIXTURES
// ========
// Known seeds that produce specific edge cases. Use these for regression tests.
// See tests/common/mod.rs::fixtures for the full list:
//   - SEED_MIN_ENTRANCE: minimum entrance room size (6 squares)
//   - SEED_MAX_ENTRANCE: maximum entrance room size (12 squares)  
//   - SEED_EARLY_CORRIDOR: generates corridor on first exit
//   - SEED_SMALL_ROOM: produces small room (≤6 squares)
//   - SEED_DOUBLES: triggers double roll dimension expansion
//   - SEED_LARGE_ROOM: generates large room (≥32 squares)
//   - SEED_DETERMINISTIC: general-purpose seed for determinism tests

// ASSERTIONS
// ==========
// Use helper functions from tests/common/mod.rs::assertions:
//   - assert_valid_dimensions(room): width/height > 0
//   - assert_entrance_room(room): 6-12 squares, exactly 3 exits
//   - assert_small_room(room): area ≤6
//   - assert_large_room(room): area ≥32
//   - assert_corridor(room): width=1 OR height=1
//   - assert_valid_exit_count(room): 0-3 exits (not entrance)
//   - assert_exits_on_different_walls(room): no duplicate walls

// HELPERS
// =======
// Use helper functions from tests/common/mod.rs::helpers:
//   - room_area(room): calculate width * height
//   - is_corridor(room): check if room is corridor
//   - is_small_room(room): check if area ≤6
//   - is_large_room(room): check if area ≥32
//   - count_rooms(rooms, predicate): count rooms matching predicate

// EXAMPLE USAGE
// =============
/*
use common::fixtures::*;
use common::assertions::*;
use common::helpers::*;

#[test]
fn test_corridor_generation() {
    let mut gen = DungeonGenerator::new(SEED_EARLY_CORRIDOR);
    let dungeon = gen.generate();
    
    let corridor_count = count_rooms(&dungeon, is_corridor);
    assert!(corridor_count > 0);
    
    for room in dungeon.iter().filter(|r| is_corridor(r)) {
        assert_corridor(room);
    }
}
*/

// PROPERTY-BASED TESTING
// ======================
// Use proptest for invariants that should hold for ANY seed:
//   - All rooms have valid dimensions
//   - First room is always entrance room
//   - Exit counts within valid ranges
//   - No duplicate exits on same wall
//   - Seed determinism (same seed = same dungeon)
// 
// See tests/proptest_dungeon.rs for examples

// TEST NAMING
// ===========
// Unit tests: test_{what_is_tested}
// Integration tests: test_{feature_being_verified}
// Property tests: prop_{invariant_being_checked}

// IGNORED TESTS
// =============
// Tests marked #[ignore] are waiting for implementation.
// Data will remove #[ignore] as features are completed.
// Run ignored tests with: cargo test -- --ignored

// ADDING NEW TESTS
// ================
// 1. For unit tests: add #[cfg(test)] mod in the same file
// 2. For integration tests: add to tests/dungeon_generation.rs
// 3. For property tests: add to tests/proptest_dungeon.rs
// 4. Use existing fixtures and assertions when possible
// 5. Add new fixtures to tests/common/mod.rs::fixtures if needed

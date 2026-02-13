# Progressive Generation Testing Strategy

## Overview

This document describes the testing approach for Phase 6 - Progressive Dungeon Generation. The new architecture generates dungeons room-by-room as the explorer discovers exits, rather than all-at-once at startup.

## Key Changes from Original Architecture

### Before (All-at-Once Generation)
- `DungeonGenerator::generate()` produced complete dungeon (~20 rooms)
- Same seed → identical dungeon (fully deterministic)
- Tests validated seed reproducibility

### After (Progressive Generation)
- `DungeonGenerator::generate_entrance()` creates only entrance room
- `DungeonGenerator::add_room(room_id, wall)` generates connected rooms on-demand
- Same seed + different explorer paths → different dungeons (emergent behavior)
- Tests validate progressive growth and connectivity

## Test Coverage

### 1. Entrance Generation (`progressive_generation.rs`)
**Test:** `test_entrance_generation_only`
- Verifies entrance is created with no connected rooms
- Validates entrance has exits but all are unexplored (`connected_room_id: None`)

### 2. Progressive Room Addition (`progressive_generation.rs`)
**Test:** `test_progressive_room_addition`
- Calls `add_room()` on entrance exit
- Verifies new room has valid dimensions (2D6 rules)
- Checks room ID increments properly

### 3. Emergent Behavior (`progressive_generation.rs`)
**Test:** `test_same_seed_different_dungeons`
- Same seed, explorer goes North vs South
- Validates entrance rooms are identical (same RNG state)
- Confirms dungeon structure emerges from exploration order

### 4. Growth Limit (`progressive_generation.rs`)
**Test:** `test_dungeon_growth_limit`
- Simulates exhaustive exploration of all exits
- Verifies dungeon stops growing around 20 rooms (15-25 range)
- Prevents infinite generation

### 5. Exit Tracking (`progressive_generation.rs`)
**Test:** `test_exit_tracking`
- Validates unexplored exits have `connected_room_id: None`
- After `add_room()`, connection should be established
- Ensures explorer can distinguish explored vs unexplored

### 6. Connectivity (`progressive_generation.rs`)
**Test:** `test_connectivity_no_orphans`
- Generates 10 rooms progressively
- Verifies every room (except entrance) has at least one parent
- Prevents orphaned rooms floating in space

### 7. Pathfinding to Exits (`progressive_generation.rs`)
**Test:** `test_explorer_pathfinding_to_unexplored_exits`
- Validates pathfinder can route to exit positions
- Critical for explorer discovering new rooms

### 8. Visual Growth Simulation (`progressive_generation.rs`)
**Test:** `test_visual_room_by_room_growth_simulation`
- Logs growth: 1 → 2 → 3 → 4 → 5 → 6 rooms
- Simulates user experience (dungeon appears incrementally)

### 9. Infinite Loop Prevention (`progressive_generation.rs`)
**Test:** `test_no_infinite_generation`
- Safety limit of 1000 iterations
- Ensures generation terminates
- Validates room count ≤ 30

### 10. Integration Tests (`integration_test.rs`)
**Updated Tests:**
- `test_full_progressive_generation_and_export`: End-to-end with map export
- `test_progressive_generation_creates_different_dungeons`: Emergent behavior validation
- `test_export_filename_format_no_seed`: No seed in filename (non-reproducible)

### 11. Explorer Integration (`explorer_tests.rs`)
**Updated Tests:**
- `test_explorer_visits_progressively_generated_rooms`: Explorer triggers room generation
- `test_explorer_maintains_valid_position_progressive`: Position validation during growth
- `test_multiple_dungeons_explorer_works`: Cross-seed progressive generation

### 12. Dungeon Generation (`dungeon_generation.rs`)
**Updated Tests:**
- `test_entrance_room_properties`: Uses `generate_entrance()`
- `test_corridor_detection_progressive`: Finds corridors via `add_room()`
- `test_small_room_properties_progressive`: Probabilistic small room generation
- `test_large_room_from_doubles`: Large rooms via progressive generation

## Testing Philosophy

### Determinism → Emergent Behavior
- **Old:** Tests expected identical dungeons from same seed
- **New:** Tests expect different dungeons (structure emerges from choices)
- Seed still controls dice rolls, not dungeon layout

### All-at-Once → Progressive
- **Old:** Tests called `generate()` and validated 20 rooms
- **New:** Tests call `generate_entrance()` then `add_room()` repeatedly
- Simulates real-world explorer-driven generation

### Seed Reproducibility → Growth Constraints
- **Old:** Tests validated `dungeon1 == dungeon2` with same seed
- **New:** Tests validate room count bounds (15-25) and connectivity
- Focus on constraints, not exact structure

## Test Execution Notes

### Compilation Status
All tests are written against the **NEW API** (`generate_entrance()`, `add_room()`).

**Current Status:** Tests will NOT compile until Data implements the new API in Phase 1.

**Expected Timeline:**
1. Data completes Phase 1 refactoring
2. New API methods exist: `generate_entrance()`, `add_room()`
3. Tests compile and run
4. Brand validates test results

### Known Probabilistic Tests
Some tests rely on RNG:
- `test_small_room_properties_progressive`: Might not find small room in 15 tries (low probability)
- `test_large_room_from_doubles`: Large rooms are rare (requires doubles)

These use seed fixtures from `tests/common/fixtures.rs` chosen to maximize success rate.

### Integration with Main Loop
Real integration testing happens when:
1. Chunk integrates progressive generation into `main.rs` (Phase 2)
2. Explorer behavior updated to detect exit crossings (Phase 3)
3. Pathfinder handles dynamic dungeon updates (Phase 4)

Brand's tests validate the **generator API**, not the full integration.

## Future Test Additions

### When Chunk Completes Phase 2-4:
- [ ] Test: Explorer triggers `add_room()` callback on exit crossing
- [ ] Test: Pathfinder rebuilds correctly after new room added
- [ ] Test: Camera tracks explorer through progressively generated rooms
- [ ] Test: No visual glitches when room appears mid-frame

### Performance Tests:
- [ ] Test: `add_room()` completes in <10ms (won't block 10 FPS)
- [ ] Test: Memory usage stays reasonable (20 rooms * ~200 bytes/room)
- [ ] Test: Pathfinder rebuild time <5ms

## References
- **Plan:** `/home/frank/.copilot/session-state/0f62829d-c26a-4fef-9275-5643cc42ef01/plan.md`
- **2D6 Rules:** `docs/2D6 Rules.md`
- **Test Conventions:** `tests/TEST_CONVENTIONS.md`
- **Fixtures:** `tests/common/fixtures.rs`
- **Assertions:** `tests/common/assertions.rs`

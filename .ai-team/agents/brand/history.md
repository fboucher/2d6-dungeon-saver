# History — Brand

## Project Learnings (from import)

**Owner:** fboucher (fboucher@outlook.com)

**Project:** Terminal-based dungeon screensaver with 2D6 procedural generation and AI explorer.

**Test Priorities:**
1. **2D6 Mechanics:** Room dimensions, corridor detection, small/large room classification
2. **Exit Placement:** All constraint rules from 2D6 Rules.md
3. **Boundary Enforcement:** Outer Boundary, room overlap prevention
4. **Pathfinding:** Explorer reaches all reachable rooms
5. **Seed Determinism:** Same seed → same dungeon
6. **Performance:** 10 FPS target, minimal CPU

**Edge Cases to Cover:**
- Entrance room always 6-12 squares, always 3 exits
- Corridors (1 on dimension roll) have all archway exits
- Small rooms (≤6 squares) use different generation rules
- Doubles trigger re-roll and dimension expansion
- Exit placement near boundaries, on walls with existing exits
- Dungeon generation reaching ~20 rooms

**Tech Stack:** TBD — test framework depends on chosen language

---

## Learnings

### Test Infrastructure Architecture (2025-01-20)

Created comprehensive test scaffolding for Phases 2-5:
- **tests/common/mod.rs**: Shared utilities, 8 known seed fixtures, 7 assertion helpers, 5 common helpers
- **tests/dungeon_generation.rs**: Integration test suite (6 tests, 5 ignored pending implementation)
- **tests/proptest_dungeon.rs**: Property-based tests using proptest crate (6 properties)
- **tests/TEST_CONVENTIONS.md**: Documentation of testing patterns and usage examples
- **src/lib.rs**: Added library crate for test compilation

Fixtures cover all major edge cases: min/max entrance rooms, corridors, small rooms, doubles expansion, large rooms, and determinism.

Assertions validate 2D6 rules: room dimensions, entrance special rules, corridor detection, size classifications, exit placement constraints.

Property tests verify invariants across arbitrary seeds: valid dimensions, entrance consistency, exit count limits, wall uniqueness, determinism.

Tests marked #[ignore] until Data implements generate(). Two tests pass immediately (deterministic generation stub, helper usage example).

Fixed compilation issue: Added Hash derive to Wall enum for exit placement validation in generator.

**Key files:**
- tests/common/mod.rs: Test utilities and fixtures
- tests/dungeon_generation.rs: Integration tests
- tests/proptest_dungeon.rs: Property-based tests  
- tests/TEST_CONVENTIONS.md: Testing documentation
- src/lib.rs: Library crate entry point

**Dependencies added:**
- proptest = "1" for property-based testing

**Convention established:** Tests use fixtures by name (SEED_MIN_ENTRANCE), assertions for validation (assert_entrance_room), helpers for queries (is_corridor). Property tests verify invariants hold for any seed.

---

📌 Team update (2026-02-12): Test Infrastructure for Phases 2-5 approved — proptest scaffolding, fixtures, assertions, conventions — decided by Brand
📌 Team update (2026-02-12): Phased Implementation Plan approved — Brand owns test infrastructure across all phases — decided by Mikey
📌 Team update (2026-02-12): Tech Stack finalized (Rust + proptest) — enables comprehensive test coverage for 2D6 rules and pathfinding — decided by Mikey
📌 Team update (2026-02-12): Phase 1 Terminal Event Loop complete — Brand can integrate into test infrastructure — decided by Chunk



---

📌 Team update (2026-02-12): Phases 2 (dungeon generation) and 3 (rendering) acceptance criteria met; 39 tests passing (24 generation + 7 camera + 8 infra) — decided by Data, Mouth

### Progressive Generation Test Suite (2025-01-20)

Created comprehensive test suite for Phase 6 (Progressive Generation) before Data's Phase 1 API refactoring:

**New test file:** `tests/progressive_generation.rs` with 10 test cases:
1. Entrance-only generation validation
2. Progressive room addition via `add_room()`
3. Emergent behavior (same seed → different dungeons based on explorer path)
4. Growth limit enforcement (~20 rooms)
5. Exit tracking (unexplored vs explored states)
6. Connectivity validation (no orphaned rooms)
7. Pathfinding to unexplored exits
8. Visual room-by-room growth simulation
9. Infinite loop prevention
10. Integration with explorer behavior

**Updated existing tests:**
- `tests/integration_test.rs`: Removed seed determinism tests, added progressive generation validation
- `tests/dungeon_generation.rs`: Updated to use `generate_entrance()` + `add_room()` instead of `generate()`
- `tests/explorer_tests.rs`: Added progressive generation simulation in explorer movement tests

**Test strategy:**
- Tests written against NEW API (won't compile until Data implements Phase 1)
- Focus shifted from determinism to emergent behavior and constraints
- Progressive growth: 1 room → 2 → 3 ... → ~20 (vs all-at-once)
- Exit tracking: `connected_room_id: None` (unexplored) vs `Some(id)` (explored)

**Key architectural validations:**
- Dungeon starts with entrance only
- Rooms generated on-demand as exits explored
- Same seed produces different dungeons (emergent structure from explorer choices)
- Every room reachable (no orphans)
- Generation stops at ~20 rooms (prevents infinite loops)

**Documentation:** Created `tests/PROGRESSIVE_GENERATION_TESTS.md` explaining test philosophy, coverage, and execution strategy.

**Files modified:**
- tests/progressive_generation.rs (new, 10 tests)
- tests/integration_test.rs (updated 3 tests)
- tests/dungeon_generation.rs (updated 4 tests)
- tests/explorer_tests.rs (updated 5 tests)
- tests/PROGRESSIVE_GENERATION_TESTS.md (new documentation)

**Dependencies:** Tests compile once Data implements:
- `DungeonGenerator::generate_entrance() -> Room`
- `DungeonGenerator::add_room(room_id: usize, wall: Wall) -> Room`

**Convention established:** Progressive generation tests simulate explorer-driven discovery, validating emergent behavior rather than deterministic structure.

---

### Dead Code Cleanup (2025-01-20)

Eliminated all 5 compiler warnings from deprecated batch generation code:

**Preserved with `#[allow(dead_code)]`:**
- `Room::area()` and `Room::is_corridor()`: Used by test suites, valid utility methods
- `DungeonGenerator::generate()` + helpers (`collect_available_exits`, `collect_room_exits`): Already deprecated, kept for backward compatibility with 56 existing tests
- `AvailableExit` struct: Only used by deprecated batch generation
- `Camera::is_visible()`: Public API method with test coverage, reserved for future visibility culling
- `Theme.corridor` field: Part of theming system, not yet rendered

**Rationale:** Code marked `#[allow(dead_code)]` instead of deleted when:
1. Method is tested and part of public API (e.g., `is_visible()`)
2. Method supports backward-compatible tests (e.g., deprecated `generate()`)
3. Field is part of a coherent design (e.g., `Theme.corridor`)

Build now produces zero warnings. All 45 tests pass.

**Files modified:**
- src/dungeon/room.rs: Added `#[allow(dead_code)]` to `area()` and `is_corridor()`
- src/dungeon/generator.rs: Added `#[allow(dead_code)]` to deprecated batch generation methods and `AvailableExit` struct
- src/renderer/camera.rs: Added `#[allow(dead_code)]` to `is_visible()`
- src/theme.rs: Added `#[allow(dead_code)]` to `corridor` field

**Convention established:** Use `#[allow(dead_code)]` over deletion when code has test coverage, serves backward compatibility, or is intentionally reserved for future features. Add comments explaining the rationale.

---

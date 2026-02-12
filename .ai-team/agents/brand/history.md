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



### 2025-01-20: Progressive Generation Testing Strategy

**By:** Brand

**What:** Created Phase 6 test suite validating progressive dungeon generation where rooms are created on-demand as explorer discovers exits, rather than all-at-once at startup. Tests cover entrance-only initialization, room addition API, emergent behavior, connectivity, and growth limits.

**Why:** Progressive generation fundamentally changes the test approach:

1. **Emergent behavior replaces determinism:** Same seed now produces different dungeons depending on explorer's path choices. Tests validate constraints (room count, connectivity, exit tracking) instead of exact structure matching.

2. **API-first testing:** Wrote tests against new API (`generate_entrance()`, `add_room()`) before Data's implementation. This establishes the contract and catches interface issues early.

3. **Growth simulation:** Tests simulate room-by-room discovery (1 → 2 → 3 ... → 20) to validate the visual user experience matches pen & paper gameplay.

4. **Connectivity guarantees:** Every generated room must be reachable (no orphans floating in space). Progressive generation introduces risk of disconnected rooms if exit tracking fails.

5. **Infinite loop prevention:** Without upfront room count limits, generation could continue forever. Tests enforce ~20 room cap and detect runaway loops.

**Test coverage:** 10 new tests in `progressive_generation.rs`, updates to 12 existing tests across `integration_test.rs`, `dungeon_generation.rs`, and `explorer_tests.rs`. Created `PROGRESSIVE_GENERATION_TESTS.md` documenting strategy.

**Key validations:**
- Entrance starts with unexplored exits (`connected_room_id: None`)
- `add_room()` creates valid rooms following 2D6 rules
- Explorer can pathfind to exit positions
- Generation stops at 15-25 rooms
- No orphaned rooms
- Different explorer paths → different dungeons (emergent)

**Next:** Tests will compile once Data implements new generator API in Phase 1. Brand validates results after Data's refactoring.

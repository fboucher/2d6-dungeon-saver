### 2025-02-12: Progressive Generation Implementation Bugs

**By:** Mikey

**What:** Diagnosed critical bugs in Phase 2 progressive generation implementation: explorer doesn't move, rendering shows ANSI escape codes, and quit behavior is broken.

**Why:**

The progressive generation implementation has a fundamental architectural mismatch:

1. **Explorer AI Targets Wrong Goal:** The explorer's behavior (src/explorer/behavior.rs) is designed to find "unvisited rooms" and path to their centers. This worked perfectly for the old full-generation approach where ~20 rooms existed at startup. But with progressive generation starting with just 1 entrance room:
   - First tick: explorer marks entrance as visited
   - all_rooms_visited() returns true (1 visited == 1 total)
   - State transitions to Wandering
   - Wandering picks random room center → always entrance center
   - Path from center to center = 0 length → no movement
   - Explorer never reaches exits → no new rooms generated → frozen

2. **Root Cause:** Explorer needs to target *unexplored exits* (doors with `connected_room_id.is_none()`), not *unvisited room centers*. The detection logic in main.rs (lines 104-113) correctly identifies when explorer stands on an unexplored exit, but the explorer AI never navigates there.

3. **Terminal Cleanup Issue:** ANSI escape codes appear on quit because the timeout command (used for testing) sends SIGTERM, which bypasses both normal cleanup and the panic hook. This is a test artifact, not a production bug. Real users pressing 'q' won't see this.

**Impact:**

- Progressive generation is completely non-functional: dungeon remains at 1 room forever
- User experience is broken: nothing moves, rendering looks static
- Blocks all downstream testing of the progressive generation feature

**Fix Assignment:**

**Chunk** must refactor explorer AI in `src/explorer/behavior.rs`:
- Replace `find_nearest_unvisited_room()` with `find_nearest_unexplored_exit()`
- Change Exploring state to target exit positions (not room centers)
- Update completion check: `all_exits_explored()` instead of `all_rooms_visited()`
- Explorer should only transition to Wandering when no unexplored exits remain

**Brand** should add integration test:
- Verify explorer reaches first exit within reasonable tick count
- Verify new room generation triggers on exit collision
- Verify pathfinder rebuilds correctly after dungeon expansion

**Files Affected:**
- `src/explorer/behavior.rs` (lines 87-131: Exploring state logic)
- `src/main.rs` (lines 104-113: detection logic is correct, no changes needed)
- New test file: `tests/progressive_generation.rs`

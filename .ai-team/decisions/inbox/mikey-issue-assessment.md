### 2025-02-12: Issue Assessment and Priority Plan

**By:** Mikey

**Context:** User reports "many errors" and UI issues after progressive generation implementation. Investigation reveals these are compilation warnings (not errors) and a terminal rendering artifact.

## Actual State

**Build Status:** ✅ SUCCESS
- No compilation errors
- 5 compiler warnings (dead_code)
- Application runs and works functionally

**Runtime State:** ⚠️ FUNCTIONAL BUT VISUAL ISSUES
- Explorer moves correctly, targets unexplored exits
- Progressive generation works (new rooms generate on exploration)
- Terminal cleanup has ANSI escape code bleed on forced exit (timeout)

## Issue Categories

### P0: Terminal ANSI Bleed (HIGH PRIORITY)
**What:** Raw ANSI codes like `[56;1H` and `;46m` appear in terminal output when timeout kills the process
**Impact:** Breaks terminal state, user sees garbage on screen
**Root Cause:** Terminal cleanup doesn't execute when process is terminated by signal (SIGTERM)
**Owner:** Chunk
**Fix:** Improve signal handling in main.rs - register SIGTERM/SIGINT handlers that call cleanup_terminal() before exit

### P1: Unused Code Warnings (LOW PRIORITY - COSMETIC)
**What:** 5 dead_code warnings for methods/fields not currently used
**Impact:** None (warnings only, no functional issues)
**List:**
1. `Room::area()` - unused method (src/dungeon/room.rs:40)
2. `Room::is_corridor()` - unused method (src/dungeon/room.rs:44)
3. `AvailableExit` struct - unused (src/dungeon/generator.rs:12)
4. `DungeonGenerator::generate()` - old batch generation method (src/dungeon/generator.rs:78)
5. `DungeonGenerator::collect_available_exits()` - unused (src/dungeon/generator.rs:115)
6. `DungeonGenerator::collect_room_exits()` - unused (src/dungeon/generator.rs:123)
7. `Camera::is_visible()` - unused method (src/renderer/camera.rs:79)
8. `Theme::corridor` - unused field (src/theme.rs:7)

**Owner:** Brand (code review)
**Fix:** Either delete dead code OR add `#[allow(dead_code)]` with comment explaining future use

### P2: UI Enhancement - Tile Scale
**What:** User previously requested larger tiles (Enhance-1 from backlog)
**Status:** DEFERRED - blocked by P0 fix first
**Owner:** Chunk (when unblocked)

## Recommendation

**Immediate Actions (This Sprint):**

1. **Chunk: Fix terminal cleanup (P0)** - 1-2 hours
   - Add signal handler for SIGTERM/SIGINT using `ctrlc` crate or manual signal registration
   - Ensure cleanup_terminal() is called before process exit
   - Test with timeout command to verify ANSI bleed is resolved
   - Files: `src/main.rs`, `Cargo.toml` (add signal handling dependency)

2. **Brand: Clean up warnings (P1)** - 30 minutes
   - Review each unused item:
     - `Room::area()`, `Room::is_corridor()` → likely needed for future room logic, add `#[allow(dead_code)]` with TODO
     - `AvailableExit`, old `generate()` methods → delete if truly obsolete
     - `Camera::is_visible()` → either implement viewport culling OR delete
     - `Theme::corridor` → delete if unused, or mark allowed if planned for future
   - Goal: Zero warnings on `cargo build`

**Next Sprint:**

3. **Chunk: Tile scaling enhancement** - User wants tiles more visible
   - Increase TILE_SCALE from 2 to 3 or 4 in canvas.rs
   - OR add CLI flag: `--tile-scale <n>`
   - Test with various terminal sizes
   - Files: `src/renderer/canvas.rs`, possibly `src/main.rs` for CLI

## Why This Order

1. **P0 first:** Broken terminal is a show-stopper for users
2. **P1 quick win:** Cleaning warnings is fast and makes codebase healthier
3. **P2 when stable:** UX polish after core functionality is solid

## Files to Watch

- `src/main.rs` - terminal lifecycle, signal handling
- `src/dungeon/generator.rs` - progressive generation (working, just has old dead code)
- `src/renderer/canvas.rs` - tile scale constant
- `Cargo.toml` - dependencies for signal handling

## Decision

**Accept these priorities:** P0 (terminal cleanup) → P1 (warnings) → P2 (tile scale)
**Start immediately:** Chunk begins P0 work now

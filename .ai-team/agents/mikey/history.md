# History — Mikey

## Project Learnings (from import)

**Owner:** fboucher (fboucher@outlook.com)

**Project:** Terminal-based dungeon explorer screensaver. Procedurally generates dungeons using 2D6 dice mechanics (adapted from pen & paper game). Watches an AI explorer navigate through rooms using pathfinding. Top-down ASCII/Unicode view with Catppuccin Mocha colors. Real-time animation at 10 FPS. Linux terminal application.

**Key Documents:**
- `Rules.md` — high-level features: ~20 rooms per dungeon, color theming, map export, explorer behavior, screen panning
- `2D6 Rules.md` — detailed dungeon generation mechanics: room dimensions (D66 roll), corridors (1 on dimension roll), small rooms (≤6 squares), large rooms (≥32 squares), exit placement rules, Outer Boundary constraints

**Tech Stack:** TBD — likely Python, Rust, or Go for terminal apps

**Core Requirements:**
- Full terminal takeover (screensaver-style)
- Seed-based generation (same seed → same dungeon)
- Terminal responsive (adapts to terminal size)
- Explorer stays roughly centered (pan when explorer reaches last quarter of screen)
- Dungeons are single-level (ignore multi-level rules from 2D6 Rules.md)
- Ignore "what's in the room" content for now — just draw rooms
- Map export on exit to `maps/yyyy-MM-dd_HHmm_seed<seed>.txt`

---

## Learnings

### Post-Launch Enhancement Backlog (2025-02-12)

**Structured three user-requested enhancements:**

1. **Enhance-1: Tile Scaling (P1, Medium)** — User reports tiles too small to see. Camera already supports `tile_scale`; increase from 2×2 to 3×3 or 4×4, or add CLI flag. Chunk owns implementation.

2. **Enhance-2: Fog of War (P1, Large)** — Core gameplay feature: reveal dungeon as explorer visits rooms. Requires Data to track visited rooms, Chunk to filter rendering. This completes the "procedural discovery" loop mentioned in Rules.md.

3. **Enhance-3: Compiler Warnings Cleanup (P2, Small)** — Three unused methods (`Room::area`, `Room::is_corridor`, `Camera::is_visible`) and one unread field (`Theme::corridor`). Brand reviews each; delete or add `#[allow(dead_code)]` if intentional for future phases.

**Recommended order:** Enhance-2 (fog of war) first for gameplay completeness, then Enhance-1 (tile scaling) as UX polish, then Enhance-3 (warnings) as background maintenance.

**File:** `.ai-team/decisions/inbox/mikey-backlog-enhancements.md`

---

### Tech Stack Decision (2025-01-20)

**Chosen:** Rust + Ratatui

**Rationale:**
- Ratatui is the modern successor to tui-rs, actively maintained with excellent terminal abstraction
- Rust's `rand` crate with `rand_chacha` provides cryptographically-strong seed-based RNG (same seed = exact same dungeon)
- Zero-cost abstractions mean 10 FPS animation with negligible CPU usage — critical for a screensaver
- `crossterm` backend handles terminal takeover, raw mode, alternate screen, and cleanup on panic
- True color support out-of-box (Catppuccin Mocha requires 24-bit color)
- Single binary distribution — no runtime dependencies for users
- `pathfinding` crate provides A* and BFS implementations

**Key Crates:**
- `ratatui` — TUI framework
- `crossterm` — Terminal backend (cross-platform, but we only need Linux)
- `rand` + `rand_chacha` — Deterministic seed-based RNG
- `pathfinding` — A* for explorer movement
- `chrono` — Timestamp for map export filename

**Alternatives Rejected:**
- Python + curses: GIL limits animation smoothness, curses API is dated
- Go + tcell: Good option, but pathfinding ecosystem weaker than Rust
- Python + blessed: Same Python issues, plus blessed is less actively maintained

**Project Structure Pattern:**
```
src/
  main.rs          — entry point, terminal setup, main loop
  dungeon/         — room generation, 2D6 mechanics
  explorer/        — pathfinding, AI behavior
  renderer/        — ratatui widgets, drawing
  theme/           — Catppuccin colors
  rng.rs           — seeded ChaCha8Rng wrapper
```

### Project Setup (2025-01-20)

**Created:**
- `Cargo.toml` with all dependencies pinned, optimized profiles for dev/release
- `src/` module structure: dungeon/, explorer/, renderer/, with stubs for all key types
- `.gitignore` for Rust projects + generated maps/
- `README.md` with build/run instructions and project overview
- Type definitions: Room, Exit, Direction, Explorer, ExplorerState, Theme, Camera, Canvas, Pathfinder

**Key Files & Purposes:**
- `.ai-team/decisions/inbox/mikey-phased-plan.md` — 5-phase roadmap (Phase 1: terminal, Phase 2: generation, Phase 3: rendering, Phase 4: AI, Phase 5: polish)
- `src/rng.rs` — SeededRng wrapper (ChaCha8Rng) for deterministic generation
- `src/dungeon/generator.rs` — DungeonGenerator (2D6 implementation target)
- `src/dungeon/room.rs` — Room, Exit, Direction types
- `src/explorer/behavior.rs` — Explorer state machine (Exploring/Wandering/Pausing)
- `src/explorer/pathfinder.rs` — A* wrapper
- `src/renderer/camera.rs` — Viewport panning logic
- `src/renderer/canvas.rs` — Ratatui rendering
- `src/theme.rs` — Catppuccin Mocha color palette

**Module Boundaries (per tech stack decision):**
- Dungeon generation isolated in `dungeon/` — can be tested independently
- Explorer behavior + pathfinding in `explorer/` — depends on dungeon, independent of rendering
- Rendering in `renderer/` — consumes dungeon + explorer state, outputs to terminal
- Theme in `theme.rs` — centralized color management for easy future themes

**Builds:** `cargo check` succeeds with 24 warnings (unused stubs — expected)

---

📌 Team update (2026-02-12): Phased Implementation Plan approved — 5 phases with clear ownership and handoff strategy — decided by Mikey
📌 Team update (2026-02-12): Tech Stack finalized (Rust + Ratatui + crossterm + rand_chacha + pathfinding) — decided by Mikey
📌 Team update (2026-02-12): Phase 1 Terminal Event Loop Implementation complete — detailed rationale on panic hooks, non-blocking polling, frame timing — decided by Chunk
📌 Team update (2026-02-12): Test Infrastructure for Phases 2-5 scaffolding in place (proptest, fixtures, assertions, conventions) — decided by Brand

### Progressive Generation Architecture (2025-02-12)

**Current Issue:** Dungeon is generated entirely at startup (main.rs:74-76 calls `DungeonGenerator::generate()` which creates all ~20 rooms upfront). This violates 2D6 pen & paper rules: "you generate dungeon spaces as you encounter them."

**User Requirements Clarification:**
- **Approach:** True progressive generation (rooms don't exist until explorer reaches an unexplored door)
- **Determinism:** Emergent (dungeon structure depends on explorer's door choices, not seed)
- **Seed Purpose:** Only for dice rolling (D6/D66 mechanics), NOT for dungeon layout reproducibility
- **Backlog Impact:** Cancel Enhance-2 (fog of war) - replaced by progressive generation experience

**Architectural Pattern - Progressive Generation:**
```
Startup → Generate entrance room only
Explorer reaches door → Detect exit tile → Generate connected room → Add to dungeon Vec
Pathfinder → Rebuild graph when room added (or use lazy query)
```

**Key Files Requiring Changes:**
- `src/dungeon/generator.rs` — refactor `generate()` to `generate_entrance()` + `add_room(parent_id, wall)`
- `src/main.rs` — keep DungeonGenerator alive in loop, detect door events, trigger room generation
- `src/explorer/behavior.rs` — change targets from "unvisited rooms" to "unexplored exits"
- `src/explorer/pathfinder.rs` — add `add_room()` or make lazy (rebuild graph on new room)
- `Rules.md`, `README.md` — remove seed-based reproducibility claims, remove --seed flag

**Implementation Plan:** `/home/frank/.copilot/session-state/0f62829d-c26a-4fef-9275-5643cc42ef01/plan.md` (6 phases: generator refactor, main loop integration, explorer updates, pathfinder changes, requirements cleanup, testing)

**Decision File:** `.ai-team/decisions/inbox/mikey-progressive-generation.md`

---

### Simple Grid Renderer (2025-02-12)

**Architectural Decision:** Scrap complex tile-scaling renderer (TILE_SCALE=4, multi-char walls, corridor drawing) in favor of 1:1 grid mapping (1 dungeon cell = 1 terminal character).

**Core Design:**
- **Coordinate System:** Dungeon space = Screen space (no conversion math needed)
- **Rooms:** Rectangle perimeter = `#` (walls), interior = `.` (floors)
- **Doors:** Replace wall chars with `|` (vertical) or `-` (horizontal) at exit positions
- **Explorer:** Single `@` character at position
- **Corridors:** Empty space between rooms (no special rendering)

**Camera Simplification:**
- Remove tile_scale field entirely
- Remove all "dungeon space → screen space" conversion logic
- Direct 1:1 coordinate mapping for update(), center_on(), is_visible()

**Rendering Algorithm:**
1. For each room: double loop over width/height, draw walls (`#`) or floors (`.`)
2. For each exit: overwrite wall character with door (`|` or `-`)
3. Draw explorer: single `@` character

**Files Affected:**
- `src/renderer/canvas.rs` — delete TILE_SCALE, get_scaled_wall_char(), render_corridors(); simplify render_room() to ~30 lines
- `src/renderer/camera.rs` — remove tile_scale field and conversion math
- `tests/camera_rendering.rs` — update tests to remove tile_scale expectations

**Trade-offs:**
- ✅ Dead simple mental model (1 cell = 1 char)
- ✅ Easy debugging (coordinates match screen exactly)
- ✅ Less code (~200 lines deleted from canvas.rs)
- ❌ Less visual detail (but Frank specifically requested simple squares/rectangles)

**Design Document:** `.ai-team/decisions/inbox/mikey-simple-grid-renderer.md`

---

### Progressive Generation Implementation Bugs (2025-02-12)

**Critical Issues Found:**

1. **Explorer Doesn't Move:** With only a single entrance room (no connected rooms), the Pathfinder has no valid targets for the explorer to path to. The explorer tries to find "nearest unvisited room" but the entrance is immediately marked as visited on first update, leaving no destinations. The explorer's `find_new_path()` returns None, so no movement occurs.

2. **Rendering ANSI Bleed:** Terminal shows raw ANSI escape codes like `[56;1H` and `;46m` in the output. This happens when the terminal cleanup fails or is interrupted. The timeout/quit scenarios don't properly restore terminal state.

3. **No Exit Detection on Entrance:** The entrance room is generated with 3 exits, but the explorer starts at the room's center (entrance.x + width/2, entrance.y + height/2). Since the room is ~6-12 units wide/tall and explorer starts in the middle, it would take several ticks of movement to reach an exit. However, the explorer has NO path because there are no unvisited rooms to target, so it never moves toward exits.

**Root Cause Analysis:**

The progressive generation architecture assumes the explorer will naturally walk to exits, triggering room generation. But the explorer's AI (in `explorer/behavior.rs`) is hardcoded to:
1. Find nearest **unvisited room** (Exploring state)
2. Pick random **room center** (Wandering state)

With only 1 room (the entrance), after the first tick:
- visited_rooms = {0} (entrance discovered immediately)
- all_rooms_visited() = true (1 visited == 1 total)
- State transitions to Wandering
- Wandering picks random room center → always entrance center
- Pathfinder paths from center to center → 0-length path → no movement

**What Needs Fixing:**

1. **Explorer AI must target unexplored exits, not unvisited rooms** (behavior.rs lines 87-111)
   - Change Exploring state: instead of `find_nearest_unvisited_room()`, implement `find_nearest_unexplored_exit()`
   - Look for exits where `connected_room_id.is_none()`
   - Path to the exit position (not room center)
   - Only transition to Wandering when ALL exits are explored (not just all rooms visited)

2. **Terminal cleanup must be more robust** (main.rs lines 56-60)
   - The cleanup_terminal() function works, but may not execute if the timeout kills the process
   - The panic hook exists, but doesn't help if the process is terminated by signal
   - This is likely a test artifact (timeout command sends SIGTERM), not a real bug

3. **Initial camera position may be wrong** (main.rs line 95)
   - Camera centers on explorer at entrance center, which is correct
   - Not a bug, but rendering looks weird due to lack of connected rooms

**Assignment:**

- **Chunk** must refactor `explorer/behavior.rs` to implement exit-based targeting:
  - Add `find_nearest_unexplored_exit()` method
  - Change Exploring state logic to use exit positions as targets
  - Update `all_rooms_visited()` → `all_exits_explored()` check
  - Ensure explorer transitions to Wandering only when no unexplored exits remain

- **Brand** should add integration test for progressive generation:
  - Verify explorer reaches first exit within N ticks
  - Verify new room is generated when explorer stands on exit
  - Verify pathfinder rebuilds after new room added

**File Locations:**
- Main issue: `src/explorer/behavior.rs` lines 87-131 (Exploring state logic)
- Detection logic: `src/main.rs` lines 104-113 (detect_unexplored_exit works correctly)
- Pathfinder rebuild: `src/main.rs` line 111 (works correctly)

---

### Issue Triage: Compiler Warnings vs. Actual Errors (2025-02-12)

**User Report:** "Many errors" affecting UI after progressive generation implementation

**Investigation Results:**
- **No actual compilation errors** - build succeeds with 5 warnings (all dead_code)
- **Application is functionally correct** - explorer moves, rooms generate progressively
- **Two issues identified:**
  1. **P0: Terminal ANSI bleed** - timeout kills process before cleanup_terminal() executes, leaves raw ANSI codes in output (`;46m`, `[56;1H`)
  2. **P1: Unused code warnings** - 8 items flagged as dead_code (old batch generation methods, helper methods not yet used)

**Root Causes:**
1. **Terminal cleanup:** Signal-based termination (SIGTERM from timeout) bypasses normal cleanup path. Panic hook doesn't help with signals.
2. **Dead code:** Progressive generation replaced batch generation, leaving `DungeonGenerator::generate()` and related methods unused. Some helper methods (area, is_corridor, is_visible) written proactively but not yet integrated.

**Priority Decision:**
- P0 (terminal): Chunk implements signal handlers (SIGTERM/SIGINT) to ensure cleanup_terminal() runs
- P1 (warnings): Brand reviews each item - delete obsolete code, add #[allow(dead_code)] with TODO for future-use items
- P2 (deferred): Tile scaling enhancement from backlog (Enhance-1)

**Files:**
- Assessment document: `.ai-team/decisions/inbox/mikey-issue-assessment.md`
- Affected code: `src/main.rs` (cleanup), `src/dungeon/generator.rs` (old methods), `src/dungeon/room.rs`, `src/renderer/camera.rs`, `src/theme.rs`

**Outcome:** User's "many errors" are actually warnings (non-blocking). The real issue is visual (ANSI bleed) which has a clear fix path.


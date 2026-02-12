# Decisions — Dungeon Saver

This file tracks architectural, scope, and process decisions made by the team. Agents write to `.ai-team/decisions/inbox/` and Scribe merges entries here.

---

### 2025-01-20: Test Infrastructure for Phases 2-5

**By:** Brand

**What:** Set up comprehensive test infrastructure including fixtures, assertions, property-based tests, and test conventions documentation. Added proptest dependency and created test utilities module with 8 seed fixtures and 12 helper functions.

**Why:** Phases 2-5 require extensive testing (50+ test cases for dungeon generation, pathfinding correctness, seed determinism). By creating the scaffolding now, Data can write tests alongside implementation without infrastructure delays. Property-based testing with proptest ensures 2D6 rules hold across arbitrary seeds, not just hand-picked cases.

**Structure:**
- tests/common/mod.rs: Fixtures for known edge cases, assertions for 2D6 rule compliance, helpers for common queries
- tests/dungeon_generation.rs: Integration tests (6 tests covering entrance, corridors, small/large rooms, room count, determinism)
- tests/proptest_dungeon.rs: Property-based tests (6 invariants: valid dimensions, corridor detection, entrance consistency, exit limits, wall uniqueness, determinism)
- tests/TEST_CONVENTIONS.md: Documentation and usage examples
- src/lib.rs: Library crate for test compilation

**Tests currently passing:** 2/13 (deterministic generation stub, helper usage example)
**Tests ignored:** 11 (waiting for Data to implement DungeonGenerator::generate())

**Next:** Data removes #[ignore] markers as features are completed. Brand adds camera/pathfinding tests in Phase 3/4.

---

### 2025-01-20: Phase 1 Terminal Event Loop Implementation

**By:** Chunk

**What:** Implemented fullscreen terminal takeover with 10 FPS event loop, quit handling, and panic cleanup for Phase 1.

**Why:**
1. **Panic hook essential for terminal apps:** Without `std::panic::set_hook` to restore terminal state, any panic leaves the terminal broken. This hook runs `cleanup_terminal()` before the default panic handler.

2. **Non-blocking event polling:** Using `event::poll(Duration::ZERO)` instead of blocking `event::read()` ensures frame timing isn't disrupted by waiting for input. The loop maintains 10 FPS whether or not keys are pressed.

3. **Separate cleanup function:** Both normal exit and panic hook call `cleanup_terminal()`, avoiding duplication and ensuring consistent cleanup path.

4. **Frame timing precision:** Calculate elapsed time with `Instant`, sleep only for the remainder to hit target FPS. This adapts to variable render times while maintaining smooth 100ms frames.

5. **Canvas abstraction:** Keeping `Canvas::render()` separate from the event loop makes Phase 3 integration cleaner - just swap placeholder widget for dungeon rendering without touching main loop logic.

**Testing:** Integration tests validate 10 FPS timing simulation and quit signal detection (q, Q, Ctrl+C).

---

### 2025-01-20: Phased Implementation Plan

**By:** Mikey

**What:** Broke Dungeon Saver into 5 phases with clear deliverables and team ownership, from MVP terminal setup through full feature-complete screensaver.

**Why:** Team needs a concrete roadmap from skeleton to production. Each phase has defined handoff points between architecture, backend logic, and presentation.

---

## Phase 1: Terminal Takeover & Main Loop
**Goal:** Establish terminal control, event handling, and 10 FPS animation loop  
**Owner:** Chunk (infrastructure)  
**Deliverables:**
- Terminal setup (raw mode, alternate screen, panic cleanup via crossterm)
- Event loop (10 FPS tick, input handling for q/Q/Ctrl+C)
- Graceful shutdown with map export path setup
- Basic Ratatui widget skeleton (empty canvas)
- Tests: event loop runs at correct FPS, quit signals work

**Acceptance:** Application launches fullscreen, responds to quit, exits cleanly

---

## Phase 2: Dungeon Generation (2D6 Rules Engine)
**Goal:** Implement complete procedural dungeon generation following 2D6 rules  
**Owner:** Data (game logic)  
**Deliverables:**
- Room dimension rolling (D66 with double handling)
- Corridor logic (roll 1 = corridor, not a room)
- Small room detection (≤6 squares)
- Large room detection (≥32 squares after doubling)
- Exit placement algorithm (D6 roll for count, clockwise placement, boundary checks)
- Entrance room special handling (3 fixed exits, no doubling)
- Deterministic generation via SeededRng (same seed = same dungeon)
- Tests: 50+ test cases covering edge cases (doubles, boundary collisions, exit placement restrictions)

**Acceptance:** `DungeonGenerator::generate()` produces valid room lists; visual verification of ~20 rooms per dungeon

---

## Phase 3: Rendering & Camera
**Goal:** Draw dungeon on screen with camera following explorer  
**Owner:** Mouth (presentation)  
**Deliverables:**
- ASCII/Unicode walls, floors, corridors, doors, explorer sprite
- Ratatui canvas that draws rooms + explorer position
- Camera system: keeps explorer roughly centered (not flushed center, but middle area)
- Pan logic: when explorer reaches last quarter of screen, shift viewport
- Handle terminal resize events gracefully
- Color theming (Catppuccin Mocha via theme.rs)
- Tests: camera pans correctly, explorer stays in viewport bounds

**Acceptance:** Run with a fixed dungeon, watch explorer's starting position centered on screen

---

## Phase 4: Explorer AI & Pathfinding
**Goal:** Implement explorer behavior (exploring → wandering) and A* movement  
**Owner:** Chunk (pathfinding)  
**Deliverables:**
- Pathfinder: A* algorithm via pathfinding crate
- Explorer behavior state machine: Exploring (visit unvisited rooms) → Wandering (random) → Pausing (brief pause on room entry)
- Room discovery: mark rooms as visited when explorer enters
- Movement: pathfind to next unvisited room, or random target if all visited
- Pause mechanics: 1-3 second pause on new room discovery
- Tests: explorer visits all rooms, returns sensible paths, pauses happen

**Acceptance:** Watch explorer autonomously explore all dungeon rooms, then wander

---

## Phase 5: Polish & Map Export
**Goal:** Add finishing touches and persistent output  
**Owner:** Scribe (shipping)  
**Deliverables:**
- Map export on exit: save dungeon to `maps/yyyy-MM-dd_HHmm_seed<seed>.txt`
- Export format: ASCII representation + metadata (room count, dimensions, seed)
- CLI argument parsing (--seed flag for deterministic runs)
- Performance tuning: confirm <1% CPU at 10 FPS (release build profiling)
- Error handling: graceful degradation on small terminals
- Documentation: README with build/run instructions
- Tests: export produces valid files, seeds reproducible
- Integration test: full end-to-end run

**Acceptance:** Full feature parity with Rules.md requirements, ready for user distribution

---

## Handoff Strategy

1. **Phase 1 → Phase 2:** Chunk delivers working terminal loop + empty canvas; Data works in parallel on dungeon generation
2. **Phase 2 → Phase 3:** Data delivers `DungeonGenerator` with unit tests; Mouth integrates rendering
3. **Phase 3 → Phase 4:** Mouth delivers camera + viewport; Chunk builds explorer on top of dungeon+camera
4. **Phase 4 → Phase 5:** All components done; Scribe polishes, exports, and ships
5. **Parallel work:** Data can build unit tests independently; Mouth can prototype UI without waiting for explorer

## Risk Mitigation

- **Large rooms/doubles:** Data writes focused unit tests early (Phase 2)
- **A* correctness:** Chunk uses pathfinding crate (battle-tested), tests with known dungeon layouts
- **Camera jitter:** Mouth tests pan logic extensively before Phase 4
- **Build performance:** Release profile already optimized; check at Phase 5 if needed

---

**Next Steps:**
1. Chunk: Begin Phase 1 (terminal setup, event loop)
2. Data: Begin Phase 2 (2D6 room generation)
3. Mouth: Prepare renderer stubs, read Catppuccin palette
4. Brand: Prepare test infrastructure for Phase 2 → Phase 5

---

### 2025-01-20: Tech Stack — Rust + Ratatui

**By:** Mikey

**What:** Dungeon Saver will be built in Rust using Ratatui (TUI framework) with crossterm backend, rand_chacha for deterministic RNG, and the pathfinding crate for A*.

**Why:**

1. **Performance for screensaver use-case**: Rust compiles to native code with no GC pauses. A 10 FPS animation loop will use <1% CPU. This is non-negotiable for something that runs continuously in the background.

2. **Deterministic RNG**: The `rand_chacha` crate gives us ChaCha8Rng — a seedable, deterministic PRNG. Same seed = same dungeon, every time, on every machine. This is core to the product spec.

3. **Ratatui ecosystem**: Ratatui is the most actively maintained TUI framework in any language right now. The `crossterm` backend handles raw mode, alternate screen, true color (24-bit for Catppuccin), and graceful cleanup even on panic.

4. **Single binary**: Users get a single executable with no runtime dependencies. No Python interpreters, no Go runtime. Just `./dungeon-saver`.

5. **Pathfinding crate**: Production-ready A* implementation. We're not here to reinvent graph algorithms.

**Trade-offs:**

- Steeper learning curve than Python/Go — but the team can handle it
- Longer compile times — mitigated by incremental builds
- No REPL for quick prototyping — but we have `cargo check` and tests

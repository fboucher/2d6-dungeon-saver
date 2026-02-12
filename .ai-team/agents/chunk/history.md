# History — Chunk

## Project Learnings (from import)

**Owner:** fboucher (fboucher@outlook.com)

**Project:** Terminal-based dungeon explorer screensaver. Full terminal takeover. 10 FPS animation. Linux terminal only.

**Color Theme:** Catppuccin Mocha (https://catppuccin.com/palette/) — define colors as a theme group for future theme swapping. Each element (walls, floor, corridor floor, doors, explorer) has its own color.

**Terminal Behavior:**
- Full screen takeover (like a screensaver)
- Explorer should stay roughly centered (not flush center, but in the middle area)
- When explorer reaches last quarter of screen, pan to keep them in the middle area
- Terminal-responsive (adapts to terminal size)
- Controls: q/Q to quit, Ctrl-C for emergency exit

**Animation:**
- 10 FPS target
- Minimal CPU usage
- Watch the explorer discover rooms in real-time
- Brief pause when explorer enters a new room

**Tech Stack:** TBD — likely Python (curses), Rust (crossterm/ratatui), or Go (tcell)

---

## Learnings

### Terminal Infrastructure (Phase 1 Implementation)

**Main Loop Architecture:**
- Event loop runs at 10 FPS (100ms frame duration) via `TARGET_FPS` constant
- Terminal setup: `enable_raw_mode()` + `EnterAlternateScreen` for fullscreen takeover
- Panic hook installed to ensure terminal cleanup on crash via `std::panic::set_hook`
- Quit signals: q/Q and Ctrl+C handled via `crossterm::event` polling
- Frame timing: `Instant` tracking with sleep to maintain target FPS

**File Structure:**
- `src/main.rs`: Event loop, terminal setup/cleanup, panic handling
- `src/renderer/canvas.rs`: Ratatui rendering (Phase 1: placeholder widget)
- `tests/integration_test.rs`: FPS timing tests and quit signal validation

**Crossterm Patterns:**
- Always use `event::poll(Duration::ZERO)` for non-blocking input checks
- `execute!` macro for terminal commands (cleaner than direct writes)
- Cleanup function reused in both normal exit and panic hook

**Ratatui Rendering:**
- `terminal.draw(|frame| {...})` for each frame
- Canvas abstraction keeps rendering logic separate from event loop
- Phase 1 uses placeholder Block widget; real dungeon rendering in Phase 3

---

📌 Team update (2026-02-12): Phase 1 Terminal Event Loop Implementation complete — panic hooks, non-blocking polling, frame timing — decided by Chunk
📌 Team update (2026-02-12): Phased Implementation Plan approved — Chunk owns Phase 1 (terminal) and Phase 4 (AI/pathfinding) — decided by Mikey
📌 Team update (2026-02-12): Tech Stack finalized (Rust + Ratatui) — impacts terminal infrastructure throughout all phases — decided by Mikey
📌 Team update (2026-02-12): Test Infrastructure scaffolding ready for Phase 2+ — Chunk will benefit from integration tests framework — decided by Brand

### Phase 1 Status (2025-01-20)

**Phase 1 Complete and Verified:**
- Terminal setup with raw mode and alternate screen takeover
- Panic cleanup hook ensures graceful terminal restoration on crashes
- 10 FPS event loop with precise frame timing via `Instant` and calculated sleep
- Non-blocking input handling: q/Q and Ctrl+C for quit
- Placeholder Ratatui canvas (cyan-bordered block with quit instructions)
- Integration tests passing: FPS timing simulation and quit signal detection
- Application builds successfully and launches fullscreen

**Files Implementing Phase 1:**
- `src/main.rs`: Event loop, terminal lifecycle, panic hooks
- `src/renderer/canvas.rs`: Phase 1 placeholder widget
- `tests/integration_test.rs`: 6 passing tests validating timing and quit behavior

**Ready for Phase 2:** Data can now implement dungeon generation independently. Canvas will integrate dungeon rendering in Phase 3.



---

📌 Team update (2026-02-12): Phase 3 camera system complete — intelligent panning, resize handling, and rendering integration ready for Phase 4 — decided by Mouth

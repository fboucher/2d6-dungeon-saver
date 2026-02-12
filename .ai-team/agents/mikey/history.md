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



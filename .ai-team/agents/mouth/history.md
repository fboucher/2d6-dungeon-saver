# History — Mouth

## Project Learnings (from import)

**Owner:** fboucher (fboucher@outlook.com)

**Project:** Terminal-based dungeon screensaver with AI explorer.

**Explorer Behavior (from Rules.md):**
- **Discovery phase:** Automatically explores unvisited rooms first
- **Wander phase:** Randomly wanders after exploring all rooms
- **Pause:** Brief pause when discovering new rooms
- **Visibility:** Always visible, roughly centered on screen (Chunk handles panning)

**Movement Context:**
- Smooth animation at 10 FPS (Chunk handles frame timing)
- Dungeon is generated AS the explorer opens doors (work with Data on this)
- Explorer moves through Exit Squares to enter new rooms
- No combat, no obstacles — just navigation

**Tech Stack:** TBD — need to coordinate with Data on dungeon structure representation

---

## Learnings

---

📌 Team update (2026-02-12): Phased Implementation Plan approved — Mouth owns Phase 3 (rendering & camera) — decided by Mikey
📌 Team update (2026-02-12): Tech Stack finalized (Rust + Ratatui + Catppuccin Mocha theme) — Mouth will implement camera panning and dungeon rendering — decided by Mikey
📌 Team update (2026-02-12): Phase 1 Terminal Loop complete — Mouth can integrate rendering atop Canvas abstraction in Phase 3 — decided by Chunk
📌 Team update (2026-02-12): Test Infrastructure ready — Mouth will add camera/pathfinding tests in Phase 3/4 — decided by Brand

### Phase 3 Complete (2025-01-20)

**Rendering Architecture:**
- `src/renderer/canvas.rs`: Main rendering widget using Ratatui's Buffer API
- `src/renderer/camera.rs`: Viewport management with intelligent panning logic
- Dungeon rooms rendered with ASCII walls (+, -, |), floor dots (.), and door symbols (▢)
- Explorer sprite rendered as '@' character
- Catppuccin Mocha theme applied (walls: lavender, floors: latte, doors: peach, explorer: green)

**Camera Panning Strategy:**
- Camera keeps explorer in "comfort zone" (middle 50% of screen)
- Pans when explorer reaches outer quarters (last 25% on any edge)
- Uses saturating subtraction to handle origin edge cases gracefully
- Resize events update camera dimensions without breaking viewport

**Key Files:**
- `src/renderer/canvas.rs`: DungeonWidget renders rooms and explorer on Ratatui buffer
- `src/renderer/camera.rs`: Camera struct with update(), center_on(), resize(), is_visible()
- `src/main.rs`: Integrated dungeon generation with camera-based rendering
- `tests/camera_rendering.rs`: 7 integration tests for panning, centering, visibility

**Patterns Established:**
- Screen space conversion: world coordinates → camera offset → screen coordinates
- Visibility culling: skip rendering rooms outside viewport bounds
- Wall character selection: corners use '+', horizontals use '-', verticals use '|'
- Door rendering: separate pass after walls to ensure visibility

**Fixed Seed for Acceptance:**
- Using seed 42 for deterministic testing
- Entrance room spawns at (10, 10) with 3 exits
- Explorer starts centered in entrance room
- Camera initializes centered on explorer position



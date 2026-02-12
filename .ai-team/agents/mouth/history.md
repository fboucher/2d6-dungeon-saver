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



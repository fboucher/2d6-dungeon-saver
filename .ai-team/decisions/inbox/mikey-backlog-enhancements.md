# 2025-02-12: Post-Launch Enhancement Backlog

**By:** Mikey

**What:** Structured 3 user-requested enhancements into actionable backlog work items to improve visibility and usability post-launch.

**Why:** Project is feature-complete per original 5-phase plan. These three items address UX polish (tile size), core gameplay feature (fog of war), and technical debt (compiler warnings). Structured backlog enables parallel team assignments.

---

## Enhancement Backlog

| ID | Title | Owner | Size | Priority | Notes |
|:---|:------|:------|:-----|:---------|:------|
| **Enhance-1** | Increase Tile Size for Better Visibility | Chunk | M | P1 | Camera already uses `tile_scale: 2` (2×2 chars per cell). User feedback: tiles too small to see clearly. Option A: increase to 3×3 or 4×4. Option B: add CLI flag `--tile-scale N`. Impacts camera viewport calculations and room rendering. Test with both small (80×24) and large terminals. |
| **Enhance-2** | Fog of War — Track & Render Visited Rooms Only | Data + Chunk | L | P1 | Gameplay feature: dungeon reveals as explorer visits rooms. Requires: (1) Data adds `visited: bool` field to Room struct, (2) Explorer behavior tracks room visits, (3) Chunk filters render queue to skip unvisited rooms. Current renderer draws all rooms; needs conditional rendering. Integration point: Room discovery in Phase 4 explorer logic. |
| **Enhance-3** | Clean Up Compiler Warnings | Brand | S | P2 | Current warnings on exit: (1) `Room::area()` unused, (2) `Room::is_corridor()` unused, (3) `Camera::is_visible()` unused, (4) `Theme::corridor` field unread. Action: Review each—either remove dead code or re-enable with `#[allow(dead_code)]` if intended for future features. Brand decides; Chunk/Data/Mouth defer if they claim future use. |

---

## Recommendation

**Start with:** Enhance-2 (Fog of War) immediately after current iteration completes.

**Why this order:**

1. **Enhance-2 (P1, 2-person work):** This is the marquee feature that completes "procedural dungeon reveals as player explores" gameplay loop. It touches both backend (Data) and rendering (Chunk), but has clear handoff: Data adds visited tracking → Chunk wires it to renderer. Unblock Team for next iteration.

2. **Enhance-1 (P1, Chunk solo):** Tile scaling is a quick win once Enhance-2 is shipping. Chunk can parallelize this with final fog-of-war rendering tweaks. Consider adding CLI flag for user flexibility.

3. **Enhance-3 (P2, Brand solo):** Compiler cleanup is maintenance work—do it when team has a breather (e.g., between phases). Ask each team member if they intended those methods for future phases. If "no," delete; if "yes," add `#[allow(dead_code)]` with a TODO comment linking to future work.

**Dependencies:**
- Enhance-2 depends on nothing (can start immediately)
- Enhance-1 depends on nothing (can run in parallel with Enhance-2)
- Enhance-3 depends on nothing (background task)

**Milestone Proposal:** Ship Enhance-2 + Enhance-1 together as "Polish v0.2" (better visibility + fog of war = core UX loop). Then ship Enhance-3 as "Cleanup v0.3" once linting complete.

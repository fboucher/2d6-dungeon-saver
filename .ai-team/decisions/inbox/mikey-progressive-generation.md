### 2025-02-12: Progressive Dungeon Generation - Cancel Seed Determinism & Enhance-2

**By:** Mikey

**What:** User clarified expectation: dungeon should be generated room-by-room as explorer opens doors (progressive generation), not all at once at startup. This requires cancelling seed-based determinism requirement and Enhance-2 (fog of war) backlog item.

**Why:** 

1. **Matches 2D6 Pen & Paper Intent:** Original rules state "you generate dungeon spaces as you encounter them" - current implementation generates all 20 rooms upfront, violating this principle.

2. **User Chose Emergent Over Predetermined:** When asked, user selected emergent dungeon generation (structure depends on explorer choices) over seed-based determinism. Same seed should only control dice rolls (D6/D66), not dungeon layout.

3. **Enhance-2 Obsolete:** Fog of war backlog item (P1, Large) planned to "reveal dungeon as explorer visits." With true progressive generation, rooms literally don't exist until explorer opens the door - no need for separate fog-of-war rendering.

4. **Seed Feature Misunderstood:** User explicitly stated seed "shouldn't have been present in the requirements" - it was added based on initial interpretation, but doesn't align with the pen & paper experience.

**Impact:**

- **Architecture:** Major refactor required (see plan.md) - DungeonGenerator must live in main loop, generate rooms on-demand
- **Requirements:** Remove seed-based reproducibility claims from Rules.md, README.md, and CLI (--seed flag)
- **Backlog:** Cancel Enhance-2, keep Enhance-1 (tile scaling) and Enhance-3 (warnings)
- **Testing:** Tests must handle non-deterministic layouts (same seed ≠ same dungeon structure)

**Next Steps:** 

Created detailed implementation plan at `/home/frank/.copilot/session-state/0f62829d-c26a-4fef-9275-5643cc42ef01/plan.md` with 6 phases covering DungeonGenerator refactoring, main loop integration, explorer behavior updates, pathfinder changes, requirements cleanup, and testing.

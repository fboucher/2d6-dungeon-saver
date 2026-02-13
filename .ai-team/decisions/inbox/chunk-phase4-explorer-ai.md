### 2025-01-20: Phase 4 Explorer AI & Pathfinding Implementation

**By:** Chunk

**What:** Implemented autonomous explorer with A* pathfinding, room discovery, and behavior state machine (Exploring → Pausing → Wandering). Explorer autonomously visits all dungeon rooms, pauses briefly on discovery (1-3 seconds), then wanders randomly.

**Why:**

1. **A* with Pre-computed Walkable Set:** The Pathfinder builds a HashSet of all walkable tiles during initialization from the dungeon layout. This enables O(1) neighbor lookups during A* search, avoiding repeated dungeon traversal. The pathfinding crate provides battle-tested A* implementation.

2. **State Machine Design:** Explorer transitions through three states:
   - **Exploring:** Finds nearest unvisited room via pathfinding, creating intentional exploration behavior
   - **Pausing:** 10-30 tick pause on room entry (1-3 seconds at 10 FPS), giving user time to observe discovery
   - **Wandering:** Random room selection once all rooms visited, maintaining visual interest indefinitely

3. **Path Following Pattern:** Explorer maintains internal path queue from A* results and follows it step-by-step each frame. This creates smooth, deliberate movement instead of teleporting or recalculating every frame.

4. **Room Discovery Tracking:** HashSet of visited room IDs enables efficient O(1) lookup for "have I been here?" checks. Room entry detection uses simple bounds checking (x, y within room rectangle).

5. **Integration with Camera:** Camera.update() is called after Explorer.update() to ensure camera follows current position. Camera panning logic (from Phase 3) keeps explorer in the "comfort zone" without requiring explorer to know about viewport.

**Testing:** 6 integration tests verify: all rooms eventually visited, paths are valid, pausing triggers on room entry, explorer stays within room bounds, and behavior works across different dungeon seeds.

**Next:** Phase 5 can now add map export on quit and CLI argument parsing (--seed flag).

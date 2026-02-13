# History — Data

## Project Learnings (from import)

**Owner:** fboucher (fboucher@outlook.com)

**Project:** Terminal-based dungeon screensaver with 2D6 procedural generation.

**2D6 Generation Rules (from 2D6 Rules.md):**

**Room Dimensions:**
- Roll D66 (two D6 dice): Primary die = X-axis, Secondary die = Y-axis
- Entrance room: always 6-12 squares (re-roll if under/over), always 3 archway exits
- Doubles: roll again and ADD to dimensions (except double 6, except entrance room)
- Small rooms: ≤6 squares → different table, all archways
- Large rooms: ≥32 squares → different table
- Corridors: 1 on dimension roll (not double 1) → narrow space, all archways

**Exit Placement:**
- D6 roll: 1 = no exits, 2-3 = 1 exit, 4-5 = 2 exits, 6 = 3 exits
- Cannot place on entrance wall
- Cannot place on Outer Boundary
- Cannot place on wall with existing exit
- Cannot place if it would lead to space 1 square from a wall
- Max 4 exits per room (including entrance)
- Each exit gets an Exit Square (single square corridor stub)

**Dungeon Constraints:**
- Target ~20 rooms
- Single level only (ignore multi-level rules)
- Rooms cannot overlap
- Rooms can butt up against each other
- Outer Boundary stops room expansion
- Ignore "what's in the room" content for now

**Map Export:**
- On exit, save to `maps/yyyy-MM-dd_HHmm_seed<seed>.txt`
- Include ASCII representation + room count + dimensions

**Tech Stack:** TBD — seed-based RNG required

---

## Learnings

---

📌 Team update (2026-02-12): Phased Implementation Plan approved — Data owns Phase 2 (dungeon generation with 2D6 rules) — decided by Mikey
📌 Team update (2026-02-12): Tech Stack finalized (Rust + rand_chacha for deterministic RNG) — Data will use SeededRng for generate() — decided by Mikey
📌 Team update (2026-02-12): Test Infrastructure ready (proptest, fixtures, assertions) — Data can implement generate() with comprehensive test coverage — decided by Brand
📌 Team update (2026-02-12): Phase 1 Terminal Loop complete — Data can work in parallel, delivering DungeonGenerator for Phase 2 → Phase 3 handoff — decided by Chunk


### Core Architecture Files

**src/rng.rs**: SeededRng wrapper around ChaCha8Rng providing d6() and d66() dice rolls. Deterministic generation via seed_from_u64().

**src/dungeon/room.rs**: Room struct with RoomType enum (Entrance, Normal, Small, Large, Corridor). Exit struct uses Wall enum (North, East, South, West) + position offset.

**src/dungeon/generator.rs**: DungeonGenerator implements full 2D6 rules. Key methods: generate_entrance_room(), generate_room(), place_exits(), roll_exit_count().

**examples/generate_dungeon.rs**: Example program for testing dungeon generation with different seeds. Shows room details and summary statistics.

### 2D6 Rule Implementation Details

**Double handling**: When d66() rolls doubles (except 6,6), immediately roll again and add to dimensions. Check is `width == height && width != 6`. Entrance room never applies doubling.

**Corridor detection**: Any room with width==1 OR height==1 is flagged as Corridor type. Occurs naturally from D66 rolling 1 on either die.

**Exit placement algorithm**: Clockwise from North starting at random wall offset. Skip walls that: (1) match entrance wall, (2) already have exit, (3) would be on boundary. Max 3 exits for normal rooms, always 3 for entrance.

**Room type classification**: Applied after dimension calculation and doubling:
- Corridor: width==1 or height==1
- Small: area ≤ 6 and not corridor  
- Large: is_double && area ≥ 32
- Normal: everything else
- Entrance: first room, special handling

### Testing Strategy

24 unit tests in generator.rs covering:
- Deterministic generation (same seed = same dungeon)
- Entrance constraints (3 exits, 6-12 area, no doubling)
- Corridor/Small/Large room detection edge cases
- Exit count distribution (0-3 exits for normal rooms)
- No duplicate exits on same wall (HashSet check)
- Dimension bounds (1-12 range after all rolls)
- D6/D66 range validation (1-6 inclusive)

Test seeds chosen to exercise specific scenarios: 42 (baseline), 999 (large rooms), 12345 (mixed), etc.

### Known Limitations (Phase 2 Scope)

**Room positioning**: Currently all rooms positioned at (0,0) placeholder. Spatial layout and collision detection deferred to Phase 3 (rendering) or Phase 4 (pathfinding).

**Exit boundary checks**: Logic for "cannot place on Outer Boundary" implemented but needs actual boundary dimensions from dungeon parameters (not in Phase 2 spec).

**Exit square generation**: Exit positions calculated but actual Exit Square placement (single square corridor stub) deferred to rendering phase.

### Phase 5 Map Export Architecture

**src/export.rs**: MapExporter handles dungeon serialization to ASCII format. Creates `maps/` directory, generates timestamped filenames with seed (format: `yyyy-MM-dd_HHmm_seed<seed>.txt`). Export includes metadata header (seed, timestamp, room count, dungeon dimensions) plus ASCII grid representation.

**ASCII rendering algorithm**: Calculates dungeon bounds from room positions, creates 2D grid, renders each room with walls (+/-/|), floors (.), corners (+), and doors (▢). Entrance room marked with 'E' at center. Grid coordinates offset by minimum bounds to normalize to (0,0) origin.

**CLI argument parsing**: `parse_seed()` function in main.rs parses `--seed <value>` from command line. Falls back to time-based random seed if not provided. Seed passed to DungeonGenerator for reproducible generation.

**Export on exit**: MapExporter called after main event loop terminates, before cleanup_terminal(). Export errors logged to stderr but don't prevent graceful shutdown. Filepath printed to stderr for user confirmation.

**Terminal size handling**: Added graceful degradation warning for small terminals (< 40x20). Application continues but warns user that experience may be suboptimal. Camera and renderer adapt to available space.

**Integration testing**: Added 4 Phase 5 tests in integration_test.rs: full generation+export workflow, seed reproducibility verification, different seeds produce different dungeons, filename format validation. All export tests use temp files with cleanup.

---

### 2026-02-12: Fixed Dungeon Connectivity — Rooms Now Actually Connect

**Problem Diagnosed**: Generator created rooms but never connected them spatially. All rooms (except entrance) were positioned at (0,0) because:
1. Generator created rooms independently without tracking parent connections
2. Layout.rs tried to guess positions but had no actual connection data
3. 2D6 rules specify rooms should connect via exits, but this was not implemented

**Solution Implemented**:

1. **Added connection tracking to Room struct**:
   - `parent_id: Option<usize>` — which room this connects to
   - `parent_wall: Option<Wall>` — which wall of parent this connects through
   - `Exit.connected_room_id: Option<usize>` — which room an exit leads to

2. **Rewrote generator to build connected dungeons**:
   - Track available exits (exits without connected rooms)
   - When generating new room, pick a random available exit to connect to
   - Calculate new room position based on parent's position and exit wall direction
   - Update both parent exit and child room with connection info
   - Add child's exits to available pool for future rooms

3. **Room positioning algorithm**: 
   - Each new room placed adjacent to parent based on exit wall:
     - North exit → place child above parent
     - South exit → place child below parent  
     - East exit → place child to right
     - West exit → place child to left
   - Entrance wall of child (opposite of parent wall) excluded from exit placement

4. **Removed layout.rs**: No longer needed — generator now handles positioning directly during generation

**Results**:
- All 20 rooms now have unique spatial positions forming connected graph
- Exported maps show proper dungeon structure with connected rooms
- All 41 existing tests still pass
- Dungeon is navigable (explorer can pathfind through connected rooms)
- ASCII export displays actual dungeon layout instead of overlapping rooms at origin

**Files Modified**:
- `src/dungeon/room.rs`: Added parent_id, parent_wall, Exit.connected_room_id fields
- `src/dungeon/generator.rs`: Complete rewrite of generate() to track connections
- `src/dungeon/mod.rs`: Removed layout module reference
- `src/dungeon/layout.rs`: Deleted (functionality moved to generator)
- Test files: Updated Room initializations to include new fields (pathfinder.rs, behavior.rs, common/mod.rs)

**Technical Details**:
- AvailableExit struct tracks room_id, wall, and exit_index for connection
- generate() maintains pool of available exits, consuming one per new room
- opposite_wall() helper maps parent wall to child entrance wall
- calculate_room_position() uses saturating arithmetic to handle dungeon boundaries
- Generation stops when either 20 rooms created OR no available exits remain


### 2026-02-13: Progressive Dungeon Generation Architecture (Phase 1)

**Refactored DungeonGenerator for on-demand room creation**:

1. **New API methods**:
   - `generate_entrance()` — Creates only entrance room (replaces upfront generation)
   - `add_room(parent_id, wall) -> Room` — Generates connected room on-demand when explorer reaches exit
   - `generate()` marked deprecated with note to use progressive API

2. **Room limit handling**:
   - `add_room()` returns dummy room (id=usize::MAX, 0 exits) when dungeon hits 20 rooms
   - Callers must check for dummy room to avoid adding it to dungeon Vec
   - Generator maintains internal room count and stops at 20

3. **Connection tracking preserved**:
   - Existing `generate_connected_room()` helper unchanged
   - `add_room()` updates parent exit's `connected_room_id` field
   - Rooms still positioned relative to parent based on wall direction

4. **Test coverage**:
   - Added 4 new progressive generation tests (entrance-only, connections, limit, determinism)
   - Kept 24 legacy `generate()` tests with `#[allow(deprecated)]` for backward compatibility
   - Integration test updated to handle dummy room signal (skip id=usize::MAX)
   - 1 explorer integration test ignored (requires Phase 2 main loop work)

**Architectural implications**:
- Generator must now live in main loop (not just dungeon Vec)
- Dungeon grows from 1 room at startup to ~20 rooms during animation
- Seed-based determinism removed from dungeon layout (only dice rolls use RNG)
- Explorer behavior will target "unexplored exits" instead of "unvisited rooms"
- Pathfinder must rebuild when new rooms added (Phase 4 work)

**Files Modified**:
- `src/dungeon/generator.rs`: Added generate_entrance(), add_room(), deprecated generate()
- `tests/explorer_tests.rs`: Updated progressive test, added dummy room check, ignored integration test
- `tests/integration_test.rs`: Fixed progressive generation test assertions

**Next Steps** (Phase 2 - Chunk):
- Keep DungeonGenerator alive in main loop
- Add exit detection: when explorer position == exit position → call add_room()
- Initialize with `let mut dungeon = vec![generator.generate_entrance()]`
- Rebuild pathfinder when dungeon grows


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


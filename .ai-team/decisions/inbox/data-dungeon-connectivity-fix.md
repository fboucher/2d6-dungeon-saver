# Decision: Generator-Integrated Room Connection System

**Date:** 2026-02-12  
**By:** Data  
**Status:** Implemented

## Problem

The dungeon generator created individual rooms following 2D6 rules but never actually connected them into a spatial dungeon:
- All rooms except entrance positioned at (0,0)
- No parent-child relationships tracked
- Chunk's layout.rs tried to guess connections but had no real data
- Exported maps showed overlapping rooms, not actual dungeons
- User complaint: "that isn't a dungeon and it's NOT an animation"

## Decision

**Move connection logic INTO the generator itself** rather than having a separate layout phase.

The generator now:
1. Tracks available exits (exits without connected rooms)
2. Generates each new room by connecting it to a random available exit
3. Positions room spatially based on parent position + exit direction
4. Stores bidirectional connection data (parent→child and exit→room)

## Why This Approach

**Alternatives Considered:**

1. **Keep separate layout phase** (Chunk's approach)
   - ❌ Requires guessing which exits connect where
   - ❌ No guarantee of respecting 2D6 exit placement rules
   - ❌ Two-phase process complicates debugging

2. **Post-process connections after generation**
   - ❌ Hard to retrofit connections to already-placed rooms
   - ❌ May violate 2D6 rules (e.g., placing exits where they shouldn't be)

3. **Generator-integrated connections** (chosen)
   - ✅ Connections are guaranteed valid (they drive generation)
   - ✅ Room positioning happens naturally as part of connection
   - ✅ Single-phase generation easier to reason about
   - ✅ Respects 2D6 rules: each room connects via parent's exit
   - ✅ No separate layout module needed

## Implementation Details

**New fields in Room:**
```rust
pub parent_id: Option<usize>,     // Which room this connects to
pub parent_wall: Option<Wall>,    // Which wall of parent
```

**New field in Exit:**
```rust
pub connected_room_id: Option<usize>,  // Which room this exit leads to
```

**Core algorithm:**
```rust
1. Generate entrance room at fixed position (10, 10)
2. Collect all available exits (entrance has 3)
3. While rooms.len() < 20 AND available_exits not empty:
   a. Pick random available exit
   b. Generate new room dimensions (D66 + doubles)
   c. Position new room adjacent to parent (based on exit wall)
   d. Set child's entrance wall = opposite of parent wall
   e. Update parent exit to point to new room
   f. Add child's exits to available pool
```

**Positioning logic:**
- North exit → child at (parent.x, parent.y - child.height - 1)
- South exit → child at (parent.x, parent.y + parent.height + 1)
- East exit → child at (parent.x + parent.width + 1, parent.y)
- West exit → child at (parent.x - child.width - 1, parent.y)

## Impact

**Data (this agent):**
- ✅ Generator now produces spatially-connected dungeons
- ✅ All 24 existing tests still pass
- ✅ Removed dependency on layout.rs

**Chunk (rendering):**
- ✅ Renderer now displays actual connected dungeons
- ✅ No need to guess room positions
- ✅ layout.rs deleted (11 fewer lines to maintain)

**Mouth (pathfinding):**
- ✅ Explorer can now navigate connected rooms
- ✅ Connection data available for pathfinding optimization (future)

**Overall:**
- ✅ Exported maps show proper dungeon structure
- ✅ Rooms no longer overlap at origin
- ✅ Follows 2D6 pen & paper rules more faithfully
- ✅ Simpler architecture (one module instead of two)

## Lessons Learned

**For Data:** Game logic should own spatial relationships when those relationships are fundamental to generation rules. Trying to separate "what" (rooms) from "where" (layout) violated the 2D6 model where exits DEFINE connections.

**For Chunk:** When rendering attempts fail, check if the underlying data model is correct. Layout.rs was trying to fix a generator problem.

**For Team:** When user says "that isn't a dungeon," they're right — always validate core assumptions (like "do rooms actually connect?") before adding layers of complexity.

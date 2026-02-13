# Simple Grid Renderer Implementation

**By:** Chunk  
**Date:** 2025-02-12  
**Status:** Implemented

## Problem

Previous renderer used complex 4x4 tile scaling (TILE_SCALE=4), multi-character wall patterns, and corridor drawing logic. This made the code complex (~300 lines) and hard to understand. User feedback requested simpler rendering: "rooms as rectangles on an invisible grid, walls adjoin cleanly."

## Solution

Implemented Mikey's simple grid-based renderer design with **1:1 coordinate mapping**:
- Each dungeon cell = exactly 1 terminal character
- No tile scaling
- Simple character mapping: `#` for walls, `.` for floors, `|`/`-` for doors, `@` for explorer

## Changes Made

### src/renderer/canvas.rs
- **Removed:** TILE_SCALE constant (was 4)
- **Removed:** get_scaled_wall_char() method (complex multi-character wall logic)
- **Removed:** render_corridors() method (corridors now just empty space)
- **Simplified:** render_room() to simple double loop with wall/floor detection
- **Simplified:** render_exits() to single character overwrite (no 4x4 block)
- **Simplified:** render_explorer() to single `@` character (no multi-char sprite)
- **Result:** ~165 lines (from ~300), much cleaner

### src/renderer/camera.rs
- **Removed:** tile_scale field
- **Removed:** All dungeon-to-screen coordinate conversion math
- **Simplified:** update(), center_on(), is_visible() to work with 1:1 coordinates
- **Result:** Camera now operates directly in dungeon space (no conversion needed)

### tests/camera_rendering.rs
- **Updated:** All tests to reflect 1:1 coordinate mapping
- **Removed:** References to tile_scale multiplication
- **Fixed:** test_camera_centers_on_explorer_initial_position to use dungeon coordinates directly

## Technical Details

**Coordinate System:**
```
Dungeon (x, y) = Screen (x, y)
Room at (10, 5) → renders at terminal col 10, row 5
Room width=8 → occupies 8 terminal columns
```

**Rendering:**
```rust
// Walls and floors (1 character each)
for dy in 0..room.height {
    for dx in 0..room.width {
        let is_wall = dy == 0 || dy == room.height - 1 
            || dx == 0 || dx == room.width - 1;
        let ch = if is_wall { '#' } else { '.' };
    }
}

// Doors (overwrite walls)
let door_char = match exit.wall {
    Wall::North | Wall::South => '|',
    Wall::East | Wall::West => '-',
};
```

**Explorer:**
- Single `@` character at explorer position
- No multi-character sprite
- Simple and clear

## Testing

✅ All 45 lib unit tests passing  
✅ All 7 camera integration tests passing  
✅ All 6 explorer tests passing  
✅ All 9 progressive generation tests passing  
✅ Code compiles without warnings  
✅ Visual verification: clean rectangular rooms with clear walls/floors/doors

## Trade-offs

**Gained:**
- Simple mental model (1 cell = 1 char)
- Easy debugging (coordinates match screen)
- No scaling bugs or conversion errors
- Less code (~135 lines reduced)
- Faster rendering (no nested tile loops)

**Lost:**
- Rooms appear smaller on screen (6x6 room = 6 characters, not 24x24)
- Less visual detail (no fancy wall patterns)
- Might be harder to see on very large terminals

**Note:** User specifically requested this simpler approach, so trade-off is intentional.

## Why This Matters

Mikey's design principle: "Each dungeon grid cell maps to exactly ONE terminal character." This eliminates an entire class of bugs (coordinate conversion, scaling math) and makes the codebase much easier to understand and maintain. The renderer is now straightforward enough that anyone can read it and understand what's happening.

## Next Steps

- Monitor user feedback on visual clarity
- If rooms too small, can adjust room generation sizes (not renderer)
- Camera panning logic unchanged (still works perfectly)
- Explorer AI unchanged (still uses dungeon coordinates)

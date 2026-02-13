# Simple Grid Renderer Architecture

**By:** Mikey  
**Date:** 2025-02-12  
**Status:** Design Proposal

## Problem

Current renderer (canvas.rs) uses complex tile scaling (TILE_SCALE=4, each dungeon cell = 4×4 terminal chars), multi-character wall patterns, corridor drawing logic, and visibility culling. Frank reports this is hard to see and wants a simpler approach: rooms as rectangles on an invisible grid, walls adjoin cleanly, doors connect rooms, straightforward coordinate mapping.

## Design: One Cell = One Character

**Core Principle:** Each dungeon grid cell maps to exactly ONE terminal character. No scaling, no complex math.

### Coordinate System

```
Dungeon Space = Screen Space (1:1 mapping)
Room at (x: 10, y: 5) renders starting at terminal column 10, row 5
Room width=8, height=6 → occupies terminal cols 10-17, rows 5-10
```

**Camera:** Still tracks viewport (x, y, width, height) in terminal space, but no tile_scale conversion needed.

### Room Rendering

Each room is a filled rectangle:

1. **Walls (perimeter):** Draw `#` on edges (y=0, y=height-1, x=0, x=width-1)
2. **Floors (interior):** Draw `.` for all other cells
3. **Corners:** Use `#` (treat as walls, keep it simple)

**Example:** 4×3 room
```
####
#..#
####
```

### Door Rendering

Doors replace wall characters at exit positions:

- **North/South walls:** Replace `#` with `|` (vertical door)
- **East/West walls:** Replace `#` with `-` (horizontal door)

**Example:** 6×4 room with North exit at position 3, East exit at position 1
```
###|##
#....#
#....-
######
```

### Grid Alignment

Rooms are positioned with spacing (3 cells minimum between rooms):

```
Room A ends at x=10 → Room B (connected via exit) starts at x=14
  x=10: Room A east wall
  x=11-13: corridor/empty space
  x=14: Room B west wall
```

**Corridors:** Just empty space (background char) between rooms. Doors mark entry/exit points.

### Rendering Algorithm

```rust
for room in dungeon.rooms {
    // 1. Draw walls and floors
    for dy in 0..room.height {
        for dx in 0..room.width {
            let screen_x = room.x + dx - camera.x;
            let screen_y = room.y + dy - camera.y;
            
            // Skip if outside viewport
            if screen_x < 0 || screen_y < 0 || screen_x >= camera.width || screen_y >= camera.height {
                continue;
            }
            
            let is_wall = (dy == 0 || dy == room.height-1 || dx == 0 || dx == room.width-1);
            let ch = if is_wall { '#' } else { '.' };
            
            draw_char(screen_x, screen_y, ch, wall_or_floor_color);
        }
    }
    
    // 2. Draw exits (overwrite walls)
    for exit in room.exits {
        let (exit_x, exit_y) = match exit.wall {
            Wall::North => (room.x + exit.position, room.y),
            Wall::South => (room.x + exit.position, room.y + room.height - 1),
            Wall::West  => (room.x, room.y + exit.position),
            Wall::East  => (room.x + room.width - 1, room.y + exit.position),
        };
        
        let screen_x = exit_x - camera.x;
        let screen_y = exit_y - camera.y;
        
        if in_viewport(screen_x, screen_y) {
            let door_char = match exit.wall {
                Wall::North | Wall::South => '|',
                Wall::East | Wall::West => '-',
            };
            draw_char(screen_x, screen_y, door_char, door_color);
        }
    }
}

// 3. Draw explorer (overwrites floor)
let explorer_screen_x = explorer_pos.0 - camera.x;
let explorer_screen_y = explorer_pos.1 - camera.y;
draw_char(explorer_screen_x, explorer_screen_y, '@', explorer_color);
```

## Camera Changes

**Remove:** tile_scale field and all conversion logic  
**Keep:** x, y, width, height (now in dungeon space = screen space)  
**Update methods:**
- `update(explorer_pos)` — no conversion needed, explorer_pos is already in correct space
- `center_on(pos)` — direct calculation
- `is_visible(pos)` — simple bounds check

## What Chunk Implements

1. **Rewrite canvas.rs:**
   - Delete TILE_SCALE constant
   - Delete get_scaled_wall_char() method
   - Delete render_corridors() method (corridors are just gaps)
   - Simplify render_room() to double loop with wall/floor logic
   - Simplify render_exits() to single character overwrite
   - Simplify render_explorer() to single character

2. **Update camera.rs:**
   - Remove tile_scale field
   - Remove all "dungeon space → screen space" conversion math
   - Update update(), center_on(), is_visible() to work with 1:1 coordinates

3. **Theme colors remain the same** (wall, floor, door, explorer, background)

## Aesthetics Match

Frank's example:
```
Walls: #
Floors: .
Doors: | or -
Explorer: @
```

This design delivers exactly that. Clean, readable, no complex tile math.

## Trade-offs

**Gained:**
- Simple mental model (1 cell = 1 char)
- Easy debugging (coordinates match screen)
- No scaling bugs
- Less code (300 → ~100 lines in canvas.rs)

**Lost:**
- Large rooms harder to see details (but Frank said current is too small anyway)
- Less visual variety (no fancy wall patterns)
- Explorer sprite is single char (but that's what Frank wants)

## Testing

Brand should update camera tests to remove tile_scale expectations. Rendering tests can use simpler fixtures (no need to account for 4×4 expansion).

---

**Next Step:** Route to Chunk for implementation.

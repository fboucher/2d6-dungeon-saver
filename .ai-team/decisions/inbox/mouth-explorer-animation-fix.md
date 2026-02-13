# Explorer Animation: Corridor Pathfinding and Movement Speed

**Date:** 2025-01-20  
**By:** Mouth  
**Status:** Implemented

## Problem

Explorer was not moving despite `update()` being called every frame. The `@` character remained static at its starting position, making the screensaver appear broken.

## Root Cause

The pathfinder only marked room interior tiles as walkable. Since rooms are positioned with 1-tile gaps between them (see `DungeonGenerator::calculate_room_position()`), there were no walkable paths connecting rooms. The A* algorithm couldn't find routes between rooms.

## Solution

### 1. Corridor Tile Generation
Modified `Pathfinder::new()` to add corridor tiles connecting rooms through their exits:
- Each exit creates one walkable tile in the direction of its wall
- North exits: add tile at (room.x + position, room.y - 1)
- South exits: add tile at (room.x + position, room.y + height)
- East exits: add tile at (room.x + width, room.y + position)
- West exits: add tile at (room.x - 1, room.y + position)

### 2. Movement Speed Control
Added `move_cooldown` to Explorer to throttle movement from 10 tiles/sec to ~3 tiles/sec:
- Cooldown of 2 ticks between moves (move every 3rd frame)
- Makes animation visible and smooth over SSH connections
- Prevents explorer from "teleporting" across the screen

### 3. Visual Corridor Rendering
Added `render_corridors()` to show connecting tiles between rooms as floor dots (`.`), making the dungeon connectivity visually clear.

## Impact

- **Explorer AI:** Now successfully pathfinds between all rooms and explores the entire dungeon
- **Animation:** Visible, smooth movement at ~3 tiles/sec (down from 10 tiles/sec)
- **User Experience:** Screensaver actually animates instead of appearing frozen

## Files Changed

- `src/explorer/pathfinder.rs`
- `src/explorer/behavior.rs`
- `src/renderer/canvas.rs`
- `examples/test_explorer_movement.rs` (test harness)

## Testing

Created `test_explorer_movement.rs` example that simulates 100 frames:
- Verified 9 movements over 100 frames (~3 tiles/sec)
- Confirmed explorer discovers rooms and transitions states correctly
- Pathfinding works across all 20 rooms in generated dungeon

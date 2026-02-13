# Decision: 2x2 Tile Scaling for Readable Rendering

**By:** Chunk  
**Date:** 2025-01-20

## What

Implemented 2x2 tile scaling system where each dungeon cell renders as a 2×2 block of terminal characters. Introduced coordinate space separation: Camera operates in screen space (terminal characters), while Explorer and Pathfinder operate in dungeon space (dungeon cells).

## Why

**Problem:** User testing over SSH revealed unreadable rendering - dungeons looked like "a bunch of lines and plus, not like a map or a plan floor." With 1:1 rendering (1 dungeon cell = 1 terminal char), a 6x6 room was only 6x6 characters on screen. Corridors (1×1 cells) were invisible. Small rooms were illegible.

**Solution:** Scale up tiles to 2×2 terminal characters per dungeon cell. This makes:
- 6×6 room → 12×12 characters (clearly visible)
- Corridors → 2 characters wide (readable)
- Walls distinguishable from floors
- Explorer prominent (2×2 sprite vs single `@`)

## Implementation

1. **Constant:** `TILE_SCALE = 2` in canvas.rs (easy to adjust if needed)

2. **Rendering:** Each dungeon cell loops through TILE_SCALE×TILE_SCALE terminal chars:
   - Corners: `█` (solid block)
   - Walls: `──` (horizontal), `││` (vertical)
   - Floors: space (empty, cleaner than `.`)
   - Doors: `▢` in center, spaces around (creates opening)
   - Explorer: `@` in center, `·` on outline

3. **Camera:** Converts dungeon coords to screen coords:
   ```rust
   screen_x = dungeon_x * TILE_SCALE
   screen_y = dungeon_y * TILE_SCALE
   ```
   - Camera viewport stays in screen space
   - Explorer/Pathfinder stay in dungeon space
   - Translation happens at camera update/render time

4. **Tests:** Updated to account for coordinate space separation

## Impact

- **Readability:** Night-and-day improvement - floor plans are immediately recognizable
- **SSH Compatibility:** Uses basic Unicode box drawing, no font dependencies
- **Performance:** Negligible impact (scales linearly with screen area, not dungeon complexity)
- **Explorer/Pathfinding:** Zero changes needed (still operate in dungeon space)

## Trade-offs

- **Viewport Size:** Effective dungeon area visible on screen is halved (80×24 screen sees 40×12 dungeon cells)
- **Scroll Distance:** Camera pans more frequently (explorer covers more screen space per move)
- **Memory:** Minimal (same dungeon data, just different rendering)

## Alternatives Considered

1. **1×1 tiles (original):** Rejected - too cramped, unreadable
2. **3×3 tiles:** Excessive scrolling, too little visible dungeon area
3. **Dynamic scaling based on terminal size:** Added complexity, 2×2 works well for standard terminals
4. **Colored blocks instead of box drawing:** SSH compatibility issues, harder to read

## Next Steps

If users find 2×2 too large or small on ultra-wide/tiny terminals, we can:
- Make TILE_SCALE configurable via CLI flag
- Auto-scale based on detected terminal size
- But for now, 2×2 is the right default for 80×24 to 120×40 terminals

## Files Changed

- src/renderer/canvas.rs
- src/renderer/camera.rs  
- tests/camera_rendering.rs

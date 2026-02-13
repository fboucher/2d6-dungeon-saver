### 2025-01-20: Room Spacing Increased to 3 Cells for Proper Visual Separation

**By:** Chunk

**What:** Increased room spacing from 1 to 3 dungeon cells and updated corridor rendering to draw 3-tile connecting paths. This provides proper visual separation when rooms are rendered with 2x2 tile scaling.

**Why:** With 2x2 tile scaling (each dungeon cell = 2x2 terminal characters), the original 1-cell spacing between rooms was too cramped. Rooms appeared to overlap and corridors were barely visible. The visual rendering looked "like everything stacked on top of each other" (user feedback). By increasing spacing to 3 cells, we get 6 screen characters of separation (3 cells × 2 characters), which creates clean, readable dungeon layouts.

**Implementation:**
- Added `ROOM_SPACING = 3` constant in `src/dungeon/generator.rs` 
- Updated `calculate_room_position()` to use ROOM_SPACING instead of hardcoded +1
- Modified `render_corridors()` in `src/renderer/canvas.rs` to draw 3-tile corridor lines
- Corridors use directional characters (│ for vertical, ─ for horizontal)

**Impact:**
- Dungeons now look like clean floor plans with distinct rooms and visible corridors
- Matches Frank's requirements: "clean separate rooms with proper spacing, clear room boundaries"
- All tests passing (no regression)
- Visual verification shows proper spacing (e.g., Room 0 at (10,10) size 2×5, next room at (15,10))

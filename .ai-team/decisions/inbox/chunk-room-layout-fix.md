### 2025-01-20: Room Layout System for Spatial Positioning

**By:** Chunk

**What:** Implemented RoomLayouter algorithm to fix critical rendering bug where all rooms were positioned at (0,0), causing overlapping and fragmented display. New layout system positions rooms spatially based on exit connections.

**Why:** 

1. **Bug Discovery:** User reported broken rendering with misaligned walls, fragmented rooms, and doors showing as `▢`. Investigation revealed all generated rooms (except entrance) had placeholder coordinates (0,0), causing them to render on top of each other.

2. **Missing Implementation:** Data's Phase 2 dungeon generation left room positioning as "Will be positioned later" placeholder. Phase 3 and 4 integrated without addressing this gap, assuming layout would be handled elsewhere.

3. **Layout Algorithm Choice:** Simple parent-child connection approach based on room exits:
   - Entrance anchors at (10,10)
   - Each new room connects to most recent room with exits
   - Position calculated from parent's first exit direction
   - Fallback to grid pattern for rooms without viable parents

4. **Separation of Concerns:** Keeping layout separate from 2D6 generation rules maintains clean architecture - generator focuses on room properties/exits, layouter handles spatial positioning.

**Implementation:**
- `src/dungeon/layout.rs`: New module with RoomLayouter::layout() function
- `src/dungeon/generator.rs`: Calls layout after room generation
- `src/dungeon/mod.rs`: Exports layout module

**Testing:** All 54 existing tests still pass (41 lib + 13 integration). Visual verification confirms rooms now render at distinct coordinates.

**Impact:** Fixes critical Phase 3 rendering regression. Explorer movement and camera panning now work correctly with properly spaced rooms.

### 2025-01-20: Explorer AI targets unexplored exits instead of room centers

**By:** Chunk

**What:** Refactored Explorer behavior to path toward exits with `connected_room_id: None` rather than centers of unvisited rooms.

**Why:** 
- Progressive generation starts with only an entrance room, which the explorer immediately marks as visited
- Targeting room centers caused the explorer to path to the entrance center where it already stood → zero movement
- Explorer never reached exits → no new rooms generated → dungeon stuck at 1 room
- Targeting unexplored exits ensures the explorer moves toward generation triggers, enabling progressive dungeon growth

**Impact:**
- Explorer now actively seeks out exits that lead to undiscovered areas
- Dungeon generation progresses as explorer reaches unexplored exits
- Transition to Wandering state happens when all exits are explored (dungeon fully generated)
- Movement cooldown and pausing behavior unchanged

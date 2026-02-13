### 2025-01-20: Progressive Room Generation Synchronization Fix

**By:** Chunk

**What:** Fixed critical bug where all progressively-generated rooms were positioned at identical coordinates, causing overlapping/stacked visual appearance. Root cause was passing array index instead of room.id to generator, plus lack of exit connection synchronization between generator's internal state and main dungeon vec.

**Why:** 

1. **Index vs ID confusion:** `detect_unexplored_exit()` used `enumerate()` which returns array indices, but passed this to `generator.add_room()` which expects entity IDs. Generator searches rooms using `r.id == parent_id`, so passing index 5 when looking for room ID 5 works initially but breaks if IDs diverge from indices.

2. **Dual state synchronization:** Generator maintains `self.rooms` vec (internal state with updated exit connections), while main.rs maintains separate `dungeon` vec (for rendering/pathfinding). When generator marks an exit as connected in self.rooms, the main dungeon vec still shows it as unexplored, causing the same exit to trigger multiple room generations on consecutive frames.

**Solution:** Return `room.id` from detect_unexplored_exit instead of enumerate index, and synchronize exit connections in main dungeon vec after room generation.

**Impact:** Rooms now position correctly relative to parents (North/South/East/West), creating clean separated dungeon layout as intended. Visual went from "cramped overlapping mess" to "clear room separation with proper spacing."

**Related Files:**
- src/main.rs: detect_unexplored_exit() and room generation loop
- src/dungeon/generator.rs: add_room() and calculate_room_position()

## Project Context

**Project:** 2d6-dungeon-saver — a terminal screensaver written in C# (.NET) that simulates a dungeon explorer navigating rooms by rolling dice (2D6 Dungeon game rules).
**Requested by:** Frank Boucher
**Branch:** squad/dungeon-fixes (worktree-local .squad/ state)

**Tech Stack:** C# / .NET 10, console/terminal rendering

**Key Source Files:**
- `src/Core/GameLoop.cs` — main game loop
- `src/Core/ExplorerAI.cs` — explorer decision-making
- `src/Core/DungeonBuilder.cs` — builds dungeon structure
- `src/Core/RoomGenerator.cs` — generates rooms
- `src/Core/ExitGenerator.cs` — generates exits/doors
- `src/Core/Pathfinder.cs` — pathfinding
- `src/Models/Room.cs` — room model
- `src/Models/Exit.cs` — exit/door model
- `src/Models/Dungeon.cs` — dungeon model
- `src/Models/Explorer.cs` — explorer model
- `src/Rendering/Renderer.cs` — terminal rendering
- `src/Rendering/ColorTheme.cs` — color theming
- `src/Utils/Rectangle.cs`, `src/Utils/Point.cs` — geometry helpers

**Known Issues (from notes.md):**
1. When the explorer opens a door, the wall between the current room and the next should be adjoining (shared), but currently it's duplicated — causing the explorer to get stuck in the new room.
2. When the explorer opens a door, they teleport to the middle of the next room instead of walking through the door to the correct entry position.

**Docs:**
- `docs/2d6 Rules.md` — official game rules
- `docs/Rules.md` — technical design doc
- `docs/2d6-flow-page1.png`, `docs/2d6-flow-page2.png` — game flow diagrams

## Learnings

### 2025-03 — Fixed double wall bug and unexplored door rendering

**Problem:** Adjacent rooms were rendering with double walls (`##`) instead of sharing a single wall (`#`). Also, unexplored exits showed as `+` in map exports instead of `?`.

**Root Cause:**
1. `Rectangle.Intersects` used `<=`/`>=` which treated touching rooms as overlapping
2. `DungeonBuilder.CalculateNewRoomPosition` placed rooms 1 cell too far from exits
3. `MapExporter.GetCharAt` ignored exit exploration state

**Fixes:**
1. **src/Utils/Rectangle.cs** — Changed `Intersects` to use strict inequalities (`<` and `>`). Rooms that share a wall edge are now correctly treated as non-overlapping.
2. **src/Core/DungeonBuilder.cs** — Fixed `CalculateNewRoomPosition` for all 4 directions so adjacent rooms share their boundary wall:
   - East: new room's LEFT wall = exitPos.X (removed +1)
   - West: new room's RIGHT wall = exitPos.X (added +1)
   - South: new room's TOP wall = exitPos.Y (removed +1)
   - North: new room's BOTTOM wall = exitPos.Y (added +1)
3. **src/Core/MapExporter.cs** — Fixed `GetCharAt` to return `?` for unexplored exits and `+` for explored ones. Updated legend to include both symbols.

**Key Insight:** Adjacent rooms share walls, not gaps. The exit sits ON the shared wall column/row.

### 2025-03 — Fixed explorer teleportation through doors

**Problem:** When the explorer reached a door (exit tile), they would instantly teleport to the middle of the new room instead of walking through frame by frame.

**Root Cause:** `CheckExitCrossing` in `src/Core/ExplorerAI.cs` was overriding `_explorer.Position` to a point deep inside the connected room and clearing the path. This caused instant teleportation instead of continuous movement.

**Fixes:**
1. **src/Core/ExplorerAI.cs** — Simplified `CheckExitCrossing` to only mark exits as explored and reveal connected rooms. Removed position override and path clearing. `UpdateCurrentRoom` already handles room transitions naturally.
2. **src/Core/ExplorerAI.cs** — Enhanced `UpdateCurrentRoom` to detect when the explorer is on an exit tile and has moved into the connected room's bounds, properly transitioning CurrentRoom.
3. **src/Core/ExplorerAI.cs** — Updated `NavigateToExit` to path one step INTO the connected room (not just TO the exit tile), creating a continuous walking path through the door.
4. **src/Core/Pathfinder.cs** — Extended `FindPath` to accept an optional connected room parameter, allowing cross-room pathing through exit tiles.

**Key Insight:** Movement should be continuous. No position overrides. The pathfinder already treats exits as walkable, so extending paths to cross room boundaries creates natural frame-by-frame transitions.


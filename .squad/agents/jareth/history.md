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

## Room Adjacency Model: Shared-Wall (2026-03-11)

**Critical Model for Your Work:** Adjacent rooms in the dungeon **SHARE their boundary wall**, not create separate walls with a gap.

### The Model

When two rooms are adjacent via an exit:
- The exit position sits ON the shared wall coordinate
- The new room's boundary edge is placed AT the exit position (same coordinate as parent room's opposite edge)
- There is NO gap between rooms
- `Rectangle.Intersects` uses strict inequalities (`<`, `>`), so touching rooms are NOT flagged as collisions

### Example: East Exit

```
Parent Room (columns 0-10)        New Room (columns 10-20)
         v shared wall
         10
  0------+-------10  10---------+--------20
  |      |        |  |          |         |
  | ROOM |        |  |  NEW RM  |         |
  |      |        |  |          |         |
  0------+-------10  10---------+--------20
         ^ exit here on shared wall
```

The exit (`+`) appears at column 10, which is both the parent room's right edge and the new room's left edge.

### Rendering

- `GetRoomAt(position)` returns the parent room at the shared wall coordinate
- Exit rendering is checked BEFORE wall rendering
- Result: `+` displays on the shared wall (not a double `##`)

### Map Symbols

- `#` — room wall
- `?` — unexplored exit
- `+` — explored exit
- `.` — empty floor/interior

### Files to Know

- `src/Utils/Rectangle.cs` — `Intersects` method (strict inequalities)
- `src/Core/DungeonBuilder.cs` — `CalculateNewRoomPosition` (implements shared-wall placement)
- `src/Rendering/MapExporter.cs` — `GetCharAt` (renders symbols correctly)

## Learnings

### Code Pathways (Explorer Jump Diagnosis, 2026-03-11)

**Pathfinder fallback is the teleport vector:** `Pathfinder.FindPath` (line 76-77) unconditionally returns `{ start, goal }` when A* fails. This is a **direct-line fallback with no validation**—it will jump the explorer across empty space if the goal is unreachable.

**TryAdjustRoomPosition breaks entry points:** When collision avoidance moves a room by ±1 or ±2 squares (line 141-164 in DungeonBuilder), the room's interior shifts. But `GetStepInsideRoom` (ExplorerAI line 207-220) assumes the interior is exactly one step from the exit. If the room was adjusted, that target point may be outside both the original and adjusted room bounds, causing A* to fail and trigger fallback.

**Shared-wall model hidden assumption:** The decision document (decisions.md) says rooms share walls and exits are on the boundary. But when a room is adjusted, the exit position (on the original shared wall) no longer aligns with the adjusted room's interior. Code assumes static room positions; adjustments break this contract invisibly.

**Stale path context after room adjustment:** ExplorerAI's `NavigateToExit` (line 193-198) calls `FindPath(start, target, currentRoom, targetRoom)`. If `targetRoom` was adjusted after generation, the pathfinder's `IsWalkable` checks (line 143-154) still use the stale `connectedRoom` bounds, not the room's actual position in the dungeon.

**UpdateCurrentRoom weak transition detection:** The shared-wall peek-ahead check (line 73: `exit.ConnectedRoom.Bounds.Contains(_explorer.Position)`) only works if the explorer is at least one step into the adjusted room's interior. But if the explorer jumps via fallback to a floating disconnected room, this check may falsely pass or the room may not exist in GetRoomAt, leaving CurrentRoom stale.

**Recommendation priority:**
1. **Immediate:** Kill the fallback path. Return empty list, not `{ start, goal }`. This prevents silent jumps.
2. **Short-term:** Validate targets in `NavigateToExit` before pathfinding. Ensure target is inside the connected room.
3. **Long-term:** Decouple exit position from room bounds. Store room position changes and update exit references.


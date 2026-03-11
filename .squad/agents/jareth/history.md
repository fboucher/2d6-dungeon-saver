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


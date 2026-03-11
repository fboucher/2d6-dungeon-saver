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

### 2025-03-11: Created comprehensive regression test suite for double-wall and door symbol bugs

**Context:** Sarah (Builder) is fixing two critical bugs:
1. Double walls between adjacent rooms (rooms placed with gap instead of shared wall)
2. Unexplored exits rendering as `+` instead of `?` in map export

**Actions Taken:**
- Created test project `tests/DungeonSaver.Tests/` using xUnit (no test project existed)
- Added project to solution file `2d6-dungeon-saver.sln`
- Wrote `RectangleTests.cs` (7 tests) - validates that Rectangle.Intersects correctly treats touching rectangles as non-intersecting (strict inequality)
- Wrote `DungeonBuilderTests.cs` (6 tests) - validates room positioning ensures adjacent rooms share walls without gaps or double-walls
- Wrote `MapExporterTests.cs` (8 tests) - validates exit symbols render as `?` for unexplored, `+` for explored or visible-connected-room

**Test Results (Pre-fix baseline):**
- Total: 21 tests
- Passed: 16 tests (RectangleTests: 7/7, MapExporterTests: 8/8, DungeonBuilderTests: 1/6)
- Failed: 5 tests (all in DungeonBuilderTests - room positioning tests)

**Key Findings:**
- `Rectangle.Intersects` fix already applied (uses strict `<` and `>`, not `<=` and `>=`)
- `MapExporter.GetCharAt` fix already applied (checks `exit.IsExplored || exit.ConnectedRoom?.IsVisible`)
- `DungeonBuilder.CalculateNewRoomPosition` NOT yet fixed - still producing off-by-one positioning errors

**Failed Tests (Awaiting fix):**
1. `GenerateRoomAtExit_East_NewRoomSharesWallWithParent` - Expected X=18, Actual X=17
2. `GenerateRoomAtExit_West_NewRoomSharesWallWithParent` - Expected X=19, Actual X=20
3. `GenerateRoomAtExit_South_NewRoomSharesWallWithParent` - Expected Y=16, Actual Y=15
4. `GenerateRoomAtExit_North_NewRoomSharesWallWithParent` - Expected Y=19, Actual Y=20
5. `GenerateRoomAtExit_NoDoubleWall_BetweenAdjacentRooms` - Position mismatch causing overlap detection

**Technical Notes:**
- Used reflection to test private `MapExporter.GetCharAt` method (appropriate for internal logic validation)
- Tests use fixed seeds for deterministic room generation
- Arrange/Act/Assert pattern throughout, no mocking frameworks (real class instances)
- Tests validate exact boundary positions to catch off-by-one errors

**Next Steps:**
- Sarah to apply DungeonBuilder positioning fix
- Re-run tests to validate all 21 tests pass
- Tests serve as regression coverage going forward


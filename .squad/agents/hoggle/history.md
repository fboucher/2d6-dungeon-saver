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

## Work Log

### 2025-03-12: Enhanced Movement Trace Diagnostic Detail

**Task:** Add richer diagnostic information to the movement trace for room placement debugging.

**Changes Made:**

1. **DungeonBuilder.cs - Enhanced RetryAttempt logging:**
   - Added attempt number in format `[N/20]`
   - Added room bounds in format `bounds(x,y,w,h)`
   - Added specific failure reasons: "ok", "collision", "corner", "off-boundary"
   - Created `GetReachabilityFailureReason()` method to distinguish between corner placement vs off-boundary issues
   - Track last attempted room and failure reason for SealDoor logging

2. **DungeonBuilder.cs - Enhanced SealDoor logging:**
   - Added retry count "20 retries failed"
   - Added last attempted room bounds and failure reason
   - Format: `Dir:{direction} | 20 retries failed | last: bounds(x,y,w,h) {reason}`

3. **DungeonBuilder.cs - Enhanced RoomGenerated logging:**
   - Added room bounds to show final placed room geometry
   - Format includes dice log, bounds, and exit dice log

4. **MapExporter.cs - Updated trace formatting:**
   - Adjusted RetryAttempt formatting for better alignment with new detail
   - Fixed action variable scoping (moved into conditional blocks)

**Test Results:** All 29 tests pass ✅

**Impact:** Frank can now understand WHY placement decisions are made by seeing:
- Detailed retry attempts with bounds and failure reasons
- Retry counts and last attempted bounds for sealed doors
- Room bounds for all generated rooms
- Specific failure reasons (corner, collision, off-boundary) instead of generic "blocked"

## Learnings


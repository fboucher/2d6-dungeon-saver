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

### 2025-03 — Added movement trace logging for diagnostics

**Problem:** Explorer was observed jumping to disconnected areas. Needed diagnostic trace to identify root cause of teleportation issues.

**Solution:** Implemented comprehensive movement trace logging system capturing all significant movement events.

**Implementation:**
1. **src/Models/Explorer.cs** — Added `MovementEvent` record and `MovementTrace` list with `AddTrace()` method that caps at 1000 events (rolling buffer).
2. **src/Core/ExplorerAI.cs** — Added trace logging for:
   - `Move`: Every position change with from/to coordinates
   - `ExitCrossed`: When explorer crosses an exit threshold
   - `RoomSwitch`: When CurrentRoom changes (logs from/to room IDs)
   - `PathPlanned`: When path is calculated to an exit
   - `PathFallback`: When pathfinder returns fallback direct line (potential smoking gun)
3. **src/Core/MapExporter.cs** — Added movement trace section to map export:
   - Shows last 500 events (capped for readability)
   - Groups consecutive Move events in same room ("Move x5 Room:2 (10,12)→(15,12)")
   - Formats with timestamps, action types, room IDs, positions, and details
4. **src/Core/GameLoop.cs** — Updated `ExportMap` call to pass `_explorer` parameter.

**Key Insight:** Diagnostic traces are essential for debugging complex state machines. The trace captures the complete movement history, making it easy to identify when and why unexpected position changes occur.

### 2025-03 — Fixed explorer backtracking via BFS

**Problem:** When the explorer exhausted all unexplored exits in the current room, it would stop exploring and wander randomly forever. It never backtracked through already-explored doors (`+`) to find other rooms in the dungeon that still had unexplored exits (`?`).

**Root Cause:** `FindExitToRoomWithUnexploredExits` in `src/Core/ExplorerAI.cs` only checked rooms ONE hop away (directly connected to current room). If those rooms were also fully explored, it returned null and the explorer entered wandering mode — never searching deeper into the dungeon graph.

**Fixes:**
1. **src/Core/ExplorerAI.cs** — Replaced `FindExitToRoomWithUnexploredExits()` with `FindPathToNearestUnexploredExit()` that uses BFS to traverse the entire connected room graph. Finds the nearest room (by hop count) with unexplored exits and returns the first exit from the current room that starts the path toward it.
2. **src/Core/ExplorerAI.cs** — Updated `DecideNextDestination` to use the new BFS method as priority 2, between local unexplored exits (priority 1) and wandering (priority 3).
3. **src/Core/ExplorerAI.cs** — Fixed dead-end handling in `FindUnexploredExit`: when `GenerateRoomAtExit` returns null (couldn't place a room), mark the exit as explored so BFS doesn't revisit it forever.
4. **src/Core/ExplorerAI.cs** — Added "Backtrack" movement trace event when BFS finds a path, making backtracking visible in movement logs.

**Key Insight:** Exploration requires graph traversal, not just local search. BFS naturally finds the shortest path to unexplored areas. Marking dead-end exits as explored prevents infinite retry loops.

### 2025-03 — Fixed entrance wall exclusion and retry logging

**Problem 1:** Room 2 was getting duplicate East exits and sealed doors. The entrance wall was not being excluded correctly when generating exits.

**Root Cause:** `GetAvailableWalls` in `src/Core/ExitGenerator.cs` was removing the OPPOSITE of the entrance direction instead of the entrance direction itself. When Room 1's West exit created Room 2 with entrance direction East, it should have excluded East from available walls but instead excluded West. This caused Room 2 to generate an exit on its entrance wall (East), creating a duplicate exit that led to failed placement and sealed doors.

**Problem 2:** Retry attempts in `GenerateRoomAtExit` were not being logged to the trace, making it impossible to see what dice rolls were attempted when room placement failed.

**Fixes:**
1. **src/Core/ExitGenerator.cs** — Changed `GetAvailableWalls` line 95 to remove `entranceDirection.Value` directly instead of `GetOppositeDirection(entranceDirection.Value)`. Now correctly excludes the actual entrance wall.
2. **src/Core/DungeonBuilder.cs** — Added `RetryAttempt` logging inside the retry loop (lines 96-122). Each retry now logs a `GenerationLogEntry` with the candidate's dice log and whether it passed (`"ok"`) or failed (`"blocked"`).
3. **src/Core/MapExporter.cs** — Added `RetryAttempt` case in `FormatMovementEvent` to render retry attempts in the trace output with proper formatting.

**Key Insight:** When calculating which walls are available for exit placement, the entrance direction passed to `GenerateExits` is already the actual entrance wall — no need to flip it. The confusion came from the fact that `DungeonBuilder.GenerateRoomAtExit` already calls `GetOppositeDirection` to compute the entrance direction before passing it to `GenerateExits`.

### 2026-03-12 — Fixed fast-path sealing bug

**Problem:** When a room placement had no collision but `IsExitReachableInRoom` returned false (e.g., exit landed at room corner after `EnsureSeparation` shift), the fast path sealed the door immediately with ZERO retries. Meanwhile, the collision path had a 20-attempt retry loop. This caused many doors to be sealed prematurely even when different room shapes could have fit.

**Real Examples:**
- Room 4 South exit (59,64) — sealed with "exit at room corner"
- Room 14 West exit (44,45) — sealed with "exit at room corner"
- Room 14 North exit (46,43) — sealed with "exit at room corner"
- Room 19 East exit (35,40) — sealed with "exit at room corner"

**Root Cause:** The fast path (no collision on initial placement) executed:
```
EnsureSeparation → no collision → ClampToBoundary → IsExitReachableInRoom → FALSE → SEAL IMMEDIATELY ❌
```

The slow path (collision on initial placement) correctly executed:
```
TryAdjustRoomPosition → IsExitReachableInRoom → if false → retry loop (20 attempts) ✓
```

**Fixes:**
1. **src/Core/DungeonBuilder.cs** — Refactored `GenerateRoomAtExit` so both collision AND reachability failure trigger the same retry loop. Lines 71-171 now:
   - Check initial room for collision OR reachability failure
   - If either fails, enter unified retry loop (20 attempts)
   - Each retry generates a fresh room shape and tests both collision AND reachability
   - Only after all 20 attempts fail does it seal the door
2. **src/Core/DungeonBuilder.cs** — Added `GetReachabilityFailureReason` helper method that returns:
   - "corner" if exit is at a room corner (all 4 adjacent tiles are walls or outside)
   - "off-boundary" if exit position is outside room bounds
   - "collision" if room overlaps with existing rooms
3. **src/Core/DungeonBuilder.cs** — Enhanced retry logging to include:
   - Detailed failure reason for each attempt (not just "blocked")
   - Room bounds in retry log: `"[2][3] 4x5 Normal - corner (bounds:55,64,6,4)"`
4. **src/Core/DungeonBuilder.cs** — Changed SealDoor message from "exit at room corner" to "no valid placement found" since we now try 20 different shapes before sealing.

**Key Insight:** Both collision and reachability are placement constraints that should trigger the same retry logic. Treating them differently created an asymmetry where some sealings had zero retries while others had 20. The unified retry loop ensures every exit gets a fair chance with multiple room shapes before being sealed.

**Test Results:** All 29 tests pass, including `GenerateRoomAtExit_East_AllPositionsBlocked_ExitIsBlocked` which validates sealing when truly blocked.


### 2025-05 — Added dice result to RetryAttempt trace entries

**Problem:** `RetryAttempt` log entries in the generation trace showed the failure reason and bounds, but the attempt number wasn't shown in `[Retry       ]`, and the initial failed attempt (before the retry loop) was never logged at all.

**Fixes:**
1. **src/Core/DungeonBuilder.cs** — Added a log entry for the initial attempt (`0/20`) right before the retry loop starts, capturing `roomDiceLog` and the reason for failure (`collision` or reachability reason).
2. **src/Core/DungeonBuilder.cs** — Updated retry loop log entries to prefix the detail string with `{attempt+1}/20` so the attempt number travels with the log entry.
3. **src/Core/MapExporter.cs** — Updated `RetryAttempt` case in `FormatMovementEvent` to parse the `N/M` prefix out of the detail string and display it as `[Retry  N/20]`, with dice and failure reason following.

**Result:** Trace now shows: `Room:6    [Retry  1/20] (52,59)         [4][1] Corridor - corner (bounds:...)` for each attempt.

**Key Insight:** The detail string is the only data channel from `GenerationLogEntry` to `MapExporter`. Embedding the attempt number as a structured prefix (`N/20`) in the detail keeps `GenerationLogEntry` record unchanged and avoids adding new fields to the record.

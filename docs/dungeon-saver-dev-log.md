# 2D6 Dungeon Saver — Developer Session Log

> Branch: `squad/dungeon-fixes`  
> Session date: 2026-03-11

---

## Overview

C# / .NET 10 terminal screensaver that simulates an explorer autonomously navigating a procedurally generated dungeon following 2D6 Dungeon tabletop game rules.

This session focused on diagnosing and fixing explorer AI bugs — primarily around room placement geometry, pathfinder fallbacks, and backtracking logic — plus adding debugging and display improvements.

---

## Bugs Fixed

### 1. Room Placement Gap + Pathfinder Teleport + Explorer Freeze

**Symptom:** After opening a door, the explorer would freeze (infinite `[PathPlanned] Steps:1` spin).

**Root cause:** `TryAdjustRoomPosition` tried both X and Y offsets for all exit directions. For East exits, an X offset shifted the connected room, creating a gap column between rooms. `GetStepInsideRoom` targeted a point in the gap; A* failed; the old pathfinder fallback (`return new List<Point> { start, goal }`) teleported the explorer into the gap; `GetRoomAt` returned null there; the same exit kept being retargeted → trivial path → empty → infinite spin.

**Fixes:**
- **Axis-constrained slides:** East/West exits slide Y-only; North/South exits slide X-only
- **Retry with new room shape** (3x) before giving up
- **`Exit.IsBlocked`** property added; sealed doors log to `Dungeon.Messages` with room number
- **Pathfinder** now returns empty list (not `[start, goal]`) on A* failure
- **ExplorerAI** marks exit explored on empty path and moves on

---

### 2. Visual Double-Wall (Adjacent Unrelated Rooms)

**Symptom:** Room 7 placed immediately adjacent to an unrelated room, creating a double-wall and sealing Room 7's west exits.

**Root cause:** Centering formula put Room 7's left wall at the column immediately next to an existing room's right wall. `Rectangle.Intersects` uses strict `<`/`>` so touching rooms aren't flagged as collisions.

**Fix:** `EnsureSeparation()` in `DungeonBuilder.GenerateRoomAtExit` — after `CalculateNewRoomPosition`, checks if perpendicular walls are immediately adjacent to existing rooms and shifts by 1 to create separation. Applied before collision check (if shift causes collision, `TryAdjustRoomPosition` handles it).

---

### 3. Explorer Stuck When Dungeon at Capacity

**Symptom:** When all rooms were generated (dungeon at capacity), the explorer wandered forever in the last room.

**Root cause:** BFS `FindPathToNearestUnexploredExit` excluded exits with `ConnectedRoom==null` — both `hasUnexplored` (required `ConnectedRoom!=null`) and `canGenerate` (required `rooms.Count < TargetRoomCount`) returned false. With a full dungeon, the BFS found nowhere to go.

**Fix:** Simplified BFS to `bool hasUnexplored = current.Exits.Any(e => !e.IsExplored)` — navigates toward any room with unvisited exits regardless of capacity. The actual generation/dead-end logic stays in `FindUnexploredExit`.

---

### 4. One-Way Exit Connections (Explorer Stuck in New Rooms)

**Symptom:** Explorer entered a new room and immediately got stuck — couldn't backtrack.

**Root cause:** `GenerateExits` skips the entrance direction for new rooms, so new rooms have no exit pointing back to the source room. BFS seeds from exits that are `IsExplored && ConnectedRoom != null`, but all new-room exits are unexplored → BFS queue starts empty → returns null → wander loop.

**Fix (two-step):**
1. `GenerateRoomAtExit` manually adds a back-exit to new rooms:
   ```csharp
   var backExit = new Exit(exit.Position, entranceDir) { ConnectedRoom = fromRoom };
   newRoom.Exits.Add(backExit);
   ```
2. `CheckExitCrossing` marks the back-exit explored when the forward exit is crossed → two-way BFS traversal works.

---

## Features Added

### `--room-ids` CLI Flag

Displays the room's numeric ID at the interior center of each room on the map. Enabled via:

```
dotnet run -- --room-ids
```

Flow: `Program.cs` parses flag → `GameLoop(showRoomIds)` → `MapExporter(showRoomIds)` → `GetCharAt` overlays ID digits.

### `X` for Sealed / Blocked Exits

Exits marked `IsBlocked` now render as `X` (uppercase) on the map. Legend updated. Previously they rendered as `+`, indistinguishable from open doors.

### Sealed Message Includes Room Number

When an exit is sealed, the log message now reads:  
`"Room N, exit Direction: sealed (reason)"`

---

## Architecture Notes

### Shared-Wall Model

Adjacent rooms share a boundary column/row. Room positioning for an East exit: `newRoom.X = exit.Position.X` (left wall = exit column). `Rectangle.Intersects` uses strict `<`/`>` so touching rooms are NOT treated as collisions.

### Room Generation Flow

```
ExplorerAI.Update
  → DecideNextDestination
  → FindUnexploredExit
  → DungeonBuilder.GenerateRoomAtExit
      → RoomGenerator.GenerateRoom (2D6 dice roll)
      → CalculateNewRoomPosition
      → EnsureSeparation
      → HasCollision check
      → TryAdjustRoomPosition (Y-only or X-only, 3 retries)
      → ExitGenerator.GenerateExits (skips entrance direction)
      → back-exit manually added
      → exit.ConnectedRoom = newRoom
```

### Map Symbols

| Symbol | Meaning |
|--------|---------|
| `#` | Wall |
| `.` | Floor |
| `+` | Explored door |
| `?` | Unexplored door |
| `X` | Sealed / blocked door |
| `@` | Explorer position |

---

## Files Modified

| File | Changes |
|------|---------|
| `src/Core/DungeonBuilder.cs` | Axis-constrained slides, retry logic, `EnsureSeparation()`, `IsBlocked` flag, sealed message with room ID, back-exit added |
| `src/Core/Pathfinder.cs` | Returns empty list instead of `[start, goal]` on A* failure |
| `src/Core/ExplorerAI.cs` | Empty path handler; BFS simplified; `CheckExitCrossing` marks back-exit explored |
| `src/Core/MapExporter.cs` | `showRoomIds` flag; `X` for blocked exits; room ID overlay on floor; legend updated |
| `src/Core/GameLoop.cs` | `showRoomIds` param threaded through to MapExporter |
| `src/Program.cs` | Parse `--room-ids` CLI flag |
| `src/Models/Exit.cs` | `IsBlocked` property added |
| `src/Models/Dungeon.cs` | `Messages` list added |
| `tests/DungeonSaver.Tests/DungeonBuilderTests.cs` | 3 new regression tests |
| `tests/DungeonSaver.Tests/PathfinderTests.cs` | New file — 1 test for empty path on unreachable goal |

---

## Test Coverage

24 tests total, all passing (`xUnit`, `net10.0`).

---

## Potential Edge Cases to Watch

- Explorer may exhibit edge-case wandering if it enters a room where ALL exits (including back-exit) are blocked/explored before the back-exit gets marked — the two-way connectivity fix should prevent this in most cases.
- `--room-ids` display may overlap with exit tiles if the room center happens to be an exit position — low priority.
- No tests yet cover the back-exit two-way connectivity behavior — worth adding if bugs resurface.

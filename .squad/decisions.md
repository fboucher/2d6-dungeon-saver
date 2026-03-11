# Squad Decisions

## Active Decisions

### Adjacent Rooms Share Walls

**Date:** 2025-03  
**Decided by:** Sarah (C# Developer)  
**Status:** Implemented

#### Context

When generating adjacent rooms in the dungeon, there was a bug causing double walls (`##`) to appear between rooms instead of a shared single wall (`#`). This created visual artifacts and could cause navigation issues.

#### Decision

Adjacent rooms in the dungeon share their boundary wall, not create separate walls.

When a new room is placed adjacent to an existing room via an exit:
- The exit position lies ON the shared wall
- The new room's boundary (left/right/top/bottom edge) is placed AT the exit position
- There is NO gap between rooms

#### Technical Implementation

1. **Collision detection** (`Rectangle.Intersects`): Rooms that share an edge (touching) are NOT considered overlapping. Changed from `<=`/`>=` to strict `<`/`>` comparisons.

2. **Room positioning** (`DungeonBuilder.CalculateNewRoomPosition`):
   - **East exit:** New room's LEFT wall = exitPos.X
   - **West exit:** New room's RIGHT wall = exitPos.X  
   - **South exit:** New room's TOP wall = exitPos.Y
   - **North exit:** New room's BOTTOM wall = exitPos.Y

#### Consequences

**Positive:**
- Maps render correctly with single shared walls
- Cleaner visual representation
- Consistent with dungeon generation conventions
- Explorer can move properly between adjacent rooms

**Related Files:**
- `src/Utils/Rectangle.cs` — collision detection
- `src/Core/DungeonBuilder.cs` — room positioning logic

### Room Transitions via Continuous Walking

**Date:** 2025-03  
**Proposed by:** Sarah (C# Developer)  
**Status:** Implemented

#### Context

The explorer was teleporting to the middle of new rooms when crossing through exits, breaking the illusion of continuous movement and making the animation feel unnatural.

#### Decision

Room transitions happen through continuous frame-by-frame walking, not position overrides.

When the explorer reaches an exit:
1. The exit is marked as explored
2. The connected room becomes visible
3. The explorer's path continues through the exit tile into the connected room
4. No position override occurs
5. `UpdateCurrentRoom` naturally detects the room transition as the explorer walks

#### Technical Implementation

**ExplorerAI Changes:**
- `CheckExitCrossing`: Only marks exits as explored and reveals connected rooms. Does NOT override position or clear path.
- `UpdateCurrentRoom`: Enhanced to detect when explorer is on an exit tile and has moved into the connected room's interior bounds.
- `NavigateToExit`: Creates paths that extend one step into the connected room (past the exit tile), ensuring continuous movement.

**Pathfinder Changes:**
- `FindPath`: Now accepts optional `connectedRoom` parameter to support cross-room pathing.
- `IsWalkable`: Checks both the primary room and optional connected room, allowing movement through exits into adjacent rooms.

#### Consequences

**Positive:**
- Natural, continuous movement through doors
- No jarring teleportation effects
- Explorer visibly walks through the transition
- Room changes detected automatically by position
- Consistent frame-by-frame animation

**Negative:**
- Slightly more complex pathfinding logic (but worth it for natural movement)

#### Related Files

- `src/Core/ExplorerAI.cs` — movement and room transition logic
- `src/Core/Pathfinder.cs` — cross-room pathfinding

### Root Cause: Explorer Teleportation

**Date:** 2026-03-11  
**Diagnosed by:** Jareth (Explorer & Pathfinder)  
**Status:** Documented

#### Context

Explorer was teleporting to disconnected room areas, breaking continuous movement immersion. Investigation revealed a collision between pathfinder fallback logic and room position adjustments.

#### Root Cause

**Primary:** Unconditional fallback path in `Pathfinder.FindPath`. When A* exhausts neighbors, returns `{ start, goal }` without validating walkability, enabling direct jumps of any distance.

**Secondary:** `GetStepInsideRoom` assumes target is exactly one step from exit, but `TryAdjustRoomPosition` can offset rooms ±2 squares, placing calculated targets in empty space. This breaks path validation and forces fallback.

**Tertiary:** When fallback jump lands in adjusted room, `UpdateCurrentRoom` shared-wall peek-ahead fails to detect transition, leaving `CurrentRoom` stale. Subsequent pathfinding treats old room as current, causing cascading disconnections.

#### Technical Details

- **Evidence location 1:** `Pathfinder.FindPath (line 76-77)` returns unconditional fallback
- **Evidence location 2:** `NavigateToExit (lines 173-205)` calculates target without validating against adjusted room bounds
- **Evidence location 3:** `GetStepInsideRoom (lines 207-220)` assumes fixed one-step offset (no adjustment awareness)
- **Evidence location 4:** `TryAdjustRoomPosition (lines 138-164)` moves room without re-validating exit-to-interior path
- **Evidence location 5:** `UpdateCurrentRoom (line 73)` uses stale room bounds after adjustment

#### Recommended Fix

1. **Validate target before pathfinding:** In `NavigateToExit`, after calculating target, verify `targetRoom.Contains(target)`. If not, adjust to valid interior floor point.

2. **Validate fallback path:** In `FindPath`, before returning `{ start, goal }`, check if goal is actually reachable. If not, return empty path and let ExplorerAI handle it (don't silently jump).

3. **Sync room position after adjustment:** In `DungeonBuilder.GenerateRoomAtExit`, after `TryAdjustRoomPosition`, recalculate entry target using final room bounds.

4. **Strengthen room transition validation:** In `UpdateCurrentRoom`, after room switch, validate new room is connected to known exit. If switch was caused by fallback jump into floating room, revert and flag error.

#### Trace Requirements

To verify the fix, capture and correlate:
- Room generation trace (initial position → collision check → adjustment → final bounds)
- Pathfinding trace (start/goal/result, fallback triggers)
- Movement trace (position changes, room switches, mismatches)
- Exit crossing trace (exit position, target calculated, connected room bounds)

**Related decision:** Movement Trace Event Format (Sarah, 2026-03-11)

#### Related Files

- `src/Core/Pathfinder.cs` — fallback logic and validation
- `src/Core/ExplorerAI.cs` — path following and room updates
- `src/Core/DungeonBuilder.cs` — room positioning and adjustment
- `src/Models/Explorer.cs` — movement trace (newly added)
- `src/Core/MapExporter.cs` — trace export

### Movement Trace Event Format

**Date:** 2026-03-11  
**Implemented by:** Sarah (C# Developer)  
**Status:** Implemented

#### Decision

Capture all significant movement events in a structured, timestamped trace. This enables post-mortem debugging of explorer movement issues (e.g., teleportation) by providing a complete event history.

#### Event Types

1. **Move** — Basic position change within same room
2. **ExitCrossed** — Explorer stepped onto exit tile with direction and connected room info
3. **RoomSwitch** — CurrentRoom changed during movement
4. **PathPlanned** — Path calculated to an exit with step count
5. **PathFallback** — Pathfinder returned fallback direct jump (no valid path found)

#### Implementation

```csharp
public record MovementEvent(
    DateTime Timestamp,
    Point From,
    Point To,
    string Action,
    int? RoomId,
    string? Detail
);
```

**Storage:** Rolling buffer of 1000 events in Explorer (≈ 100KB)  
**Export:** Last 500 events written to map file with Move grouping  
**Format:** `{HH:mm:ss.fff} [{Action,-12}] Room:{id,-3} {positions} {detail}`

#### Consequences

**Positive:**
- Complete diagnostic history of movement
- PathFallback events directly identify when pathfinding fails
- Timestamps enable correlation with room generation
- Grouped output keeps trace readable
- Memory bounded at 1000 events

**Negative:**
- Negligible performance overhead per event

#### Related Files

- `src/Models/Explorer.cs` — MovementEvent record + trace buffer
- `src/Core/ExplorerAI.cs` — event capture instrumentation
- `src/Core/MapExporter.cs` — trace export and grouping
- `src/Core/GameLoop.cs` — explorer reference passing

**Related decision:** Root Cause: Explorer Teleportation (Jareth, 2026-03-11)

## Governance

- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction

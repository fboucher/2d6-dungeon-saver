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

### BFS Backtracking for Explorer Navigation

**Date:** 2025-03  
**Decided by:** Sarah (C# Developer)  
**Status:** Implemented

#### Context

The explorer AI had three priorities when deciding where to move next:
1. Unexplored exits in the current room
2. Explored exits leading to directly connected rooms with unexplored exits (single-hop check)
3. Random wandering

When the explorer reached a room where all local and adjacent rooms were fully explored, it would enter wandering mode and never find the remaining unexplored areas of the dungeon.

#### Problem

`FindExitToRoomWithUnexploredExits()` only checked rooms ONE hop away from the current room. In a branching dungeon, this meant:
- Explorer enters Room A → explores all exits
- Explorer backtracks to Room B → explores all its exits
- Explorer backtracks to Room C (entrance) → all adjacent rooms (A, B) are fully explored
- Method returns null → explorer wanders forever
- Rooms D, E, F deeper in the dungeon (accessible via A or B) are never discovered

The explorer had no ability to backtrack MORE than one hop to find unexplored areas.

#### Decision

Replace single-hop adjacency check with **breadth-first search (BFS)** through the entire connected room graph.

##### Algorithm

`FindPathToNearestUnexploredExit()`:
1. Start from current room
2. Enqueue all explored exits leading to connected rooms
3. For each room visited via BFS:
   - Check if it has unexplored exits (already connected OR can generate new rooms)
   - If yes: return the FIRST exit from the current room that starts the path toward it
   - If no: enqueue its explored exits and continue searching
4. If BFS exhausts all reachable rooms: return null (dungeon fully explored)

##### Key Properties

- **Nearest room first:** BFS guarantees shortest path (fewest hops) to next unexplored area
- **Backtrack via explored doors:** Only traverses exits marked `IsExplored = true` with `ConnectedRoom != null`
- **No teleportation:** Returns the exit in the CURRENT room to take as first step
- **Handles ungenerated rooms:** Checks for exits where `ConnectedRoom == null` but `Rooms.Count < TargetRoomCount` (room generation still possible)
- **Avoids dead ends:** Dead-end exits (where `GenerateRoomAtExit` returned null) are marked explored, so BFS skips them

#### Implementation

**Changes:**
1. **src/Core/ExplorerAI.cs** — Added `FindPathToNearestUnexploredExit()` method (BFS implementation)
2. **src/Core/ExplorerAI.cs** — Updated `DecideNextDestination()` to call new BFS method as priority 2
3. **src/Core/ExplorerAI.cs** — Removed `FindExitToRoomWithUnexploredExits()` (replaced by BFS)
4. **src/Core/ExplorerAI.cs** — Fixed `FindUnexploredExit()` to mark dead-end exits as explored
5. **src/Core/ExplorerAI.cs** — Added "Backtrack" trace event when BFS finds a path

#### Consequences

**Positive:**
- Explorer now fully explores the entire connected dungeon graph
- No more premature wandering when unexplored areas remain
- BFS finds shortest path to next unexplored area (efficient navigation)
- Dead-end exits handled correctly (marked explored, not retried forever)
- Backtracking behavior visible in movement trace logs

**Negative:**
- Slightly more complex logic (BFS vs simple adjacency check)
- Small performance overhead (negligible for typical dungeon sizes)

**Edge Cases Handled:**
1. **Ungenerated exits:** BFS checks `ConnectedRoom == null` with `TargetRoomCount` limit
2. **Dead ends:** Exits that couldn't place a room are marked explored
3. **Fully explored dungeon:** BFS returns null → wandering is correct behavior

#### Related Files

- `src/Core/ExplorerAI.cs` — exploration logic and BFS implementation
- `src/Models/Explorer.cs` — movement trace ("Backtrack" action type)

### Move Room Generation to Exit Arrival

**Date:** 2025-07  
**Decided by:** Jareth (Lead)  
**Status:** Implemented

#### Problem

Two related issues share the same root cause:

1. **Frank's request:** "I would like the explorer to walk in front of the door before the next room is generated." Currently, `FindUnexploredExit()` calls `GenerateRoomAtExit()` *before the explorer moves at all*. The room appears instantly while the explorer is still across the room.

2. **Bug — door 6N sealed during backtrack:** Explorer backtracks from room 10 to room 6. While still walking in from room 10, `FindUnexploredExit()` fires for room 6's north exit, generation fails (no space north of room 6), and the door silently changes from `?` to `X` before the explorer reaches it. Frank sees a door seal for no visible reason.

#### Decision

Move room generation OUT of `FindUnexploredExit()` and INTO `CheckExitCrossing()`. Generation now happens when the explorer physically arrives at the door — not when the AI picks a destination.

#### Implementation Changes

**`FindUnexploredExit()` — Simplified to pure filter:**
- Iterates `CurrentRoom.Exits`
- Skips exits with `IsExplored || IsBlocked`
- Returns first match regardless of whether `ConnectedRoom == null`
- An exit with `ConnectedRoom == null && !IsExplored && !IsBlocked` is now valid: "a door the explorer should walk to"

**`CheckExitCrossing()` — Extended to handle unconnected exits:**
- When explorer reaches door position AND exit has no `ConnectedRoom`, is not explored, and is not blocked
- Attempts `GenerateRoomAtExit()`
- If generation leaves `ConnectedRoom == null`, marks `IsExplored = true` and adds `DoorSealed` trace
- If generation succeeds, falls through to existing connected-exit handling
- Sequential `if` blocks (using `continue` on position mismatch) prevent code duplication

**`FindPathToNearestUnexploredExit()` — Defensive filter:**
- Changed predicate from `!e.IsExplored` to `!e.IsExplored && !e.IsBlocked`
- Prevents BFS routing toward exits that can never be opened

**`NavigateToExit()` — No changes**
- Already targets exit position correctly

#### Edge Cases

- **Room-switch timing:** After generation succeeds, exit is marked explored and `ConnectedRoom` is set. On next tick, `UpdateCurrentRoom` detects explorer is on shared wall → room switch → discovery pause fires.
- **Pathfinding from door:** Explorer can path from entrance (exit tile) to any interior or other exit in new room; `IsWalkable` returns true for exit tiles.
- **BFS still finds rooms:** An unconnected, unexplored, unblocked exit satisfies `!e.IsExplored` check; BFS routes explorer to these doors.
- **No double-trigger:** After generation fails, `IsExplored = true` prevents re-trigger of the new unconnected block.

#### Consequences

**Positive:**
- Explorer visibly walks to doors before rooms appear
- Doors only seal when explorer is physically at the door
- Cleaner separation: `FindUnexploredExit` is pure selection, `CheckExitCrossing` is action point
- No changes to DungeonBuilder, Pathfinder, or models

**Negative:**
- Explorer may walk to doors that fail generation (wasted trip). Intentional — visual approach+seal is better than invisible remote sealing.

#### Technical Insight

Sequential `if` blocks in `CheckExitCrossing` (using `continue` on position mismatch) allow unconnected-exit block to fall through to connected block on success without duplicating mark-explored and room-visibility logic.

#### Related Files

- `src/Core/ExplorerAI.cs` — all changes (FindUnexploredExit simplified, CheckExitCrossing extended, FindPathToNearestUnexploredExit defensive filter)

## Governance

- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction

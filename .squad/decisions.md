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

## Governance

- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction

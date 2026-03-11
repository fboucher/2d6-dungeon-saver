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

## Governance

- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction

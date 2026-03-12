# Ludo — History

## Project Context

**Project:** 2d6-dungeon-saver — a C#/.NET 10 dungeon screensaver that procedurally generates rooms using 2D6 dice rules.

**User:** Frank Boucher

**Stack:** C# / .NET 10 / WinUI 3

**Tech Stack:** C# / .NET 10, console/terminal rendering

**Key Source Files:**
- `src/Core/DungeonBuilder.cs` — builds dungeon structure, places rooms at exits
- `src/Core/ExitGenerator.cs` — generates exits/doors on room walls
- `src/Core/RoomGenerator.cs` — generates room dimensions
- `src/Models/Room.cs` — room model with bounds and exits
- `src/Models/Exit.cs` — exit/door model
- `src/Models/Dungeon.cs` — dungeon structure containing all rooms and exits
- `src/Utils/Rectangle.cs`, `src/Utils/Point.cs` — geometry helpers

## Room Placement Algorithm

The core placement logic lives in `DungeonBuilder.GenerateRoomAtExit()`:

1. **CalculateNewRoomPosition()** — Centers the new room on the exit coordinate, then calls:
   - `EnsureSeparation()` — Shifts room by 1 to avoid double-walls (separates interior from boundary)
   - `TryAdjustRoomPosition()` — Slides room ±1 or ±2 perpendicular to exit direction to find clearance
2. **HasCollision()** — Checks if placement overlaps any existing room
3. **IsExitReachableInRoom()** — Verifies the exit position has an interior floor neighbor in the new room
4. **Retry Loop** — If initial placement has collision or exit unreachable, retries up to 20 times with fresh candidate rooms

### The Fast-Path Sealing Bug (Day 1 Finding)

When `HasCollision` is **false** but `IsExitReachableInRoom` is also **false**, the code seals the door with **ZERO retries**:

```csharp
if (!hasCollision)
{
    if (!IsExitReachableInRoom(candidate, exitPos))
    {
        // Door seals here — no retries!
        exit.IsSealed = true;
        return null;
    }
    return candidate; // Room placed successfully
}
```

**The Problem:** EnsureSeparation pushes the exit to a corner of the room, making it unreachable. The algorithm should **retry** with a different room shape, not seal the door.

**Fix Status:** Sarah is implementing retry logic when `IsExitReachableInRoom` fails.

## Known Placement Patterns & Issues

### Issue: Corner Exits After EnsureSeparation

When `EnsureSeparation()` shifts a room by 1 to avoid overlapping walls, it can push the exit position to a corner of the new room bounds — a position with no interior floor neighbors. Example:

- Exit at (10, 10) on the shared wall
- Room normally placed: (9, 9) to (14, 14) — exit is on the edge, has floor neighbors
- After EnsureSeparation shift: (9, 10) to (14, 15) — exit position now has NO interior neighbors

**Geometric Reality:** The exit is valid; the room shape is just poorly aligned. Retrying with a different room size would fix this.

### Issue: Unreachable Exits Due to Room Size/Shape

A room might be placed collision-free, but the exit position falls on an edge with no accessible interior. This usually means:
1. Room is too small relative to exit position
2. Exit landed on a corner of the room bounds
3. EnsureSeparation pushed the exit to an inaccessible edge

**The Fix:** Retry with a different candidate room rather than sealing.

## Key Geometric Concepts

### Adjacent Rooms Share Walls (Not Gaps)

Two rooms are adjacent when they share a wall edge:
- Room A: (0,0) to (5,5)
- Room B: (5,0) to (10,5) — RIGHT edge of A = LEFT edge of B = shared wall at x=5

The exit sits ON the shared wall. `Rectangle.Intersects` must use strict inequalities (`<`, `>` not `<=`, `>=`) to treat adjacent rooms as non-overlapping.

### Interior vs. Boundary

Interior floor positions are accessible. Boundary positions (edges) are walls or inaccessible.

- Interior neighbors: all 4 adjacent cells are within room bounds
- Boundary: at least one adjacent cell is outside room bounds

`IsExitReachableInRoom()` checks that the exit has an interior floor neighbor — meaning there's a floor tile one step into the room from the exit.

## Learnings

### 2025-03 — Room Placement Algorithm Analysis (Ludo Onboarded)

Ludo joins the team to audit room placement and sealing decisions. Initial findings:

1. **Fast-path sealing is too aggressive** — seals doors when a simple retry would work
2. **EnsureSeparation can push exits to corners** — the separation logic is sound, but corner exits need retry logic
3. **Retry loop should be the default path** — collisionless placement + unreachable exit = try different room size, not seal

**Next Steps:** Monitor placement patterns in generated dungeons. Identify rooms that should have connected but were sealed unnecessarily. Propose algorithm tuning if needed.

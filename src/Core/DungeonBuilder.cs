using DungeonSaver.Models;
using DungeonSaver.Utils;

namespace DungeonSaver.Core;

public record GenerationLogEntry(string Action, int? RoomId, Point Position, string Detail);

/// <summary>
/// Builds the dungeon progressively as the explorer moves
/// </summary>
public class DungeonBuilder
{
    private readonly DiceRoller _dice;
    private readonly RoomGenerator _roomGenerator;
    private readonly ExitGenerator _exitGenerator;
    private readonly Dungeon _dungeon;
    private readonly List<GenerationLogEntry> _generationLog = new();

    public DungeonBuilder(Dungeon dungeon, int? seed = null)
    {
        _dice = new DiceRoller(seed);
        _roomGenerator = new RoomGenerator(_dice);
        _exitGenerator = new ExitGenerator(_dice);
        _dungeon = dungeon;
    }

    public List<GenerationLogEntry> DrainGenerationLog()
    {
        var entries = _generationLog.ToList();
        _generationLog.Clear();
        return entries;
    }

    /// <summary>
    /// Initialize the dungeon with an entrance room
    /// </summary>
    public Room CreateEntranceRoom()
    {
        // Start entrance roughly in the center of boundary
        Point startPos = new Point(_dungeon.Boundary.Width / 2, _dungeon.Boundary.Height / 2);
        var (entrance, roomDiceLog) = _roomGenerator.GenerateEntranceRoom(startPos);
        
        // Generate 3 exits for entrance room
        string exitDiceLog = _exitGenerator.GenerateExits(entrance);
        
        _dungeon.Rooms.Add(entrance);
        return entrance;
    }

    /// <summary>
    /// Generate a new room connected to an exit
    /// </summary>
    public Room? GenerateRoomAtExit(Exit exit, Room fromRoom)
    {
        if (exit.ConnectedRoom != null)
            return exit.ConnectedRoom;

        // Generate the room first (at temporary position)
        var (newRoom, roomDiceLog) = _roomGenerator.GenerateRoom(new Point(0, 0));
        
        // Now position it correctly based on exit direction and new room size
        Point newRoomPosition = CalculateNewRoomPosition(exit, fromRoom, newRoom);
        newRoom.Bounds = new Rectangle(
            newRoomPosition.X, 
            newRoomPosition.Y, 
            newRoom.Bounds.Width, 
            newRoom.Bounds.Height
        );
        
        // Shift the room by 1 if its perpendicular walls are immediately adjacent to an
        // existing room — prevents visual double-walls and blocked exit generation.
        newRoom = EnsureSeparation(newRoom, exit.Direction);

        // Try to adjust position if there's a collision, then validate both collision and reachability
        Room? adjustedRoom = null;
        if (HasCollision(newRoom))
        {
            adjustedRoom = TryAdjustRoomPosition(newRoom, exit);
        }
        else
        {
            adjustedRoom = newRoom;
        }
        
        // Check if the adjusted room is valid (no collision and exit is reachable)
        bool needsRetry = false;
        if (adjustedRoom != null && !HasCollision(adjustedRoom))
        {
            adjustedRoom = ClampToBoundary(adjustedRoom);
            if (!IsExitReachableInRoom(adjustedRoom, exit.Position))
            {
                needsRetry = true; // Exit not reachable, enter retry loop
            }
        }
        else
        {
            needsRetry = true; // Collision or adjustment failed, enter retry loop
        }
        
        if (needsRetry)
        {
            // Retry up to 20 times with a freshly generated room shape
            adjustedRoom = null;
            string lastSuccessfulRoomDiceLog = roomDiceLog;
            
            for (int attempt = 0; attempt < 20; attempt++)
            {
                var (candidate, candidateDiceLog) = _roomGenerator.GenerateRoom(new Point(0, 0));
                Point candidatePos = CalculateNewRoomPosition(exit, fromRoom, candidate);
                candidate.Bounds = new Rectangle(
                    candidatePos.X,
                    candidatePos.Y,
                    candidate.Bounds.Width,
                    candidate.Bounds.Height
                );
                candidate = EnsureSeparation(candidate, exit.Direction);
                Room? retryRoom = HasCollision(candidate)
                    ? TryAdjustRoomPosition(candidate, exit)
                    : candidate;

                // Check both collision and exit reachability
                bool placementOk = false;
                string failureReason = "";
                if (retryRoom != null && !HasCollision(retryRoom))
                {
                    retryRoom = ClampToBoundary(retryRoom);
                    if (IsExitReachableInRoom(retryRoom, exit.Position))
                    {
                        adjustedRoom = retryRoom;
                        lastSuccessfulRoomDiceLog = candidateDiceLog;
                        placementOk = true;
                    }
                    else
                    {
                        failureReason = GetReachabilityFailureReason(retryRoom, exit.Position);
                    }
                }
                else
                {
                    failureReason = "collision";
                }
                
                // Log this retry attempt with detailed failure reason
                string result = placementOk ? "ok" : failureReason;
                string boundsStr = retryRoom != null 
                    ? $" (bounds:{retryRoom.Bounds.X},{retryRoom.Bounds.Y},{retryRoom.Bounds.Width},{retryRoom.Bounds.Height})"
                    : "";
                _generationLog.Add(new GenerationLogEntry(
                    "RetryAttempt",
                    fromRoom.Id,
                    exit.Position,
                    $"{candidateDiceLog} - {result}{boundsStr}"
                ));
                
                if (placementOk)
                {
                    break;
                }
            }

            if (adjustedRoom == null)
            {
                // Truly can't place a room — block the exit
                exit.IsBlocked = true;
                _dungeon.Messages.Add(
                    $"A passage is sealed — no valid placement found (Room {fromRoom.Id}, exit {exit.Direction}).");
                
                // Add SealDoor log entry
                _generationLog.Add(new GenerationLogEntry(
                    "SealDoor",
                    fromRoom.Id,
                    exit.Position,
                    $"Dir:{exit.Direction} - no valid placement found"
                ));
                
                return null;
            }
            
            roomDiceLog = lastSuccessfulRoomDiceLog;
        }
        
        newRoom = adjustedRoom;
        
        // Generate exits for the new room, then add a back-exit so BFS can traverse back
        Direction entranceDir = GetOppositeDirection(exit.Direction);
        string exitDiceLog = _exitGenerator.GenerateExits(newRoom, entranceDir);
        var backExit = new Exit(exit.Position, entranceDir) { ConnectedRoom = fromRoom };
        newRoom.Exits.Add(backExit);
        
        // Connect the rooms
        exit.ConnectedRoom = newRoom;
        exit.IsExplored = false; // Will be explored when entered
        
        _dungeon.Rooms.Add(newRoom);
        
        // Add RoomGenerated log entry
        var center = new Point(
            newRoom.Bounds.X + newRoom.Bounds.Width / 2,
            newRoom.Bounds.Y + newRoom.Bounds.Height / 2
        );
        _generationLog.Add(new GenerationLogEntry(
            "RoomGenerated",
            newRoom.Id,
            center,
            $"{roomDiceLog} {newRoom.Bounds.Width - 2}x{newRoom.Bounds.Height - 2} {newRoom.Type} {exitDiceLog}"
        ));
        
        return newRoom;
    }

    private Point CalculateNewRoomPosition(Exit exit, Room fromRoom, Room newRoom)
    {
        // Position the new room adjacent to the exit
        // The exit is ON the wall, so the new room should touch it
        Point exitPos = exit.Position;
        int newWidth = newRoom.Bounds.Width;
        int newHeight = newRoom.Bounds.Height;
        
        return exit.Direction switch
        {
            // North: new room's bottom wall = exitPos.Y
            Direction.North => new Point(
                exitPos.X - newWidth / 2,     // Center on exit
                exitPos.Y - newHeight + 1     // Bottom wall at exit position
            ),
            
            // South: new room's top wall = exitPos.Y
            Direction.South => new Point(
                exitPos.X - newWidth / 2,     // Center on exit
                exitPos.Y                     // Top wall at exit position
            ),
            
            // East: new room's left wall = exitPos.X
            Direction.East => new Point(
                exitPos.X,                    // Left wall at exit position
                exitPos.Y - newHeight / 2     // Center on exit
            ),
            
            // West: new room's right wall = exitPos.X
            Direction.West => new Point(
                exitPos.X - newWidth + 1,     // Right wall at exit position
                exitPos.Y - newHeight / 2     // Center on exit
            ),
            
            _ => exitPos
        };
    }

    private bool HasCollision(Room room)
    {
        foreach (var existingRoom in _dungeon.Rooms)
        {
            if (room.Intersects(existingRoom))
                return true;
        }
        return false;
    }

    private Room? TryAdjustRoomPosition(Room room, Exit exit)
    {
        // Only slide perpendicular to the exit direction to avoid creating gaps
        // East/West exits: only Y offsets; North/South exits: only X offsets
        int[] offsets = { -2, -1, 1, 2 };

        bool isHorizontal = exit.Direction == Direction.East || exit.Direction == Direction.West;

        foreach (int offset in offsets)
        {
            int xOff = isHorizontal ? 0 : offset;
            int yOff = isHorizontal ? offset : 0;

            var adjusted = new Room(
                room.Id,
                new Rectangle(
                    room.Bounds.X + xOff,
                    room.Bounds.Y + yOff,
                    room.Bounds.Width,
                    room.Bounds.Height
                ),
                room.Type
            );
            
            if (!HasCollision(adjusted))
                return adjusted;
        }
        
        return null;
    }

    /// <summary>
    /// Shifts a newly positioned room by 1 unit if its perpendicular walls are
    /// immediately adjacent to an existing room, preventing visual double-walls.
    /// Only shifts on the axis perpendicular to the exit direction.
    /// </summary>
    private Room EnsureSeparation(Room newRoom, Direction exitDirection)
    {
        bool isVerticalExit = exitDirection == Direction.North || exitDirection == Direction.South;

        foreach (var existing in _dungeon.Rooms)
        {
            if (isVerticalExit)
            {
                // For North/South exits, check left/right adjacency (X axis)
                if (existing.Bounds.Right + 1 == newRoom.Bounds.Left)
                    return new Room(newRoom.Id, new Rectangle(
                        newRoom.Bounds.X + 1, newRoom.Bounds.Y,
                        newRoom.Bounds.Width, newRoom.Bounds.Height), newRoom.Type);

                if (newRoom.Bounds.Right + 1 == existing.Bounds.Left)
                    return new Room(newRoom.Id, new Rectangle(
                        newRoom.Bounds.X - 1, newRoom.Bounds.Y,
                        newRoom.Bounds.Width, newRoom.Bounds.Height), newRoom.Type);
            }
            else
            {
                // For East/West exits, check top/bottom adjacency (Y axis)
                if (existing.Bounds.Bottom + 1 == newRoom.Bounds.Top)
                    return new Room(newRoom.Id, new Rectangle(
                        newRoom.Bounds.X, newRoom.Bounds.Y + 1,
                        newRoom.Bounds.Width, newRoom.Bounds.Height), newRoom.Type);

                if (newRoom.Bounds.Bottom + 1 == existing.Bounds.Top)
                    return new Room(newRoom.Id, new Rectangle(
                        newRoom.Bounds.X, newRoom.Bounds.Y - 1,
                        newRoom.Bounds.Width, newRoom.Bounds.Height), newRoom.Type);
            }
        }

        return newRoom;
    }

    private Room ClampToBoundary(Room room)
    {
        int x = Math.Max(1, Math.Min(room.Bounds.X, _dungeon.Boundary.Width - room.Bounds.Width - 1));
        int y = Math.Max(1, Math.Min(room.Bounds.Y, _dungeon.Boundary.Height - room.Bounds.Height - 1));
        
        if (x != room.Bounds.X || y != room.Bounds.Y)
        {
            return new Room(
                room.Id,
                new Rectangle(x, y, room.Bounds.Width, room.Bounds.Height),
                room.Type
            );
        }
        
        return room;
    }

    private Direction GetOppositeDirection(Direction dir)
    {
        return dir switch
        {
            Direction.North => Direction.South,
            Direction.South => Direction.North,
            Direction.East => Direction.West,
            Direction.West => Direction.East,
            _ => Direction.North
        };
    }

    /// <summary>
    /// Returns a human-readable reason why the exit is not reachable in the room.
    /// </summary>
    private string GetReachabilityFailureReason(Room room, Point exitPosition)
    {
        var b = room.Bounds;
        
        // Check if exit is on boundary
        bool onBoundary = (exitPosition.X == b.Left || exitPosition.X == b.Right) && 
                          exitPosition.Y >= b.Top && exitPosition.Y <= b.Bottom ||
                          (exitPosition.Y == b.Top || exitPosition.Y == b.Bottom) && 
                          exitPosition.X >= b.Left && exitPosition.X <= b.Right;
        
        if (!onBoundary)
            return "off-boundary";
        
        // Check if it's at a corner (no adjacent interior floor tiles)
        var adjacent = new[]
        {
            new Point(exitPosition.X + 1, exitPosition.Y),
            new Point(exitPosition.X - 1, exitPosition.Y),
            new Point(exitPosition.X, exitPosition.Y + 1),
            new Point(exitPosition.X, exitPosition.Y - 1),
        };

        bool hasReachableTile = adjacent.Any(p =>
        {
            if (!room.Contains(p)) return false;
            bool isWall = p.X == b.Left || p.X == b.Right || p.Y == b.Top || p.Y == b.Bottom;
            return !isWall;
        });
        
        return hasReachableTile ? "reachable" : "corner";
    }

    /// <summary>
    /// Returns true if the exit position has at least one reachable interior floor tile
    /// adjacent to it within the given room. If false, the room is placed such that the
    /// explorer can never step through this exit into the room's interior.
    /// </summary>
    private bool IsExitReachableInRoom(Room room, Point exitPosition)
    {
        var b = room.Bounds;
        
        // First check: is the exit actually on this room's boundary?
        bool onBoundary = (exitPosition.X == b.Left || exitPosition.X == b.Right) && 
                          exitPosition.Y >= b.Top && exitPosition.Y <= b.Bottom ||
                          (exitPosition.Y == b.Top || exitPosition.Y == b.Bottom) && 
                          exitPosition.X >= b.Left && exitPosition.X <= b.Right;
        
        if (!onBoundary)
            return false; // Exit not on room boundary - room placement is invalid
        
        var adjacent = new[]
        {
            new Point(exitPosition.X + 1, exitPosition.Y),
            new Point(exitPosition.X - 1, exitPosition.Y),
            new Point(exitPosition.X, exitPosition.Y + 1),
            new Point(exitPosition.X, exitPosition.Y - 1),
        };

        return adjacent.Any(p =>
        {
            if (!room.Contains(p)) return false;
            bool isWall = p.X == b.Left || p.X == b.Right || p.Y == b.Top || p.Y == b.Bottom;
            return !isWall;
        });
    }
}

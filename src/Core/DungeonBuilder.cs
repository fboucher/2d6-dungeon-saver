using DungeonSaver.Models;
using DungeonSaver.Utils;

namespace DungeonSaver.Core;

/// <summary>
/// Builds the dungeon progressively as the explorer moves
/// </summary>
public class DungeonBuilder
{
    private readonly DiceRoller _dice;
    private readonly RoomGenerator _roomGenerator;
    private readonly ExitGenerator _exitGenerator;
    private readonly Dungeon _dungeon;

    public DungeonBuilder(Dungeon dungeon, int? seed = null)
    {
        _dice = new DiceRoller(seed);
        _roomGenerator = new RoomGenerator(_dice);
        _exitGenerator = new ExitGenerator(_dice);
        _dungeon = dungeon;
    }

    /// <summary>
    /// Initialize the dungeon with an entrance room
    /// </summary>
    public Room CreateEntranceRoom()
    {
        // Start entrance roughly in the center of boundary
        Point startPos = new Point(_dungeon.Boundary.Width / 2, _dungeon.Boundary.Height / 2);
        Room entrance = _roomGenerator.GenerateEntranceRoom(startPos);
        
        // Generate 3 exits for entrance room
        _exitGenerator.GenerateExits(entrance);
        
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

        // Calculate position for new room based on exit direction
        Point newRoomPosition = CalculateNewRoomPosition(exit, fromRoom);
        
        // Generate the room
        Room newRoom = _roomGenerator.GenerateRoom(newRoomPosition);
        
        // Check for collisions with existing rooms
        if (HasCollision(newRoom))
        {
            // Try to adjust position slightly
            Room? adjustedRoom = TryAdjustRoomPosition(newRoom, exit);
            
            if (adjustedRoom == null || HasCollision(adjustedRoom))
            {
                // Can't place room, mark exit as dead end
                return null;
            }
            
            newRoom = adjustedRoom;
        }

        // Ensure room stays within boundary (adjust if needed)
        newRoom = ClampToBoundary(newRoom);
        
        // Generate exits for the new room (entrance is from the connected exit)
        Direction entranceDir = GetOppositeDirection(exit.Direction);
        _exitGenerator.GenerateExits(newRoom, entranceDir);
        
        // Connect the rooms
        exit.ConnectedRoom = newRoom;
        exit.IsExplored = false; // Will be explored when entered
        
        _dungeon.Rooms.Add(newRoom);
        return newRoom;
    }

    private Point CalculateNewRoomPosition(Exit exit, Room fromRoom)
    {
        // Position the new room just outside the exit
        // Exits have a 1-square corridor stub
        Point exitPos = exit.Position;
        
        return exit.Direction switch
        {
            Direction.North => new Point(exitPos.X - 2, exitPos.Y - 6),  // Room goes north
            Direction.South => new Point(exitPos.X - 2, exitPos.Y + 2),  // Room goes south
            Direction.East => new Point(exitPos.X + 2, exitPos.Y - 2),   // Room goes east
            Direction.West => new Point(exitPos.X - 6, exitPos.Y - 2),   // Room goes west
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
        // Try small adjustments (±1 or ±2 squares)
        int[] offsets = { -2, -1, 1, 2 };
        
        foreach (int xOffset in offsets)
        {
            foreach (int yOffset in offsets)
            {
                var adjusted = new Room(
                    room.Id,
                    new Rectangle(
                        room.Bounds.X + xOffset,
                        room.Bounds.Y + yOffset,
                        room.Bounds.Width,
                        room.Bounds.Height
                    ),
                    room.Type
                );
                
                if (!HasCollision(adjusted))
                    return adjusted;
            }
        }
        
        return null;
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
}

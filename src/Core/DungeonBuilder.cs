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

        // Generate the room first (at temporary position)
        Room newRoom = _roomGenerator.GenerateRoom(new Point(0, 0));
        
        // Now position it correctly based on exit direction and new room size
        Point newRoomPosition = CalculateNewRoomPosition(exit, fromRoom, newRoom);
        newRoom.Bounds = new Rectangle(
            newRoomPosition.X, 
            newRoomPosition.Y, 
            newRoom.Bounds.Width, 
            newRoom.Bounds.Height
        );
        
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

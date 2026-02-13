using DungeonSaver.Models;
using DungeonSaver.Utils;

namespace DungeonSaver.Core;

/// <summary>
/// Generates rooms based on 2D6 rules
/// </summary>
public class RoomGenerator
{
    private readonly DiceRoller _dice;
    private int _nextRoomId = 0;

    public RoomGenerator(DiceRoller dice)
    {
        _dice = dice;
    }

    /// <summary>
    /// Generate the entrance room (special rules: 6-12 squares, 3 exits)
    /// </summary>
    public Room GenerateEntranceRoom(Point position)
    {
        var (x, y) = _dice.RollD66();
        int area = x * y;

        // Entrance room must be 6-12 squares
        if (area < 6)
        {
            x = 3;
            y = 2;
        }
        else if (area > 12)
        {
            x = 4;
            y = 3;
        }

        var bounds = new Rectangle(position.X, position.Y, x, y);
        var room = new Room(_nextRoomId++, bounds, RoomType.Entrance);
        
        // Entrance room is always visible
        room.IsVisible = true;
        room.IsExplored = true;
        
        return room;
    }

    /// <summary>
    /// Generate a normal room following 2D6 rules
    /// </summary>
    public Room GenerateRoom(Point position)
    {
        var (x, y) = _dice.RollD66();
        
        // Check for corridor (any dimension is 1, but not double-1)
        if ((x == 1 || y == 1) && !(x == 1 && y == 1))
        {
            return GenerateCorridor(position, x, y);
        }

        // Check for doubles (except double-6)
        if (_dice.IsDouble(x, y) && !_dice.IsDouble6(x, y))
        {
            var (addX, addY) = _dice.Roll2D6();
            x += addX;
            y += addY;
        }

        int area = x * y;
        RoomType type = DetermineRoomType(area);
        
        var bounds = new Rectangle(position.X, position.Y, x, y);
        return new Room(_nextRoomId++, bounds, type);
    }

    private Room GenerateCorridor(Point position, int x, int y)
    {
        var bounds = new Rectangle(position.X, position.Y, x, y);
        return new Room(_nextRoomId++, bounds, RoomType.Corridor);
    }

    private RoomType DetermineRoomType(int area)
    {
        if (area <= 6)
            return RoomType.Small;
        if (area >= 32)
            return RoomType.Large;
        return RoomType.Normal;
    }
}

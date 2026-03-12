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
    public (Room room, string diceLog) GenerateEntranceRoom(Point position)
    {
        var (x, y) = _dice.RollD66();
        string diceLog = $"[{x}][{y}]";
        int area = x * y;

        // Entrance room must be 6-12 squares (floor area)
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

        // Dimensions are floor area, add 2 for walls (1 on each side)
        var bounds = new Rectangle(position.X, position.Y, x + 2, y + 2);
        var room = new Room(_nextRoomId++, bounds, RoomType.Entrance);
        
        // Entrance room is always visible
        room.IsVisible = true;
        room.IsExplored = true;
        
        return (room, diceLog);
    }

    /// <summary>
    /// Generate a normal room following 2D6 rules
    /// </summary>
    public (Room room, string diceLog) GenerateRoom(Point position)
    {
        var (x, y) = _dice.RollD66();
        string diceLog;
        
        // Check for corridor (any dimension is 1, but not double-1)
        if ((x == 1 || y == 1) && !(x == 1 && y == 1))
        {
            diceLog = $"[{x}][{y}]";
            return (GenerateCorridor(position, x, y), diceLog);
        }

        // Check for doubles (except double-6)
        if (_dice.IsDouble(x, y) && !_dice.IsDouble6(x, y))
        {
            var (addX, addY) = _dice.Roll2D6();
            diceLog = $"[{x}][{y}] doubles+[{addX}][{addY}]";
            x += addX;
            y += addY;
        }
        else
        {
            diceLog = $"[{x}][{y}]";
        }

        int area = x * y;
        RoomType type = DetermineRoomType(area);
        
        // Dimensions are floor area, add 2 for walls (1 on each side)
        var bounds = new Rectangle(position.X, position.Y, x + 2, y + 2);
        return (new Room(_nextRoomId++, bounds, type), diceLog);
    }

    private Room GenerateCorridor(Point position, int x, int y)
    {
        // For corridors, add 2 to the non-1 dimension for walls
        // If x=1, it's a vertical corridor, add 2 to x for walls
        // If y=1, it's a horizontal corridor, add 2 to y for walls
        var bounds = new Rectangle(position.X, position.Y, x + 2, y + 2);
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

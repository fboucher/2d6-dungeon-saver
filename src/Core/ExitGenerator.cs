using DungeonSaver.Models;
using DungeonSaver.Utils;

namespace DungeonSaver.Core;

/// <summary>
/// Generates exits for rooms based on 2D6 rules
/// </summary>
public class ExitGenerator
{
    private readonly DiceRoller _dice;

    public ExitGenerator(DiceRoller dice)
    {
        _dice = dice;
    }

    /// <summary>
    /// Determine number of exits based on D6 roll
    /// 1 = 0 exits, 2-3 = 1 exit, 4-5 = 2 exits, 6 = 3 exits
    /// </summary>
    public (int count, string diceLog) DetermineExitCount()
    {
        int roll = _dice.D6();
        int count = roll switch
        {
            1 => 0,
            2 or 3 => 1,
            4 or 5 => 2,
            6 => 3,
            _ => 1
        };
        string meaning = count switch
        {
            0 => "no exits",
            1 => "one exit",
            2 => "two exits",
            3 => "three exits",
            _ => "?"
        };
        return (count, $"[{roll}] - {meaning}");
    }

    /// <summary>
    /// Generate exits for a room following the clockwise placement rules
    /// </summary>
    public string GenerateExits(Room room, Direction? entranceDirection = null)
    {
        int exitCount;
        string diceLog;
        
        // Entrance room always has 3 exits
        if (room.Type == RoomType.Entrance)
        {
            exitCount = 3;
            diceLog = "entrance - 3 exits";
        }
        else
        {
            var (count, log) = DetermineExitCount();
            exitCount = count;
            diceLog = log;
        }

        if (exitCount == 0)
            return diceLog;

        // Get available walls (all except entrance wall if applicable)
        var availableWalls = GetAvailableWalls(entranceDirection);
        
        // Determine starting wall for first exit
        int startWallIndex = DetermineStartWall(availableWalls.Count);
        
        // Place exits clockwise
        for (int i = 0; i < exitCount && i < availableWalls.Count; i++)
        {
            int wallIndex = (startWallIndex + i) % availableWalls.Count;
            Direction dir = availableWalls[wallIndex];
            
            Point exitPos = GetExitPositionOnWall(room, dir);
            var exit = new Exit(exitPos, dir, ExitType.Archway);
            room.Exits.Add(exit);
        }

        return diceLog;
    }

    private List<Direction> GetAvailableWalls(Direction? entranceDirection)
    {
        var allWalls = new List<Direction> { Direction.North, Direction.East, Direction.South, Direction.West };
        
        if (entranceDirection.HasValue)
        {
            // Remove entrance wall directly
            allWalls.Remove(entranceDirection.Value);
        }
        
        return allWalls;
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

    private int DetermineStartWall(int availableWallCount)
    {
        if (availableWallCount == 0)
            return 0;
            
        int roll = _dice.D6();
        return roll switch
        {
            1 or 2 => 0,  // First wall clockwise
            3 or 4 => 1,  // Second wall clockwise
            5 or 6 => 2,  // Third wall clockwise
            _ => 0
        };
    }

    private Point GetExitPositionOnWall(Room room, Direction direction)
    {
        Rectangle bounds = room.Bounds;
        
        // Place exit roughly in the middle of the wall
        return direction switch
        {
            Direction.North => new Point(bounds.X + bounds.Width / 2, bounds.Y),
            Direction.South => new Point(bounds.X + bounds.Width / 2, bounds.Y + bounds.Height - 1),
            Direction.East => new Point(bounds.X + bounds.Width - 1, bounds.Y + bounds.Height / 2),
            Direction.West => new Point(bounds.X, bounds.Y + bounds.Height / 2),
            _ => new Point(bounds.X, bounds.Y)
        };
    }
}

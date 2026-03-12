namespace DungeonSaver.Models;

public enum ExitType
{
    Archway,
    Door,
    SecretDoor
}

public enum Direction
{
    North,
    East,
    South,
    West
}

/// <summary>
/// Represents an exit from a room
/// </summary>
public class Exit
{
    public Utils.Point Position { get; set; }
    public Direction Direction { get; set; }
    public ExitType Type { get; set; }
    public Room? ConnectedRoom { get; set; }
    public bool IsExplored { get; set; }
    public bool IsBlocked { get; set; } = false;
    public bool IsNavigationBlocked { get; set; }

    public Exit(Utils.Point position, Direction direction, ExitType type = ExitType.Archway)
    {
        Position = position;
        Direction = direction;
        Type = type;
        IsExplored = false;
    }
}

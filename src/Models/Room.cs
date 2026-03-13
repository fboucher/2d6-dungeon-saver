using DungeonSaver.Utils;

namespace DungeonSaver.Models;

public enum RoomType
{
    Entrance,
    Normal,
    Small,
    Large,
    Corridor
}

/// <summary>
/// Represents a room in the dungeon
/// </summary>
public class Room
{
    public int Id { get; set; }
    public Rectangle Bounds { get; set; }
    public RoomType Type { get; set; }
    public List<Exit> Exits { get; set; }
    public bool IsExplored { get; set; }
    public bool IsVisible { get; set; }
    public HashSet<Point> RevealedTiles { get; } = new();

    public Room(int id, Rectangle bounds, RoomType type)
    {
        Id = id;
        Bounds = bounds;
        Type = type;
        Exits = new List<Exit>();
        IsExplored = false;
        IsVisible = false;
    }

    public int Area => Bounds.Area;
    
    public bool Contains(Point point) => Bounds.Contains(point);
    
    public bool Intersects(Room other) => Bounds.Intersects(other.Bounds);
}

using DungeonSaver.Utils;

namespace DungeonSaver.Models;

/// <summary>
/// Represents the entire dungeon with all its rooms
/// </summary>
public class Dungeon
{
    public List<Room> Rooms { get; set; }
    public Rectangle Boundary { get; set; }
    public int TargetRoomCount { get; set; }
    public List<string> Messages { get; } = new();
    
    public Dungeon(int targetRoomCount = 20)
    {
        Rooms = new List<Room>();
        TargetRoomCount = targetRoomCount;
        // Start with a reasonable boundary, will expand as needed
        Boundary = new Rectangle(0, 0, 100, 100);
    }
    
    public Room? GetRoomAt(Point position)
    {
        return Rooms.FirstOrDefault(r => r.Contains(position));
    }
    
    public bool IsComplete => Rooms.Count >= TargetRoomCount;
}

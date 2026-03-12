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
    
    /// <summary>
    /// Search all rooms for any exit at the given position.
    /// Prioritizes blocked exits over explored/unexplored exits.
    /// Rooms can share wall coordinates, so multiple rooms may contain the position.
    /// </summary>
    public Exit? GetExitAt(Point position)
    {
        Exit? explored = null;
        Exit? unexplored = null;
        foreach (var room in Rooms)
        {
            var exit = room.Exits.FirstOrDefault(e => e.Position == position);
            if (exit == null) continue;
            if (exit.IsBlocked) return exit;  // Blocked always wins
            if (exit.IsExplored || exit.ConnectedRoom?.IsVisible == true)
                explored ??= exit;
            else
                unexplored ??= exit;
        }
        return explored ?? unexplored;
    }
    
    public bool IsComplete => Rooms.Count >= TargetRoomCount;
}

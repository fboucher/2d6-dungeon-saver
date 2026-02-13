using DungeonSaver.Utils;

namespace DungeonSaver.Models;

public enum ExplorerState
{
    Idle,
    Moving,
    DiscoveringRoom,
    Wandering
}

/// <summary>
/// Represents the explorer character navigating the dungeon
/// </summary>
public class Explorer
{
    public Point Position { get; set; }
    public ExplorerState State { get; set; }
    public Room? CurrentRoom { get; set; }
    public List<Point> CurrentPath { get; set; }
    public HashSet<int> VisitedRoomIds { get; set; }
    public DateTime LastMoveTime { get; set; }
    public DateTime PauseUntil { get; set; }

    public Explorer(Point startPosition)
    {
        Position = startPosition;
        State = ExplorerState.Idle;
        CurrentPath = new List<Point>();
        VisitedRoomIds = new HashSet<int>();
        LastMoveTime = DateTime.Now;
        PauseUntil = DateTime.Now;
    }

    public bool IsPaused => DateTime.Now < PauseUntil;
    
    public void Pause(int milliseconds)
    {
        PauseUntil = DateTime.Now.AddMilliseconds(milliseconds);
    }
}

using DungeonSaver.Utils;

namespace DungeonSaver.Models;

public enum ExplorerState
{
    Idle,
    Moving,
    DiscoveringRoom,
    FogWalking,
    Wandering
}

/// <summary>
/// Records a movement event for diagnostic purposes
/// </summary>
public record MovementEvent(
    DateTime Timestamp,
    Point From,
    Point To,
    string Action,
    int? RoomId,
    string? Detail
);

/// <summary>
/// Represents the explorer character navigating the dungeon
/// </summary>
public class Explorer
{
    private const int MAX_TRACE_EVENTS = 1000;
    
    public Point Position { get; set; }
    public ExplorerState State { get; set; }
    public Room? CurrentRoom { get; set; }
    public List<Point> CurrentPath { get; set; }
    public HashSet<int> VisitedRoomIds { get; set; }
    public DateTime LastMoveTime { get; set; }
    public DateTime PauseUntil { get; set; }
    public List<MovementEvent> MovementTrace { get; set; }

    public Explorer(Point startPosition)
    {
        Position = startPosition;
        State = ExplorerState.Idle;
        CurrentPath = new List<Point>();
        VisitedRoomIds = new HashSet<int>();
        LastMoveTime = DateTime.Now;
        PauseUntil = DateTime.Now;
        MovementTrace = new List<MovementEvent>();
    }

    public bool IsPaused => DateTime.Now < PauseUntil;
    
    public void Pause(int milliseconds)
    {
        PauseUntil = DateTime.Now.AddMilliseconds(milliseconds);
    }

    public void AddTrace(MovementEvent evt)
    {
        MovementTrace.Add(evt);
        if (MovementTrace.Count > MAX_TRACE_EVENTS)
        {
            MovementTrace.RemoveAt(0);
        }
    }
}

using DungeonSaver.Models;
using DungeonSaver.Utils;

namespace DungeonSaver.Core;

/// <summary>
/// AI that controls the explorer's behavior
/// </summary>
public class ExplorerAI
{
    private readonly Explorer _explorer;
    private readonly Dungeon _dungeon;
    private readonly DungeonBuilder _dungeonBuilder;
    private readonly Pathfinder _pathfinder;
    private readonly Random _random;

    private const int MOVE_DELAY_MS = 500;  // 500ms between moves (slow contemplative pace)
    private const int DISCOVERY_PAUSE_MS = 1500;  // 1.5 second pause on room discovery

    public ExplorerAI(Explorer explorer, Dungeon dungeon, DungeonBuilder builder)
    {
        _explorer = explorer;
        _dungeon = dungeon;
        _dungeonBuilder = builder;
        _pathfinder = new Pathfinder();
        _random = new Random();
    }

    /// <summary>
    /// Update the explorer's state and movement
    /// </summary>
    public void Update()
    {
        // Check if explorer is paused
        if (_explorer.IsPaused)
            return;

        // Check if enough time has passed since last move
        if ((DateTime.Now - _explorer.LastMoveTime).TotalMilliseconds < MOVE_DELAY_MS)
            return;

        // Update current room
        UpdateCurrentRoom();

        // If no path, decide where to go next
        if (_explorer.CurrentPath.Count == 0)
        {
            DecideNextDestination();
        }

        // Move along current path
        if (_explorer.CurrentPath.Count > 0)
        {
            MoveToNextPosition();
        }
    }

    private void UpdateCurrentRoom()
    {
        var room = _dungeon.GetRoomAt(_explorer.Position);
        
        if (room != null && room != _explorer.CurrentRoom)
        {
            // Entering a new room
            _explorer.CurrentRoom = room;
            
            // Mark room as explored
            if (!room.IsExplored)
            {
                room.IsExplored = true;
                room.IsVisible = true;
                _explorer.VisitedRoomIds.Add(room.Id);
                
                // Pause to appreciate the new room
                _explorer.Pause(DISCOVERY_PAUSE_MS);
                _explorer.State = ExplorerState.DiscoveringRoom;
            }
            else
            {
                _explorer.State = ExplorerState.Moving;
            }
        }
    }

    private void DecideNextDestination()
    {
        // First priority: Find unexplored exits in current room
        var unexploredExit = FindUnexploredExit();
        
        if (unexploredExit != null)
        {
            _explorer.State = ExplorerState.Moving;
            NavigateToExit(unexploredExit);
            return;
        }
        
        // Second priority: Go back to a room that has unexplored exits
        var exitToOtherRoom = FindExitToRoomWithUnexploredExits();
        if (exitToOtherRoom != null)
        {
            _explorer.State = ExplorerState.Moving;
            NavigateToExit(exitToOtherRoom);
            return;
        }
        
        // No unexplored exits anywhere - just wander
        _explorer.State = ExplorerState.Wandering;
        WanderRandomly();
    }

    private Exit? FindUnexploredExit()
    {
        // First, look for unexplored exits in the CURRENT room
        if (_explorer.CurrentRoom != null)
        {
            foreach (var exit in _explorer.CurrentRoom.Exits.Where(e => !e.IsExplored))
            {
                // Generate room at this exit if not already done
                if (exit.ConnectedRoom == null && _dungeon.Rooms.Count < _dungeon.TargetRoomCount)
                {
                    _dungeonBuilder.GenerateRoomAtExit(exit, _explorer.CurrentRoom);
                }
                
                if (exit.ConnectedRoom != null)
                {
                    return exit;
                }
            }
        }
        
        return null;
    }

    private Exit? FindExitToRoomWithUnexploredExits()
    {
        // Find an explored exit in current room that leads to a room with unexplored exits
        if (_explorer.CurrentRoom == null)
            return null;

        foreach (var exit in _explorer.CurrentRoom.Exits.Where(e => e.IsExplored && e.ConnectedRoom != null))
        {
            var connectedRoom = exit.ConnectedRoom;
            if (connectedRoom != null && connectedRoom.Exits.Any(e => !e.IsExplored))
            {
                // This room has unexplored exits, go back to it
                return exit;
            }
        }

        return null;
    }

    private void NavigateToExit(Exit exit)
    {
        if (_explorer.CurrentRoom == null)
            return;

        // Create path to exit position
        _explorer.CurrentPath = _pathfinder.FindPath(
            _explorer.Position,
            exit.Position,
            _explorer.CurrentRoom
        );

        // Remove current position from path
        if (_explorer.CurrentPath.Count > 0 && _explorer.CurrentPath[0] == _explorer.Position)
        {
            _explorer.CurrentPath.RemoveAt(0);
        }
    }

    private void WanderRandomly()
    {
        if (_explorer.CurrentRoom == null)
            return;

        // Pick a random point in the current room
        var bounds = _explorer.CurrentRoom.Bounds;
        Point randomPoint = new Point(
            _random.Next(bounds.X + 1, bounds.X + bounds.Width - 1),
            _random.Next(bounds.Y + 1, bounds.Y + bounds.Height - 1)
        );

        _explorer.CurrentPath = _pathfinder.FindPath(
            _explorer.Position,
            randomPoint,
            _explorer.CurrentRoom
        );

        if (_explorer.CurrentPath.Count > 0 && _explorer.CurrentPath[0] == _explorer.Position)
        {
            _explorer.CurrentPath.RemoveAt(0);
        }
    }

    private void MoveToNextPosition()
    {
        if (_explorer.CurrentPath.Count == 0)
            return;

        Point nextPos = _explorer.CurrentPath[0];
        _explorer.CurrentPath.RemoveAt(0);
        
        _explorer.Position = nextPos;
        _explorer.LastMoveTime = DateTime.Now;

        // Check if we crossed an exit
        CheckExitCrossing(nextPos);
    }

    private void CheckExitCrossing(Point position)
    {
        // Check if we're at an exit position in the current room
        if (_explorer.CurrentRoom == null)
            return;

        foreach (var exit in _explorer.CurrentRoom.Exits)
        {
            if (exit.Position == position && exit.ConnectedRoom != null)
            {
                exit.IsExplored = true;
                
                // Move explorer into the connected room
                var connectedRoom = exit.ConnectedRoom;
                
                // Find the entrance point in the connected room
                // It should be on the opposite wall from where we're coming
                Point entrancePos = GetEntrancePosition(exit, connectedRoom);
                
                _explorer.Position = entrancePos;
                _explorer.CurrentRoom = connectedRoom;
                _explorer.CurrentPath.Clear(); // Clear path since we're in a new room
                
                // Make the connected room visible
                if (!connectedRoom.IsVisible)
                {
                    connectedRoom.IsVisible = true;
                }
                
                break;
            }
        }
    }

    private Point GetEntrancePosition(Exit exit, Room connectedRoom)
    {
        // Find a floor position just inside the connected room
        // based on which direction we're entering from
        Rectangle bounds = connectedRoom.Bounds;
        
        return exit.Direction switch
        {
            // Coming from south, enter near north wall
            Direction.North => new Point(bounds.X + bounds.Width / 2, bounds.Y + 1),
            // Coming from north, enter near south wall
            Direction.South => new Point(bounds.X + bounds.Width / 2, bounds.Y + bounds.Height - 2),
            // Coming from west, enter near east wall
            Direction.East => new Point(bounds.X + bounds.Width - 2, bounds.Y + bounds.Height / 2),
            // Coming from east, enter near west wall
            Direction.West => new Point(bounds.X + 1, bounds.Y + bounds.Height / 2),
            _ => new Point(bounds.X + 1, bounds.Y + 1)
        };
    }
}

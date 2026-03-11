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
        
        // Also check: is explorer on an exit tile that leads to a connected room?
        // The exit tile belongs to the parent room via GetRoomAt (shared-wall model),
        // but if the explorer's path continues into the connected room, they're transitioning.
        if (room != null && _explorer.CurrentRoom != null && room == _explorer.CurrentRoom)
        {
            // Check if standing on an explored exit
            foreach (var exit in _explorer.CurrentRoom.Exits)
            {
                if (exit.Position == _explorer.Position && exit.ConnectedRoom != null && exit.IsExplored)
                {
                    // Explorer is on the threshold — check if they've moved into the connected room's interior
                    if (exit.ConnectedRoom.Bounds.Contains(_explorer.Position))
                    {
                        room = exit.ConnectedRoom;
                        break;
                    }
                }
            }
        }
        
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

        Point target;
        Room? targetRoom = null;
        
        // If this exit leads to a connected room, path to one step inside it
        if (exit.ConnectedRoom != null)
        {
            target = GetStepInsideRoom(exit);
            targetRoom = exit.ConnectedRoom;
        }
        else
        {
            target = exit.Position;
        }

        // Create path to target
        _explorer.CurrentPath = _pathfinder.FindPath(
            _explorer.Position,
            target,
            _explorer.CurrentRoom,
            targetRoom
        );

        // Remove current position from path
        if (_explorer.CurrentPath.Count > 0 && _explorer.CurrentPath[0] == _explorer.Position)
        {
            _explorer.CurrentPath.RemoveAt(0);
        }
    }

    private Point GetStepInsideRoom(Exit exit)
    {
        if (exit.ConnectedRoom == null) return exit.Position;
        
        // One step past the shared wall into the connected room's floor
        return exit.Direction switch
        {
            Direction.North => new Point(exit.Position.X, exit.Position.Y - 1),
            Direction.South => new Point(exit.Position.X, exit.Position.Y + 1),
            Direction.East  => new Point(exit.Position.X + 1, exit.Position.Y),
            Direction.West  => new Point(exit.Position.X - 1, exit.Position.Y),
            _ => exit.Position
        };
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
                
                // Make the connected room visible
                if (!exit.ConnectedRoom.IsVisible)
                {
                    exit.ConnectedRoom.IsVisible = true;
                }
                
                break;
            }
        }
    }

}

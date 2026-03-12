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
            var prevRoom = _explorer.CurrentRoom;
            _explorer.CurrentRoom = room;
            
            // Log room transition
            _explorer.AddTrace(new MovementEvent(
                DateTime.Now,
                _explorer.Position,
                _explorer.Position,
                "RoomSwitch",
                room.Id,
                $"From:{prevRoom?.Id} To:{room.Id}"
            ));
            
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
        // Priority 1: Unexplored exit in current room
        var unexploredExit = FindUnexploredExit();
        if (unexploredExit != null)
        {
            _explorer.State = ExplorerState.Moving;
            NavigateToExit(unexploredExit);
            return;
        }

        // Priority 2: BFS to find nearest room with unexplored exits — backtrack via explored doors
        var backtrackExit = FindPathToNearestUnexploredExit();
        if (backtrackExit != null)
        {
            _explorer.State = ExplorerState.Moving;
            NavigateToExit(backtrackExit);
            return;
        }

        // Priority 3: Dungeon fully explored — wander
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
                    
                    // Drain generation log and add to trace
                    var genLog = _dungeonBuilder.DrainGenerationLog();
                    foreach (var entry in genLog)
                    {
                        _explorer.AddTrace(new MovementEvent(
                            DateTime.Now,
                            entry.Position,
                            entry.Position,
                            entry.Action,
                            entry.RoomId,
                            entry.Detail
                        ));
                    }
                }
                
                if (exit.ConnectedRoom != null)
                {
                    return exit;
                }
                else
                {
                    // Dead end: couldn't place a room here. Mark as explored so we don't retry forever.
                    exit.IsExplored = true;
                }
            }
        }
        
        return null;
    }

    /// <summary>
    /// BFS through connected explored rooms to find the nearest room with unexplored exits.
    /// Returns the exit in the CURRENT room to take as the first step toward that room.
    /// Returns null if all reachable rooms are fully explored.
    /// </summary>
    private Exit? FindPathToNearestUnexploredExit()
    {
        if (_explorer.CurrentRoom == null)
            return null;

        // BFS: queue holds (room, firstExitFromCurrentRoom)
        // firstExit = the exit in CurrentRoom we'd take to start the journey
        var visited = new HashSet<int> { _explorer.CurrentRoom.Id };
        var queue = new Queue<(Room room, Exit firstExit)>();

        // Seed with all explored exits from current room
        foreach (var exit in _explorer.CurrentRoom.Exits
            .Where(e => e.IsExplored && e.ConnectedRoom != null && !e.IsNavigationBlocked))
        {
            queue.Enqueue((exit.ConnectedRoom!, exit));
        }

        while (queue.Count > 0)
        {
            var (current, firstExit) = queue.Dequeue();

            if (visited.Contains(current.Id))
                continue;
            visited.Add(current.Id);

            // Does this room have any exits not yet marked explored?
            // (Includes exits with connected rooms to enter AND exits that may still
            // generate a room or will be marked dead-end when the explorer arrives.)
            bool hasUnexplored = current.Exits.Any(e => !e.IsExplored);

            if (hasUnexplored)
            {
                // Log backtracking decision
                _explorer.AddTrace(new MovementEvent(
                    DateTime.Now, _explorer.Position, firstExit.Position,
                    "Backtrack", _explorer.CurrentRoom?.Id,
                    $"HeadingTo:{firstExit.Direction} ToFindUnexplored"
                ));
                return firstExit; // Take this exit from current room to start the journey
            }

            // Keep searching — enqueue this room's explored exits
            foreach (var exit in current.Exits
                .Where(e => e.IsExplored && e.ConnectedRoom != null
                         && !e.IsNavigationBlocked
                         && !visited.Contains(e.ConnectedRoom.Id)))
            {
                queue.Enqueue((exit.ConnectedRoom!, firstExit));
            }
        }

        return null; // All reachable rooms fully explored
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

        // No route found — mark exit as explored so we don't retry it
        if (_explorer.CurrentPath.Count == 0)
        {
            exit.IsExplored = true;
            // If we have a connected room but can't navigate there, mark it navigation-blocked
            // so the BFS won't keep routing us back to this same dead-end exit
            if (exit.ConnectedRoom != null)
                exit.IsNavigationBlocked = true;
            _explorer.AddTrace(new MovementEvent(
                DateTime.Now,
                _explorer.Position,
                exit.Position,
                "PathBlocked",
                _explorer.CurrentRoom?.Id,
                $"ExitDir:{exit.Direction} NoPath"
            ));
            return;
        }

        // Log path planning
        _explorer.AddTrace(new MovementEvent(
            DateTime.Now,
            _explorer.Position,
            target,
            "PathPlanned",
            _explorer.CurrentRoom.Id,
            $"ExitDir:{exit.Direction} Steps:{_explorer.CurrentPath.Count}"
        ));

        // Check if this is a fallback path (pathfinder returned direct line)
        if (_explorer.CurrentPath.Count == 2 && 
            _explorer.CurrentPath[0] == _explorer.Position)
        {
            _explorer.AddTrace(new MovementEvent(
                DateTime.Now,
                _explorer.Position,
                target,
                "PathFallback",
                _explorer.CurrentRoom.Id,
                $"FallbackDirect ExitDir:{exit.Direction}"
            ));
        }

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

        Point prevPos = _explorer.Position;
        Point nextPos = _explorer.CurrentPath[0];
        _explorer.CurrentPath.RemoveAt(0);
        
        _explorer.Position = nextPos;
        _explorer.LastMoveTime = DateTime.Now;

        // Log movement
        _explorer.AddTrace(new MovementEvent(
            DateTime.Now,
            prevPos,
            nextPos,
            "Move",
            _explorer.CurrentRoom?.Id,
            null
        ));

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
                
                // Also mark the back-exit in the connected room so BFS can traverse back
                var backExit = exit.ConnectedRoom.Exits
                    .FirstOrDefault(e => e.Position == position && e.ConnectedRoom == _explorer.CurrentRoom);
                if (backExit != null)
                    backExit.IsExplored = true;
                
                // Log exit crossing
                _explorer.AddTrace(new MovementEvent(
                    DateTime.Now,
                    position,
                    position,
                    "ExitCrossed",
                    _explorer.CurrentRoom.Id,
                    $"Dir:{exit.Direction} ConnectedRoom:{exit.ConnectedRoom.Id}"
                ));
                
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

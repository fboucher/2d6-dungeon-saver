using DungeonSaver.Models;
using DungeonSaver.Utils;

namespace DungeonSaver.Core;

/// <summary>
/// A* pathfinding for explorer movement
/// </summary>
public class Pathfinder
{
    private class PathNode
    {
        public Point Position { get; set; }
        public int GCost { get; set; }  // Distance from start
        public int HCost { get; set; }  // Heuristic distance to target
        public int FCost => GCost + HCost;
        public PathNode? Parent { get; set; }

        public PathNode(Point position)
        {
            Position = position;
        }
    }

    /// <summary>
    /// Find a path from start to goal within a room, optionally crossing into a connected room
    /// </summary>
    public List<Point> FindPath(Point start, Point goal, Room room, Room? connectedRoom = null)
    {
        var openSet = new List<PathNode>();
        var closedSet = new HashSet<Point>();
        
        var startNode = new PathNode(start) { GCost = 0, HCost = ManhattanDistance(start, goal) };
        openSet.Add(startNode);

        while (openSet.Count > 0)
        {
            // Get node with lowest F cost
            var current = GetLowestFCost(openSet);
            
            if (current.Position == goal)
            {
                return ReconstructPath(current);
            }

            openSet.Remove(current);
            closedSet.Add(current.Position);

            // Check neighbors
            foreach (var neighbor in GetNeighbors(current.Position, room, connectedRoom))
            {
                if (closedSet.Contains(neighbor))
                    continue;

                int tentativeGCost = current.GCost + 1;
                
                var neighborNode = openSet.FirstOrDefault(n => n.Position == neighbor);
                if (neighborNode == null)
                {
                    neighborNode = new PathNode(neighbor)
                    {
                        GCost = tentativeGCost,
                        HCost = ManhattanDistance(neighbor, goal),
                        Parent = current
                    };
                    openSet.Add(neighborNode);
                }
                else if (tentativeGCost < neighborNode.GCost)
                {
                    neighborNode.GCost = tentativeGCost;
                    neighborNode.Parent = current;
                }
            }
        }

        // No path found, return direct line as fallback
        return new List<Point> { start, goal };
    }

    private PathNode GetLowestFCost(List<PathNode> nodes)
    {
        var lowest = nodes[0];
        foreach (var node in nodes)
        {
            if (node.FCost < lowest.FCost)
                lowest = node;
        }
        return lowest;
    }

    private List<Point> ReconstructPath(PathNode endNode)
    {
        var path = new List<Point>();
        var current = endNode;
        
        while (current != null)
        {
            path.Add(current.Position);
            current = current.Parent;
        }
        
        path.Reverse();
        return path;
    }

    private List<Point> GetNeighbors(Point pos, Room room, Room? connectedRoom = null)
    {
        var neighbors = new List<Point>
        {
            new(pos.X, pos.Y - 1),  // North
            new(pos.X + 1, pos.Y),  // East
            new(pos.X, pos.Y + 1),  // South
            new(pos.X - 1, pos.Y)   // West
        };

        // Filter to positions that are walkable (inside room and either floor or exit)
        return neighbors.Where(p => IsWalkable(p, room, connectedRoom)).ToList();
    }

    private bool IsWalkable(Point pos, Room room, Room? connectedRoom = null)
    {
        // Check if walkable in the primary room
        if (room.Contains(pos))
        {
            // Check if it's an exit (exits are walkable)
            if (room.Exits.Any(e => e.Position == pos))
                return true;

            // Check if it's a wall (walls are not walkable unless they're exits)
            Rectangle bounds = room.Bounds;
            bool onLeft = pos.X == bounds.Left;
            bool onRight = pos.X == bounds.Right;
            bool onTop = pos.Y == bounds.Top;
            bool onBottom = pos.Y == bounds.Bottom;
            
            bool isWall = onLeft || onRight || onTop || onBottom;
            
            // Floor is walkable, walls are not (unless exit, which we checked above)
            return !isWall;
        }

        // Also check if walkable in the connected room (for cross-room pathing)
        if (connectedRoom != null && connectedRoom.Contains(pos))
        {
            Rectangle bounds = connectedRoom.Bounds;
            bool onLeft = pos.X == bounds.Left;
            bool onRight = pos.X == bounds.Right;
            bool onTop = pos.Y == bounds.Top;
            bool onBottom = pos.Y == bounds.Bottom;
            
            bool isWall = onLeft || onRight || onTop || onBottom;
            
            return !isWall;
        }

        return false;
    }

    private int ManhattanDistance(Point a, Point b)
    {
        return Math.Abs(a.X - b.X) + Math.Abs(a.Y - b.Y);
    }
}

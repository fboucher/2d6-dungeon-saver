using DungeonSaver.Models;
using DungeonSaver.Utils;
using System.Text;

namespace DungeonSaver.Core;

/// <summary>
/// Exports dungeon maps to text files
/// </summary>
public class MapExporter
{
    private const string MAP_FOLDER = "maps";

    public MapExporter()
    {
        // Ensure maps folder exists
        if (!Directory.Exists(MAP_FOLDER))
        {
            Directory.CreateDirectory(MAP_FOLDER);
        }
    }

    /// <summary>
    /// Export the dungeon to a text file
    /// </summary>
    public void ExportMap(Dungeon dungeon, Explorer explorer)
    {
        string filename = GenerateFilename();
        string filepath = Path.Combine(MAP_FOLDER, filename);

        var content = GenerateMapContent(dungeon, explorer);
        
        File.WriteAllText(filepath, content);
        Console.WriteLine($"Map exported to: {filepath}");
    }

    private string GenerateFilename()
    {
        return DateTime.Now.ToString("yyyy-MM-dd_HHmm") + ".txt";
    }

    private string GenerateMapContent(Dungeon dungeon, Explorer explorer)
    {
        var sb = new StringBuilder();

        // Header
        sb.AppendLine("═══════════════════════════════════════════════════════");
        sb.AppendLine("              DUNGEON SAVER - MAP EXPORT               ");
        sb.AppendLine("═══════════════════════════════════════════════════════");
        sb.AppendLine();
        sb.AppendLine($"Generated: {DateTime.Now:yyyy-MM-dd HH:mm:ss}");
        sb.AppendLine($"Total Rooms: {dungeon.Rooms.Count}");
        sb.AppendLine();

        // Room statistics
        var roomTypes = dungeon.Rooms.GroupBy(r => r.Type)
            .Select(g => new { Type = g.Key, Count = g.Count() });
        
        sb.AppendLine("Room Types:");
        foreach (var type in roomTypes)
        {
            sb.AppendLine($"  {type.Type}: {type.Count}");
        }
        sb.AppendLine();

        // Calculate map bounds
        int minX = dungeon.Rooms.Min(r => r.Bounds.X);
        int maxX = dungeon.Rooms.Max(r => r.Bounds.Right);
        int minY = dungeon.Rooms.Min(r => r.Bounds.Y);
        int maxY = dungeon.Rooms.Max(r => r.Bounds.Bottom);

        sb.AppendLine($"Map Dimensions: {maxX - minX + 1}x{maxY - minY + 1}");
        sb.AppendLine();
        sb.AppendLine("═══════════════════════════════════════════════════════");
        sb.AppendLine("                      MAP VIEW                         ");
        sb.AppendLine("═══════════════════════════════════════════════════════");
        sb.AppendLine();

        // Generate map view
        for (int y = minY; y <= maxY; y++)
        {
            for (int x = minX; x <= maxX; x++)
            {
                Point pos = new Point(x, y);
                sb.Append(GetCharAt(pos, dungeon));
            }
            sb.AppendLine();
        }

        sb.AppendLine();
        sb.AppendLine("═══════════════════════════════════════════════════════");
        sb.AppendLine("Legend:");
        sb.AppendLine("  # = Wall");
        sb.AppendLine("  . = Floor");
        sb.AppendLine("  : = Corridor");
        sb.AppendLine("  + = Exit/Door (Explored)");
        sb.AppendLine("  ? = Unexplored Exit");
        sb.AppendLine("═══════════════════════════════════════════════════════");

        // Add movement trace
        AppendMovementTrace(sb, explorer);

        return sb.ToString();
    }

    private char GetCharAt(Point pos, Dungeon dungeon)
    {
        Room? room = dungeon.GetRoomAt(pos);
        
        if (room == null)
            return ' ';

        // Check if exit
        foreach (var exit in room.Exits)
        {
            if (exit.Position == pos)
                return (exit.IsExplored || exit.ConnectedRoom?.IsVisible == true) ? '+' : '?';
        }

        // Check if wall
        if (IsWall(room, pos))
            return '#';

        // Floor
        return room.Type == RoomType.Corridor ? ':' : '.';
    }

    private bool IsWall(Room room, Point pos)
    {
        Rectangle bounds = room.Bounds;
        
        bool onLeft = pos.X == bounds.Left;
        bool onRight = pos.X == bounds.Right;
        bool onTop = pos.Y == bounds.Top;
        bool onBottom = pos.Y == bounds.Bottom;

        return onLeft || onRight || onTop || onBottom;
    }

    private void AppendMovementTrace(StringBuilder sb, Explorer explorer)
    {
        sb.AppendLine();
        sb.AppendLine("═══════════════════════════════════════════════════════");
        sb.AppendLine("                   MOVEMENT TRACE                      ");
        sb.AppendLine("═══════════════════════════════════════════════════════");
        
        var trace = explorer.MovementTrace;
        
        // Cap at last 500 events for export
        var eventsToShow = trace.Count > 500 ? trace.Skip(trace.Count - 500).ToList() : trace.ToList();
        
        sb.AppendLine($"Total events: {eventsToShow.Count}{(trace.Count > 500 ? " (showing last 500)" : "")}");
        sb.AppendLine();

        if (eventsToShow.Count == 0)
        {
            sb.AppendLine("No movement events recorded.");
            return;
        }

        // Group consecutive Move events in the same room
        var grouped = new List<(string line, int count)>();
        MovementEvent? lastMove = null;
        int moveCount = 0;
        Point? moveStart = null;

        foreach (var evt in eventsToShow)
        {
            if (evt.Action == "Move" && lastMove != null && 
                lastMove.Action == "Move" && lastMove.RoomId == evt.RoomId)
            {
                // Same room, consecutive moves - group them
                moveCount++;
            }
            else
            {
                // Flush previous group
                if (lastMove != null && lastMove.Action == "Move" && moveCount > 1)
                {
                    string line = FormatMovementEvent(lastMove, moveStart, moveCount);
                    grouped.Add((line, moveCount));
                }
                else if (lastMove != null)
                {
                    string line = FormatMovementEvent(lastMove, null, 1);
                    grouped.Add((line, 1));
                }

                // Start new group
                if (evt.Action == "Move")
                {
                    moveStart = evt.From;
                    moveCount = 1;
                }
                else
                {
                    moveCount = 0;
                    moveStart = null;
                }

                lastMove = evt;
            }
        }

        // Flush final group
        if (lastMove != null)
        {
            if (lastMove.Action == "Move" && moveCount > 1)
            {
                string line = FormatMovementEvent(lastMove, moveStart, moveCount);
                grouped.Add((line, moveCount));
            }
            else
            {
                string line = FormatMovementEvent(lastMove, null, 1);
                grouped.Add((line, 1));
            }
        }

        // Output all lines
        foreach (var (line, _) in grouped)
        {
            sb.AppendLine(line);
        }

        sb.AppendLine("═══════════════════════════════════════════════════════");
    }

    private string FormatMovementEvent(MovementEvent evt, Point? groupStart, int groupCount)
    {
        string time = evt.Timestamp.ToString("HH:mm:ss.fff");
        string action = evt.Action.PadRight(12);
        string roomInfo = evt.RoomId.HasValue ? $"Room:{evt.RoomId,-3} " : "Room:?   ";

        if (evt.Action == "Move" && groupCount > 1 && groupStart.HasValue)
        {
            // Grouped moves
            string positions = $"({groupStart.Value.X},{groupStart.Value.Y})→({evt.To.X},{evt.To.Y})";
            return $"{time} [Move x{groupCount,-5}] {roomInfo}{positions}";
        }
        else if (evt.Action == "Move")
        {
            // Single move
            string positions = $"({evt.From.X},{evt.From.Y})→({evt.To.X},{evt.To.Y})";
            return $"{time} [{action}] {roomInfo}{positions}";
        }
        else if (evt.Action == "RoomSwitch" || evt.Action == "ExitCrossed")
        {
            // Position doesn't change for these
            string position = $"({evt.From.X},{evt.From.Y})";
            string detail = evt.Detail ?? "";
            return $"{time} [{action}] {roomInfo}{position,-15} {detail}";
        }
        else
        {
            // PathPlanned, PathFallback
            string positions = $"→({evt.To.X},{evt.To.Y})";
            string detail = evt.Detail ?? "";
            return $"{time} [{action}] {roomInfo}{positions,-20} {detail}";
        }
    }
}

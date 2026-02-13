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
    public void ExportMap(Dungeon dungeon)
    {
        string filename = GenerateFilename();
        string filepath = Path.Combine(MAP_FOLDER, filename);

        var content = GenerateMapContent(dungeon);
        
        File.WriteAllText(filepath, content);
        Console.WriteLine($"Map exported to: {filepath}");
    }

    private string GenerateFilename()
    {
        return DateTime.Now.ToString("yyyy-MM-dd_HHmm") + ".txt";
    }

    private string GenerateMapContent(Dungeon dungeon)
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
        sb.AppendLine("  + = Exit/Door");
        sb.AppendLine("═══════════════════════════════════════════════════════");

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
                return '+';
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
}

using DungeonSaver.Models;
using DungeonSaver.Utils;
using System.Text;

namespace DungeonSaver.Rendering;

/// <summary>
/// Handles terminal rendering with ANSI escape codes
/// </summary>
public class Renderer
{
    private readonly ColorTheme _theme;
    private int _terminalWidth;
    private int _terminalHeight;
    private Point _cameraOffset;

    // ASCII characters (matching map export)
    private const char WALL = '#';
    private const char FLOOR = '.';
    private const char CORRIDOR_FLOOR = ':';
    private const char EXPLORER_CHAR = '@';
    private const char EXIT_EXPLORED = '+';
    private const char EXIT_UNEXPLORED = '?';
    private const char EXIT_SEALED = 'X';
    private const char FOG = ' ';  // Empty space for fog

    public Renderer()
    {
        _theme = new ColorTheme();
        _cameraOffset = new Point(0, 0);
        UpdateTerminalSize();
    }

    public void UpdateTerminalSize()
    {
        _terminalWidth = Console.WindowWidth;
        _terminalHeight = Console.WindowHeight;
    }

    public void ClearScreen()
    {
        Console.Clear();
        Console.Write(_theme.Background);
    }

    public void HideCursor()
    {
        Console.CursorVisible = false;
    }

    public void ShowCursor()
    {
        Console.CursorVisible = true;
    }

    /// <summary>
    /// Update camera to keep explorer centered
    /// </summary>
    public void UpdateCamera(Point explorerPos)
    {
        int centerX = _terminalWidth / 2;
        int centerY = _terminalHeight / 2;

        _cameraOffset.X = explorerPos.X - centerX;
        _cameraOffset.Y = explorerPos.Y - centerY;
    }

    /// <summary>
    /// Render the entire dungeon
    /// </summary>
    public void Render(Dungeon dungeon, Explorer explorer)
    {
        UpdateCamera(explorer.Position);
        
        var buffer = new StringBuilder();
        
        // Build the screen buffer
        for (int screenY = 0; screenY < _terminalHeight; screenY++)
        {
            for (int screenX = 0; screenX < _terminalWidth; screenX++)
            {
                int worldX = screenX + _cameraOffset.X;
                int worldY = screenY + _cameraOffset.Y;
                Point worldPos = new Point(worldX, worldY);

                // Check if explorer is at this position
                if (explorer.Position == worldPos)
                {
                    buffer.Append(_theme.Explorer);
                    buffer.Append(EXPLORER_CHAR);
                    buffer.Append(ColorTheme.Reset);
                    continue;
                }

                // Check if position is an exit (search all visible rooms — exits can share wall coords)
                Exit? exit = GetExitAtPosition(dungeon, worldPos);
                if (exit != null)
                {
                    RenderExit(buffer, exit);
                    continue;
                }

                // Find room at this position
                Room? room = dungeon.GetRoomAt(worldPos);
                
                if (room != null)
                {
                    // Apply fog of war
                    if (!room.IsVisible)
                    {
                        buffer.Append(_theme.FogOfWar);
                        buffer.Append(FOG);
                        buffer.Append(ColorTheme.Reset);
                        continue;
                    }

                    // Room is visible — check per-tile reveal state
                    if (!room.RevealedTiles.Contains(worldPos))
                    {
                        buffer.Append(_theme.FogOfWar);
                        buffer.Append(FOG);
                        buffer.Append(ColorTheme.Reset);
                        continue;
                    }

                    // Check if position is a wall or floor
                    if (IsWall(room, worldPos))
                    {
                        buffer.Append(_theme.Wall);
                        buffer.Append(GetWallChar(room, worldPos, dungeon));
                        buffer.Append(ColorTheme.Reset);
                    }
                    else
                    {
                        // Floor
                        string floorColor = room.Type == RoomType.Corridor ? 
                            _theme.CorridorFloor : _theme.RoomFloor;
                        char floorChar = room.Type == RoomType.Corridor ? 
                            CORRIDOR_FLOOR : FLOOR;
                        
                        buffer.Append(floorColor);
                        buffer.Append(floorChar);
                        buffer.Append(ColorTheme.Reset);
                    }
                }
                else
                {
                    // Empty space
                    buffer.Append(' ');
                }
            }
            
            if (screenY < _terminalHeight - 1)
                buffer.AppendLine();
        }

        // Render buffer to console
        Console.SetCursorPosition(0, 0);
        Console.Write(buffer.ToString());
    }

    private Exit? GetExitAtPosition(Dungeon dungeon, Point pos)
    {
        Exit? explored = null;
        Exit? unexplored = null;
        foreach (var room in dungeon.Rooms)
        {
            if (!room.IsVisible) continue;
            var exit = room.Exits.FirstOrDefault(e => e.Position == pos);
            if (exit == null) continue;
            if (exit.IsBlocked) return exit;
            if (exit.IsExplored || exit.ConnectedRoom?.IsVisible == true)
                explored ??= exit;
            else
                unexplored ??= exit;
        }
        return explored ?? unexplored;
    }

    private void RenderExit(StringBuilder buffer, Exit exit)
    {
        if (exit.IsBlocked)
        {
            buffer.Append(_theme.Wall);
            buffer.Append(EXIT_SEALED);
        }
        else if (exit.IsExplored || exit.ConnectedRoom?.IsVisible == true)
        {
            buffer.Append(_theme.ExploredExit);
            buffer.Append(EXIT_EXPLORED);
        }
        else
        {
            buffer.Append(_theme.UnexploredExit);
            buffer.Append(EXIT_UNEXPLORED);
        }
        buffer.Append(ColorTheme.Reset);
    }

    private bool IsWall(Room room, Point pos)
    {
        Rectangle bounds = room.Bounds;
        
        // Check if on border
        bool onLeft = pos.X == bounds.Left;
        bool onRight = pos.X == bounds.Right;
        bool onTop = pos.Y == bounds.Top;
        bool onBottom = pos.Y == bounds.Bottom;

        return onLeft || onRight || onTop || onBottom;
    }

    private char GetWallChar(Room room, Point pos, Dungeon dungeon)
    {
        // Simple ASCII walls - just use #
        return WALL;
    }
}

using DungeonSaver.Models;
using DungeonSaver.Core;
using DungeonSaver.Rendering;
using DungeonSaver.Utils;

namespace DungeonSaver;

/// <summary>
/// Main game loop for the dungeon explorer screensaver
/// </summary>
public class GameLoop
{
    private readonly Dungeon _dungeon;
    private readonly Explorer _explorer;
    private readonly DungeonBuilder _builder;
    private readonly ExplorerAI _ai;
    private readonly Renderer _renderer;
    
    private const int TARGET_FPS = 10;
    private const int FRAME_TIME_MS = 1000 / TARGET_FPS;
    
    private bool _running;
    private readonly bool _showRoomIds;

    public GameLoop(bool showRoomIds = false)
    {
        _showRoomIds = showRoomIds;
        _dungeon = new Dungeon(targetRoomCount: 20);
        _builder = new DungeonBuilder(_dungeon);
        
        // Create entrance room
        Room entrance = _builder.CreateEntranceRoom();
        
        // Place explorer in center of entrance room
        Point startPos = new Point(
            entrance.Bounds.X + entrance.Bounds.Width / 2,
            entrance.Bounds.Y + entrance.Bounds.Height / 2
        );
        
        _explorer = new Explorer(startPos);
        _explorer.CurrentRoom = entrance;
        _explorer.VisitedRoomIds.Add(entrance.Id);
        
        _ai = new ExplorerAI(_explorer, _dungeon, _builder);
        _renderer = new Renderer();
        _running = false;
    }

    /// <summary>
    /// Start the game loop
    /// </summary>
    public void Run()
    {
        try
        {
            _running = true;
            
            // Setup terminal
            _renderer.HideCursor();
            _renderer.ClearScreen();
            
            // Handle Ctrl+C gracefully
            Console.CancelKeyPress += (sender, e) =>
            {
                e.Cancel = true;
                _running = false;
            };

            DateTime lastFrameTime = DateTime.Now;

            while (_running)
            {
                DateTime frameStart = DateTime.Now;

                // Handle input
                if (Console.KeyAvailable)
                {
                    var key = Console.ReadKey(intercept: true);
                    if (key.Key == ConsoleKey.Q || key.KeyChar == 'q' || key.KeyChar == 'Q')
                    {
                        _running = false;
                        break;
                    }
                }

                // Update
                _ai.Update();

                // Render
                try
                {
                    _renderer.Render(_dungeon, _explorer);
                }
                catch (Exception)
                {
                    // Ignore rendering errors (terminal resize, etc.)
                }

                // Frame timing
                TimeSpan elapsed = DateTime.Now - frameStart;
                int sleepTime = FRAME_TIME_MS - (int)elapsed.TotalMilliseconds;
                
                if (sleepTime > 0)
                {
                    Thread.Sleep(sleepTime);
                }

                lastFrameTime = DateTime.Now;
            }
        }
        finally
        {
            // Cleanup
            Shutdown();
        }
    }

    private void Shutdown()
    {
        _renderer.ShowCursor();
        Console.Clear();
        Console.WriteLine();
        Console.WriteLine("═══════════════════════════════════════════");
        Console.WriteLine("         Dungeon Saver - Exiting");
        Console.WriteLine("═══════════════════════════════════════════");
        Console.WriteLine($"Generated {_dungeon.Rooms.Count} rooms");
        Console.WriteLine();
        
        // Export map
        try
        {
            var exporter = new MapExporter(showRoomIds: _showRoomIds);
            exporter.ExportMap(_dungeon, _explorer);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Error exporting map: {ex.Message}");
        }
        
        Console.WriteLine();
    }
}

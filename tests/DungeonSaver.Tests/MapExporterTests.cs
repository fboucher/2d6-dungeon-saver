using DungeonSaver.Core;
using DungeonSaver.Models;
using DungeonSaver.Utils;
using Xunit;
using System.Reflection;

namespace DungeonSaver.Tests;

public class MapExporterTests
{
    [Fact]
    public void GetCharAt_UnexploredExit_ReturnsQuestionMark()
    {
        // Arrange
        var dungeon = new Dungeon();
        var room = new Room(1, new Rectangle(10, 10, 8, 6), RoomType.Normal);
        room.IsVisible = true;
        
        var exitPos = new Point(room.Bounds.Right, 12);
        var exit = new Exit(exitPos, Direction.East);
        exit.IsExplored = false;
        exit.ConnectedRoom = null; // Not connected yet
        room.Exits.Add(exit);
        dungeon.Rooms.Add(room);

        var exporter = new MapExporter();

        // Act
        char result = InvokeGetCharAt(exporter, exitPos, dungeon);

        // Assert
        Assert.Equal('?', result);
    }

    [Fact]
    public void GetCharAt_ExploredExit_ReturnsPlusSign()
    {
        // Arrange
        var dungeon = new Dungeon();
        var room = new Room(1, new Rectangle(10, 10, 8, 6), RoomType.Normal);
        room.IsVisible = true;
        
        var exitPos = new Point(room.Bounds.Right, 12);
        var exit = new Exit(exitPos, Direction.East);
        exit.IsExplored = true; // Explored exit
        room.Exits.Add(exit);
        dungeon.Rooms.Add(room);

        var exporter = new MapExporter();

        // Act
        char result = InvokeGetCharAt(exporter, exitPos, dungeon);

        // Assert
        Assert.Equal('+', result);
    }

    [Fact]
    public void GetCharAt_ExitWithVisibleConnectedRoom_ReturnsPlusSign()
    {
        // Arrange
        var dungeon = new Dungeon();
        var room1 = new Room(1, new Rectangle(10, 10, 8, 6), RoomType.Normal);
        room1.IsVisible = true;
        
        var room2 = new Room(2, new Rectangle(18, 10, 8, 6), RoomType.Normal);
        room2.IsVisible = true; // Connected room is visible
        
        var exitPos = new Point(room1.Bounds.Right, 12);
        var exit = new Exit(exitPos, Direction.East);
        exit.IsExplored = false; // Not explored yet
        exit.ConnectedRoom = room2; // But connected to visible room
        room1.Exits.Add(exit);
        
        dungeon.Rooms.Add(room1);
        dungeon.Rooms.Add(room2);

        var exporter = new MapExporter();

        // Act
        char result = InvokeGetCharAt(exporter, exitPos, dungeon);

        // Assert
        Assert.Equal('+', result);
    }

    [Fact]
    public void GetCharAt_ExitWithNullConnectedRoom_ReturnsQuestionMark()
    {
        // Arrange
        var dungeon = new Dungeon();
        var room = new Room(1, new Rectangle(10, 10, 8, 6), RoomType.Normal);
        room.IsVisible = true;
        
        var exitPos = new Point(room.Bounds.Right, 12);
        var exit = new Exit(exitPos, Direction.East);
        exit.IsExplored = false;
        exit.ConnectedRoom = null; // No connected room
        room.Exits.Add(exit);
        dungeon.Rooms.Add(room);

        var exporter = new MapExporter();

        // Act
        char result = InvokeGetCharAt(exporter, exitPos, dungeon);

        // Assert
        Assert.Equal('?', result);
    }

    [Fact]
    public void GetCharAt_ExitWithInvisibleConnectedRoom_ReturnsQuestionMark()
    {
        // Arrange
        var dungeon = new Dungeon();
        var room1 = new Room(1, new Rectangle(10, 10, 8, 6), RoomType.Normal);
        room1.IsVisible = true;
        
        var room2 = new Room(2, new Rectangle(18, 10, 8, 6), RoomType.Normal);
        room2.IsVisible = false; // Connected room is NOT visible
        
        var exitPos = new Point(room1.Bounds.Right, 12);
        var exit = new Exit(exitPos, Direction.East);
        exit.IsExplored = false;
        exit.ConnectedRoom = room2; // Connected but not visible
        room1.Exits.Add(exit);
        
        dungeon.Rooms.Add(room1);
        dungeon.Rooms.Add(room2);

        var exporter = new MapExporter();

        // Act
        char result = InvokeGetCharAt(exporter, exitPos, dungeon);

        // Assert
        Assert.Equal('?', result);
    }

    [Fact]
    public void GetCharAt_WallPosition_ReturnsHash()
    {
        // Arrange
        var dungeon = new Dungeon();
        var room = new Room(1, new Rectangle(10, 10, 8, 6), RoomType.Normal);
        room.IsVisible = true;
        dungeon.Rooms.Add(room);

        var exporter = new MapExporter();
        var wallPos = new Point(room.Bounds.Left, room.Bounds.Top);

        // Act
        char result = InvokeGetCharAt(exporter, wallPos, dungeon);

        // Assert
        Assert.Equal('#', result);
    }

    [Fact]
    public void GetCharAt_FloorPosition_ReturnsDot()
    {
        // Arrange
        var dungeon = new Dungeon();
        var room = new Room(1, new Rectangle(10, 10, 8, 6), RoomType.Normal);
        room.IsVisible = true;
        dungeon.Rooms.Add(room);

        var exporter = new MapExporter();
        var floorPos = new Point(room.Bounds.Left + 1, room.Bounds.Top + 1);

        // Act
        char result = InvokeGetCharAt(exporter, floorPos, dungeon);

        // Assert
        Assert.Equal('.', result);
    }

    [Fact]
    public void GetCharAt_CorridorFloor_ReturnsColon()
    {
        // Arrange
        var dungeon = new Dungeon();
        var corridor = new Room(1, new Rectangle(10, 10, 3, 3), RoomType.Corridor);
        corridor.IsVisible = true;
        dungeon.Rooms.Add(corridor);

        var exporter = new MapExporter();
        var floorPos = new Point(corridor.Bounds.Left + 1, corridor.Bounds.Top + 1);

        // Act
        char result = InvokeGetCharAt(exporter, floorPos, dungeon);

        // Assert
        Assert.Equal(':', result);
    }

    // Helper method to invoke private GetCharAt method using reflection
    private char InvokeGetCharAt(MapExporter exporter, Point pos, Dungeon dungeon)
    {
        var method = typeof(MapExporter).GetMethod("GetCharAt", 
            BindingFlags.NonPublic | BindingFlags.Instance);
        
        if (method == null)
            throw new InvalidOperationException("GetCharAt method not found");
        
        return (char)method.Invoke(exporter, new object[] { pos, dungeon })!;
    }
}

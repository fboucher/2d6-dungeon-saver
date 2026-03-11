using DungeonSaver.Core;
using DungeonSaver.Models;
using DungeonSaver.Utils;
using Xunit;

namespace DungeonSaver.Tests;

public class DungeonBuilderTests
{
    [Fact]
    public void GenerateRoomAtExit_East_NewRoomSharesWallWithParent()
    {
        // Arrange
        var dungeon = new Dungeon();
        var builder = new DungeonBuilder(dungeon, seed: 42);
        
        var parentRoom = new Room(1, new Rectangle(10, 10, 8, 6), RoomType.Normal);
        var exitPos = new Point(parentRoom.Bounds.Right, 12); // On right wall
        var exit = new Exit(exitPos, Direction.East);
        parentRoom.Exits.Add(exit);
        dungeon.Rooms.Add(parentRoom);

        // Act
        var newRoom = builder.GenerateRoomAtExit(exit, parentRoom);

        // Assert
        Assert.NotNull(newRoom);
        // New room's left wall should touch parent's right wall (share the wall)
        Assert.Equal(parentRoom.Bounds.Right + 1, newRoom.Bounds.Left);
        // Verify no gap and no double wall
        Assert.Equal(exitPos.X + 1, newRoom.Bounds.Left);
    }

    [Fact]
    public void GenerateRoomAtExit_West_NewRoomSharesWallWithParent()
    {
        // Arrange
        var dungeon = new Dungeon();
        var builder = new DungeonBuilder(dungeon, seed: 42);
        
        var parentRoom = new Room(1, new Rectangle(20, 10, 8, 6), RoomType.Normal);
        var exitPos = new Point(parentRoom.Bounds.Left, 12); // On left wall
        var exit = new Exit(exitPos, Direction.West);
        parentRoom.Exits.Add(exit);
        dungeon.Rooms.Add(parentRoom);

        // Act
        var newRoom = builder.GenerateRoomAtExit(exit, parentRoom);

        // Assert
        Assert.NotNull(newRoom);
        // New room's right wall should touch parent's left wall (share the wall)
        Assert.Equal(parentRoom.Bounds.Left - 1, newRoom.Bounds.Right);
        // Verify exit position is the shared wall
        Assert.Equal(exitPos.X, parentRoom.Bounds.Left);
    }

    [Fact]
    public void GenerateRoomAtExit_South_NewRoomSharesWallWithParent()
    {
        // Arrange
        var dungeon = new Dungeon();
        var builder = new DungeonBuilder(dungeon, seed: 42);
        
        var parentRoom = new Room(1, new Rectangle(10, 10, 8, 6), RoomType.Normal);
        var exitPos = new Point(13, parentRoom.Bounds.Bottom); // On bottom wall
        var exit = new Exit(exitPos, Direction.South);
        parentRoom.Exits.Add(exit);
        dungeon.Rooms.Add(parentRoom);

        // Act
        var newRoom = builder.GenerateRoomAtExit(exit, parentRoom);

        // Assert
        Assert.NotNull(newRoom);
        // New room's top wall should touch parent's bottom wall (share the wall)
        Assert.Equal(parentRoom.Bounds.Bottom + 1, newRoom.Bounds.Top);
        // Verify no gap
        Assert.Equal(exitPos.Y + 1, newRoom.Bounds.Top);
    }

    [Fact]
    public void GenerateRoomAtExit_North_NewRoomSharesWallWithParent()
    {
        // Arrange
        var dungeon = new Dungeon();
        var builder = new DungeonBuilder(dungeon, seed: 42);
        
        var parentRoom = new Room(1, new Rectangle(10, 20, 8, 6), RoomType.Normal);
        var exitPos = new Point(13, parentRoom.Bounds.Top); // On top wall
        var exit = new Exit(exitPos, Direction.North);
        parentRoom.Exits.Add(exit);
        dungeon.Rooms.Add(parentRoom);

        // Act
        var newRoom = builder.GenerateRoomAtExit(exit, parentRoom);

        // Assert
        Assert.NotNull(newRoom);
        // New room's bottom wall should touch parent's top wall (share the wall)
        Assert.Equal(parentRoom.Bounds.Top - 1, newRoom.Bounds.Bottom);
        // Verify exit position is at the shared wall
        Assert.Equal(exitPos.Y, parentRoom.Bounds.Top);
    }

    [Fact]
    public void GenerateRoomAtExit_NoDoubleWall_BetweenAdjacentRooms()
    {
        // Arrange
        var dungeon = new Dungeon();
        var builder = new DungeonBuilder(dungeon, seed: 42);
        
        var room1 = new Room(1, new Rectangle(10, 10, 8, 6), RoomType.Normal);
        var exitPos = new Point(room1.Bounds.Right, 12); // East exit
        var exit = new Exit(exitPos, Direction.East);
        room1.Exits.Add(exit);
        dungeon.Rooms.Add(room1);

        // Act
        var room2 = builder.GenerateRoomAtExit(exit, room1);

        // Assert
        Assert.NotNull(room2);
        
        // The exit position should be the shared wall position
        // Room1's right wall is at X = room1.Bounds.Right
        // Room2's left wall is at X = room2.Bounds.Left
        // They should be adjacent: room2.Left = room1.Right + 1
        Assert.Equal(room1.Bounds.Right + 1, room2.Bounds.Left);
        
        // The exit is on the wall at room1.Bounds.Right
        // This means there's only ONE wall column at that X coordinate, not two
        Assert.Equal(room1.Bounds.Right, exitPos.X);
        Assert.Equal(exitPos.X + 1, room2.Bounds.Left);
        
        // Verify rooms don't overlap (they should be touching but not intersecting)
        Assert.False(room1.Intersects(room2), 
            "Adjacent rooms sharing a wall should not intersect (touches only)");
    }

    [Fact]
    public void GenerateRoomAtExit_MultipleRooms_NoCollisions()
    {
        // Arrange
        var dungeon = new Dungeon();
        var builder = new DungeonBuilder(dungeon, seed: 123);
        
        // Create entrance
        var entrance = builder.CreateEntranceRoom();
        
        // Act - Generate rooms at multiple exits
        Room? eastRoom = null;
        Room? westRoom = null;
        
        foreach (var exit in entrance.Exits)
        {
            if (exit.Direction == Direction.East && eastRoom == null)
            {
                eastRoom = builder.GenerateRoomAtExit(exit, entrance);
            }
            else if (exit.Direction == Direction.West && westRoom == null)
            {
                westRoom = builder.GenerateRoomAtExit(exit, entrance);
            }
        }

        // Assert
        if (eastRoom != null && westRoom != null)
        {
            Assert.False(eastRoom.Intersects(westRoom), 
                "Rooms generated from the same parent should not overlap");
        }
        
        // Verify all rooms in dungeon don't overlap
        for (int i = 0; i < dungeon.Rooms.Count; i++)
        {
            for (int j = i + 1; j < dungeon.Rooms.Count; j++)
            {
                Assert.False(dungeon.Rooms[i].Intersects(dungeon.Rooms[j]),
                    $"Room {i} and Room {j} should not overlap");
            }
        }
    }
}

using DungeonSaver.Models;
using DungeonSaver.Utils;
using Xunit;

namespace DungeonSaver.Tests;

public class DungeonTests
{
    [Fact]
    public void GetExitAt_BlockedExitAndExploredExitAtSamePosition_ReturnsBlockedExit()
    {
        var dungeon = new Dungeon();
        
        var sharedPos = new Point(14, 10);
        
        var room1 = new Room(1, new Rectangle(10, 5, 9, 6), RoomType.Normal);
        room1.IsVisible = true;
        var exploredExit = new Exit(sharedPos, Direction.South) { IsExplored = true };
        room1.Exits.Add(exploredExit);
        dungeon.Rooms.Add(room1);
        
        var room2 = new Room(2, new Rectangle(10, 10, 9, 6), RoomType.Normal);
        room2.IsVisible = true;
        var blockedExit = new Exit(sharedPos, Direction.North) { IsBlocked = true, IsExplored = true };
        room2.Exits.Add(blockedExit);
        dungeon.Rooms.Add(room2);
        
        var result = dungeon.GetExitAt(sharedPos);
        
        Assert.NotNull(result);
        Assert.True(result.IsBlocked);
    }

    [Fact]
    public void GetExitAt_NoExitAtPosition_ReturnsNull()
    {
        var dungeon = new Dungeon();
        var room = new Room(1, new Rectangle(10, 10, 8, 6), RoomType.Normal);
        room.IsVisible = true;
        dungeon.Rooms.Add(room);
        
        var result = dungeon.GetExitAt(new Point(11, 11));  // Floor position, no exit
        
        Assert.Null(result);
    }

    [Fact]
    public void GetExitAt_SingleBlockedExit_ReturnsIt()
    {
        var dungeon = new Dungeon();
        var room = new Room(1, new Rectangle(10, 10, 8, 6), RoomType.Normal);
        room.IsVisible = true;
        var pos = new Point(17, 13);
        var exit = new Exit(pos, Direction.East) { IsBlocked = true, IsExplored = true };
        room.Exits.Add(exit);
        dungeon.Rooms.Add(room);
        
        var result = dungeon.GetExitAt(pos);
        
        Assert.NotNull(result);
        Assert.True(result.IsBlocked);
    }
}

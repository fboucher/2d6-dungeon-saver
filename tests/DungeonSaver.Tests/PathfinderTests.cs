using DungeonSaver.Core;
using DungeonSaver.Models;
using DungeonSaver.Utils;
using Xunit;

namespace DungeonSaver.Tests;

public class PathfinderTests
{
    [Fact]
    public void FindPath_GoalOutsideRoom_ReturnsEmptyList()
    {
        // Arrange
        var room = new Room(1, new Rectangle(10, 10, 8, 6), RoomType.Normal);
        var pathfinder = new Pathfinder();

        var start = new Point(11, 11); // floor tile inside the room
        var goal = new Point(5, 5);    // outside the room — not walkable

        // Act
        var result = pathfinder.FindPath(start, goal, room);

        // Assert — no path exists; must return empty list, not [start, goal]
        Assert.Empty(result);
    }
}

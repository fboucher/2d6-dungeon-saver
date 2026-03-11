using DungeonSaver.Utils;
using Xunit;

namespace DungeonSaver.Tests;

public class RectangleTests
{
    [Fact]
    public void Intersects_OverlappingRects_ReturnsTrue()
    {
        // Arrange
        var rect1 = new Rectangle(0, 0, 10, 10);
        var rect2 = new Rectangle(5, 5, 10, 10);

        // Act
        bool result = rect1.Intersects(rect2);

        // Assert
        Assert.True(result, "Overlapping rectangles should intersect");
    }

    [Fact]
    public void Intersects_TouchingRects_ReturnsFalse()
    {
        // Arrange - Two rectangles sharing a wall (touching but not overlapping)
        var rect1 = new Rectangle(0, 0, 10, 10); // Right edge at X=9
        var rect2 = new Rectangle(10, 0, 10, 10); // Left edge at X=10

        // Act
        bool result = rect1.Intersects(rect2);

        // Assert
        Assert.False(result, "Rectangles that only touch at an edge should NOT intersect (fixes double wall bug)");
    }

    [Fact]
    public void Intersects_DisjointRects_ReturnsFalse()
    {
        // Arrange
        var rect1 = new Rectangle(0, 0, 5, 5);
        var rect2 = new Rectangle(10, 10, 5, 5);

        // Act
        bool result = rect1.Intersects(rect2);

        // Assert
        Assert.False(result, "Disjoint rectangles should not intersect");
    }

    [Fact]
    public void Intersects_SameRect_ReturnsTrue()
    {
        // Arrange
        var rect1 = new Rectangle(5, 5, 10, 10);
        var rect2 = new Rectangle(5, 5, 10, 10);

        // Act
        bool result = rect1.Intersects(rect2);

        // Assert
        Assert.True(result, "Identical rectangles should intersect");
    }

    [Fact]
    public void Intersects_PartialOverlap_ReturnsTrue()
    {
        // Arrange - One rectangle partially inside another
        var rect1 = new Rectangle(0, 0, 10, 10);
        var rect2 = new Rectangle(8, 8, 5, 5);

        // Act
        bool result = rect1.Intersects(rect2);

        // Assert
        Assert.True(result, "Partially overlapping rectangles should intersect");
    }

    [Fact]
    public void Intersects_TouchingVertically_ReturnsFalse()
    {
        // Arrange - Two rectangles stacked vertically (touching top to bottom)
        var rect1 = new Rectangle(0, 0, 10, 10); // Bottom edge at Y=9
        var rect2 = new Rectangle(0, 10, 10, 10); // Top edge at Y=10

        // Act
        bool result = rect1.Intersects(rect2);

        // Assert
        Assert.False(result, "Rectangles touching vertically should NOT intersect");
    }

    [Fact]
    public void Intersects_TouchingHorizontally_ReturnsFalse()
    {
        // Arrange - Two rectangles side by side (touching left to right)
        var rect1 = new Rectangle(0, 0, 10, 10); // Right edge at X=9
        var rect2 = new Rectangle(10, 0, 10, 10); // Left edge at X=10

        // Act
        bool result = rect1.Intersects(rect2);

        // Assert
        Assert.False(result, "Rectangles touching horizontally should NOT intersect");
    }
}

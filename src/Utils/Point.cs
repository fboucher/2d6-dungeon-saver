namespace DungeonSaver.Utils;

/// <summary>
/// Represents a point in 2D space
/// </summary>
public struct Point
{
    public int X { get; set; }
    public int Y { get; set; }

    public Point(int x, int y)
    {
        X = x;
        Y = y;
    }

    public static Point operator +(Point a, Point b) => new(a.X + b.X, a.Y + b.Y);
    public static Point operator -(Point a, Point b) => new(a.X - b.X, a.Y - b.Y);
    
    public override string ToString() => $"({X}, {Y})";
    
    public override bool Equals(object? obj) => obj is Point other && X == other.X && Y == other.Y;
    public override int GetHashCode() => HashCode.Combine(X, Y);
    
    public static bool operator ==(Point left, Point right) => left.Equals(right);
    public static bool operator !=(Point left, Point right) => !left.Equals(right);
}

namespace DungeonSaver.Utils;

/// <summary>
/// Represents a rectangle in 2D space
/// </summary>
public struct Rectangle
{
    public int X { get; set; }
    public int Y { get; set; }
    public int Width { get; set; }
    public int Height { get; set; }

    public Rectangle(int x, int y, int width, int height)
    {
        X = x;
        Y = y;
        Width = width;
        Height = height;
    }

    public int Left => X;
    public int Right => X + Width - 1;
    public int Top => Y;
    public int Bottom => Y + Height - 1;
    
    public int Area => Width * Height;
    
    public bool Contains(Point point) => 
        point.X >= X && point.X < X + Width && 
        point.Y >= Y && point.Y < Y + Height;
    
    public bool Intersects(Rectangle other) =>
        Left <= other.Right && Right >= other.Left &&
        Top <= other.Bottom && Bottom >= other.Top;
    
    public override string ToString() => $"Rect({X},{Y} {Width}x{Height})";
}

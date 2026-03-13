namespace DungeonSaver.Rendering;

/// <summary>
/// Catppuccin Mocha color theme
/// Reference: https://catppuccin.com/palette/
/// </summary>
public class ColorTheme
{
    // Catppuccin Mocha colors (ANSI 256 color codes)
    public string Wall { get; }              // Lavender (foreground)
    public string RoomFloor { get; }         // Surface1 background — revealed room floor
    public string CorridorFloor { get; }     // Surface0 background — revealed corridor floor
    public string RoomFloorFg { get; }       // Subtle foreground texture on revealed room floor
    public string CorridorFloorFg { get; }   // Subtle foreground texture on revealed corridor floor
    public string FoggedFloor { get; }       // Crust background — fogged tile inside visible room
    public string FoggedFloorFg { get; }     // Very dim foreground on fogged tile
    public string ExploredExit { get; }      // Green
    public string UnexploredExit { get; }    // Yellow
    public string Explorer { get; }          // Peach
    public string Background { get; }        // Base (very dark global bg)
    public string FogOfWar { get; }          // Crust foreground — invisible room
    public string Text { get; }              // Text

    public ColorTheme()
    {
        // Using ANSI 256 colors approximating Catppuccin Mocha
        Wall = "\x1b[38;5;183m";              // Lavender (light purple)
        RoomFloor = "\x1b[48;5;238m";         // Surface1 bg — clearly visible revealed floor
        CorridorFloor = "\x1b[48;5;236m";     // Surface0 bg — slightly darker corridor
        RoomFloorFg = "\x1b[38;5;240m";       // Dim foreground texture on revealed room floor
        CorridorFloorFg = "\x1b[38;5;237m";   // Dim foreground texture on revealed corridor
        FoggedFloor = "\x1b[48;5;233m";       // Crust bg — very dark, nearly invisible fog
        FoggedFloorFg = "\x1b[38;5;234m";     // Barely visible dot on fogged tile
        ExploredExit = "\x1b[38;5;115m";      // Green
        UnexploredExit = "\x1b[38;5;222m";    // Yellow
        Explorer = "\x1b[38;5;216m";          // Peach (orange)
        Background = "\x1b[48;5;232m";        // Base background (very dark)
        FogOfWar = "\x1b[38;5;233m";          // Crust (barely visible)
        Text = "\x1b[38;5;205m";              // Text (light)
    }

    public const string Reset = "\x1b[0m";
}

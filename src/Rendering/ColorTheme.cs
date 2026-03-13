namespace DungeonSaver.Rendering;

/// <summary>
/// Catppuccin Mocha color theme
/// Reference: https://catppuccin.com/palette/
/// </summary>
public class ColorTheme
{
    // Catppuccin Mocha colors (ANSI 256 color codes)
    public string Wall { get; }           // Lavender
    public string RoomFloor { get; }      // Surface0
    public string CorridorFloor { get; }  // Base
    public string ExploredExit { get; }   // Green
    public string UnexploredExit { get; } // Yellow
    public string Explorer { get; }       // Peach
    public string Background { get; }     // Base (dark)
    public string FogOfWar { get; }       // Crust (very dark)
    public string FoggedFloor { get; }    // Mantle (dim — unrevealed floor in visible room)
    public string Text { get; }           // Text

    public ColorTheme()
    {
        // Using ANSI 256 colors approximating Catppuccin Mocha
        Wall = "\x1b[38;5;183m";           // Lavender (light purple)
        RoomFloor = "\x1b[38;5;236m";      // Surface0 (dark gray)
        CorridorFloor = "\x1b[38;5;234m";  // Base (darker gray)
        ExploredExit = "\x1b[38;5;115m";   // Green
        UnexploredExit = "\x1b[38;5;222m"; // Yellow
        Explorer = "\x1b[38;5;216m";       // Peach (orange)
        Background = "\x1b[48;5;232m";     // Base background (very dark)
        FogOfWar = "\x1b[38;5;233m";       // Crust (barely visible)
        FoggedFloor = "\x1b[38;5;235m";    // Mantle (dim shadow — unrevealed floor within visible room)
        Text = "\x1b[38;5;205m";           // Text (light)
    }

    public const string Reset = "\x1b[0m";
}

# Dungeon Saver

A terminal-based dungeon explorer screensaver that runs in your terminal. Watch an explorer as they discover each room and wander through a procedurally generated dungeon.

## Features

- **Full terminal takeover** - Uses the entire terminal window like a screensaver
- **Procedurally generated dungeons** - Each dungeon is unique, based on 2D6 pen & paper game rules
- **AI explorer** - An autonomous explorer character that navigates the dungeon using A* pathfinding
- **Top-down view** - Classic ASCII/Unicode top-down perspective with beautiful box-drawing characters
- **Real-time animation** - Watch the explorer discover rooms and wander the dungeon
- **Fog of war** - Unvisited rooms are hidden; only explored areas are visible
- **Catppuccin Mocha theme** - Beautiful color scheme for walls, floors, and the explorer
- **Map export** - Saves generated dungeons to text files in the `maps/` folder

## Build & Run

### Requirements
- .NET 10.0 or later
- Linux terminal with Unicode support

### Build
```bash
cd src
dotnet build
```

### Run
```bash
cd src
dotnet run
```

Or from the build output:
```bash
./src/bin/Debug/net10.0/DungeonSaver
```

### Controls
- **q** or **Q** - Quit the screensaver
- **Ctrl+C** - Emergency exit

## How It Works

- When the application starts, the screen shows only the entrance room
- As time passes, watch the explorer (@) navigate through the dungeon
- Each time the explorer enters a new room, they pause briefly before continuing
- The dungeon is generated progressively as the explorer opens doors, one room at a time
- Unexplored exits are marked with **?** in yellow
- Explored exits are marked with **▪** in green
- The viewport automatically pans to keep the explorer centered on screen

## Dungeon Generation Rules

The dungeon generator follows strict 2D6 pen & paper game rules:

### Dungeon
- **Size:** Dungeons have around 20 rooms
- **Progressive generation:** Rooms are created as the explorer discovers them

### Colors
Each element has its own color from the Catppuccin Mocha theme:
- **Walls:** Lavender
- **Room floors:** Dark gray
- **Corridor floors:** Darker gray
- **Explored exits:** Green
- **Unexplored exits:** Yellow
- **Explorer:** Peach (orange)
- **Fog of war:** Very dark (unexplored areas)

### Visual Example

```
     ####
     #..#
####+#+####
#:::::::..#
#:::::::..#
###########+####
          #...@#
          #....#
          ######
```

- `@` = Explorer
- `+` = Explored exit (green)
- `?` = Unexplored exit (yellow)
- `.` = Floor
- `:` = Corridor
- `#` = Walls

### Map Export
When the application exits, the dungeon is automatically saved to:
```
maps/yyyy-MM-dd_HHmm.txt
```

The file includes:
- ASCII representation of the full dungeon
- Room count and statistics
- Legend for map symbols

## Technical Details

- **Terminal responsive**: Adapts to your terminal size
- **Smooth animation**: Runs at 10 FPS with minimal CPU usage
- **Platform**: Linux terminal application
- **Language**: C# (.NET 10.0)
- **Pathfinding**: A* algorithm for explorer movement
- **Fog of war**: Only explored rooms are visible (except entrance)

## The Explorer
- Automatically explores unvisited rooms first
- Uses pathfinding to navigate efficiently
- Wanders randomly after exploring all rooms
- Pauses 1-2 seconds when discovering new rooms
- Moves at a contemplative pace (500ms between steps)

## Project Structure

```
src/
├── Core/           # Game logic and dungeon generation
│   ├── DiceRoller.cs
│   ├── DungeonBuilder.cs
│   ├── ExitGenerator.cs
│   ├── ExplorerAI.cs
│   ├── GameLoop.cs
│   ├── MapExporter.cs
│   ├── Pathfinder.cs
│   └── RoomGenerator.cs
├── Models/         # Data models
│   ├── Dungeon.cs
│   ├── Exit.cs
│   ├── Explorer.cs
│   └── Room.cs
├── Rendering/      # Terminal rendering
│   ├── ColorTheme.cs
│   └── Renderer.cs
├── Utils/          # Utility classes
│   ├── Point.cs
│   └── Rectangle.cs
└── Program.cs      # Entry point
```

## Credits

Based on 2D6 dungeon generation rules from the pen & paper game system.
Color theme: [Catppuccin Mocha](https://catppuccin.com/palette/)


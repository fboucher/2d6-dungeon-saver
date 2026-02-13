# Dungeon Saver

A terminal-based dungeon explorer screensaver that runs in your terminal. Watch an explorer as he discover each rooms and wander of a dungeon.

## Features

- **Full terminal takeover** - Uses the entire terminal window like a screensaver
- **Procedurally generated dungeons** - Each dungeon is unique
- **AI explorer** - An autonomous explorer character that navigates the dungeon using pathfinding
- **Top-down view** - Classic ASCII/Unicode top-down perspective
- **Real-time animation** - Watch the explorer discover rooms and wander the dungeon
- **Map export** - Saves generated dungeons to text files in the `maps/` folder


### Controls
- **q** or **Q** - Quit the screensaver
- **Ctrl-C** - Emergency exit

## Dungeon Generation Rules

The dungeon generator follows strict rules to ensure consistency:

### Dungeon

- **Size:** Dungeon have arround 20 rooms

### Colors

- Each important element of the dungeon, walls, floor of a corridor, or floor of a room, doors, explorer, have their own colours.
- Define the color in a group as a theme. So in the future if we want to add different themes it will be easier.
- For the default theme, use color from Catppuccin Mocha (ref: https://catppuccin.com/palette/).

### Map Export
When the application exits, the dungeon is automatically saved to:
```
maps/yyyy-MM-dd_HHmm_seed<seed>.txt
```

The file includes:
- ASCII representation of the full dungeon
- Room count and dimensions


## How It Works

- When the application start, the screen is empty.
- As time pass, we watch the explorer exploring the dungeon.
- Each time the explorer enter a new room, it can make a short pause before continuing.
- The dungeon is generated as the explorer open doors.
- To know how to generate a room and doors and all the dungeon, refer to the document [2D6 Rules](./2D6%20Rules.md).
  - In the Rules D6 or 2D6 refer to a rolling dice 1 or 2 dice with 6 faces.   
  - This is an adaptation of a pen & paper game, so we don't care about we don't care about the grid paper and the size of the dungeon.
  - For now we can ignore what's in the room. We just draw rooms.
  - For now, let's forget about different level. This dungeon is one level only.
  - If the dungeon go outside the size of the screen, just pan the screen so we can still see it.
  - The explorer should always be kind of in the middle of the screen, not necessarily like the flush center, but middle.So when the explorer arrive, let's say at the last quarter of the screen, pan the screen so it stay in the kind of middle area.


### The Explorer
- Automatically explores unvisited rooms first
- Wanders randomly after exploring all rooms
- Pauses briefly when discovering new rooms

## Technical Details

- **Terminal responsive**: Adapts to your terminal size
- **Smooth animation**: Runs at 10 FPS with minimal CPU usage
- **platform**: Works on Linux in the terminal. This is a terminal application.
- **D6 or 2D6**: Refer to a rolling dice 1 or 2 dice

# Dungeon Saver

A terminal-based dungeon explorer screensaver. Watch an AI-controlled explorer discover and navigate through a procedurally generated dungeon using the 2D6 dice mechanics.

## Features

- **Terminal screensaver** — Full terminal takeover with graceful exit
- **Procedurally generated dungeons** — Using 2D6 dice mechanics from the original pen & paper rules
- **Deterministic generation** — Same seed produces identical dungeons every time
- **AI explorer** — Autonomous pathfinding explorer that discovers rooms and wanders
- **Smart camera** — Follows explorer with smooth panning (comfort zone logic)
- **ASCII/Unicode rendering** — Classic top-down dungeon map perspective
- **Real-time animation** — Smooth 10 FPS animation with <1% CPU usage (release build)
- **Catppuccin Mocha theme** — Beautiful color palette for walls, floors, doors
- **Map export** — Dungeons automatically saved to `maps/` directory on exit with metadata
- **CLI arguments** — Use `--seed` flag for reproducible dungeons

## Controls

- **q** or **Q** — Quit the screensaver
- **Ctrl+C** — Emergency exit

## Building

```bash
cargo build --release
```

## Running

```bash
# With a random seed
cargo run --release

# With a specific seed (e.g., 12345)
cargo run --release -- --seed 12345
```

The screensaver will run fullscreen and automatically export the dungeon map to `maps/` when you quit (press `q` or `Ctrl+C`).

## Project Structure

- `src/main.rs` — Application entry point, terminal setup, main event loop
- `src/dungeon/` — Dungeon generation (2D6 rules, room placement, exit handling)
- `src/explorer/` — Explorer AI and pathfinding logic
- `src/renderer/` — Ratatui rendering and screen management
- `src/theme/` — Color palette and theming
- `src/rng.rs` — Seeded RNG wrapper for deterministic generation

## Technical Details

- **Language:** Rust (no runtime dependencies)
- **Terminal Framework:** Ratatui with Crossterm backend
- **Randomness:** rand_chacha for deterministic seed-based generation
- **Pathfinding:** A* algorithm via the pathfinding crate
- **Color Support:** 24-bit true color (Catppuccin Mocha)

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run dungeon generator tests only
cargo test --lib dungeon::generator

# Run with output
cargo test -- --nocapture
```

### Testing Dungeon Generation

The `generate_dungeon` example lets you visualize dungeon generation with different seeds:

```bash
# Generate with default seed (42)
cargo run --example generate_dungeon

# Generate with specific seed
cargo run --example generate_dungeon 999

# Quiet mode (summary only)
cargo run --quiet --example generate_dungeon 12345
```

### Implementation Status

✅ **Phase 1** — Terminal takeover, event loop, graceful shutdown  
✅ **Phase 2** — 2D6 dungeon generation engine (24 comprehensive tests)  
✅ **Phase 3** — Rendering and camera system with Catppuccin Mocha theming  
✅ **Phase 4** — Explorer AI and pathfinding (autonomous room discovery)  
✅ **Phase 5** — Map export, CLI arguments, polish (ready for distribution)

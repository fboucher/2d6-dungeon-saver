# Routing — Dungeon Saver

## Routing Table

| Domain | Agent | Example Tasks |
|--------|-------|---------------|
| Architecture, scope, decisions | Mikey | Project structure, tech stack selection, 2D6 rules interpretation, code review |
| Terminal UI, rendering, colors, animation | Chunk | TUI framework setup, ASCII/Unicode rendering, Catppuccin Mocha theme, animation loop, screen panning |
| Dungeon generation, dice mechanics, room logic | Data | 2D6 dice implementation, room generation, corridor logic, exit placement, dungeon boundary rules |
| Explorer AI, pathfinding, behavior | Mouth | Explorer movement, pathfinding algorithm, room discovery logic, wander behavior |
| Tests, validation, edge cases | Brand | Generation validation, boundary tests, dice distribution tests, pathfinding tests |
| Memory, decisions, logs | Scribe | Session logging, decision merging, cross-agent updates |
| Work queue, backlog monitoring | Ralph | Issue tracking, PR monitoring, work pipeline |

## Routing Notes

- **Multi-domain requests** (e.g., "build the dungeon explorer"): Spawn Mikey + relevant implementers in parallel
- **2D6 rules questions**: Route to Data (game logic expert) or Mikey (if architectural decision needed)
- **Visual/rendering questions**: Route to Chunk
- **AI behavior questions**: Route to Mouth
- **"Team" requests**: Spawn all relevant agents in parallel based on the task scope

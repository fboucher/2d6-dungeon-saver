# Work Routing

How to decide who handles what.

## Routing Table

| Work Type | Route To | Examples |
|-----------|----------|---------|
| Architecture, design, scoping | Jareth | "how should rooms connect?", "what's the fix approach?" |
| Code review, quality gates | Jareth | PR review, rejections, approvals |
| Game logic, movement, room transitions | Sarah | Explorer positioning, door entry, wall adjacency bugs |
| Bug fixes in Core/ and Models/ | Sarah | ExplorerAI, Pathfinder, GameLoop, Room, Exit, Dungeon |
| Terminal rendering, wall drawing | Hoggle | Renderer.cs, ColorTheme.cs, visual correctness |
| Adjoining wall visual bugs | Hoggle + Sarah | Data (Sarah) + rendering (Hoggle) — fan-out both |
| Test writing, edge cases, QA | Sir Didymus | New tests, regression coverage, validating fixes |
| Session logging | Scribe | Automatic — never needs routing |
| Work queue, backlog monitoring | Ralph | "Ralph, go", "what's on the board?" |

## Rules

1. **Eager by default** — spawn all agents who could usefully start work, including anticipatory downstream work.
2. **Scribe always runs** after substantial work, always as `mode: "background"`. Never blocks.
3. **Quick facts → coordinator answers directly.** Don't spawn an agent for "what port does the server run on?"
4. **Fan-out for cross-cutting bugs** — wall adjacency touches both rendering (Hoggle) and data (Sarah). Spawn both.
5. **"Team, ..." → fan-out.** Spawn all relevant agents in parallel as `mode: "background"`.
6. **Sir Didymus writes tests in parallel** when Sarah is fixing bugs — don't wait for the fix to be done.

# SKILL: A* Pathfinding with Pre-computed Walkable Grid

## Metadata
- **Confidence:** medium
- **Source:** earned
- **Language:** Rust
- **Domain:** Game development, pathfinding, grid-based navigation

## Pattern

When implementing A* pathfinding for grid-based movement (dungeons, tilemaps, etc.), pre-compute a HashSet of walkable coordinates from the game world at initialization. This enables O(1) neighbor lookups during A* search instead of re-querying the game world structure on every node expansion.

## Implementation

```rust
use pathfinding::prelude::astar;
use std::collections::HashSet;

pub struct Pathfinder {
    walkable_tiles: HashSet<(u32, u32)>,
}

impl Pathfinder {
    pub fn new(rooms: &[Room]) -> Self {
        let mut walkable_tiles = HashSet::new();
        
        // Pre-compute all walkable positions
        for room in rooms {
            for y in room.y..room.y + room.height {
                for x in room.x..room.x + room.width {
                    walkable_tiles.insert((x, y));
                }
            }
        }
        
        Self { walkable_tiles }
    }
    
    pub fn find_path(&self, start: (u32, u32), goal: (u32, u32)) -> Option<Vec<(u32, u32)>> {
        astar(
            &start,
            |&(x, y)| self.successors(x, y),
            |&(x, y)| self.heuristic((x, y), goal),
            |&pos| pos == goal,
        ).map(|(path, _cost)| path)
    }
    
    fn successors(&self, x: u32, y: u32) -> Vec<((u32, u32), u32)> {
        let mut neighbors = Vec::new();
        
        // 4-directional movement
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            
            if new_x >= 0 && new_y >= 0 {
                let new_pos = (new_x as u32, new_y as u32);
                if self.walkable_tiles.contains(&new_pos) {
                    neighbors.push((new_pos, 1)); // Cost = 1
                }
            }
        }
        
        neighbors
    }
    
    fn heuristic(&self, pos: (u32, u32), goal: (u32, u32)) -> u32 {
        // Manhattan distance for 4-directional movement
        let dx = if pos.0 > goal.0 { pos.0 - goal.0 } else { goal.0 - pos.0 };
        let dy = if pos.1 > goal.1 { pos.1 - goal.1 } else { goal.1 - pos.1 };
        dx + dy
    }
}
```

## Benefits

1. **Performance:** O(1) walkability checks instead of O(n) room iteration per neighbor lookup
2. **Decoupling:** Pathfinder doesn't need complex game world queries during search
3. **Cache-Friendly:** HashSet lookup is fast for small-to-medium grids (<10k tiles)
4. **Immutable World:** Works well when world geometry is static (most roguelikes, strategy games)

## When to Use

- Grid-based pathfinding with static or rarely-changing world geometry
- Game worlds with <100k walkable tiles (HashSet memory overhead acceptable)
- When neighbor lookups during A* dominate performance profile
- Tile-based games, roguelikes, dungeon crawlers, tactical RPGs

## When NOT to Use

- Extremely large worlds (millions of tiles) — memory overhead becomes prohibitive
- Highly dynamic worlds where walkability changes every frame (real-time destruction)
- Sparse grids where most of the coordinate space is empty (use spatial partitioning instead)

## Trade-offs

- **Memory:** Stores every walkable coordinate explicitly (4-8 bytes per tile)
- **Initialization Cost:** Must traverse entire world once at startup
- **Dynamic Changes:** Requires manual HashSet updates if world geometry changes

## Related Patterns

- For dynamic worlds: Implement on-the-fly walkability checks with caching
- For huge worlds: Use chunk-based pathfinding or hierarchical A*
- For 8-directional movement: Adjust successors to include diagonals (different heuristic)

## Alternatives

1. **Direct World Queries:** Query game world on every neighbor check (simpler, slower)
2. **2D Bool Array:** Pre-allocate walkability grid (faster lookup, more memory for sparse grids)
3. **Spatial Partitioning:** Quadtree/grid-based room lookup (better for dynamic worlds)

---
name: "testing-emergent-behavior"
description: "How to test systems with emergent behavior where same inputs produce different outputs based on execution path"
domain: "testing"
confidence: "low"
source: "earned"
---

## Context

When testing systems where behavior emerges from user choices or execution paths (not just inputs), traditional deterministic testing breaks down. Examples include:
- Progressive generation (content created on-demand during exploration)
- AI/ML systems (learning changes behavior)
- User-driven simulations (path-dependent outcomes)
- Dynamic world building (events trigger cascading changes)

**Key characteristic:** Same seed/inputs → different outcomes based on choices made during execution.

## Problem

Traditional approach:
```rust
// ❌ Doesn't work for emergent systems
#[test]
fn test_seed_determinism() {
    let result1 = generate(seed);
    let result2 = generate(seed);
    assert_eq!(result1, result2); // FAILS - results differ!
}
```

Why it fails: If generation depends on explorer's path choices, same seed produces different dungeons.

## Patterns

### 1. Test Constraints, Not Exact Structure

Instead of testing `output == expected`, test invariants:

```rust
#[test]
fn test_progressive_generation_constraints() {
    let mut generator = Generator::new(seed);
    let mut world = vec![generator.generate_entrance()];
    
    // Simulate exploration
    while world.len() < MAX_SIZE {
        if let Some(unexplored) = find_unexplored(&world) {
            world.push(generator.add_entity(unexplored));
        } else {
            break;
        }
    }
    
    // Validate constraints, not exact structure
    assert!(world.len() >= 15 && world.len() <= 25);
    assert_no_orphans(&world);
    assert_all_connected(&world);
}
```

### 2. Test Initial State Consistency

Even with emergent behavior, initial state should be deterministic:

```rust
#[test]
fn test_entrance_deterministic() {
    let mut gen1 = Generator::new(seed);
    let mut gen2 = Generator::new(seed);
    
    let entrance1 = gen1.generate_entrance();
    let entrance2 = gen2.generate_entrance();
    
    // Entrance should be identical (RNG state same)
    assert_eq!(entrance1.width, entrance2.width);
    assert_eq!(entrance1.exits, entrance2.exits);
}
```

### 3. Test Path Divergence

Validate that different paths produce different results (emergent behavior works):

```rust
#[test]
fn test_different_paths_different_results() {
    let seed = 999;
    
    // Path A: Go north first
    let mut gen_a = Generator::new(seed);
    let entrance_a = gen_a.generate_entrance();
    let room_a = gen_a.add_room(entrance_a.id, Wall::North);
    
    // Path B: Go south first
    let mut gen_b = Generator::new(seed);
    let entrance_b = gen_b.generate_entrance();
    let room_b = gen_b.add_room(entrance_b.id, Wall::South);
    
    // Entrances identical, but subsequent rooms differ
    assert_eq!(entrance_a.width, entrance_b.width);
    // Rooms CAN differ (emergent behavior)
    // Don't assert room_a == room_b
}
```

### 4. Simulate Exhaustive Exploration

Test worst-case scenarios (explore everything):

```rust
#[test]
fn test_no_infinite_generation() {
    let mut gen = Generator::new(seed);
    let mut world = vec![gen.generate_entrance()];
    
    let mut iterations = 0;
    let max_iterations = 1000; // Safety limit
    
    while iterations < max_iterations {
        if let Some(unexplored) = find_unexplored(&world) {
            world.push(gen.add_entity(unexplored));
            iterations += 1;
        } else {
            break; // No more unexplored - success
        }
    }
    
    assert!(iterations < max_iterations, "Infinite loop detected!");
    assert!(world.len() <= 30, "Generation didn't stop");
}
```

### 5. Test Growth Patterns

Validate incremental growth (1 → 2 → 3 ... → N):

```rust
#[test]
fn test_incremental_growth() {
    let mut gen = Generator::new(seed);
    let mut world = vec![gen.generate_entrance()];
    let mut growth_log = vec![world.len()];
    
    for _ in 0..5 {
        if let Some(unexplored) = find_unexplored(&world) {
            world.push(gen.add_entity(unexplored));
            growth_log.push(world.len());
        }
    }
    
    // Verify sequential growth: [1, 2, 3, 4, 5, 6]
    assert_eq!(growth_log, vec![1, 2, 3, 4, 5, 6]);
}
```

### 6. Test Connectivity Invariants

Ensure no orphaned/disconnected entities:

```rust
#[test]
fn test_no_orphans() {
    let mut gen = Generator::new(seed);
    let mut world = generate_progressive(&mut gen, 10);
    
    // Every entity except root should have a parent
    for entity in world.iter().skip(1) {
        let has_parent = world.iter().any(|parent| {
            parent.connections.contains(&entity.id)
        });
        
        assert!(has_parent, "Entity {} is orphaned!", entity.id);
    }
}
```

### 7. Test State Tracking

Validate unexplored vs explored states:

```rust
#[test]
fn test_exit_tracking() {
    let mut gen = Generator::new(seed);
    let entrance = gen.generate_entrance();
    
    // Initially all unexplored
    for exit in &entrance.exits {
        assert!(exit.connected_id.is_none());
    }
    
    // After exploration, should be marked
    let new_entity = gen.add_entity(entrance.id, exit.position);
    // Parent's exit should now point to new entity
    // (Test implementation verifies this tracking)
}
```

## Testing Philosophy Shift

### Deterministic Systems
- **Goal:** Same input → same output
- **Strategy:** Test exact equality (`assert_eq!`)
- **Focus:** Reproducibility
- **Example:** Parser, compiler, hash function

### Emergent Systems
- **Goal:** Same input → valid (but different) outputs
- **Strategy:** Test constraints and invariants
- **Focus:** Correctness bounds
- **Example:** Progressive generation, AI pathfinding, simulations

## Examples

### Rust (Game Generation)
```rust
// Progressive dungeon generation tests
#[test]
fn test_dungeon_growth_limit() {
    let mut gen = DungeonGenerator::new(seed);
    let mut dungeon = vec![gen.generate_entrance()];
    
    while let Some((room_id, wall)) = find_unexplored_exit(&dungeon) {
        if dungeon.len() >= 25 { break; }
        let new_room = gen.add_room(room_id, wall);
        update_connections(&mut dungeon, room_id, wall, new_room.id);
        dungeon.push(new_room);
    }
    
    assert!(dungeon.len() >= 15 && dungeon.len() <= 25);
}
```

### Python (Simulation)
```python
def test_simulation_bounds():
    sim = Simulation(seed=42)
    world = [sim.generate_initial_state()]
    
    for _ in range(100):
        event = sim.next_event()
        if event:
            world.append(sim.process(event))
    
    # Test constraints, not exact state
    assert 10 <= len(world) <= 50
    assert all(entity.is_valid() for entity in world)
    assert no_orphans(world)
```

## Anti-Patterns

❌ **Don't**: Assert exact structure equality for emergent systems
✅ **Do**: Assert constraints (size bounds, connectivity, validity)

❌ **Don't**: Use `seed` for reproducibility if behavior is path-dependent
✅ **Do**: Use `seed` for RNG, document that structure is emergent

❌ **Don't**: Test implementation details (order of generation)
✅ **Do**: Test observable properties (reachability, bounds, invariants)

❌ **Don't**: Expect deterministic integration tests
✅ **Do**: Simulate multiple runs, validate constraints hold in all cases

❌ **Don't**: Skip testing because "it's non-deterministic"
✅ **Do**: Test constraints, growth patterns, and invariants

## When to Apply

Use this pattern when:
- Output depends on user choices or execution path
- Same input can produce different valid outputs
- System has growth/evolution mechanics
- Behavior emerges from interactions
- Traditional equality tests fail despite correct behavior

Don't use when:
- Exact reproducibility is required (parsers, compilers)
- Output is fully determined by input
- Deterministic testing already works

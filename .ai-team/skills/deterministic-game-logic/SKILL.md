---
name: "deterministic-game-logic"
description: "Implementing deterministic procedural generation with seed-based RNG for reproducible game content"
domain: "game-development"
confidence: "low"
source: "earned"
---

## Context

When implementing procedural generation for games, simulations, or any system requiring reproducibility, wrap your RNG in a seed-based abstraction layer. This enables deterministic behavior (same seed = same output) which is critical for debugging, testing, multiplayer synchronization, and player experience features (sharing seeds, replaying scenarios).

Applies to:
- Procedural generation systems (dungeons, terrain, quests)
- Roguelikes and games with randomized content
- Simulations requiring reproducible runs
- Systems needing deterministic testing
- Multiplayer games requiring sync'd randomness

## Patterns

### 1. Seed-Based RNG Wrapper
Create a typed wrapper around platform RNG with explicit seeding:

```rust
use rand_chacha::ChaCha8Rng;
use rand::{Rng, SeedableRng};

pub struct SeededRng {
    rng: ChaCha8Rng,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }
    
    pub fn d6(&mut self) -> u32 {
        self.rng.gen_range(1..=6)
    }
    
    pub fn d66(&mut self) -> (u32, u32) {
        (self.d6(), self.d6())
    }
}
```

### 2. Domain-Specific Dice Methods
Don't expose raw RNG — provide typed methods for your domain (d6, d20, roll_damage, etc.):

```rust
// ✅ Good: Clear intent, testable ranges
pub fn d6(&mut self) -> u32 { self.rng.gen_range(1..=6) }
pub fn d66(&mut self) -> (u32, u32) { (self.d6(), self.d6()) }

// ❌ Bad: Exposes internals, harder to test
pub fn rng(&mut self) -> &mut ChaCha8Rng { &mut self.rng }
```

### 3. Generator Owns RNG State
Pass RNG by ownership or mutable reference to generators:

```rust
pub struct DungeonGenerator {
    rng: SeededRng,
}

impl DungeonGenerator {
    pub fn new(seed: u64) -> Self {
        Self { rng: SeededRng::new(seed) }
    }
    
    pub fn generate(&mut self) -> Vec<Room> {
        let (width, height) = self.rng.d66();
        // ... use RNG throughout generation
    }
}
```

### 4. Test Determinism First
Validate seed reproducibility before implementing complex logic:

```rust
#[test]
fn test_deterministic_generation() {
    let mut gen1 = DungeonGenerator::new(42);
    let mut gen2 = DungeonGenerator::new(42);
    
    let result1 = gen1.generate();
    let result2 = gen2.generate();
    
    assert_eq!(result1, result2, "Same seed must produce identical output");
}
```

### 5. Use Cryptographic RNG for Reproducibility
Prefer ChaCha8Rng (or similar CSPRNG) over platform RNG:

```rust
// ✅ Good: Cross-platform deterministic
use rand_chacha::ChaCha8Rng;

// ❌ Bad: Platform-dependent, may vary
use rand::thread_rng;
```

Rationale: CSPRNGs have well-defined algorithms that produce identical sequences across platforms, OS versions, and architectures.

### 6. Expose Seed in API
Let users provide seeds for reproducibility:

```rust
// CLI
fn main() {
    let seed = args.seed.unwrap_or_else(|| SystemTime::now().as_secs());
    let mut gen = DungeonGenerator::new(seed);
    println!("Seed: {}", seed); // Always log it!
}

// Tests
#[test]
fn test_specific_edge_case() {
    let mut gen = DungeonGenerator::new(12345); // Known problematic seed
    // ...
}
```

### 7. Separate RNG Streams (Advanced)
For complex systems, use separate RNG instances per subsystem:

```rust
pub struct GameWorld {
    dungeon_rng: SeededRng,
    loot_rng: SeededRng,
    enemy_rng: SeededRng,
}

impl GameWorld {
    pub fn new(seed: u64) -> Self {
        Self {
            dungeon_rng: SeededRng::new(seed),
            loot_rng: SeededRng::new(seed.wrapping_add(1)),
            enemy_rng: SeededRng::new(seed.wrapping_add(2)),
        }
    }
}
```

This prevents cross-system interference (e.g., adding a loot drop doesn't change enemy spawns).

## Examples

### Rust Implementation
```rust
// rng.rs
use rand_chacha::ChaCha8Rng;
use rand::{Rng, SeedableRng};

pub struct SeededRng {
    rng: ChaCha8Rng,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        Self { rng: ChaCha8Rng::seed_from_u64(seed) }
    }
    
    pub fn d6(&mut self) -> u32 { self.rng.gen_range(1..=6) }
    pub fn d66(&mut self) -> (u32, u32) { (self.d6(), self.d6()) }
}

// generator.rs
pub struct DungeonGenerator {
    rng: SeededRng,
}

impl DungeonGenerator {
    pub fn new(seed: u64) -> Self {
        Self { rng: SeededRng::new(seed) }
    }
    
    pub fn generate(&mut self) -> Dungeon {
        let (width, height) = self.rng.d66();
        // ... procedural generation using self.rng
    }
}
```

### Python Implementation
```python
import random

class SeededRng:
    def __init__(self, seed: int):
        self.rng = random.Random(seed)
    
    def d6(self) -> int:
        return self.rng.randint(1, 6)
    
    def d66(self) -> tuple[int, int]:
        return (self.d6(), self.d6())

class DungeonGenerator:
    def __init__(self, seed: int):
        self.rng = SeededRng(seed)
    
    def generate(self) -> list:
        width, height = self.rng.d66()
        # ... procedural generation
```

## Anti-Patterns

❌ **Don't**: Use system time for critical generation without logging seed
```rust
let seed = SystemTime::now().as_secs();
let dungeon = generate(seed); // Irreproducible bugs!
```

✅ **Do**: Always log the seed or provide explicit seed API
```rust
let seed = args.seed.unwrap_or_else(|| {
    let s = SystemTime::now().as_secs();
    println!("Using seed: {}", s);
    s
});
```

❌ **Don't**: Mix deterministic and non-deterministic RNG
```rust
let mut gen = SeededRng::new(42);
let x = gen.d6();
let y = rand::random::<u32>() % 6; // BREAKS DETERMINISM!
```

✅ **Do**: Use only the seeded RNG instance
```rust
let mut gen = SeededRng::new(42);
let x = gen.d6();
let y = gen.d6(); // Both from same stream
```

❌ **Don't**: Mutate state before all RNG calls
```rust
// Order matters! This changes the sequence:
if condition {
    let _ = self.rng.d6(); // Consumes RNG state
}
let room = self.generate_room(); // Different output now
```

✅ **Do**: Generate all random values first, then apply logic
```rust
let roll = self.rng.d6();
if condition {
    apply_modifier(roll);
}
let room = self.generate_room(); // Consistent
```

❌ **Don't**: Forget to test range boundaries
```rust
pub fn d6(&mut self) -> u32 {
    self.rng.gen_range(1..6) // BUG: Only 1-5!
}
```

✅ **Do**: Write range validation tests
```rust
#[test]
fn test_d6_range() {
    let mut rng = SeededRng::new(12345);
    for _ in 0..1000 {
        let roll = rng.d6();
        assert!(roll >= 1 && roll <= 6);
    }
}
```

## Benefits

- **Debugging**: Reproduce bugs by seed, no "worked on my machine"
- **Testing**: Exhaustively test edge cases with known problematic seeds
- **Player Experience**: Share seeds, challenge runs, speedrun categories
- **Multiplayer**: Sync'd generation without network bandwidth
- **Regression**: Detect changes in generation algorithm via seed-based snapshots

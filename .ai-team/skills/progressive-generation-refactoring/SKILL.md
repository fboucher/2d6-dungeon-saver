---
name: "progressive-generation-refactoring"
description: "Refactoring batch generation systems to support on-demand/progressive creation patterns"
domain: "architecture"
confidence: "low"
source: "earned"
---

## Context

When refactoring a system that generates all content upfront (batch generation) to support on-demand creation (progressive generation), maintain backward compatibility while introducing the new API. This is common when:

- Moving from "generate all dungeons rooms at startup" to "create rooms as player explores"
- Changing from "load all data" to "lazy loading"
- Transitioning from "batch processing" to "streaming"
- Implementing "just-in-time" patterns over "eager initialization"

## Patterns

### 1. Deprecate, Don't Delete
Keep the old API marked as deprecated during transition:

```rust
#[deprecated(note = "Use generate_entrance() for progressive generation")]
pub fn generate(&mut self) -> Vec<Room> {
    // Old implementation stays working
}

pub fn generate_entrance(&mut self) -> Room {
    // New progressive API
}

pub fn add_item(&mut self, parent_id: usize, context: Context) -> Item {
    // On-demand creation method
}
```

**Why**: Existing tests, examples, and integrations continue working while you migrate incrementally.

### 2. Extract Shared Logic Into Helpers
Both batch and progressive paths should use the same core logic:

```rust
// Shared helper - used by both APIs
fn generate_connected_room(&mut self, parent_id: usize, wall: Wall) -> Room {
    let (width, height) = self.rng.d66();
    // ... 2D6 rules implementation
}

// Old API - uses helper in loop
pub fn generate(&mut self) -> Vec<Room> {
    for _ in 0..20 {
        let room = self.generate_connected_room(parent_id, wall);
        rooms.push(room);
    }
}

// New API - exposes helper directly
pub fn add_room(&mut self, parent_id: usize, wall: Wall) -> Room {
    self.generate_connected_room(parent_id, wall)
}
```

**Why**: Reduces duplication, ensures consistent behavior, simplifies testing.

### 3. Sentinel Values Over Option Types
For on-demand APIs that have limits, consider sentinel values instead of Option:

```rust
// ✅ Good: Sentinel pattern (simpler caller code)
pub fn add_room(&mut self, parent_id: usize, wall: Wall) -> Room {
    if self.rooms.len() >= LIMIT {
        return Room::dummy(); // id = usize::MAX or similar
    }
    self.generate_room(parent_id, wall)
}

// Usage
let room = gen.add_room(parent_id, wall);
if room.id != usize::MAX {
    dungeon.push(room);
}

// ❌ Alternative: Option (more type-safe, more boilerplate)
pub fn add_room(&mut self, parent_id: usize, wall: Wall) -> Option<Room> {
    if self.rooms.len() >= LIMIT { return None; }
    Some(self.generate_room(parent_id, wall))
}

// Usage
if let Some(room) = gen.add_room(parent_id, wall) {
    dungeon.push(room);
}
```

**Trade-off**: Sentinel is less type-safe but reduces unwrapping in tight loops. Use Option if compiler enforcement is critical.

### 4. Test Both Paths Independently
Write separate test suites for batch vs progressive:

```rust
// Progressive API tests
#[test]
fn test_generate_entrance_only() {
    let mut gen = Generator::new(42);
    let entrance = gen.generate_entrance();
    assert_eq!(entrance.id, 0);
}

#[test]
fn test_add_room_creates_connection() {
    let mut gen = Generator::new(42);
    gen.generate_entrance();
    let room = gen.add_room(0, Wall::North);
    assert_eq!(room.parent_id, Some(0));
}

// Legacy batch API tests
#[test]
#[allow(deprecated)]
fn test_batch_generation() {
    let mut gen = Generator::new(42);
    let rooms = gen.generate();
    assert_eq!(rooms.len(), 20);
}
```

**Why**: Ensures both APIs work correctly during transition period.

### 5. Keep Internal State Consistent
Progressive generation requires tracking state across calls:

```rust
pub struct Generator {
    rng: SeededRng,
    rooms: Vec<Room>,    // Internal state must persist
    next_id: usize,      // IDs must be sequential
}

impl Generator {
    pub fn generate_entrance(&mut self) -> Room {
        self.rooms.clear();  // Reset state
        self.next_id = 0;
        
        let entrance = self.create_entrance();
        self.rooms.push(entrance.clone());
        entrance
    }
    
    pub fn add_room(&mut self, parent_id: usize, wall: Wall) -> Room {
        // Uses self.next_id, increments it
        let room = self.create_room(parent_id, wall);
        self.rooms.push(room.clone());
        self.next_id += 1;
        room
    }
}
```

**Why**: Caller expects each `add_room()` call to increment IDs, maintain connections, etc.

### 6. Document the Migration Path
Add clear documentation showing how to migrate:

```rust
/// Generate entrance room only (progressive generation API).
///
/// # Example
/// ```
/// let mut gen = DungeonGenerator::new(42);
/// let entrance = gen.generate_entrance();
/// 
/// // Later, when player reaches exit:
/// let room1 = gen.add_room(entrance.id, Wall::North);
/// ```
///
/// # Migration from batch API
/// ```
/// // Old (deprecated):
/// let rooms = generator.generate();
///
/// // New (progressive):
/// let entrance = generator.generate_entrance();
/// let mut dungeon = vec![entrance];
/// // ... call add_room() as needed
/// ```
pub fn generate_entrance(&mut self) -> Room { ... }
```

## Examples

### Before Refactoring (Batch)
```rust
pub struct Generator {
    rng: SeededRng,
}

impl Generator {
    pub fn generate(&mut self) -> Vec<Room> {
        let mut rooms = Vec::new();
        let entrance = self.create_entrance();
        rooms.push(entrance);
        
        for _ in 0..19 {
            let parent = &rooms[rooms.len() - 1];
            let room = self.create_connected_room(parent.id);
            rooms.push(room);
        }
        
        rooms
    }
}

// Usage
let rooms = generator.generate(); // All 20 rooms created upfront
```

### After Refactoring (Progressive)
```rust
pub struct Generator {
    rng: SeededRng,
    rooms: Vec<Room>,  // Internal state
    next_id: usize,
}

impl Generator {
    pub fn generate_entrance(&mut self) -> Room {
        self.rooms.clear();
        self.next_id = 0;
        
        let entrance = self.create_entrance();
        self.rooms.push(entrance.clone());
        entrance
    }
    
    pub fn add_room(&mut self, parent_id: usize, wall: Wall) -> Room {
        if self.rooms.len() >= 20 {
            return Room::dummy(); // Sentinel
        }
        
        let room = self.create_connected_room(parent_id, wall);
        self.rooms.push(room.clone());
        room
    }
    
    #[deprecated(note = "Use generate_entrance() + add_room()")]
    pub fn generate(&mut self) -> Vec<Room> {
        let entrance = self.generate_entrance();
        let mut result = vec![entrance];
        
        for _ in 0..19 {
            if let Some(room) = result.last() {
                let new_room = self.add_room(room.id, Wall::North);
                if new_room.id != usize::MAX {
                    result.push(new_room);
                }
            }
        }
        
        result
    }
}

// Usage (progressive)
let entrance = generator.generate_entrance();
let mut dungeon = vec![entrance];

// Later, on-demand:
let room = generator.add_room(dungeon[0].id, Wall::North);
if room.id != usize::MAX {
    dungeon.push(room);
}
```

## Anti-Patterns

❌ **Don't**: Break existing callers immediately
```rust
// Removing the old API breaks all existing code:
// pub fn generate(&mut self) -> Vec<Room> { ... } // DELETED
```

✅ **Do**: Deprecate with helpful migration message
```rust
#[deprecated(note = "Use generate_entrance() for progressive generation")]
pub fn generate(&mut self) -> Vec<Room> { ... }
```

❌ **Don't**: Duplicate core logic in both paths
```rust
pub fn generate() -> Vec<Room> {
    // 200 lines of room generation logic
}

pub fn add_room() -> Room {
    // SAME 200 lines duplicated
}
```

✅ **Do**: Extract shared helpers
```rust
fn generate_room_internal(&mut self, ...) -> Room {
    // Shared logic
}

pub fn generate() -> Vec<Room> {
    rooms.push(self.generate_room_internal(...));
}

pub fn add_room() -> Room {
    self.generate_room_internal(...)
}
```

❌ **Don't**: Forget to test state persistence
```rust
// Test forgets to verify that multiple add_room() calls work correctly
#[test]
fn test_add_room() {
    let room1 = gen.add_room(...);
    assert_eq!(room1.id, 1); // What about room2, room3?
}
```

✅ **Do**: Test sequential calls
```rust
#[test]
fn test_add_room_sequence() {
    let entrance = gen.generate_entrance();
    let room1 = gen.add_room(entrance.id, Wall::North);
    let room2 = gen.add_room(entrance.id, Wall::South);
    
    assert_eq!(room1.id, 1);
    assert_eq!(room2.id, 2);
    assert_eq!(gen.rooms.len(), 3); // entrance + 2 rooms
}
```

## Benefits

- **Incremental Migration**: Old code keeps working while you adopt new pattern
- **Reduced Memory**: Create content only when needed (lazy evaluation)
- **Better UX**: Faster startup (entrance only), progressive loading
- **Testability**: Test batch and progressive independently
- **Flexibility**: Caller controls when/how many items to create

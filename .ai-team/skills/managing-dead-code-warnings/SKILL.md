---
name: "managing-dead-code-warnings"
description: "Handling compiler dead code warnings in evolving codebases with deprecated APIs and future-reserved functionality"
domain: "code-quality"
confidence: "low"
source: "earned"
---

## Context

When refactoring APIs or implementing features incrementally, you often have code that's temporarily unused but serves a legitimate purpose (backward compatibility, future functionality, tested APIs). Blindly deleting "dead code" can destroy working tests, remove intentional design elements, or eliminate useful utilities.

Applies to:
- API migrations where old methods must coexist with new ones
- Progressive feature implementation (theme colors defined before rendering)
- Public API methods with test coverage but no production usage yet
- Test utility methods that only exist for validation

## Patterns

### 1. Use `#[allow(dead_code)]` for Intentional Reservations
When code has a valid reason to exist despite compiler warnings:

```rust
/// Check if a position is within the viewport
/// Reserved for future visibility culling optimization
#[allow(dead_code)]
pub fn is_visible(&self, pos: (u32, u32)) -> bool {
    // Implementation...
}
```

**When to use:**
- Public API methods with test coverage but no callers yet
- Utility methods used only in tests
- Struct fields that are part of a coherent design (theming, configuration)
- Deprecated methods supporting backward-compatible tests

### 2. Mark Deprecated Methods Clearly
When phasing out an old API but keeping tests:

```rust
/// Legacy batch generation - deprecated in favor of progressive generation
#[deprecated(note = "Use generate_entrance() for progressive generation")]
#[allow(dead_code)]
pub fn generate(&mut self) -> Vec<Room> {
    // Old implementation...
}
```

**Benefits:**
- Tests continue to validate core logic (2D6 rules, exit placement)
- Deprecation warning guides new code to use modern API
- `#[allow(dead_code)]` suppresses compiler noise for intentional backward compatibility

### 3. Document Rationale in Comments
Always explain WHY code is marked `#[allow(dead_code)]`:

```rust
// ✅ Good: Explains purpose
/// Corridor color - reserved for future corridor rendering
#[allow(dead_code)]
pub corridor: Color,

// ❌ Bad: No context
#[allow(dead_code)]
pub corridor: Color,
```

### 4. Delete Code That Truly Has No Purpose
If code has no tests, no callers, no deprecation marker, and no design purpose → delete it:

```rust
// ❌ Delete this:
pub fn unused_helper(&self) -> u32 {
    // No tests, no callers, no documentation
    42
}
```

### 5. Keep Helper Structs for Deprecated Methods
If a deprecated method needs supporting types, mark them too:

```rust
/// Helper for deprecated batch generation - tracks unexplored exits
#[derive(Debug)]
#[allow(dead_code)]
struct AvailableExit {
    room_id: usize,
    wall: Wall,
    exit_index: usize,
}

#[deprecated(note = "Use generate_entrance() for progressive generation")]
#[allow(dead_code)]
pub fn generate(&mut self) -> Vec<Room> {
    let exits: Vec<AvailableExit> = self.collect_exits();
    // ...
}
```

## Examples

### Rust: Preserving Test Utilities
```rust
impl Room {
    /// Calculate room area - used by tests and generation logic
    #[allow(dead_code)]
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Check if room is a corridor (1-width dimension)
    #[allow(dead_code)]
    pub fn is_corridor(&self) -> bool {
        self.width == 1 || self.height == 1
    }
}

// In tests:
#[test]
fn test_small_room_detection() {
    let room = create_room(2, 3);
    assert_eq!(room.area(), 6);
    assert!(!room.is_corridor());
}
```

### Rust: Theming System with Incomplete Rendering
```rust
pub struct Theme {
    pub wall: Color,
    pub floor: Color,
    /// Corridor color - reserved for future corridor rendering
    #[allow(dead_code)]
    pub corridor: Color,
    pub door: Color,
    pub explorer: Color,
}

// Later when corridor rendering is added:
// 1. Remove #[allow(dead_code)]
// 2. Use theme.corridor in renderer
// 3. No API changes needed — field was always there
```

### C#: Deprecated API Pattern
```csharp
public class DungeonGenerator
{
    [Obsolete("Use GenerateEntrance() for progressive generation")]
    #pragma warning disable IDE0051 // Remove unused private members
    public List<Room> Generate()
    {
        // Old batch generation logic
        // Still used by 56 backward-compatibility tests
    }
    #pragma warning restore IDE0051

    public Room GenerateEntrance()
    {
        // New progressive API
    }
}
```

### TypeScript: Future-Reserved Public API
```typescript
export class Camera {
    /**
     * Check if a position is within the viewport
     * Reserved for future visibility culling optimization
     * @internal - has test coverage but not used in production yet
     */
    // @ts-ignore - unused for now
    isVisible(pos: [number, number]): boolean {
        const [x, y] = pos;
        return x >= this.x && x < this.x + this.width
            && y >= this.y && y < this.y + this.height;
    }
}

// In tests:
test('isVisible detects out-of-bounds', () => {
    const camera = new Camera(80, 24);
    expect(camera.isVisible([100, 100])).toBe(false);
});
```

## Anti-Patterns

❌ **Don't**: Suppress warnings without justification
```rust
#[allow(dead_code)] // Why? No one knows.
pub fn mystery_function() {}
```

✅ **Do**: Document the reason
```rust
/// Reserved for Phase 4 pathfinding optimizations
#[allow(dead_code)]
pub fn mystery_function() {}
```

❌ **Don't**: Delete tested public API methods just because production code doesn't call them
```rust
// ❌ Bad: This method has 5 tests validating boundary conditions!
// pub fn is_visible(&self, pos: (u32, u32)) -> bool { ... }
```

✅ **Do**: Preserve with `#[allow(dead_code)]` and note it's tested
```rust
/// Check if position is in viewport - reserved for visibility culling
#[allow(dead_code)]
pub fn is_visible(&self, pos: (u32, u32)) -> bool { ... }
```

❌ **Don't**: Keep deprecated code without deprecation markers
```rust
// ❌ Bad: No signal to developers that this is old
#[allow(dead_code)]
pub fn generate(&mut self) -> Vec<Room> { ... }
```

✅ **Do**: Mark as deprecated if keeping for backward compatibility
```rust
#[deprecated(note = "Use generate_entrance()")]
#[allow(dead_code)]
pub fn generate(&mut self) -> Vec<Room> { ... }
```

❌ **Don't**: Keep private helper functions for deleted features
```rust
// ❌ Bad: Parent function deleted, helper orphaned
#[allow(dead_code)]
fn collect_exits(&self) -> Vec<Exit> { ... }
```

✅ **Do**: Delete helpers when their only caller is removed
```rust
// Deleted collect_exits() when the only function using it was removed
```

## Decision Framework

When facing a dead code warning, ask:

1. **Is it tested?** → Keep with `#[allow(dead_code)]` + comment
2. **Is it public API?** → Keep with `#[allow(dead_code)]` + comment
3. **Is it deprecated but supporting tests?** → Keep with `#[deprecated]` + `#[allow(dead_code)]`
4. **Is it part of incomplete feature?** (e.g., theme field not rendered yet) → Keep with `#[allow(dead_code)]` + comment
5. **None of the above?** → Delete it

## Benefits

- **Preserve test coverage** without spurious warnings
- **Signal intent** to future developers (this is here for a reason)
- **Smooth migrations** from old APIs to new ones
- **Clean builds** (zero warnings) without losing intentional code
- **Iterative development** (define theme fields before using them)

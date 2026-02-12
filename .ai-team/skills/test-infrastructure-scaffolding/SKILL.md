---
name: "test-infrastructure-scaffolding"
description: "How to set up comprehensive test infrastructure before implementation, including fixtures, assertions, and property-based tests"
domain: "testing"
confidence: "low"
source: "earned"
---

## Context

When building a feature with well-defined rules (like game mechanics, protocol implementations, or spec compliance), set up test infrastructure BEFORE full implementation. This allows parallel work: architects build skeletons, developers implement features with tests alongside, and testers validate incrementally.

Applies to:
- Rules-based systems (game mechanics, parsers, protocols)
- Projects with 50+ test cases planned
- Teams working in parallel on different phases
- Systems requiring deterministic behavior (seeded RNG, reproducible outputs)

## Patterns

### 1. Fixture-Driven Testing
Create named constants for known edge cases:
```rust
pub mod fixtures {
    pub const SEED_MIN_ENTRANCE: u64 = 12345;
    pub const SEED_EDGE_CASE: u64 = 67890;
}
```

### 2. Assertion Helpers
Extract rule validation into reusable functions:
```rust
pub mod assertions {
    pub fn assert_entrance_room(room: &Room) {
        let area = room.width * room.height;
        assert!(area >= 6 && area <= 12);
        assert_eq!(room.exits.len(), 3);
    }
}
```

### 3. Query Helpers
Provide common predicates and filters:
```rust
pub mod helpers {
    pub fn is_corridor(room: &Room) -> bool {
        room.width == 1 || room.height == 1
    }
    
    pub fn count_rooms<F>(rooms: &[Room], predicate: F) -> usize
    where F: Fn(&Room) -> bool {
        rooms.iter().filter(predicate).count()
    }
}
```

### 4. Property-Based Tests
Use tools like proptest/quickcheck for invariants:
```rust
proptest! {
    fn prop_first_room_is_entrance(seed in any::<u64>()) {
        let dungeon = generate(seed);
        assert_entrance_room(&dungeon[0]);
    }
}
```

### 5. Ignored Tests as Roadmap
Mark unimplemented tests with `#[ignore]`:
```rust
#[test]
#[ignore] // Remove when generate() is complete
fn test_corridor_detection() { ... }
```

### 6. Test Conventions Documentation
Create TEST_CONVENTIONS.md with:
- File structure explanation
- Fixture catalog
- Assertion function reference
- Usage examples
- Naming conventions

## Examples

Rust test infrastructure structure:
```
tests/
  common/
    mod.rs           # Fixtures, assertions, helpers
  feature_tests.rs   # Integration tests
  proptest.rs        # Property-based tests
  TEST_CONVENTIONS.md
```

Python equivalent:
```
tests/
  conftest.py        # Fixtures and utilities
  test_feature.py    # Integration tests
  test_properties.py # Hypothesis tests
  README.md          # Conventions
```

## Anti-Patterns

❌ **Don't**: Write 50+ tests in one file without helpers — leads to duplication
✅ **Do**: Extract common assertions and queries into shared module

❌ **Don't**: Use magic numbers/strings for test data — hard to understand intent
✅ **Do**: Use named fixtures (SEED_DOUBLE_SIX, SEED_MIN_ENTRANCE)

❌ **Don't**: Skip ignored tests in CI — they're invisible and forgotten
✅ **Do**: Track ignored test count, run `--ignored` periodically

❌ **Don't**: Mix unit and integration tests without organization
✅ **Do**: Separate by directory/module, document structure

❌ **Don't**: Hardcode expected values without context
✅ **Do**: Document why (comments, named constants)

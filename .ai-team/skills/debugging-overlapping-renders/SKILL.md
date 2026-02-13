---
name: "debugging-overlapping-renders"
description: "Diagnose and fix rendering bugs caused by coordinate/positioning errors"
domain: "graphics, rendering, debugging, tui"
confidence: "medium"
source: "earned"
---

## Context

When multiple objects render at the same coordinates, the display appears fragmented, with overlapping elements creating visual artifacts. This is common in tile-based games, TUI applications, and canvas rendering systems when positioning logic is incomplete or incorrect.

## Symptoms

- Walls appear misaligned or fragmented
- Objects render on top of each other creating visual artifacts
- Screen shows disconnected sections that should be separate
- Specific characters (like doors, sprites) appear in unexpected clusters
- Layout looks "compressed" or all elements seem to be in one area

## Diagnostic Steps

### 1. Check Coordinate Initialization

Look for placeholder values in object creation:

```rust
// RED FLAG: Placeholder coordinates
let room = Room {
    x: 0,  // Will be positioned later
    y: 0,
    // ...
};
```

### 2. Verify Positioning Logic

Search for where coordinates are actually set:

```bash
# Find where position fields are assigned
grep -r "\.x =" src/
grep -r "\.y =" src/

# Look for layout/positioning functions
grep -r "position\|layout\|place" src/
```

### 3. Sample Coordinate Values

Add debug output to check if objects have distinct positions:

```rust
for (i, obj) in objects.iter().enumerate() {
    println!("Object {}: pos=({},{})", i, obj.x, obj.y);
}
```

Expected: Different coordinates per object
Actual (if bug): All objects at (0,0) or same position

### 4. Trace Rendering Code

Check if renderer assumes positioned objects:

```rust
fn render_object(&self, obj: &Object) {
    let screen_x = obj.x - camera.x;  // Assumes obj.x is valid
    let screen_y = obj.y - camera.y;
    // ...
}
```

## Common Root Causes

1. **Placeholder Coordinates Never Updated**: Generation creates objects with default (0,0), layout step missing
2. **Layout Algorithm Not Called**: Positioning function exists but isn't invoked in generation pipeline
3. **Coordinate System Mismatch**: Different subsystems use incompatible coordinate spaces
4. **Uninitialized Memory**: Position fields not explicitly set during construction

## Fix Patterns

### Pattern 1: Post-Generation Layout

Separate generation from positioning:

```rust
pub fn generate(&mut self) -> Vec<Room> {
    // Generate rooms with properties
    self.generate_rooms();
    
    // Position rooms spatially
    RoomLayouter::layout(&mut self.rooms);
    
    self.rooms.clone()
}
```

### Pattern 2: Layout During Generation

Position each object immediately upon creation:

```rust
fn generate_room(&mut self, parent: Option<&Room>) -> Room {
    let (width, height) = self.roll_dimensions();
    let (x, y) = self.calculate_position(parent);  // Position NOW
    
    Room { x, y, width, height, /* ... */ }
}
```

### Pattern 3: Lazy Positioning

Validate and fix coordinates before rendering:

```rust
fn render(&mut self) {
    if !self.positioned {
        self.layout_objects();
        self.positioned = true;
    }
    // Now safe to render
}
```

## Testing Strategy

1. **Unit Test Coordinates**: Assert objects have non-zero, distinct positions
2. **Visual Verification**: Run with known seed, inspect coordinates in debugger
3. **Rendering Test**: Check that objects appear in different screen regions

```rust
#[test]
fn test_objects_have_distinct_positions() {
    let objects = generate_objects();
    let mut positions = HashSet::new();
    
    for obj in &objects {
        let pos = (obj.x, obj.y);
        assert!(positions.insert(pos), 
                "Duplicate position: {:?}", pos);
    }
}
```

## Prevention

- Always initialize coordinates explicitly (avoid defaults)
- Document whether generation or layout is responsible for positioning
- Add assertions that coordinates are set before rendering
- Include position checks in visual integration tests

## Real-World Example

**Symptom**: Dungeon rooms rendered as fragmented, overlapping mess  
**Diagnosis**: All rooms at (0,0) except entrance  
**Root Cause**: Generator left positioning as "TODO", layout step never implemented  
**Fix**: Added `RoomLayouter::layout()` call in generation pipeline  
**Verification**: Rooms now have distinct coordinates, visual rendering correct

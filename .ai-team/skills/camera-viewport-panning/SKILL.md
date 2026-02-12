---
name: "camera-viewport-panning"
description: "Smooth camera panning for 2D games with comfort-zone logic"
domain: "game-dev, rendering, viewport-management"
confidence: "medium"
source: "earned"
---

## Context

In 2D games where the playable area exceeds screen size, the camera must follow the player without causing jitter or disorientation. Hard-centering the camera on the player every frame creates jerky movement. A "comfort zone" approach provides smooth panning that only activates when needed.

## Patterns

### Comfort Zone Panning

Define a middle region where the player can move freely without camera adjustment:

```rust
pub fn update(&mut self, player_pos: (u32, u32)) {
    let (px, py) = player_pos;
    
    // Calculate player position relative to viewport
    let rel_x = px as i32 - self.x as i32;
    let rel_y = py as i32 - self.y as i32;
    
    // Define comfort zone (middle 50% of screen)
    let quarter_width = self.width / 4;
    let three_quarter_width = (self.width * 3) / 4;
    
    // Pan only when player reaches outer quarters
    if rel_x < quarter_width as i32 {
        self.x = px.saturating_sub(self.width / 2);
    } else if rel_x > three_quarter_width as i32 {
        self.x = px.saturating_sub(self.width / 2);
    }
    
    // Same for vertical
    let quarter_height = self.height / 4;
    let three_quarter_height = (self.height * 3) / 4;
    
    if rel_y < quarter_height as i32 {
        self.y = py.saturating_sub(self.height / 2);
    } else if rel_y > three_quarter_height as i32 {
        self.y = py.saturating_sub(self.height / 2);
    }
}
```

### Saturating Math for Origin Handling

Use `saturating_sub()` to prevent underflow when player is near world origin:

```rust
// If player is at (10, 10) and camera width is 80:
// Naive: 10 - 40 = underflow (panic in debug, wrap in release)
// Saturating: 10.saturating_sub(40) = 0 (correct)
self.x = player_x.saturating_sub(self.width / 2);
```

### Initial Centering

Provide a separate method for initial camera setup (no comfort zone checks):

```rust
pub fn center_on(&mut self, pos: (u32, u32)) {
    self.x = pos.0.saturating_sub(self.width / 2);
    self.y = pos.1.saturating_sub(self.height / 2);
}
```

### Visibility Culling

Skip rendering entities outside viewport bounds to improve performance:

```rust
pub fn is_visible(&self, pos: (u32, u32)) -> bool {
    let (x, y) = pos;
    x >= self.x && x < self.x + self.width
        && y >= self.y && y < self.y + self.height
}
```

### Resize Handling

Update camera dimensions on terminal/window resize without breaking viewport:

```rust
pub fn resize(&mut self, new_width: u32, new_height: u32) {
    self.width = new_width;
    self.height = new_height;
    // Optionally: re-center on player to maintain visibility
}
```

## Examples

**Basic Camera Struct:**
```rust
pub struct Camera {
    pub x: u32,      // Top-left X in world space
    pub y: u32,      // Top-left Y in world space
    pub width: u32,  // Viewport width
    pub height: u32, // Viewport height
}
```

**Screen Space Conversion:**
```rust
fn world_to_screen(&self, world_pos: (u32, u32)) -> (i32, i32) {
    let screen_x = world_pos.0 as i32 - self.x as i32;
    let screen_y = world_pos.1 as i32 - self.y as i32;
    (screen_x, screen_y)
}
```

**Culling Before Render:**
```rust
for entity in entities {
    if !camera.is_visible(entity.pos) {
        continue; // Skip rendering
    }
    render_entity(entity, &camera);
}
```

## Anti-Patterns

❌ **Hard-centering every frame:** Causes jitter when player moves slowly
❌ **Using wrapping arithmetic near origin:** Leads to camera teleporting to far coordinates
❌ **Not handling resize events:** Camera dimensions become stale, viewport breaks
❌ **Rendering all entities unconditionally:** Performance degrades with large worlds
❌ **Forgetting to convert world→screen coordinates:** Entities render at wrong positions

## Testing

Verify these behaviors:

1. **Stable middle area:** Small movements within comfort zone don't pan camera
2. **Edge triggering:** Reaching outer quarter triggers pan
3. **Visibility persistence:** Player remains visible after panning
4. **Origin clamping:** Camera at (0, 0) when player near origin
5. **Resize correctness:** Dimensions update without breaking viewport

## Variations

**Smooth interpolation** (for non-tile games):
```rust
// Instead of instant pan, lerp toward target
let target_x = player_x - self.width / 2;
self.x += ((target_x - self.x) as f32 * 0.1) as u32;
```

**Deadzone at screen edges** (prevent edge clipping):
```rust
let deadzone = 5; // Tiles from edge
if rel_x < deadzone { /* pan left */ }
if rel_x > self.width - deadzone { /* pan right */ }
```

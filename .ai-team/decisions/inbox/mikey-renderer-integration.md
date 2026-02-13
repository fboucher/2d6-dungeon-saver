# Renderer Integration Plan — Simple Grid to Main App

**Date:** 2025-02-12  
**Owner:** Mikey  
**Status:** Design Complete

---

## Context

Frank validated all 4 incremental renderer steps. The simple grid approach works perfectly:
- 1:1 coordinate mapping (each dungeon cell = 1 terminal character)
- Catppuccin Mocha colors
- Box-drawing characters for walls (`█`)
- Colored spaces for floor (background color)
- Smooth animation with alternate screen buffer

**Current State:**
- `src/renderer/simple_room.rs`: Working simple grid renderer (uses `#`, `.`, `-`, `|`)
- `examples/pretty_explorer_demo.rs`: Pretty version with colors and box-drawing (`█`, ` `, `─`, `│`)
- `src/renderer/canvas.rs`: Current main app renderer (also uses simple grid, but needs color upgrade)

**Goal:** Integrate the **pretty version** (colors + box-drawing) into main app.

---

## Key Insight

The current `canvas.rs` **already implements the simple grid approach** from validation:
- 1:1 coordinate mapping (lines 73-75, 94)
- Renders rooms with walls/floors (lines 91-112)
- Renders exits as doors (lines 116-142)
- Renders explorer at position (lines 145-164)
- Works with progressive dungeon generation (Vec<Room>)
- Works with camera viewport

**What's Missing:** Box-drawing characters and colored floor spaces (currently uses `#` and `.` chars instead of `█` and colored backgrounds).

---

## Changes Required

### 1. Update `src/renderer/canvas.rs` (15 lines changed)

**Line 88-90** — Replace wall/floor/door character rendering:

**OLD:**
```rust
let ch = if is_wall { '#' } else { '.' };
let style = if is_wall { wall_style } else { floor_style };
```

**NEW:**
```rust
let (ch, style) = if is_wall {
    ('█', wall_style)
} else {
    (' ', Style::default().bg(floor_style.fg.unwrap_or(self.theme.floor)))
};
```

**Line 134-137** — Replace door character rendering:

**OLD:**
```rust
let door_char = match exit.wall {
    Wall::North | Wall::South => '-',
    Wall::East | Wall::West => '|',
};
```

**NEW:**
```rust
let door_char = match exit.wall {
    Wall::North | Wall::South => '─',
    Wall::East | Wall::West => '│',
};
```

**That's it.** The rest of `canvas.rs` already works correctly:
- Takes `&[Room]` from progressive dungeon generation (line 25)
- Takes `explorer_pos: (u32, u32)` from main loop (line 25)
- Works with camera viewport for culling (lines 78-82)
- Already implements 1:1 coordinate mapping (lines 73-75, 94)

---

## How It Works with Progressive Generation

**Main Loop Flow (already working):**
1. `main.rs` starts with entrance room only: `let entrance = generator.generate_entrance()`
2. Explorer updates: `explorer.update(&dungeon, &pathfinder, &mut rng)`
3. Exit detection: `detect_unexplored_exit(explorer.x, explorer.y, &dungeon)` returns `(room_id, wall)`
4. Generate new room: `let new_room = generator.add_room(parent_room_id, exit_wall)`
5. Add to dungeon: `dungeon.push(new_room)`
6. Rebuild pathfinder: `pathfinder = Pathfinder::new(&dungeon)`
7. Render: `canvas.render(frame, &dungeon, explorer.position(), &camera)`

**Renderer receives:**
- `dungeon: &[Room]` — grows from 1 room to ~20 as explorer discovers exits
- `explorer_pos: (u32, u32)` — explorer's current world position
- `camera: &Camera` — viewport follows explorer with comfort zone panning

**Renderer outputs:**
- For each room in `&dungeon`, render walls (`█`) and floors (colored background)
- For each exit, overwrite wall with door (`─` or `│`)
- Render explorer (`@`) at world position

---

## Camera & Viewport (NO CHANGES NEEDED)

The camera already works correctly with the simple grid:
- 1:1 mapping: `screen_x = room.x - camera.x` (line 74)
- Comfort zone panning: explorer reaches last quarter → camera shifts (camera.rs:24-52)
- Resize handling: `camera.resize(width, height)` (main.rs:159)

**Why it works:**
- Dungeon coordinates = screen coordinates (no conversion)
- Camera just offsets: `room.x - camera.x` for viewport culling
- Explorer stays visible via comfort zone logic (already tested in Phase 3)

---

## Explorer Rendering (NO CHANGES NEEDED)

Explorer is already rendered correctly:
- Single `@` character at `(explorer_pos.0, explorer_pos.1)` (canvas.rs:160)
- Converted to screen coords: `screen_x = explorer_pos.0 - camera.x` (canvas.rs:150)
- Viewport check: only render if in screen bounds (canvas.rs:154-156)
- Color: `theme.explorer` (Green in Catppuccin Mocha)

---

## What MultiRoomRenderer Does (NOT NEEDED)

`simple_room.rs` has `MultiRoomRenderer` for standalone demos:
- Creates a 2D grid, renders all rooms into it
- Handles overlapping doors (merges adjacent rooms)
- Outputs a single string

**Main app doesn't need this** because:
- Ratatui's `Buffer` already handles 2D grid (canvas.rs:48)
- Camera viewport handles clipping (canvas.rs:78-82)
- No overlapping — progressive generation places rooms with proper offsets

---

## Testing

**Existing Tests (already passing):**
- `tests/camera_rendering.rs` — 7 tests for camera panning, viewport culling, resize
- Camera unit tests in `src/renderer/camera.rs` — 8 tests for panning logic

**After Integration:**
- Visual test: `cargo run` — should see colored walls (`█`), colored floors, box-drawing doors (`─`, `│`)
- Functional test: explorer should move, rooms should generate progressively
- No new unit tests needed (character changes are cosmetic)

---

## Summary

**Files to Change:**
- `src/renderer/canvas.rs` — 2 small changes (walls → `█` with fg color, floors → ` ` with bg color, doors → `─`/`│`)

**Files NOT Changed:**
- `src/renderer/camera.rs` — already works with 1:1 mapping
- `src/main.rs` — already passes Vec<Room> to renderer
- `src/renderer/simple_room.rs` — keep for demos, not used in main app
- `examples/pretty_explorer_demo.rs` — keep as reference

**Why This Works:**
The current `canvas.rs` already implements the validated simple grid approach. We're just upgrading the characters from ASCII (`#`, `.`, `-`, `|`) to box-drawing (`█`, ` `, `─`, `│`) and adding colored floor backgrounds.

**Estimated Effort:** 15 minutes (2 code changes + visual verification)

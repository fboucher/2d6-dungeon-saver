# Renderer Bug Diagnosis — Simple Grid Renderer

**Diagnosed by:** Mikey  
**Date:** 2025-02-12  
**Issue:** Rooms rendering as scattered random characters instead of clean rectangles

---

## Root Cause

**BUG IN:** `src/dungeon/generator.rs` lines 334-350 (`calculate_exit_position()`)

**THE PROBLEM:** Exit positions are calculated as offsets **within the room** (relative to room corner), but the renderer is treating them as **absolute world coordinates**.

### How Exit Position Works (Current Implementation)

In `calculate_exit_position()`:
- North/South walls: `position = random value % room.width` (range: 0 to width-1)
- East/West walls: `position = random value % room.height` (range: 0 to height-1)

This gives a **relative offset** from the room's top-left corner.

### How Renderer Uses Exit Position (canvas.rs lines 117-122)

```rust
let (exit_x, exit_y) = match exit.wall {
    Wall::North => (room.x + exit.position, room.y),
    Wall::South => (room.x + exit.position, room.y + room.height - 1),
    Wall::West => (room.x, room.y + exit.position),
    Wall::East => (room.x + room.width - 1, room.y + exit.position),
};
```

The renderer **correctly** adds `exit.position` as an offset to the room's position.

### The Contradiction

For North/South walls:
- Generator calculates: `position = d6() % room.width` → range [0, width-1]
- Renderer expects: position is offset along the wall
- **CORRECT** ✅

For East/West walls:
- Generator calculates: `position = d6() % room.height` → range [0, height-1]  
- Renderer expects: position is offset along the wall
- **CORRECT** ✅

Wait... this looks correct. Let me re-examine.

---

## ACTUAL Root Cause (Re-Analysis)

**REAL BUG:** Exit position calculation uses **modulo of D6 roll** (1-6) instead of proper range.

### The Math Error

In `calculate_exit_position()` line 338:
```rust
self.rng.d6() % room.width.max(1)
```

**Problem:** `d6()` returns 1-6, so:
- For a room with width=8: `d6() % 8` gives range [1, 6], **NEVER 0 or 7**
- For a room with width=4: `d6() % 4` gives [1, 2, 0, 1, 2, 0] (biased toward 0-2)
- For a room with width=10: `d6() % 10` gives [1, 2, 3, 4, 5, 6] (never uses positions 0, 7, 8, 9)

**But this creates a DISTRIBUTION bug, not a PLACEMENT bug.**

Let me look at the actual output again...

---

## THIRD ANALYSIS: The Real Issue

Looking at Frank's output:
```
-|####
          |#   #|#
          #-   #.-
```

Notice:
- Characters appear at positions that don't align with room rectangles
- Doors (`|`, `-`) are appearing in random positions
- The `@` (explorer) is placed correctly
- Room outlines are fragmentary

**WAIT.** Let me check if the room spacing calculation is correct...

### Room Spacing Issue

In `calculate_room_position()` (lines 220-250):
- `ROOM_SPACING = 3` (constant)
- New rooms positioned relative to parent with spacing

**For Wall::North:**
```rust
let y = parent.y.saturating_sub(height + ROOM_SPACING);
```

If `parent.y = 10`, `height = 6`, `ROOM_SPACING = 3`:
- `y = 10 - (6 + 3) = 10 - 9 = 1` ✅ Correct

**For Wall::South:**
```rust
let y = parent.y + parent.height + ROOM_SPACING;
```

If `parent.y = 10`, `parent.height = 8`, `ROOM_SPACING = 3`:
- `y = 10 + 8 + 3 = 21` ✅ Correct

Spacing logic looks fine.

---

## FOURTH ANALYSIS: Door Character Orientation

**AHA! FOUND IT!**

In `canvas.rs` lines 134-137:
```rust
let door_char = match exit.wall {
    Wall::North | Wall::South => '|',  // WRONG
    Wall::East | Wall::West => '-',    // WRONG
};
```

**THE BUG:** Door character orientation is **backwards**.

### Correct Orientation

- **North/South walls** are horizontal lines → doors should be `'-'` (horizontal gap)
- **East/West walls** are vertical lines → doors should be `'|'` (vertical gap)

### Current (Wrong) Implementation

- North/South walls get `'|'` (vertical character in horizontal wall) ❌
- East/West walls get `'-'` (horizontal character in vertical wall) ❌

### Why This Breaks Rendering

1. Walls are drawn as `#` characters forming rectangles
2. North/South walls run horizontally (row of `#` chars)
3. Doors in North/South should break the horizontal line with `'-'`
4. But the code uses `'|'` instead, which looks wrong and disrupts visual coherence
5. Same issue for East/West walls with reversed logic

---

## The Fix

**FILE:** `src/renderer/canvas.rs`  
**LINES:** 134-137

**CHANGE:**
```rust
let door_char = match exit.wall {
    Wall::North | Wall::South => '-',  // Horizontal gap in horizontal wall
    Wall::East | Wall::West => '|',    // Vertical gap in vertical wall
};
```

---

## Impact Analysis

**Severity:** P0 Critical — Rendering completely non-functional  
**Scope:** Single line fix (swap characters)  
**Testing:** Visual inspection after fix (rooms should form clean rectangles with proper doors)

---

## Assignment

**Chunk** — Fix door character orientation in `canvas.rs` line 135-136:
- Swap `'|'` and `'-'` characters
- North/South → `'-'`
- East/West → `'|'`
- Test with `cargo run` to verify clean room rendering

# Tile Scale Increase

**Decision Made By:** Chunk (Terminal Dev)
**Date:** 2024-01-XX
**Status:** Implemented

## Context
Rooms appeared cramped and tiny at TILE_SCALE=2, making the dungeon hard to read and navigate visually.

## Decision
Increased TILE_SCALE from 2 to 4 in `src/renderer/canvas.rs`

## Changes Made
- **File:** `src/renderer/canvas.rs`
- **Change:** `const TILE_SCALE: u32 = 2;` → `const TILE_SCALE: u32 = 4;`
- Each dungeon cell now renders as 4×4 terminal characters instead of 2×2

## Rationale
- Rooms now appear larger and more readable on screen
- Better visual clarity for dungeon navigation
- Matches Frank's example of more spacious room rendering
- No changes needed to animation speed (10 FPS = 100ms per frame works well)

## Testing
- ✅ Code compiles successfully
- ✅ Build completes without errors
- Application runs with larger, more readable rooms

## Impact
- **Positive:** Much better readability and visual appeal
- **Neutral:** May require larger terminal window for full dungeon view (camera handles this)
- **No issues:** Animation speed remains smooth at current 10 FPS

## Notes
- If rooms still feel small, can increase to TILE_SCALE=6 in future
- Current animation timing (TARGET_FPS=10, 100ms/frame) works well with new scale
- Camera viewport automatically adjusts to show appropriate portion of dungeon

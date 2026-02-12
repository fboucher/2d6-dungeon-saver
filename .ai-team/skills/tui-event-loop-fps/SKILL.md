---
name: "tui-event-loop-fps"
description: "Non-blocking event loop with precise FPS control for terminal UI applications"
domain: "terminal-ui, game-loops, rust"
confidence: "low"
source: "earned"
---

## Context

Terminal UI applications (especially animations, games, or screensavers) need to maintain consistent frame rates while remaining responsive to user input. Blocking on input causes frame drops; busy-waiting wastes CPU. This skill applies to any TUI app with animation requirements using crossterm/ratatui or similar.

## Patterns

### Non-Blocking Event Polling

Use `event::poll(Duration::ZERO)` to check for input without blocking the render loop:

```rust
use crossterm::event::{self, Event};

// Inside main loop
if event::poll(Duration::from_millis(0))? {
    if let Event::Key(key) = event::read()? {
        // Handle key
    }
}
```

**Why:** Blocks rendering until user input arrives. Frames drop during idle periods.

**Correct:** Poll with zero timeout checks if input is available, reads only when present.

### Frame Timing with Sleep

Track frame start time and sleep for the remainder to hit target FPS:

```rust
use std::time::{Duration, Instant};

const TARGET_FPS: u64 = 10;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / TARGET_FPS);

loop {
    let frame_start = Instant::now();
    
    // Render and handle input
    
    let elapsed = frame_start.elapsed();
    if elapsed < FRAME_DURATION {
        std::thread::sleep(FRAME_DURATION - elapsed);
    }
}
```

**Why:** Adapts to variable render times while maintaining consistent frame pacing. CPU usage stays minimal (thread sleeps when not needed).

### Event Loop Structure

Standard structure for TUI game loops:

```rust
fn run() -> io::Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    
    loop {
        let frame_start = Instant::now();
        
        // 1. Render
        terminal.draw(|frame| {
            render_frame(frame);
        })?;
        
        // 2. Handle events (non-blocking)
        if event::poll(Duration::ZERO)? {
            if let Event::Key(key) = event::read()? {
                if should_quit(&key) {
                    break;
                }
            }
        }
        
        // 3. Maintain target FPS
        let elapsed = frame_start.elapsed();
        if elapsed < FRAME_DURATION {
            std::thread::sleep(FRAME_DURATION - elapsed);
        }
    }
    
    Ok(())
}
```

## Examples

**10 FPS screensaver:**
```rust
const TARGET_FPS: u64 = 10;
const FRAME_DURATION: Duration = Duration::from_millis(100);
```

**60 FPS game:**
```rust
const TARGET_FPS: u64 = 60;
const FRAME_DURATION: Duration = Duration::from_millis(16);
```

**Variable frame time handling:**
```rust
let elapsed = frame_start.elapsed();
if elapsed < FRAME_DURATION {
    // Still have time in frame budget
    std::thread::sleep(FRAME_DURATION - elapsed);
} else {
    // Frame took longer than target - skip sleep, next frame starts immediately
}
```

## Anti-Patterns

❌ **Blocking event read:** `event::read()` without `poll()` check blocks the entire loop
❌ **Fixed sleep without timing:** `sleep(100ms)` every frame ignores render time variance
❌ **Busy-waiting:** Polling in tight loop without sleep wastes CPU
❌ **Frame timing after event handling:** Event processing time should be part of frame budget
❌ **Not handling slow frames:** If render takes longer than target, next frame should start immediately

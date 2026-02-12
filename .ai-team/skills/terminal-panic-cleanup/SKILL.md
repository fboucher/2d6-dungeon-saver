---
name: "terminal-panic-cleanup"
description: "Ensures terminal state restoration in TUI apps that panic"
domain: "terminal-ui, error-handling, rust"
confidence: "high"
source: "earned"
---

## Context

Terminal UI applications using raw mode and alternate screen must restore terminal state on exit. Without cleanup, panics leave the terminal unusable (no echo, broken rendering). This skill applies to any Rust TUI app using crossterm, termion, or similar.

## Patterns

### Panic Hook Installation

Install a panic hook early in `main()` that restores terminal state before the default panic handler runs:

```rust
fn main() -> io::Result<()> {
    // Capture original panic hook
    let original_hook = std::panic::take_hook();
    
    // Install custom hook that cleans up terminal
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = cleanup_terminal();
        original_hook(panic_info);
    }));

    setup_terminal()?;
    let result = run();
    cleanup_terminal()?;
    
    result
}
```

### Shared Cleanup Function

Define cleanup logic once, reuse in both normal exit and panic hook:

```rust
fn cleanup_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
```

### Setup/Cleanup Symmetry

Setup and cleanup must be exact inverses:

```rust
fn setup_terminal() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Ok(())
}
```

## Examples

**Crossterm (most common):**
```rust
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
```

**Full main() structure:**
```rust
fn main() -> io::Result<()> {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = cleanup_terminal();
        original_hook(panic_info);
    }));

    setup_terminal()?;
    let result = run_app();
    cleanup_terminal()?;
    result
}
```

## Anti-Patterns

❌ **Cleanup only in normal exit path:** Panics will leave terminal broken
❌ **Cleanup in Drop impl:** Panic during drop = double panic = instant abort
❌ **Ignoring cleanup errors in panic hook:** Use `let _ =` since we can't propagate errors from panic hooks
❌ **Installing panic hook after setup_terminal():** If setup panics, terminal won't be cleaned

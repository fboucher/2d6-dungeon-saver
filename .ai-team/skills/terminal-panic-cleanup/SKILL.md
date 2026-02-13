---
name: "terminal-panic-cleanup"
description: "Ensures terminal state restoration in TUI apps on all exit paths (panics, signals, normal exit)"
domain: "terminal-ui, error-handling, rust, signals"
confidence: "high"
source: "earned"
---

## Context

Terminal UI applications using raw mode and alternate screen must restore terminal state on ALL exit paths. Without cleanup, panics or signals (Ctrl+C, SIGTERM) leave the terminal unusable (no echo, broken rendering, hidden cursor). This skill applies to any Rust TUI app using crossterm, termion, or similar.

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

### Signal Handler Installation

Add signal handlers for SIGTERM/SIGINT using the `ctrlc` crate (battle-tested, cross-platform):

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() -> io::Result<()> {
    // Panic hook (as above)
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = cleanup_terminal();
        original_hook(panic_info);
    }));

    // Signal handler for Ctrl+C and SIGTERM
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        let _ = cleanup_terminal();
        r.store(false, Ordering::SeqCst);
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    setup_terminal()?;
    let result = run(running);
    cleanup_terminal()?;
    result
}
```

**Cargo.toml dependency:**
```toml
[dependencies]
ctrlc = "3.4"
```

### Shared Cleanup Function

Define cleanup logic once, reuse in normal exit, panic hook, and signal handler:

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

### Event Loop with Signal Check

Main loop should check the running flag for graceful shutdown:

```rust
fn run(running: Arc<AtomicBool>) -> io::Result<()> {
    loop {
        // Check for signal
        if !running.load(Ordering::SeqCst) {
            break;
        }
        
        // Game logic...
        
        // Handle events...
    }
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

**Full main() structure with all exit paths covered:**
```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() -> io::Result<()> {
    // Panic hook
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = cleanup_terminal();
        original_hook(panic_info);
    }));

    // Signal handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        let _ = cleanup_terminal();
        r.store(false, Ordering::SeqCst);
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    setup_terminal()?;
    let result = run_app(running);
    cleanup_terminal()?;
    result
}
```

## Three Exit Paths

1. **Normal Exit (q/Q key):** Event loop breaks → cleanup_terminal() → Ok(())
2. **Panic:** Panic hook → cleanup_terminal() → default panic handler
3. **Signal (Ctrl+C, SIGTERM):** Signal handler → cleanup_terminal() → exit(0)

All paths must call the same cleanup function for consistency.

## Anti-Patterns

❌ **Cleanup only in normal exit path:** Panics or signals will leave terminal broken  
❌ **No signal handler:** Ctrl+C is a standard user expectation and must work cleanly  
❌ **Cleanup in Drop impl:** Panic during drop = double panic = instant abort  
❌ **Ignoring cleanup errors in panic/signal hooks:** Use `let _ =` since we can't propagate errors  
❌ **Installing hooks after setup_terminal():** If setup panics/signals before hooks installed, terminal won't be cleaned  
❌ **Process managers without SIGTERM handling:** Production systems need graceful shutdown on signals

## Trade-offs

**Map Export on Signal Exit:** Signal handlers typically use `exit(0)` for immediate cleanup, skipping export logic. This is acceptable because:
- Terminal cleanup is higher priority (broken terminal > missing export)
- Normal quit (q/Q) is the expected path for saving state
- Signals are "emergency exits" where cleanup > persistence

# SKILL: Graceful Export on Application Exit

**Pattern:** Automatic data export after user-triggered shutdown, with error handling that never blocks exit

**Confidence:** medium  
**Source:** earned  
**Tags:** #file-io #error-handling #user-experience #cleanup

## Problem

Applications with valuable runtime state need to persist data on exit, but export failures shouldn't prevent graceful shutdown or leave the terminal in a broken state.

## Solution

Place export logic **after** the main event loop terminates but **before** terminal cleanup. Wrap exports in error handling that logs to stderr but allows shutdown to proceed.

## Implementation Pattern

```rust
fn main() -> io::Result<()> {
    setup_terminal()?;
    
    let result = run();  // Event loop runs here
    
    // Export after event loop, before cleanup
    match exporter.export(&data) {
        Ok(filepath) => {
            eprintln!("Data exported to: {}", filepath.display());
        }
        Err(e) => {
            eprintln!("Warning: Failed to export: {}", e);
        }
    }
    
    cleanup_terminal()?;  // Terminal cleanup always happens
    
    result
}
```

## Key Principles

1. **Export after loop exit:** User has already signaled intent to quit. Export is "bonus" not blocker.

2. **Never panic on export failure:** Wrap in `match` or `if let`, log errors to stderr, continue to cleanup.

3. **Confirm success to user:** Print export filepath to stderr so users know where to find saved data.

4. **Create directories automatically:** Use `fs::create_dir_all()` so export doesn't fail on missing directories.

5. **Order matters:** Export → Cleanup → Return. Terminal must be restored even if export fails.

## Why Not Alternatives

- ❌ Export **during** event loop: Adds latency to shutdown
- ❌ Export **before** loop starts: No runtime data to save yet
- ❌ Panic on export failure: Breaks terminal, terrible UX
- ❌ Silent failure: User doesn't know if export worked

## Testing

```rust
#[test]
fn test_export_failure_doesnt_crash() {
    // Simulate export failure (bad directory, permissions, etc)
    let exporter = MapExporter::new(seed);
    
    // Should return Err, not panic
    let result = exporter.export(&dungeon);
    assert!(result.is_err());
    
    // Application continues and cleans up normally
}
```

## Examples

- **Dungeon Saver:** Exports ASCII map with metadata on quit
- **Game save systems:** Write save file before closing window
- **Log viewers:** Export filtered logs when user quits
- **Data analysis tools:** Save current workspace/filters on exit

## Related Patterns

- See **terminal-panic-cleanup** for panic hook setup
- See **deterministic-game-logic** for seed-based generation worth exporting

# Signal Handling Verification

## Test Instructions

To verify that signal handlers properly clean up the terminal:

1. **Build the application:**
   ```bash
   cargo build --release
   ```

2. **Run the application:**
   ```bash
   ./target/release/dungeon-saver
   ```

3. **Send SIGINT (Ctrl+C):**
   - Press `Ctrl+C` while the app is running
   - Terminal should immediately return to normal state
   - No ANSI escape codes should leak
   - Cursor should be visible
   - No alternate screen residue

4. **Send SIGTERM:**
   ```bash
   ./target/release/dungeon-saver & 
   PID=$!
   sleep 2
   kill -TERM $PID
   ```
   - Terminal should return to clean state
   - Same cleanup as Ctrl+C

## Expected Behavior

**Before fix:** Killing with signals left terminal in broken state (raw mode, alternate screen, hidden cursor)

**After fix:** All exit paths (q/Q, Ctrl+C, SIGTERM, panic) properly call `cleanup_terminal()` and restore terminal state.

## Implementation Details

- **Signal Handler:** `ctrlc` crate (v3.4) installs handler for SIGTERM/SIGINT
- **Cleanup Function:** Same `cleanup_terminal()` used for normal exit, panic, and signals
- **Exit Strategy:** Handler calls cleanup, then `std::process::exit(0)` for immediate termination
- **Atomic Flag:** `Arc<AtomicBool>` shared with main loop (graceful shutdown path, though exit(0) is faster)

## Files Modified

- `Cargo.toml`: Added `ctrlc = "3.4"` dependency
- `src/main.rs`: Added signal handler installation in `main()`, running flag check in event loop

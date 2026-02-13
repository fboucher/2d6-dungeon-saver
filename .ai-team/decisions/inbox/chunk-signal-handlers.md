### 2025-01-20: Signal Handlers for Terminal Cleanup

**By:** Chunk

**What:** Added SIGTERM/SIGINT signal handlers to ensure cleanup_terminal() runs on all exit paths, not just normal 'q' quit. Uses ctrlc crate for cross-platform signal handling.

**Why:** 

1. **User Expectation:** Ctrl+C is the standard "emergency exit" for terminal applications. Users expect it to work cleanly without breaking their terminal.

2. **Production Safety:** Process managers (systemd, supervisord, etc.) send SIGTERM for graceful shutdown. Without signal handling, every deployment restart would leave terminals broken.

3. **Terminal State Corruption:** When killed without cleanup, the terminal stays in:
   - Raw mode (keyboard input broken)
   - Alternate screen (previous work hidden)
   - Hidden cursor (disorienting)
   - ANSI escape code state leaked

4. **Three Exit Paths, One Cleanup:** We now handle:
   - Normal quit (q/Q): cleanup → normal exit
   - Panic: panic hook → cleanup → panic handler
   - Signal (Ctrl+C/SIGTERM): signal handler → cleanup → exit(0)

**Implementation:**
- Added `ctrlc = "3.4"` dependency (battle-tested, cross-platform)
- Signal handler calls cleanup_terminal() then std::process::exit(0)
- Arc<AtomicBool> shared between handler and main loop for graceful path
- Immediate exit(0) prioritizes cleanup over map export (map only on normal quit)

**Trade-off:** Map export doesn't happen on signal/panic exit. This is acceptable because:
- Terminal cleanup is higher priority than export (broken terminal is worse than missing map)
- Normal quit (q/Q) is the expected exit path for saving work
- Signals/panics are emergency exits where state preservation is secondary

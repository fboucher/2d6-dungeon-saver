### 2026-02-13: Progressive Generation API — Dummy Room Sentinel

**By:** Data

**What:** Changed `DungeonGenerator::add_room()` to return a dummy Room (id=usize::MAX) instead of None/Result when hitting 20-room limit.

**Why:**

1. **API simplicity**: Returning `Room` instead of `Option<Room>` matches the success case type signature, avoiding Result/Option unwrapping in tight loops.

2. **Sentinel pattern**: The dummy room with `id == usize::MAX` acts as a clear "stop signal" that callers can check with a simple equality test (`new_room.id != usize::MAX`).

3. **Backward compatibility**: The returned Room is valid (can be inspected), just shouldn't be added to dungeon. This prevents crashes if caller forgets to check.

4. **Integration compatibility**: Tests and main loop can filter dummy rooms with simple id check before pushing to dungeon Vec.

**Trade-offs:**

- **Pro**: Simpler API, no unwrapping needed, valid Room object always returned
- **Con**: Caller must remember to check id before using the room
- **Con**: Less type-safe than `Option<Room>` (compiler doesn't enforce check)

**Alternative considered:** Return `Option<Room>` where None = limit reached. Rejected because main loop would need `.map().unwrap_or_else()` boilerplate on every exit detection.

**Pattern for callers:**
```rust
let new_room = generator.add_room(parent_id, wall);
if new_room.id != usize::MAX {
    dungeon.push(new_room);
    pathfinder = Pathfinder::new(&dungeon);
}
```

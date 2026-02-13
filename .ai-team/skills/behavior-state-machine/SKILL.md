# SKILL: Frame-Based Behavior State Machine

## Metadata
- **Confidence:** medium
- **Source:** earned
- **Language:** Rust
- **Domain:** Game AI, autonomous agents, animation systems

## Pattern

For autonomous agent behavior (NPCs, enemies, procedural animations), implement a state machine with an `update()` method called each frame. States contain their own data (e.g., tick counters) and transition logic. This creates deterministic, testable AI without callbacks or complex event systems.

## Implementation

```rust
#[derive(Debug, Clone)]
pub struct Agent {
    pub x: u32,
    pub y: u32,
    pub state: AgentState,
    // Internal state tracking
    visited_targets: HashSet<usize>,
    current_path: Vec<(u32, u32)>,
    path_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    Exploring,
    Wandering,
    Pausing { ticks_remaining: u32 },
}

impl Agent {
    pub fn update(&mut self, world: &World, pathfinder: &Pathfinder, rng: &mut Rng) {
        // Handle current state
        match self.state {
            AgentState::Pausing { ticks_remaining } => {
                if ticks_remaining > 1 {
                    // Continue pausing
                    self.state = AgentState::Pausing { 
                        ticks_remaining: ticks_remaining - 1 
                    };
                    return;
                } else {
                    // Transition to next state
                    self.state = self.next_state_after_pause(world);
                }
            }
            _ => {}
        }
        
        // Check for state transitions (e.g., discovered new target)
        if let Some(target_id) = self.current_target(world) {
            if !self.visited_targets.contains(&target_id) {
                self.visited_targets.insert(target_id);
                // Trigger pause
                self.state = AgentState::Pausing { 
                    ticks_remaining: rng.range(10, 31) as u32 
                };
                return;
            }
        }
        
        // Execute movement for current state
        self.move_agent(world, pathfinder, rng);
    }
    
    fn move_agent(&mut self, world: &World, pathfinder: &Pathfinder, rng: &mut Rng) {
        // Follow current path or find new one
        if self.path_index < self.current_path.len() {
            let next_pos = self.current_path[self.path_index];
            self.x = next_pos.0;
            self.y = next_pos.1;
            self.path_index += 1;
        } else {
            // Need new path based on state
            self.find_new_path(world, pathfinder, rng);
        }
    }
}
```

## Key Principles

1. **Single Update Entry Point:** One `update()` method called per frame keeps timing deterministic
2. **State-Owned Data:** States can contain their own data (tick counters, timers, targets)
3. **Early Returns:** Handle time-consuming states (pausing, animations) with early returns
4. **Explicit Transitions:** State changes are explicit assignments, easy to debug
5. **Separate Concerns:** Movement logic separate from state transition logic

## Benefits

- **Testable:** Can simulate N frames with controlled inputs, verify state transitions
- **Deterministic:** Same inputs + seed → same behavior (critical for replay/networking)
- **Debuggable:** Current state visible in debugger, no hidden callback chains
- **Framerate Independent:** Tick-based timing adapts to variable frame rates

## When to Use

- Game AI (NPCs, enemies, companions, procedural agents)
- Animation state machines (idle, walk, attack, hurt)
- UI state management (loading, ready, error, success)
- Any time-based behavior that needs deterministic updates

## When NOT to Use

- Event-driven systems (prefer observer pattern with callbacks)
- Highly complex state graphs (consider hierarchical state machines or behavior trees)
- Concurrent state execution (state machine is sequential by design)

## Common States Pattern

```rust
pub enum BehaviorState {
    Idle,                           // No action
    Moving { target: (u32, u32) },  // En route to target
    Pausing { ticks_remaining: u32 }, // Waiting
    Cooldown { ability_id: u32, ticks: u32 }, // Delay before next action
    Engaged { target_id: usize },   // Active interaction
}
```

## Testing Strategy

```rust
#[test]
fn test_state_transition_after_pause() {
    let mut agent = Agent::new(0, 0);
    agent.state = AgentState::Pausing { ticks_remaining: 5 };
    
    // Run 5 updates
    for _ in 0..5 {
        agent.update(&world, &pathfinder, &mut rng);
    }
    
    // Should have transitioned out of pausing
    assert_ne!(agent.state, AgentState::Pausing { ticks_remaining: 5 });
}
```

## Anti-Patterns to Avoid

1. **State Leakage:** Don't check state in rendering code; expose getters instead
2. **Callback Hell:** Keep transitions in update(), don't use event listeners per state
3. **Frame Dependence:** Base timing on ticks, not real time (breaks determinism)
4. **Implicit State:** Hidden flags (is_jumping, is_attacking) instead of explicit states

## Related Patterns

- **Hierarchical FSM:** Nested state machines for complex AI
- **Behavior Trees:** More flexible but less deterministic alternative
- **Goal-Oriented Action Planning:** Higher-level AI planning system

/// Integration tests for Phase 4: Explorer AI and Pathfinding
/// UPDATED FOR PROGRESSIVE GENERATION
use dungeon_saver::explorer::{Explorer, Pathfinder};
use dungeon_saver::dungeon::DungeonGenerator;
use dungeon_saver::rng::SeededRng;

#[test]
#[ignore = "Requires Phase 2 main loop integration - explorer door detection not yet implemented"]
fn test_explorer_visits_progressively_generated_rooms() {
    // Start with entrance only
    let mut generator = DungeonGenerator::new(12345);
    let mut dungeon = vec![generator.generate_entrance()];
    
    // Create pathfinder and explorer
    let mut pathfinder = Pathfinder::new(&dungeon);
    let entrance = &dungeon[0];
    let start_x = entrance.x + entrance.width / 2;
    let start_y = entrance.y + entrance.height / 2;
    let mut explorer = Explorer::new(start_x, start_y);
    let mut rng = SeededRng::new(12345);
    
    // Simulate progressive exploration
    for _ in 0..500 {
        explorer.update(&dungeon, &pathfinder, &mut rng);
        
        // Check if explorer reached an unexplored exit - generate new room
        let (ex, ey) = explorer.position();
        
        // Find if explorer is on an exit position
        for room in &dungeon.clone() {
            for exit in &room.exits {
                if exit.connected_room_id.is_some() {
                    continue;
                }
                
                let (exit_x, exit_y) = match exit.wall {
                    dungeon_saver::dungeon::room::Wall::North => 
                        (room.x + exit.position, room.y),
                    dungeon_saver::dungeon::room::Wall::South => 
                        (room.x + exit.position, room.y + room.height - 1),
                    dungeon_saver::dungeon::room::Wall::East => 
                        (room.x + room.width - 1, room.y + exit.position),
                    dungeon_saver::dungeon::room::Wall::West => 
                        (room.x, room.y + exit.position),
                };
                
                if ex == exit_x && ey == exit_y {
                    // Generate new room!
                    let new_room = generator.add_room(room.id, exit.wall);
                    
                    // Check if it's a real room (not dummy at limit)
                    if new_room.id != usize::MAX {
                        // Update parent exit
                        if let Some(parent) = dungeon.iter_mut().find(|r| r.id == room.id) {
                            if let Some(exit_mut) = parent.exits.iter_mut()
                                .find(|e| e.wall == exit.wall && e.connected_room_id.is_none()) {
                                exit_mut.connected_room_id = Some(new_room.id);
                            }
                        }
                        
                        dungeon.push(new_room);
                        
                        // Rebuild pathfinder with new room
                        pathfinder = Pathfinder::new(&dungeon);
                    }
                }
            }
        }
        
        if dungeon.len() >= 20 {
            break;
        }
    }
    
    // Explorer should have triggered progressive generation
    assert!(dungeon.len() > 1, "Explorer should have discovered new rooms");
}

#[test]
fn test_pathfinder_returns_valid_paths() {
    let mut generator = DungeonGenerator::new(999);
    let entrance = generator.generate_entrance();
    let dungeon = vec![entrance.clone()];
    
    let pathfinder = Pathfinder::new(&dungeon);
    
    // Test path from entrance to itself (should be trivial)
    let center = (entrance.x + entrance.width / 2, entrance.y + entrance.height / 2);
    
    let path = pathfinder.find_path(center, center);
    assert!(path.is_some());
    let path = path.unwrap();
    assert_eq!(path.len(), 1); // Path to self is just the starting point
    assert_eq!(path[0], center);
}

#[test]
fn test_pathfinder_finds_path_within_room() {
    let mut generator = DungeonGenerator::new(777);
    let entrance = generator.generate_entrance();
    
    // Entrance should be large enough (6-12 squares)
    let start = (entrance.x, entrance.y);
    let end = (entrance.x + entrance.width - 1, entrance.y + entrance.height - 1);
    
    let pathfinder = Pathfinder::new(&vec![entrance]);
    let path = pathfinder.find_path(start, end);
    
    assert!(path.is_some(), "Should find path within entrance room");
    let path = path.unwrap();
    assert_eq!(path.first(), Some(&start));
    assert_eq!(path.last(), Some(&end));
}

#[test]
fn test_explorer_pauses_on_room_entry() {
    let mut generator = DungeonGenerator::new(42);
    let entrance = generator.generate_entrance();
    let dungeon = vec![entrance.clone()];
    
    let pathfinder = Pathfinder::new(&dungeon);
    let start_x = entrance.x + entrance.width / 2;
    let start_y = entrance.y + entrance.height / 2;
    let mut explorer = Explorer::new(start_x, start_y);
    let mut rng = SeededRng::new(42);
    
    // First update should trigger room discovery and pause
    explorer.update(&dungeon, &pathfinder, &mut rng);
    
    // Explorer should be in pausing state after discovering entrance room
    match explorer.state {
        dungeon_saver::explorer::behavior::ExplorerState::Pausing { ticks_remaining } => {
            assert!(ticks_remaining > 0 && ticks_remaining <= 30);
        }
        _ => panic!("Expected explorer to be pausing after discovering a room"),
    }
}

#[test]
fn test_explorer_maintains_valid_position_progressive() {
    let mut generator = DungeonGenerator::new(555);
    let entrance = generator.generate_entrance();
    let mut dungeon = vec![entrance.clone()];
    
    let mut pathfinder = Pathfinder::new(&dungeon);
    let start_x = entrance.x + entrance.width / 2;
    let start_y = entrance.y + entrance.height / 2;
    let mut explorer = Explorer::new(start_x, start_y);
    let mut rng = SeededRng::new(555);
    
    // Run explorer for many ticks with progressive generation
    for _ in 0..200 {
        explorer.update(&dungeon, &pathfinder, &mut rng);
        
        let (x, y) = explorer.position();
        
        // Explorer should be within a room
        let in_room = dungeon.iter().any(|room| {
            x >= room.x && x < room.x + room.width &&
            y >= room.y && y < room.y + room.height
        });
        
        // Check if in a corridor tile (adjacent to a room exit)
        let in_corridor = dungeon.iter().any(|room| {
            room.exits.iter().any(|exit| {
                let (corridor_x, corridor_y) = match exit.wall {
                    dungeon_saver::dungeon::room::Wall::North => 
                        (room.x + exit.position, room.y.saturating_sub(1)),
                    dungeon_saver::dungeon::room::Wall::South => 
                        (room.x + exit.position, room.y + room.height),
                    dungeon_saver::dungeon::room::Wall::East => 
                        (room.x + room.width, room.y + exit.position),
                    dungeon_saver::dungeon::room::Wall::West => 
                        (room.x.saturating_sub(1), room.y + exit.position),
                };
                x == corridor_x && y == corridor_y
            })
        });
        
        assert!(
            in_room || in_corridor,
            "Explorer at ({}, {}) is not in a room or corridor!",
            x, y
        );
        
        // Simulate progressive generation if explorer reaches unexplored exit
        if dungeon.len() < 15 {
            for room in dungeon.clone().iter() {
                for exit in &room.exits {
                    if exit.connected_room_id.is_some() {
                        continue;
                    }
                    
                    let (exit_x, exit_y) = match exit.wall {
                        dungeon_saver::dungeon::room::Wall::North => 
                            (room.x + exit.position, room.y),
                        dungeon_saver::dungeon::room::Wall::South => 
                            (room.x + exit.position, room.y + room.height - 1),
                        dungeon_saver::dungeon::room::Wall::East => 
                            (room.x + room.width - 1, room.y + exit.position),
                        dungeon_saver::dungeon::room::Wall::West => 
                            (room.x, room.y + exit.position),
                    };
                    
                    if x == exit_x && y == exit_y {
                        let new_room = generator.add_room(room.id, exit.wall);
                        
                        if let Some(parent) = dungeon.iter_mut().find(|r| r.id == room.id) {
                            if let Some(exit_mut) = parent.exits.iter_mut()
                                .find(|e| e.wall == exit.wall && e.connected_room_id.is_none()) {
                                exit_mut.connected_room_id = Some(new_room.id);
                            }
                        }
                        
                        dungeon.push(new_room);
                        pathfinder = Pathfinder::new(&dungeon);
                    }
                }
            }
        }
    }
}

#[test]
fn test_multiple_dungeons_explorer_works() {
    // Test that explorer works with progressive generation across different seeds
    for seed in [1, 100, 1000, 9999] {
        let mut generator = DungeonGenerator::new(seed);
        let entrance = generator.generate_entrance();
        let mut dungeon = vec![entrance.clone()];
        
        let mut pathfinder = Pathfinder::new(&dungeon);
        let start_x = entrance.x + entrance.width / 2;
        let start_y = entrance.y + entrance.height / 2;
        let mut explorer = Explorer::new(start_x, start_y);
        let mut rng = SeededRng::new(seed);
        
        // Should run without panicking for at least 100 ticks
        for _ in 0..100 {
            explorer.update(&dungeon, &pathfinder, &mut rng);
            
            // Optionally generate rooms as explorer explores
            if dungeon.len() < 10 {
                let (x, y) = explorer.position();
                
                for room in dungeon.clone().iter() {
                    for exit in &room.exits {
                        if exit.connected_room_id.is_some() {
                            continue;
                        }
                        
                        let (exit_x, exit_y) = match exit.wall {
                            dungeon_saver::dungeon::room::Wall::North => 
                                (room.x + exit.position, room.y),
                            dungeon_saver::dungeon::room::Wall::South => 
                                (room.x + exit.position, room.y + room.height - 1),
                            dungeon_saver::dungeon::room::Wall::East => 
                                (room.x + room.width - 1, room.y + exit.position),
                            dungeon_saver::dungeon::room::Wall::West => 
                                (room.x, room.y + exit.position),
                        };
                        
                        if x == exit_x && y == exit_y {
                            let new_room = generator.add_room(room.id, exit.wall);
                            
                            if let Some(parent) = dungeon.iter_mut().find(|r| r.id == room.id) {
                                if let Some(exit_mut) = parent.exits.iter_mut()
                                    .find(|e| e.wall == exit.wall && e.connected_room_id.is_none()) {
                                    exit_mut.connected_room_id = Some(new_room.id);
                                }
                            }
                            
                            dungeon.push(new_room);
                            pathfinder = Pathfinder::new(&dungeon);
                        }
                    }
                }
            }
        }
    }
}

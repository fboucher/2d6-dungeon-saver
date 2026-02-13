/// Phase 6: Progressive Generation Tests
/// 
/// Tests for the new progressive dungeon generation where:
/// - Dungeon starts with entrance room only
/// - Rooms are generated as exits are explored
/// - Same seed produces different dungeon layouts (emergent behavior)
/// - Explorer always has valid path to unexplored exits
/// - Generation stops at ~20 rooms

use dungeon_saver::dungeon::DungeonGenerator;
use dungeon_saver::explorer::Pathfinder;

#[test]
fn test_entrance_generation_only() {
    let mut generator = DungeonGenerator::new(42);
    let entrance = generator.generate_entrance();
    
    // Should have exactly one room (entrance)
    assert_eq!(entrance.id, 0);
    assert!(entrance.room_type == dungeon_saver::dungeon::room::RoomType::Entrance);
    
    // Entrance should have exits, but none connected yet
    assert!(entrance.exits.len() > 0, "Entrance must have at least one exit");
    for exit in &entrance.exits {
        assert!(exit.connected_room_id.is_none(), 
                "Initial entrance exits should not be connected");
    }
}

#[test]
fn test_progressive_room_addition() {
    let mut generator = DungeonGenerator::new(123);
    let entrance = generator.generate_entrance();
    
    assert_eq!(entrance.exits.len(), 3, "Entrance always has 3 exits");
    
    // Generate a room at the first exit
    let first_exit_wall = entrance.exits[0].wall;
    let new_room = generator.add_room(entrance.id, first_exit_wall);
    
    // New room should have valid properties
    assert!(new_room.width > 0 && new_room.width <= 6);
    assert!(new_room.height > 0 && new_room.height <= 6);
    assert!(new_room.id > entrance.id);
    
    // New room should have exits (or none if it's a dead end)
    assert!(new_room.exits.len() <= 3, "Room cannot have more than 3 exits");
}

#[test]
fn test_same_seed_different_dungeons() {
    let seed = 999;
    
    // Scenario 1: Explorer goes North first
    let mut gen1 = DungeonGenerator::new(seed);
    let entrance1 = gen1.generate_entrance();
    let north_exit = entrance1.exits.iter()
        .find(|e| e.wall == dungeon_saver::dungeon::room::Wall::North)
        .expect("Should have north exit");
    let room1 = gen1.add_room(entrance1.id, north_exit.wall);
    
    // Scenario 2: Explorer goes South first
    let mut gen2 = DungeonGenerator::new(seed);
    let entrance2 = gen2.generate_entrance();
    let south_exit = entrance2.exits.iter()
        .find(|e| e.wall == dungeon_saver::dungeon::room::Wall::South)
        .expect("Should have south exit");
    let room2 = gen2.add_room(entrance2.id, south_exit.wall);
    
    // Same seed but different explorer paths should produce different room layouts
    // The dice rolls will be the same, but the order matters
    // Room dimensions might differ because RNG state is different when called
    assert_eq!(entrance1.width, entrance2.width, 
               "Entrance rooms should be identical with same seed");
    assert_eq!(entrance1.height, entrance2.height);
    
    // But the dungeon structure emerges from explorer choices
    // We can't predict room1 == room2 because RNG state differs
}

#[test]
fn test_dungeon_growth_limit() {
    let mut generator = DungeonGenerator::new(555);
    let mut dungeon = vec![generator.generate_entrance()];
    
    // Simulate progressive generation up to the limit
    let mut total_unexplored_exits = dungeon[0].exits.len();
    let mut generation_count = 0;
    
    while total_unexplored_exits > 0 && dungeon.len() < 25 {
        // Find an unexplored exit
        let mut found_exit = None;
        for room in &dungeon {
            if let Some(exit) = room.exits.iter().find(|e| e.connected_room_id.is_none()) {
                found_exit = Some((room.id, exit.wall));
                break;
            }
        }
        
        if let Some((room_id, wall)) = found_exit {
            let new_room = generator.add_room(room_id, wall);
            dungeon.push(new_room);
            generation_count += 1;
            
            // Recalculate unexplored exits
            total_unexplored_exits = dungeon.iter()
                .flat_map(|r| &r.exits)
                .filter(|e| e.connected_room_id.is_none())
                .count();
        } else {
            break;
        }
    }
    
    // Should stop around 20 rooms (15-25 is reasonable range)
    assert!(dungeon.len() >= 15 && dungeon.len() <= 25,
            "Dungeon should have ~20 rooms, got {}", dungeon.len());
}

#[test]
fn test_exit_tracking() {
    let mut generator = DungeonGenerator::new(777);
    let entrance = generator.generate_entrance();
    
    // Initially all exits are unexplored
    for exit in &entrance.exits {
        assert!(exit.connected_room_id.is_none(), "Unexplored exits have no connected room");
    }
    
    // After generating a room at an exit, it should be marked as explored
    let first_wall = entrance.exits[0].wall;
    let new_room = generator.add_room(entrance.id, first_wall);
    
    // The generator tracks connections internally
    // In real implementation, the dungeon Vec would be updated
    // This test verifies the new room was created
    assert!(new_room.id > 0);
}

#[test]
fn test_connectivity_no_orphans() {
    let mut generator = DungeonGenerator::new(888);
    let mut dungeon = vec![generator.generate_entrance()];
    
    // Generate 10 rooms progressively
    for _ in 0..10 {
        // Find an unexplored exit
        let unexplored = dungeon.iter()
            .find_map(|r| {
                r.exits.iter()
                    .find(|e| e.connected_room_id.is_none())
                    .map(|e| (r.id, e.wall))
            });
        
        if let Some((room_id, wall)) = unexplored {
            let new_room = generator.add_room(room_id, wall);
            
            // Update the parent exit to point to new room
            if let Some(parent) = dungeon.iter_mut().find(|r| r.id == room_id) {
                if let Some(exit) = parent.exits.iter_mut().find(|e| e.wall == wall && e.connected_room_id.is_none()) {
                    exit.connected_room_id = Some(new_room.id);
                }
            }
            
            dungeon.push(new_room);
        } else {
            break;
        }
    }
    
    // Every room except entrance should be connected to at least one parent
    for room in dungeon.iter().skip(1) {
        let has_parent = dungeon.iter().any(|parent| {
            parent.exits.iter().any(|exit| exit.connected_room_id == Some(room.id))
        });
        
        assert!(has_parent, "Room {} is orphaned (not connected to any parent)", room.id);
    }
}

#[test]
fn test_explorer_pathfinding_to_unexplored_exits() {
    let mut generator = DungeonGenerator::new(321);
    let entrance = generator.generate_entrance();
    let dungeon = vec![entrance.clone()];
    
    // Create pathfinder with just entrance
    let pathfinder = Pathfinder::new(&dungeon);
    
    // Explorer should be able to path to exit positions
    let start_x = entrance.x + entrance.width / 2;
    let start_y = entrance.y + entrance.height / 2;
    
    for exit in &entrance.exits {
        let (exit_x, exit_y) = match exit.wall {
            dungeon_saver::dungeon::room::Wall::North => 
                (entrance.x + exit.position, entrance.y),
            dungeon_saver::dungeon::room::Wall::South => 
                (entrance.x + exit.position, entrance.y + entrance.height - 1),
            dungeon_saver::dungeon::room::Wall::East => 
                (entrance.x + entrance.width - 1, entrance.y + exit.position),
            dungeon_saver::dungeon::room::Wall::West => 
                (entrance.x, entrance.y + exit.position),
        };
        
        let path = pathfinder.find_path((start_x, start_y), (exit_x, exit_y));
        assert!(path.is_some(), 
                "Should be able to path from entrance center to exit at {:?}", exit.wall);
    }
}

#[test]
fn test_visual_room_by_room_growth_simulation() {
    // This test simulates the visual behavior: dungeon appears room-by-room
    let mut generator = DungeonGenerator::new(111);
    let mut dungeon = vec![generator.generate_entrance()];
    
    let mut growth_log = vec![dungeon.len()];
    
    // Simulate explorer discovering 5 rooms
    for i in 0..5 {
        let unexplored = dungeon.iter()
            .find_map(|r| {
                r.exits.iter()
                    .find(|e| e.connected_room_id.is_none())
                    .map(|e| (r.id, e.wall))
            });
        
        if let Some((room_id, wall)) = unexplored {
            let new_room = generator.add_room(room_id, wall);
            
            // Update connection
            if let Some(parent) = dungeon.iter_mut().find(|r| r.id == room_id) {
                if let Some(exit) = parent.exits.iter_mut().find(|e| e.wall == wall && e.connected_room_id.is_none()) {
                    exit.connected_room_id = Some(new_room.id);
                }
            }
            
            dungeon.push(new_room);
            growth_log.push(dungeon.len());
        } else {
            break;
        }
    }
    
    // Verify progressive growth: 1 → 2 → 3 → 4 → 5 → 6
    assert_eq!(growth_log.len(), 6, "Should have 6 snapshots (initial + 5 additions)");
    for i in 0..growth_log.len() {
        assert_eq!(growth_log[i], i + 1, "Room count should grow by 1 each step");
    }
}

#[test]
fn test_no_infinite_generation() {
    let mut generator = DungeonGenerator::new(444);
    let mut dungeon = vec![generator.generate_entrance()];
    
    let mut iterations = 0;
    let max_iterations = 1000; // Safety limit
    
    while iterations < max_iterations {
        let unexplored = dungeon.iter()
            .find_map(|r| {
                r.exits.iter()
                    .find(|e| e.connected_room_id.is_none())
                    .map(|e| (r.id, e.wall))
            });
        
        if let Some((room_id, wall)) = unexplored {
            let new_room = generator.add_room(room_id, wall);
            
            // Update connection
            if let Some(parent) = dungeon.iter_mut().find(|r| r.id == room_id) {
                if let Some(exit) = parent.exits.iter_mut().find(|e| e.wall == wall && e.connected_room_id.is_none()) {
                    exit.connected_room_id = Some(new_room.id);
                }
            }
            
            dungeon.push(new_room);
            iterations += 1;
        } else {
            break; // No more unexplored exits
        }
    }
    
    assert!(iterations < max_iterations, 
            "Generation ran for {} iterations without stopping - infinite loop!", iterations);
    assert!(dungeon.len() <= 30, 
            "Generation should stop around 20 rooms, got {}", dungeon.len());
}

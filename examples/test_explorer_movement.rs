/// Test explorer movement to verify pathfinding works
use dungeon_saver::dungeon::DungeonGenerator;
use dungeon_saver::explorer::{Explorer, Pathfinder};
use dungeon_saver::rng::SeededRng;

fn main() {
    println!("Testing explorer movement with dungeon generation...\n");
    
    let seed = 42u64;
    let mut generator = DungeonGenerator::new(seed);
    let dungeon = generator.generate();
    
    println!("Generated {} rooms", dungeon.len());
    for (i, room) in dungeon.iter().take(5).enumerate() {
        println!("  Room {}: pos=({}, {}), size={}x{}, exits={}", 
                 i, room.x, room.y, room.width, room.height, room.exits.len());
    }
    println!();
    
    // Initialize pathfinder
    let pathfinder = Pathfinder::new(&dungeon);
    println!("Pathfinder initialized");
    
    // Create explorer at entrance
    let entrance = &dungeon[0];
    let start_x = entrance.x + entrance.width / 2;
    let start_y = entrance.y + entrance.height / 2;
    let mut explorer = Explorer::new(start_x, start_y);
    println!("Explorer starting at ({}, {})\n", start_x, start_y);
    
    // Initialize RNG
    let mut rng = SeededRng::new(seed);
    
    // Simulate some frames
    println!("Simulating 100 frames...");
    let mut last_pos = explorer.position();
    let mut movement_count = 0;
    
    for frame in 1..=100 {
        explorer.update(&dungeon, &pathfinder, &mut rng);
        let current_pos = explorer.position();
        
        if current_pos != last_pos {
            movement_count += 1;
            println!("Frame {}: Explorer moved from {:?} to {:?}", frame, last_pos, current_pos);
            last_pos = current_pos;
        }
    }
    
    println!("\nMovement summary:");
    println!("  Total movements: {}", movement_count);
    println!("  Final position: {:?}", last_pos);
    
    if movement_count > 0 {
        println!("\n✅ SUCCESS: Explorer is moving!");
    } else {
        println!("\n❌ FAILURE: Explorer did not move!");
    }
}

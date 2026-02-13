/// Example: Generate and display a dungeon using the 2D6 rules engine
use dungeon_saver::{DungeonGenerator};

fn main() {
    let seed = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(42);

    println!("Generating dungeon with seed: {}", seed);
    println!("=====================================\n");

    let mut generator = DungeonGenerator::new(seed);
    let rooms = generator.generate();

    println!("Total rooms: {}\n", rooms.len());

    for (i, room) in rooms.iter().enumerate() {
        println!("Room #{} (ID: {})", i, room.id);
        println!("  Type: {:?}", room.room_type);
        println!("  Position: ({}, {})", room.x, room.y);
        println!("  Dimensions: {}x{} (area: {} squares)", 
            room.width, room.height, room.area());
        println!("  Exits: {}", room.exits.len());
        for (j, exit) in room.exits.iter().enumerate() {
            println!("    Exit {}: {:?} wall at position {}", j + 1, exit.wall, exit.position);
        }
        println!();
    }

    // Summary statistics
    let entrance_count = rooms.iter().filter(|r| matches!(r.room_type, dungeon_saver::dungeon::room::RoomType::Entrance)).count();
    let corridor_count = rooms.iter().filter(|r| matches!(r.room_type, dungeon_saver::dungeon::room::RoomType::Corridor)).count();
    let small_count = rooms.iter().filter(|r| matches!(r.room_type, dungeon_saver::dungeon::room::RoomType::Small)).count();
    let large_count = rooms.iter().filter(|r| matches!(r.room_type, dungeon_saver::dungeon::room::RoomType::Large)).count();
    let normal_count = rooms.iter().filter(|r| matches!(r.room_type, dungeon_saver::dungeon::room::RoomType::Normal)).count();

    println!("=====================================");
    println!("Summary:");
    println!("  Entrance rooms: {}", entrance_count);
    println!("  Normal rooms: {}", normal_count);
    println!("  Corridors: {}", corridor_count);
    println!("  Small rooms: {}", small_count);
    println!("  Large rooms: {}", large_count);
}

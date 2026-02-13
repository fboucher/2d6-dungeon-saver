use dungeon_saver::renderer::{SimpleRoom, MultiRoomRenderer};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Step 4: Explorer moving between two connected rooms");
    println!("Press Ctrl+C to exit");
    println!();
    
    // Define the two rooms (same as Step 3)
    let room1 = SimpleRoom::new_with_doors(); // 6×4 floor with doors (8×6 total)
    let room2 = SimpleRoom::new_large();       // 8×10 floor (10×12 total)
    
    // Hardcoded path: Room 1 center → east door → Room 2 center → back
    // Room 1 is at (0,0), dimensions 8×6
    // Room 2 is at (7,0), dimensions 10×12 (sharing wall at x=7)
    
    // Room 1 center is approximately at (3, 3) - middle of the 6×4 floor
    // East door of room 1 is at (7, 4)
    // Room 2 center is approximately at (12, 6) - middle of the 8×10 floor
    
    let path = vec![
        // Start in Room 1 center
        (3, 3),
        (4, 3),
        (5, 3),
        (6, 3),
        // Move to door level
        (6, 4),
        // Through the door
        (7, 4),
        // Into Room 2
        (8, 4),
        (9, 4),
        (10, 4),
        (11, 4),
        (12, 4),
        // Move down toward center of Room 2
        (12, 5),
        (12, 6),
        // Pause at center, then return
        (12, 5),
        (12, 4),
        (11, 4),
        (10, 4),
        (9, 4),
        (8, 4),
        // Back through door
        (7, 4),
        (6, 4),
        // Back to Room 1
        (6, 3),
        (5, 3),
        (4, 3),
        (3, 3),
    ];
    
    // Animation loop - one round trip
    for pos in path {
        // Clear screen (ANSI escape code)
        print!("\x1B[2J\x1B[1;1H");
        
        // Create renderer with rooms
        let mut renderer = MultiRoomRenderer::new();
        renderer.add_room(room1.clone(), 0, 0);
        renderer.add_room(room2.clone(), 7, 0);
        
        // Set explorer position
        renderer.set_explorer_pos(pos.0, pos.1);
        
        // Render and display
        println!("Step 4: Explorer moving between two connected rooms");
        println!();
        println!("{}", renderer.render());
        println!();
        println!("Legend:");
        println!("  # = wall");
        println!("  . = floor");
        println!("  - = north door");
        println!("  | = east door");
        println!("  @ = explorer");
        println!();
        println!("Explorer at position ({}, {})", pos.0, pos.1);
        
        // Flush output
        io::stdout().flush().unwrap();
        
        // Sleep for 100ms (10 FPS)
        thread::sleep(Duration::from_millis(100));
    }
    
    println!();
    println!("Animation complete - explorer made one round trip!");
}

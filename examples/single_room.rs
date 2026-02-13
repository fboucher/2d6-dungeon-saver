/// Simple demo: shows a single 4×5 room
use dungeon_saver::renderer::SimpleRoom;

fn main() {
    println!("=== Single Room Demo ===\n");
    
    let room = SimpleRoom::new();
    println!("Room: 4×5 floor\n");
    println!("{}", room.render());
    
    println!("\nLegend:");
    println!("  # = wall");
    println!("  . = floor");
}

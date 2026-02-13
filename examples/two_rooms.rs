use dungeon_saver::renderer::{SimpleRoom, MultiRoomRenderer};

fn main() {
    println!("Step 3: Two connected rooms via east door");
    println!();
    
    let mut renderer = MultiRoomRenderer::new();
    
    // Room 1: 6×4 floor with north and east doors
    let room1 = SimpleRoom::new_with_doors();
    println!("Room 1: 6×4 floor");
    
    // Room 2: 8×10 floor
    let room2 = SimpleRoom::new_large();
    println!("Room 2: 8×10 floor");
    println!();
    
    // Position room1 at origin
    renderer.add_room(room1, 0, 0);
    
    // Position room2 to connect at room1's east door
    // Room1 is 8 characters wide, room2's west wall overlaps at position 7
    // so room2 starts at x=7 to share the wall with the door
    renderer.add_room(room2, 7, 0);
    
    println!("{}", renderer.render());
    println!();
    println!("Legend:");
    println!("  # = wall");
    println!("  . = floor");
    println!("  - = north door");
    println!("  | = east door (connects the two rooms)");
}

use dungeon_saver::renderer::SimpleRoom;

fn main() {
    println!("Step 2: Room with doors on east and north walls");
    println!();
    
    let room = SimpleRoom::new_with_doors();
    println!("Room: 6×4 floor");
    println!();
    
    println!("{}", room.render());
    println!();
    println!("Legend:");
    println!("  # = wall");
    println!("  . = floor");
    println!("  - = north door");
    println!("  | = east door");
}

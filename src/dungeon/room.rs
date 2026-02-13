/// Room definition: position, dimensions, exits
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
    pub id: usize,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub room_type: RoomType,
    pub exits: Vec<Exit>,
    pub parent_id: Option<usize>,  // Which room this connects to
    pub parent_wall: Option<Wall>,  // Which wall of parent this connects through
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomType {
    Entrance,
    Normal,
    Small,
    Large,
    Corridor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exit {
    pub wall: Wall,
    pub position: u32,
    pub connected_room_id: Option<usize>,  // Track which room this exit leads to
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Wall {
    North,
    East,
    South,
    West,
}

impl Room {
    /// Calculate room area - used by tests and generation logic
    #[allow(dead_code)]
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Check if room is a corridor (1-width dimension)
    #[allow(dead_code)]
    pub fn is_corridor(&self) -> bool {
        self.width == 1 || self.height == 1
    }
}

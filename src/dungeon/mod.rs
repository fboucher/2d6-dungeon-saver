/// Dungeon generation following 2D6 pen & paper rules
/// Implements room generation, exit placement, and corridor handling
pub mod room;
pub mod generator;

pub use generator::DungeonGenerator;
pub use room::{Room, Wall, Exit, RoomType};

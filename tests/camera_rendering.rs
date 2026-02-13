/// Camera and rendering integration tests
/// Tests for Phase 3 acceptance criteria
use dungeon_saver::{DungeonGenerator, renderer::{Camera}};

#[test]
fn test_camera_centers_on_explorer_initial_position() {
    // Generate a fixed dungeon
    let mut generator = DungeonGenerator::new(42);
    let dungeon = generator.generate();
    
    // Get entrance room
    let entrance = &dungeon[0];
    let explorer_pos = (
        entrance.x + entrance.width / 2,
        entrance.y + entrance.height / 2,
    );
    
    // Create camera and center on explorer
    let mut camera = Camera::new(80, 24);
    camera.center_on(explorer_pos);
    
    // Verify explorer is visible
    assert!(camera.is_visible(explorer_pos), "Explorer should be visible");
    
    // 1:1 coordinate mapping now
    // Calculate where explorer appears on screen
    let rel_x = explorer_pos.0 as i32 - camera.x as i32;
    let rel_y = explorer_pos.1 as i32 - camera.y as i32;
    
    // Explorer should be roughly centered (accounting for saturating_sub at origin)
    // At minimum, explorer should be on screen
    assert!(rel_x >= 0 && rel_x < camera.width as i32);
    assert!(rel_y >= 0 && rel_y < camera.height as i32);
    
    // If not at origin, should be centered
    if explorer_pos.0 >= camera.width / 2 {
        assert!(rel_x >= camera.width as i32 / 4);
        assert!(rel_x <= camera.width as i32 * 3 / 4);
    }
    if explorer_pos.1 >= camera.height / 2 {
        assert!(rel_y >= camera.height as i32 / 4);
        assert!(rel_y <= camera.height as i32 * 3 / 4);
    }
}

#[test]
fn test_camera_pans_when_explorer_reaches_edge() {
    let mut camera = Camera::new(80, 24);
    
    // Center on initial position
    let initial_pos = (100, 50);
    camera.center_on(initial_pos);
    
    let original_cam_x = camera.x;
    let original_cam_y = camera.y;
    
    // Move explorer to right edge (1:1 mapping)
    // Camera 3/4 width is 60
    let edge_pos = (initial_pos.0 + 35, initial_pos.1);
    camera.update(edge_pos);
    
    // Camera should have panned right
    assert!(camera.x > original_cam_x, "Camera should pan right when explorer moves to right edge");
    
    // Explorer should still be visible
    assert!(camera.is_visible(edge_pos), "Explorer should remain visible after pan");
}

#[test]
fn test_camera_pans_when_explorer_reaches_bottom() {
    let mut camera = Camera::new(80, 24);
    
    // Center on initial position
    let initial_pos = (100, 50);
    camera.center_on(initial_pos);
    
    let original_cam_y = camera.y;
    
    // Move explorer to bottom edge (1:1 mapping)
    // Camera 3/4 height is 18
    let edge_pos = (initial_pos.0, initial_pos.1 + 15);
    camera.update(edge_pos);
    
    // Camera should have panned down
    assert!(camera.y > original_cam_y, "Camera should pan down when explorer moves to bottom edge");
    
    // Explorer should still be visible
    assert!(camera.is_visible(edge_pos), "Explorer should remain visible after pan");
}

#[test]
fn test_camera_stays_stable_when_explorer_in_middle() {
    let mut camera = Camera::new(80, 24);
    
    // Center on position
    camera.center_on((100, 50));
    
    let original_cam_x = camera.x;
    let original_cam_y = camera.y;
    
    // Small movements within middle area shouldn't pan
    camera.update((100, 50));
    camera.update((101, 50));
    camera.update((100, 51));
    camera.update((99, 50));
    camera.update((100, 49));
    
    // Camera should remain stable
    assert_eq!(camera.x, original_cam_x, "Camera should not pan for small movements in middle area");
    assert_eq!(camera.y, original_cam_y, "Camera should not pan for small movements in middle area");
}

#[test]
fn test_camera_handles_resize() {
    let mut camera = Camera::new(80, 24);
    
    // Resize to larger terminal
    camera.resize(120, 40);
    
    assert_eq!(camera.width, 120);
    assert_eq!(camera.height, 40);
    
    // Resize to smaller terminal
    camera.resize(60, 20);
    
    assert_eq!(camera.width, 60);
    assert_eq!(camera.height, 20);
}

#[test]
fn test_explorer_stays_visible_across_pans() {
    let mut camera = Camera::new(80, 24);
    
    // Simulate explorer movement across dungeon
    let positions = vec![
        (10, 10),
        (50, 50),
        (100, 100),
        (200, 150),
        (300, 200),
    ];
    
    for pos in positions {
        camera.center_on(pos);
        camera.update(pos);
        
        assert!(camera.is_visible(pos), 
            "Explorer at {:?} should remain visible after camera update", pos);
    }
}

#[test]
fn test_dungeon_generation_produces_valid_rooms_for_rendering() {
    let mut generator = DungeonGenerator::new(42);
    let dungeon = generator.generate();
    
    // Verify we have rooms to render
    assert!(!dungeon.is_empty(), "Dungeon should have at least one room");
    assert_eq!(dungeon.len(), 20, "Should generate 20 rooms");
    
    // Verify entrance room is first
    assert_eq!(dungeon[0].room_type, dungeon_saver::dungeon::room::RoomType::Entrance);
    
    // All rooms should have valid dimensions
    for room in &dungeon {
        assert!(room.width > 0, "Room width must be > 0");
        assert!(room.height > 0, "Room height must be > 0");
    }
}

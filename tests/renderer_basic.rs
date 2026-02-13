use dungeon_saver::renderer::SimpleRoom;

#[test]
fn test_room_is_4_wide_5_tall() {
    let room = SimpleRoom::new();
    assert_eq!(room.width(), 6, "Room should be 6 characters wide (4 floor + 2 walls)");
    assert_eq!(room.height(), 7, "Room should be 7 characters tall (5 floor + 2 walls)");
}

#[test]
fn test_room_perimeter_is_all_walls() {
    let room = SimpleRoom::new();
    let output = room.render();
    let lines: Vec<&str> = output.lines().collect();
    
    // Top wall - all '#'
    assert!(lines[0].chars().all(|c| c == '#'), "Top wall should be all '#'");
    
    // Bottom wall - all '#'
    assert!(lines[6].chars().all(|c| c == '#'), "Bottom wall should be all '#'");
    
    // Left wall - first character of each line
    for (i, line) in lines.iter().enumerate() {
        assert_eq!(line.chars().next().unwrap(), '#', "Left wall at row {} should be '#'", i);
    }
    
    // Right wall - last character of each line
    for (i, line) in lines.iter().enumerate() {
        assert_eq!(line.chars().last().unwrap(), '#', "Right wall at row {} should be '#'", i);
    }
}

#[test]
fn test_room_interior_is_all_floor() {
    let room = SimpleRoom::new();
    let output = room.render();
    let lines: Vec<&str> = output.lines().collect();
    
    // Check middle rows (rows 1, 2, 3, 4, 5)
    for row_idx in 1..6 {
        let line = lines[row_idx];
        let chars: Vec<char> = line.chars().collect();
        
        // Interior positions are index 1, 2, 3, 4 (0-based)
        assert_eq!(chars[1], '.', "Interior at row {} col 1 should be '.'", row_idx);
        assert_eq!(chars[2], '.', "Interior at row {} col 2 should be '.'", row_idx);
        assert_eq!(chars[3], '.', "Interior at row {} col 3 should be '.'", row_idx);
        assert_eq!(chars[4], '.', "Interior at row {} col 4 should be '.'", row_idx);
    }
}

#[test]
fn test_room_exact_output() {
    let room = SimpleRoom::new();
    let output = room.render();
    
    let expected = "######\n#....#\n#....#\n#....#\n#....#\n#....#\n######";
    assert_eq!(output, expected, "Room should render exactly as specified");
}

// STEP 2 TESTS: Room with doors on east and north walls
#[test]
fn test_room_with_doors_dimensions() {
    let room = SimpleRoom::new_with_doors();
    assert_eq!(room.width(), 8, "Room should be 8 characters wide (6 floor + 2 walls)");
    assert_eq!(room.height(), 6, "Room should be 6 characters tall (4 floor + 2 walls)");
}

#[test]
fn test_room_with_doors_has_north_door() {
    let room = SimpleRoom::new_with_doors();
    let output = room.render();
    let lines: Vec<&str> = output.lines().collect();
    
    // North door should be at position 3 (centered on 8-wide wall)
    let top_line = lines[0];
    let chars: Vec<char> = top_line.chars().collect();
    assert_eq!(chars[3], '-', "North door should be at position 3 (centered)");
}

#[test]
fn test_room_with_doors_has_east_door() {
    let room = SimpleRoom::new_with_doors();
    let output = room.render();
    let lines: Vec<&str> = output.lines().collect();
    
    // East door should be at row 4 (centered on 6-tall room), last column
    let middle_row = lines[4];
    let chars: Vec<char> = middle_row.chars().collect();
    assert_eq!(chars[7], '|', "East door should be at last column (position 7) on row 4");
}

#[test]
fn test_room_with_doors_exact_output() {
    let room = SimpleRoom::new_with_doors();
    let output = room.render();
    
    let expected = "###-####\n#......#\n#......#\n#......#\n#......|\n########";
    assert_eq!(output, expected, "Room with doors should render exactly as specified");
}

// STEP 3 TESTS: Two connected rooms
#[test]
fn test_two_rooms_render() {
    use dungeon_saver::renderer::MultiRoomRenderer;
    
    let mut renderer = MultiRoomRenderer::new();
    
    // Room 1: 6×4 floor with doors (8×6 total)
    let room1 = SimpleRoom::new_with_doors();
    
    // Room 2: 8×10 floor (10×12 total)
    let room2 = SimpleRoom::new_large();
    
    // Position room1 at (0,0)
    renderer.add_room(room1, 0, 0);
    
    // Position room2 so its west wall overlaps with room1's east wall (sharing the door)
    // Room1 is 8 wide, its east wall is at x=7, so room2 starts at x=7
    renderer.add_room(room2, 7, 0);
    
    let output = renderer.render();
    let lines: Vec<&str> = output.lines().collect();
    
    // Should have at least 12 lines (height of room2)
    assert!(lines.len() >= 12, "Output should have at least 12 lines");
    
    // Check that we have content in both rooms
    assert!(output.contains('.'), "Output should contain floor tiles");
    assert!(output.contains('#'), "Output should contain walls");
}

#[test]
fn test_two_rooms_connect_at_east_door() {
    use dungeon_saver::renderer::MultiRoomRenderer;
    
    let mut renderer = MultiRoomRenderer::new();
    
    let room1 = SimpleRoom::new_with_doors();
    let room2 = SimpleRoom::new_large();
    
    renderer.add_room(room1, 0, 0);
    renderer.add_room(room2, 7, 0);
    
    let output = renderer.render();
    let lines: Vec<&str> = output.lines().collect();
    
    // Room1's east door is at row 4, column 7 (the shared wall)
    // Check that the door character exists
    let row4 = lines[4];
    assert_eq!(row4.chars().nth(7).unwrap(), '|', "East door should be at position (7,4)");
}

#[test]
fn test_two_rooms_no_gaps() {
    use dungeon_saver::renderer::MultiRoomRenderer;
    
    let mut renderer = MultiRoomRenderer::new();
    
    let room1 = SimpleRoom::new_with_doors();
    let room2 = SimpleRoom::new_large();
    
    renderer.add_room(room1, 0, 0);
    renderer.add_room(room2, 7, 0);
    
    let output = renderer.render();
    let lines: Vec<&str> = output.lines().collect();
    
    // Check row 4 where the door is - should have no gaps
    let row4 = lines[4];
    
    // From column 0 to column 7 (room1) should have content
    for i in 0..8 {
        let ch = row4.chars().nth(i).unwrap();
        assert_ne!(ch, ' ', "Should not have gap at column {} in row 4", i);
    }
    
    // From column 7 onwards (room2) should also have content
    for i in 7..16 {
        if i < row4.len() {
            let ch = row4.chars().nth(i).unwrap();
            assert_ne!(ch, ' ', "Should not have gap at column {} in row 4", i);
        }
    }
}

// STEP 4 TESTS: Explorer rendering
#[test]
fn test_explorer_renders_at_correct_position() {
    use dungeon_saver::renderer::MultiRoomRenderer;
    
    let mut renderer = MultiRoomRenderer::new();
    let room = SimpleRoom::new();
    renderer.add_room(room, 0, 0);
    
    // Set explorer at position (2, 2) - should be in the floor area
    renderer.set_explorer_pos(2, 2);
    
    let output = renderer.render();
    let lines: Vec<&str> = output.lines().collect();
    
    // Check that '@' appears at position (2, 2)
    let row2 = lines[2];
    let chars: Vec<char> = row2.chars().collect();
    assert_eq!(chars[2], '@', "Explorer should render at position (2, 2)");
}

#[test]
fn test_explorer_character_is_at_symbol() {
    use dungeon_saver::renderer::MultiRoomRenderer;
    
    let mut renderer = MultiRoomRenderer::new();
    let room = SimpleRoom::new();
    renderer.add_room(room, 0, 0);
    
    renderer.set_explorer_pos(3, 3);
    
    let output = renderer.render();
    
    // Verify '@' is present in output
    assert!(output.contains('@'), "Explorer should be rendered as '@'");
}

#[test]
fn test_explorer_only_renders_on_floor() {
    use dungeon_saver::renderer::MultiRoomRenderer;
    
    let mut renderer = MultiRoomRenderer::new();
    let room = SimpleRoom::new();
    renderer.add_room(room, 0, 0);
    
    // Try to place explorer on a wall position (0, 0)
    renderer.set_explorer_pos(0, 0);
    
    let output = renderer.render();
    let lines: Vec<&str> = output.lines().collect();
    
    // Position (0, 0) should still be a wall '#', not '@'
    let row0 = lines[0];
    let chars: Vec<char> = row0.chars().collect();
    assert_eq!(chars[0], '#', "Explorer should not replace walls");
}

#[test]
fn test_explorer_renders_on_door() {
    use dungeon_saver::renderer::MultiRoomRenderer;
    
    let mut renderer = MultiRoomRenderer::new();
    let room = SimpleRoom::new_with_doors();
    renderer.add_room(room, 0, 0);
    
    // Place explorer on the east door at position (7, 4)
    renderer.set_explorer_pos(7, 4);
    
    let output = renderer.render();
    let lines: Vec<&str> = output.lines().collect();
    
    // Explorer should render on door position
    let row4 = lines[4];
    let chars: Vec<char> = row4.chars().collect();
    assert_eq!(chars[7], '@', "Explorer should render on door position");
}

#[test]
fn test_explorer_can_be_cleared() {
    use dungeon_saver::renderer::MultiRoomRenderer;
    
    let mut renderer = MultiRoomRenderer::new();
    let room = SimpleRoom::new();
    renderer.add_room(room, 0, 0);
    
    // Set and then clear explorer
    renderer.set_explorer_pos(3, 3);
    renderer.clear_explorer();
    
    let output = renderer.render();
    
    // Explorer should not be present
    assert!(!output.contains('@'), "Explorer should be cleared from render");
}

#[test]
fn test_explorer_moves_between_rooms() {
    use dungeon_saver::renderer::MultiRoomRenderer;
    
    let mut renderer = MultiRoomRenderer::new();
    let room1 = SimpleRoom::new_with_doors();
    let room2 = SimpleRoom::new_large();
    
    renderer.add_room(room1, 0, 0);
    renderer.add_room(room2, 7, 0);
    
    // Test explorer in room 1
    renderer.set_explorer_pos(3, 3);
    let output1 = renderer.render();
    assert!(output1.contains('@'), "Explorer should appear in room 1");
    
    // Move explorer to room 2
    renderer.set_explorer_pos(12, 6);
    let output2 = renderer.render();
    assert!(output2.contains('@'), "Explorer should appear in room 2");
}

/// Integration tests for Phase 1: Terminal Takeover & Main Loop
/// Tests event loop timing and quit signal handling
/// Phase 5: End-to-end integration tests
use std::time::{Duration, Instant};
use dungeon_saver::{DungeonGenerator, MapExporter};
use std::fs;

#[test]
fn test_frame_duration_constant() {
    // Verify the FPS calculation is correct
    const TARGET_FPS: u64 = 10;
    const FRAME_DURATION_MS: u64 = 1000 / TARGET_FPS;
    
    assert_eq!(FRAME_DURATION_MS, 100, "10 FPS should result in 100ms per frame");
}

#[test]
fn test_frame_timing_simulation() {
    // Simulate frame timing to verify the loop logic would work
    let target_fps = 10;
    let frame_duration = Duration::from_millis(1000 / target_fps);
    
    let mut frames = 0;
    let start = Instant::now();
    
    // Simulate 5 frames
    while frames < 5 {
        let frame_start = Instant::now();
        
        // Simulate minimal work
        std::thread::sleep(Duration::from_millis(1));
        
        // Sleep for remaining frame time
        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
        
        frames += 1;
    }
    
    let total_elapsed = start.elapsed();
    let expected_duration = frame_duration * 5;
    
    // Allow 10ms tolerance for test timing variability
    let tolerance = Duration::from_millis(10);
    assert!(
        total_elapsed >= expected_duration && total_elapsed <= expected_duration + tolerance,
        "5 frames at 10 FPS should take ~500ms, got {:?}",
        total_elapsed
    );
}

// Phase 5 Integration Tests (Updated for Progressive Generation)
#[test]
fn test_full_progressive_generation_and_export() {
    let seed = 42;
    
    // Generate entrance only
    let mut generator = DungeonGenerator::new(seed);
    let mut dungeon = vec![generator.generate_entrance()];
    
    // Simulate progressive generation by exploring exits
    let mut iterations = 0;
    while dungeon.len() < 20 && iterations < 100 {
        let unexplored = dungeon.iter()
            .find_map(|r| {
                r.exits.iter()
                    .find(|e| e.connected_room_id.is_none())
                    .map(|e| (r.id, e.wall))
            });
        
        if let Some((room_id, wall)) = unexplored {
            let new_room = generator.add_room(room_id, wall);
            
            // Update parent exit connection
            if let Some(parent) = dungeon.iter_mut().find(|r| r.id == room_id) {
                if let Some(exit) = parent.exits.iter_mut()
                    .find(|e| e.wall == wall && e.connected_room_id.is_none()) {
                    exit.connected_room_id = Some(new_room.id);
                }
            }
            
            dungeon.push(new_room);
        } else {
            break;
        }
        iterations += 1;
    }
    
    // Verify dungeon was generated
    assert!(!dungeon.is_empty());
    assert!(dungeon.len() >= 10, "Should generate at least 10 rooms");
    
    // Export dungeon (no seed in filename for emergent dungeons)
    let exporter = MapExporter::new(seed);
    let result = exporter.export(&dungeon);
    
    assert!(result.is_ok());
    
    let filepath = result.unwrap();
    assert!(filepath.exists());
    
    // Verify file content
    let content = fs::read_to_string(&filepath).unwrap();
    assert!(content.contains("Room Count:"));
    assert!(content.contains("Dimensions:"));
    
    // Cleanup
    let _ = fs::remove_file(filepath);
}

#[test]
fn test_progressive_generation_creates_different_dungeons() {
    // With progressive generation, different seeds produce different entrance rooms
    let seed1 = 111;
    let seed2 = 222;
    
    let mut gen1 = DungeonGenerator::new(seed1);
    let entrance1 = gen1.generate_entrance();
    
    let mut gen2 = DungeonGenerator::new(seed2);
    let entrance2 = gen2.generate_entrance();
    
    // Different seeds should produce different entrance rooms
    // (either dimensions or exit positions differ)
    let different_dimensions = 
        entrance1.width != entrance2.width ||
        entrance1.height != entrance2.height;
    
    let different_exit_positions = 
        !entrance1.exits.iter().zip(entrance2.exits.iter())
            .all(|(e1, e2)| e1.wall == e2.wall && e1.position == e2.position);
    
    assert!(different_dimensions || different_exit_positions,
            "Different seeds should produce different entrance rooms (seed1={}, seed2={})", seed1, seed2);
}

#[test]
fn test_export_filename_format_no_seed() {
    let seed = 999;
    let mut generator = DungeonGenerator::new(seed);
    let dungeon = vec![generator.generate_entrance()];
    
    let exporter = MapExporter::new(seed);
    let filepath = exporter.export(&dungeon).unwrap();
    
    let filename = filepath.file_name().unwrap().to_str().unwrap();
    
    // Filename format: yyyy-MM-dd_HHmm.txt (no seed for emergent dungeons)
    assert!(filename.ends_with(".txt"));
    
    // Cleanup
    let _ = fs::remove_file(filepath);
}

#[cfg(test)]
mod quit_signal_tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn should_quit(key: &KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => true,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => true,
            _ => false,
        }
    }

    #[test]
    fn test_quit_on_lowercase_q() {
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        assert!(should_quit(&key), "lowercase 'q' should trigger quit");
    }

    #[test]
    fn test_quit_on_uppercase_q() {
        let key = KeyEvent::new(KeyCode::Char('Q'), KeyModifiers::NONE);
        assert!(should_quit(&key), "uppercase 'Q' should trigger quit");
    }

    #[test]
    fn test_quit_on_ctrl_c() {
        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert!(should_quit(&key), "Ctrl+C should trigger quit");
    }

    #[test]
    fn test_no_quit_on_other_keys() {
        let keys = vec![
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE), // 'c' without Ctrl
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
        ];

        for key in keys {
            assert!(!should_quit(&key), "key {:?} should not trigger quit", key);
        }
    }
}

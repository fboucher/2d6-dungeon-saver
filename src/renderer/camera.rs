/// Camera: viewport management and panning
/// Keeps explorer roughly centered on screen, follows movement
/// 1:1 coordinate mapping: dungeon space = screen space
#[derive(Debug, Clone)]
pub struct Camera {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            x: 0,
            y: 0,
            width,
            height,
        }
    }
    
    /// Update camera to keep explorer roughly centered
    /// Pan logic: when explorer reaches last quarter of screen, shift viewport
    pub fn update(&mut self, explorer_pos: (u32, u32)) {
        // Calculate explorer position relative to viewport
        let rel_x = explorer_pos.0 as i32 - self.x as i32;
        let rel_y = explorer_pos.1 as i32 - self.y as i32;
        
        // Define the "comfort zone" - middle 50% of screen
        let quarter_width = self.width / 4;
        let quarter_height = self.height / 4;
        let three_quarter_width = (self.width * 3) / 4;
        let three_quarter_height = (self.height * 3) / 4;
        
        // Pan horizontally if explorer is in the outer quarters
        if rel_x < quarter_width as i32 {
            // Explorer too far left, pan left
            self.x = explorer_pos.0.saturating_sub(self.width / 2);
        } else if rel_x > three_quarter_width as i32 {
            // Explorer too far right, pan right
            self.x = explorer_pos.0.saturating_sub(self.width / 2);
        }
        
        // Pan vertically if explorer is in the outer quarters
        if rel_y < quarter_height as i32 {
            // Explorer too far up, pan up
            self.y = explorer_pos.1.saturating_sub(self.height / 2);
        } else if rel_y > three_quarter_height as i32 {
            // Explorer too far down, pan down
            self.y = explorer_pos.1.saturating_sub(self.height / 2);
        }
    }
    
    /// Center camera on a specific position (used for initial positioning)
    pub fn center_on(&mut self, pos: (u32, u32)) {
        self.x = pos.0.saturating_sub(self.width / 2);
        self.y = pos.1.saturating_sub(self.height / 2);
    }
    
    /// Update camera dimensions on terminal resize
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
    }
    
    /// Check if a position is within the viewport
    #[allow(dead_code)]
    pub fn is_visible(&self, pos: (u32, u32)) -> bool {
        pos.0 >= self.x && pos.0 < self.x + self.width
            && pos.1 >= self.y && pos.1 < self.y + self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_camera_initialization() {
        let camera = Camera::new(80, 24);
        assert_eq!(camera.x, 0);
        assert_eq!(camera.y, 0);
        assert_eq!(camera.width, 80);
        assert_eq!(camera.height, 24);
    }
    
    #[test]
    fn test_center_on_position() {
        let mut camera = Camera::new(80, 24);
        camera.center_on((100, 50));
        
        // Camera should be centered on (100, 50) - 1:1 mapping
        // Camera x = 100 - 40 = 60, y = 50 - 12 = 38
        assert_eq!(camera.x, 60); 
        assert_eq!(camera.y, 38);
    }
    
    #[test]
    fn test_panning_when_explorer_moves_right() {
        let mut camera = Camera::new(80, 24);
        camera.center_on((40, 12));
        
        let orig_x = camera.x;
        
        // Explorer moves far right to exceed 3/4 of screen
        camera.update((80, 12));
        
        // Camera should have panned right
        assert!(camera.x > orig_x);
    }
    
    #[test]
    fn test_panning_when_explorer_moves_down() {
        let mut camera = Camera::new(80, 24);
        camera.center_on((40, 12));
        
        let orig_y = camera.y;
        
        // Explorer moves down - should trigger pan
        camera.update((40, 30));
        
        // Camera should have panned down
        assert!(camera.y > orig_y);
    }
    
    #[test]
    fn test_no_pan_when_explorer_in_middle() {
        let mut camera = Camera::new(80, 24);
        camera.center_on((100, 50));
        
        let orig_x = camera.x;
        let orig_y = camera.y;
        
        // Explorer stays in middle - no pan should occur
        camera.update((100, 50));
        
        assert_eq!(camera.x, orig_x);
        assert_eq!(camera.y, orig_y);
    }
    
    #[test]
    fn test_resize_updates_dimensions() {
        let mut camera = Camera::new(80, 24);
        camera.resize(120, 40);
        
        assert_eq!(camera.width, 120);
        assert_eq!(camera.height, 40);
    }
    
    #[test]
    fn test_is_visible() {
        let mut camera = Camera::new(80, 24);
        camera.x = 0;
        camera.y = 0;
        
        // 1:1 mapping now
        assert!(camera.is_visible((10, 10)));
        assert!(camera.is_visible((0, 0)));
        assert!(camera.is_visible((79, 23)));
        assert!(!camera.is_visible((80, 0)));
        assert!(!camera.is_visible((0, 24)));
        assert!(!camera.is_visible((100, 100)));
    }
    
    #[test]
    fn test_explorer_stays_in_viewport_during_pan() {
        let mut camera = Camera::new(80, 24);
        let explorer_pos = (200, 100);
        
        camera.center_on(explorer_pos);
        camera.update(explorer_pos);
        
        // Explorer should be visible after centering
        assert!(camera.is_visible(explorer_pos));
    }
}

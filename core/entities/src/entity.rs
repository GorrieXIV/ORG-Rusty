pub struct Entity {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    angle: f32,
    collision_detected: bool,
}

impl Entity {
    fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn move_to(&mut self) {
    
    }
}

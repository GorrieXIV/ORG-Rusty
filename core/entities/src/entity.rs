use math::vector::Vector2D;

pub struct Entity {
    position: Vector2D,
    width: f32,
    height: f32,
    angle: f32,
    collision_detected: bool,
}

impl Entity {
    fn set_position(&mut self, x: f32, y: f32) {
        self.position = Vector2D{x: x, y: y};
    }
}

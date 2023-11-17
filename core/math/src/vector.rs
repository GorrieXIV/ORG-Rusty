use std::ops;

pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl ops::Add for Vector2D {
    type Output = Vector2D;

    fn add(self, _rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl Vector2D {
    pub fn to_string(self) -> String {
        return format!("({}, {})", self.x, self.y);
    }
}

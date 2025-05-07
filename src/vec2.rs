pub struct Vec2 {
    x: f32,
    y: f32
}

impl Vec2 {
    pub fn new(a: f32, b: f32) -> Self {
        Vec2 { x: a, y: b }
    }

    pub fn mag(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
}

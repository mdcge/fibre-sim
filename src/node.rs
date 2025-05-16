use crate::vec2::Vec2;

#[derive(Clone, Debug)]
pub struct Node {
    pub r: Vec2,
    pub v: Vec2,
    pub m: f32,
}

impl Node {
    pub fn new(pos: Vec2, vel: Vec2, mass: f32) -> Node {
        Node { r: pos, v: vel, m: mass }
    }

    pub fn update(&mut self, k_force: Vec2, g_force: Vec2, c_force: Vec2, dt: f32) {
        let total_force = k_force + g_force + c_force;
        let acc = total_force / self.m;
        self.v += acc*dt;
        self.r += self.v*dt;
    }
}

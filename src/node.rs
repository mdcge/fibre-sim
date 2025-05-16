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
}

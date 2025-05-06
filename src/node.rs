pub struct Node {
    pub r: Vec<f32>,
    v: Vec<f32>,
    pub m: f32,
}

impl Node {
    pub fn new(pos: Vec<f32>, vel: Vec<f32>, mass: f32) -> Node {
        Node { r: pos, v: vel, m: mass }
    }
}

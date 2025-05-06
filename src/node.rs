pub struct Node {
    r: Vec<f32>,
    m: f32,
}

impl Node {
    pub fn new(pos: Vec<f32>, mass: f32) -> Node {
        Node { r: pos, m: mass }
    }
}

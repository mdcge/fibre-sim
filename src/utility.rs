use crate::node::Node;
use crate::vec2::Vec2;

pub fn spring_force(node1: &Node, node2: &Node, k: f32, x0: f32) -> Vec2 {
    let vec = node2.r - node1.r;
    let dist = vec.mag();
    k * (dist - x0) * vec
}

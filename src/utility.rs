use crate::node::Node;

pub fn spring_force_mag(node1: &Node, node2: &Node, k: f32, x0: f32) -> f32 {
    let vec = node1.r - node2.r;
    let dist = vec.mag();
    k * (dist - x0)
}

pub fn gravity_force_mag(node: &Node, g: f32) -> f32 {
    node.m * g
}

pub fn damping_force_mag(node: &Node, c: f32) -> f32 {
    -c * node.v.mag()
}

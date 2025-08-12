use crate::node::Node;
use crate::vec2::Vec2;
use crate::state::State;

pub fn spring_force(node1: &Node, node2: &Node, k: f32, x0: f32) -> Vec2 {
    let vec = node2.r - node1.r;
    let dist = vec.mag();
    k * (dist - x0) * vec.unit()
}

pub fn force_is_stable(simulation_state: &State, epsilon: f32) -> bool {
    let max_force = simulation_state.forces.iter().map(|f| f.mag()).fold(0.0, f32::max);
    max_force < epsilon
}

pub fn velocity_is_stable(simulation_state: &State, epsilon: f32) -> bool {
    let max_vel = simulation_state.nodes.iter().map(|n| n.v.mag()).fold(0.0, f32::max);
    println!("Max vel: {max_vel}");
    max_vel < epsilon
}

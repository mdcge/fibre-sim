use rayon::prelude::*;

use crate::vec2::Vec2;
use crate::node::Node;
use crate::utility::spring_force;

pub struct State {
    pub nodes: Vec<Node>,
    pub forces: Vec<Vec2>,
}

impl State {
    pub fn new(node_list: Vec<Node>, forces_list: Vec<Vec2>) -> Self {
        State { nodes: node_list, forces: forces_list }
    }

    pub fn step(&mut self, k: f32, x0: f32, g: f32, c: f32, dt: f32) {
        let length = self.forces.len();
        self.forces = vec![Vec2::new(0.0, 0.0); length];

        // Spring force
        for i in 1..length-1 {
            let pre_spring_force = spring_force(&self.nodes[i], &self.nodes[i-1], k, x0);
            let post_spring_force = spring_force(&self.nodes[i], &self.nodes[i+1], k, x0);
            self.forces[i] += pre_spring_force + post_spring_force;
        }

        self.forces[1..length-1].iter_mut()
                                .zip(&mut self.nodes[1..length-1])
                                .for_each(|(force, node)| {
                                    // Gravity and damping
                                    let gravity_force = node.m * Vec2::new(0.0, -g);
                                    let damping_force = -c * node.v;
                                    *force += gravity_force + damping_force;
                                    // Integration
                                    let acc = *force / node.m;
                                    let vel = node.v + acc*dt;
                                    let pos = node.r + vel*dt;
                                    node.r = pos;
                                    node.v = vel;
                                });
    }
}

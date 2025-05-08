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
        self.forces = vec![Vec2::new(0.0, 0.0); self.forces.len()];
        let mut new_forces = self.forces.clone(); // seems redundant, but necessary for parallelization

        // Spring force
        for i in 0..self.nodes.len()-1 {
            let spring_force = spring_force(&self.nodes[i], &self.nodes[i+1], k, x0);
            new_forces[i] += spring_force;
            new_forces[i+1] -= spring_force;
        }
        self.forces = new_forces;

        // Gravity and damping
        for i in 0..self.nodes.len() {
            let gravity_force = self.nodes[i].m * Vec2::new(0.0, -g);
            let damping_force = -c * self.nodes[i].v;
            self.forces[i] += gravity_force + damping_force;
        }

        // Integration
        //for (node, force) in self.nodes.iter_mut().zip(&self.forces) {
        for i in 1..self.nodes.len()-1 {
            let acc = self.forces[i] / self.nodes[i].m;
            let vel = self.nodes[i].v + acc*dt;
            let pos = self.nodes[i].r + vel*dt;
            self.nodes[i].r = pos;
            self.nodes[i].v = vel;
        }
    }
}

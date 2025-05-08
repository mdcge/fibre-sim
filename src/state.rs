use crate::vec2::Vec2;
use crate::node::Node;
use crate::utility::{spring_force_mag, gravity_force_mag};

pub struct State {
    nodes: Vec<Node>,  // length: n
    spring_forces_mag: Vec<f32>,  // length: n-1
    gravity_forces_mag: Vec<f32>,  // length: n
    damping_forces_mag: Vec<f32>  // length: n
}

impl State {
    pub fn new(node_list: Vec<Node>, k_mag_list: Vec<f32>, g_mag_list: Vec<f32>, c_mag_list: Vec<f32>) -> Self {
        State { nodes: node_list, spring_forces_mag: k_mag_list, gravity_forces_mag: g_mag_list, damping_forces_mag: c_mag_list }
    }

    pub fn update(&mut self, k: f32, x0: f32, g: f32, c: f32, dt: f32) {
        self.update_spring_forces(k, x0);
        self.update_gravity_forces(g);
        self.update_damping_forces(c);
        for i in 1..self.nodes.len()-1 {
            let spring_force = self.spring_forces_mag[i-1] * (self.nodes[i-1].r-self.nodes[i].r).unit() + self.spring_forces_mag[i] * (self.nodes[i+1].r-self.nodes[i].r).unit();
            let gravity_force = self.gravity_forces_mag[i] * Vec2::new(0.0, -1.0);
            let damping_force = self.damping_forces_mag[i] * self.nodes[i].v.unit();
            self.nodes[i].update(spring_force, gravity_force, damping_force, dt);
        }
    }

    fn update_spring_forces(&mut self, k: f32, x0: f32) {
        for i in 0..self.nodes.len()-1 {
            self.spring_forces_mag[i] = spring_force_mag(&self.nodes[i], &self.nodes[i+1], k, x0);
        }
    }

    fn update_gravity_forces(&mut self, g: f32) {
        for i in 0..self.nodes.len() {
            self.gravity_forces_mag[i] = gravity_force_mag(&self.nodes[i], g);
        }
    }

    fn update_damping_forces(&mut self, c: f32) {
        for i in 0..self.nodes.len() {
            self.gravity_forces_mag[i] = gravity_force_mag(&self.nodes[i], c);
        }
    }
}

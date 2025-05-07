use crate::vec2::Vec2;

#[derive(Clone, Debug)]
pub struct Node {
    pub r: Vec2,
    v: Vec2,
    pub m: f32,
}

impl Node {
    pub fn new(pos: Vec2, vel: Vec2, mass: f32) -> Node {
        Node { r: pos, v: vel, m: mass }
    }

    pub fn updated_node(&self, pre_node: &Node, post_node: &Node, g: f32, k: f32, x0: f32, c: f32, dt: f32) -> Node {
        let force = self.force_on_node(pre_node, post_node, g, k, x0, c);
        let acc = force / self.m;
        let vel = self.v + acc*dt;
        let pos = self.r + vel*dt;
        Node { r: pos, v: vel, m: self.m }
    }

    pub fn force_on_node(&self, pre_node: &Node, post_node: &Node, g: f32, k: f32, x0: f32, c: f32) -> Vec2 {
        let grav_force = self.gravity_force(g);
        let spr_force = self.spring_force(pre_node, post_node, k, x0);
        let damp_force = self.damping_force(c);
        grav_force + spr_force + damp_force
    }

    fn gravity_force(&self, g: f32) -> Vec2 {
        Vec2 { x: 0.0, y: -self.m*g }
    }

    fn spring_force(&self, pre_node: &Node, post_node: &Node, k: f32, x0: f32) -> Vec2 {
        let pre_force = self.single_spring_force(pre_node, k, x0);
        let post_force = self.single_spring_force(post_node, k, x0);
        pre_force + post_force
    }

    fn damping_force(&self, c: f32) -> Vec2 {
        -c * self.v
    }

    fn single_spring_force(&self, other_node: &Node, k: f32, x0: f32) -> Vec2 {
        let pos_vec = other_node.r - self.r;
        let dist = pos_vec.mag();
        let dir = pos_vec.unit();
        dir * k * (dist-x0)
    }
}

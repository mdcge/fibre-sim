pub struct Node {
    pub r: Vec<f32>,
    v: Vec<f32>,
    pub m: f32,
}

impl Node {
    pub fn new(pos: Vec<f32>, vel: Vec<f32>, mass: f32) -> Node {
        Node { r: pos, v: vel, m: mass }
    }

    pub fn force_on_node(&self, pre_node: Node, post_node: Node, g: f32, k: f32, x0: f32) -> Vec<f32> {
        let grav_force = self.gravity_force(g);
        let spr_force = self.spring_force(pre_node, post_node, k, x0);
        vec![grav_force[0] + spr_force[0], grav_force[1] + spr_force[1]]
    }

    pub fn gravity_force(&self, g: f32) -> Vec<f32> {
        vec![0.0, self.m*g]
    }

    pub fn spring_force(&self, pre_node: Node, post_node: Node, k: f32, x0: f32) -> Vec<f32> {
        let pre_force = self.single_spring_force(pre_node, k, x0);
        let post_force = self.single_spring_force(post_node, k, x0);
        vec![pre_force[0]+post_force[0], pre_force[1]+post_force[1]]
    }

    fn single_spring_force(&self, other_node: Node, k: f32, x0: f32) -> Vec<f32> {
        let vec = vec![other_node.r[0]-self.r[0], other_node.r[1]-self.r[1]];
        let dist = (vec[0].powf(2.0) + vec[1].powf(2.0)).sqrt();
        let dir = match dist {
            0.0 => vec![0.0, 0.0],
            _ => vec![vec[0]/dist, vec[1]/dist],
        };
        vec![dir[0]*k*(dist-x0), dir[1]*k*(dist-x0)]
    }
}

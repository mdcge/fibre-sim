use std::time::Instant;
use std_dev::standard_deviation;

mod vec2;
use vec2::Vec2;

mod node;
use node::Node;

mod state;
use state::State;

mod utility;

fn main() {
    let mut simulation = Simulation::new_straight(vec![-2.0, 2.0], 5000.0, 9.81, 0.0001, 4.0, 0.000005, 0.003, 2000);

    let total_steps = 1000000;
    let mut sag_history = vec![0.0; 20];
    let std_dev_limit = 0.01;

    let before = Instant::now();
    for i in 0..total_steps {
        simulation.step();
        if (i+1) % 1000 == 0 {
            sag_history.rotate_right(1);
            sag_history[0] = simulation.get_lowest_point() * 1000.0;
            let std_dev = standard_deviation(&sag_history).standard_deviation;
            if std_dev < std_dev_limit {
                println!("Standard deviation of {std_dev} dropped below {std_dev_limit}");
                break
            }
        }
        if (i+1) % 10000 == 0 {
            println!("Step {}/{total_steps}", i+1);
        }
    }
    let time = before.elapsed();

    let low_point = sag_history[0];
    for node in simulation.simstate.nodes {
        println!("{:?}", node.r);
    }
    println!("Sag: {low_point} mm");
    println!("Computation time: {:?}", time);
}

struct Simulation {
    // Simulation parameters
    simstate: State,  // simulation state
    k: f32,  // total spring constant (N/m)
    x0: f32,  // rest length of springs (m)
    g: f32,  // gravity (m/s^2)
    c: f32,  // damping coefficient
    L0: f32,  // rest length of fibre (m)
    dt: f32,  // time step size (s)
    n: usize,  // number of subdivisions (1 segment = 1 subdivision)
}

// Initialize
impl Simulation {
    fn new_straight(x_endpoints: Vec<f32>, spring_constant: f32, grav: f32, damping: f32, fibre_length: f32, timestep: f32, mass: f32, subd: usize) -> Simulation {
        let mut node_list = vec![];
        for i in 0..subd+1 {
            let x_pos = x_endpoints[0] + i as f32 * (x_endpoints[1] - x_endpoints[0]) / subd as f32;
            node_list.push(Node::new(Vec2 { x: x_pos, y: 0.0 }, Vec2 { x: 0.0, y: 0.0 }, mass/(subd as f32 + 1.0)));
        }
        let nb_nodes = node_list.len();
        Simulation { simstate: State::new(node_list, vec![Vec2::new(0.0, 0.0); nb_nodes]), k: spring_constant, x0: 0.0, g: grav, c: damping, L0: fibre_length, dt: timestep, n: subd }
    }
}

// Step
impl Simulation {
    fn step(&mut self) {
        self.simstate.step(self.k * self.n as f32, self.L0 / self.n as f32, self.g, self.c, self.dt);
    }

    fn get_lowest_point(&self) -> f32 {
        self.simstate
            .nodes
            .iter()
            .map(|x| x.r.y)
            .fold(f32::INFINITY, f32::min)
    }
}

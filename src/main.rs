use std::time::Instant;
use std_dev::standard_deviation;
use std::fs;
use std::io::Write;
use std::f32::consts::PI;

mod vec2;
use vec2::Vec2;

mod node;
use node::Node;

mod state;
use state::State;

mod utility;

fn main() {
    // rayon::ThreadPoolBuilder::new().num_threads(1).build_global().unwrap();

    // Define constants
    const E: f32 = 2.64e9; // Pa
    const A: f32 = PI * (0.001/2.0) * (0.001/2.0);
    // Define simulation parameters
    let endpoints = vec![-2.0, 2.0];  // x coordinate of the endpoints
    let g = 9.81;  // gravitational acceleration
    let c = 0.0001;  // damping constant
    let L0 = 4.0;  // fibre rest length
    let k = E * A / L0;  // spring constant
    // let dt = 0.000005;  // timestep
    let mu = 0.003;  // mass per unit length
    let n = 3000;  // subdivisions
    let omega = ((k * n as f32) / (mu * (L0/n as f32))).sqrt();  // oscillation frequency
    let dt = 0.5 / omega;  // dynamically calculated timestep
    let mut simulation = Simulation::new_straight(endpoints, k, g, c, L0, dt, mu, n);

    // Define simulation metadata
    let max_steps = 10000000;
    let mut sag_history = vec![0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0, 110.0, 120.0, 130.0, 140.0, 150.0, 160.0, 170.0, 180.0, 190.0];
    let std_dev_limit = 0.05;
    let mut std_dev = standard_deviation(&sag_history).standard_deviation;

    // Loop the simulation
    let before = Instant::now();
    for i in 0..max_steps {

        // Step
        simulation.step();

        // Check sag stability
        if (i+1) % 1000 == 0 {
            sag_history.rotate_right(1);
            sag_history[0] = simulation.get_lowest_point() * 1000.0;
            std_dev = standard_deviation(&sag_history).standard_deviation;
            if std_dev < std_dev_limit {
                println!("Standard deviation of {std_dev} dropped below {std_dev_limit}");
                break
            }
        }

        // Print progress
        if (i+1) % 10000 == 0 {
            println!("Step {}/{max_steps}     sigma = {}", i+1, std_dev);
            println!("dt = {dt}");
        }
    }
    let time = before.elapsed();

    // Write node positions to file
    let mut file = fs::File::create("node_positions.txt").expect("Could not open file.");
    for node in simulation.simstate.nodes {
        file.write_all(&node.r.x.to_string().into_bytes()).expect("Could not write element to file.");
        file.write_all(b",").expect("Could not write element to file.");
        file.write_all(&node.r.y.to_string().into_bytes()).expect("Could not write element to file.");
        file.write_all(b"\n").expect("Could not write element to file.");
    }

    // Print simulation data
    let low_point = sag_history[0];
    println!("\nSag: {low_point} mm     (with dt = {dt})");
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

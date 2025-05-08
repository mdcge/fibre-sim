use nannou::prelude::*;
use std::time::Instant;

mod vec2;
use vec2::Vec2;

mod node;
use node::Node;

mod state;
use state::State;

mod utility;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
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
    // Display parameters
    n: usize,  // number of subdivisions (1 segment = 1 subdivision)
    dn: f32,  // display size for nodes
    ds: f32,  // display size for springs
    s: f32,  // display scale
}

// Initialize
impl Simulation {
    fn new(node_list: Vec<Node>, spring_constant: f32, spring_rest_length: f32, grav: f32, damping: f32, fibre_rest_length: f32, timestep: f32, node_diameter: f32, spring_thickness: f32, scale: f32) -> Simulation {
        let s = node_list.len() - 1;
        let nb_nodes = node_list.len();
        Simulation { simstate: State::new(node_list, vec![Vec2::new(0.0, 0.0); nb_nodes]), k: spring_constant, x0: spring_rest_length, g: grav, c: damping, L0: fibre_rest_length, dt: timestep, n: s, dn: node_diameter, ds: spring_thickness, s: scale }
    }

    fn new_straight(x_endpoints: Vec<f32>, spring_constant: f32, grav: f32, damping: f32, fibre_length: f32, timestep: f32, node_diameter: f32, spring_thickness: f32, scale: f32, mass: f32, subd: usize) -> Simulation {
        let spring_rest_length = fibre_length / subd as f32;
        let k_per_node = spring_constant * subd as f32;
        let mut node_list = vec![];
        for i in 0..subd+1 {
            let x_pos = x_endpoints[0] + i as f32 * (x_endpoints[1] - x_endpoints[0]) / subd as f32;
            node_list.push(Node::new(Vec2 { x: x_pos, y: 0.0 }, Vec2 { x: 0.0, y: 0.0 }, mass/(subd as f32 + 1.0)));
        }
        let nb_nodes = node_list.len();
        Simulation { simstate: State::new(node_list, vec![Vec2::new(0.0, 0.0); nb_nodes]), k: k_per_node, x0: spring_rest_length, g: grav, c: damping, L0: fibre_length, dt: timestep, n: subd, dn: node_diameter, ds: spring_thickness, s: scale }
    }
}

// Step
impl Simulation {
    fn step(&mut self) {
        self.simstate.step(self.k as f32, self.x0, self.g, self.c, self.dt);
    }

    fn get_lowest_point(&self) -> f32 {
        self.simstate
            .nodes
            .iter()
            .map(|x| x.r.y)
            .collect::<Vec<f32>>()
            .iter()
            .fold(f32::INFINITY, |this, that| f32::min(this, *that))
    }
}

// Render
impl Simulation {
    fn render(&self, draw: &Draw, scale: f32) {
        draw.background().color(BLACK);

        for (i, node) in self.simstate.nodes.iter().enumerate() {
            draw.ellipse()
                .color(WHITE)
                .w(self.dn)
                .h(self.dn)
                .x_y(node.r.x*scale, node.r.y*scale);
            if i != (self.simstate.nodes.len() - 1) {
                draw.line()
                    .color(WHITE)
                    .start(pt2(self.simstate.nodes[i].r.x*scale, self.simstate.nodes[i].r.y*scale))
                    .end(pt2(self.simstate.nodes[i+1].r.x*scale, self.simstate.nodes[i+1].r.y*scale))
                    .weight(self.ds);
            }
        }
    }
}

fn model(_app: &App) -> Simulation {
    //                         x_endpoints      k       g      c     L0     dt      dn    ds    s      m    sub
    Simulation::new_straight(vec![-2.0, 2.0], 5367.0, 9.81, 0.0002, 4.0, 0.000001, 10.0, 2.0, 400.0, 0.004, 200)
}

// `update` is like `event` except that the only event it triggers on is clock ticks
// Basically, it's a 60Hz update function.
fn update(_app: &App, simulation: &mut Simulation, _update: Update) {
    let steps_per_frame = (1.0 / 60.0 / simulation.dt) as usize;
    for _ in 0..steps_per_frame {
        simulation.step();
    }
}

fn view(app: &App, simulation: &Simulation, frame: Frame) {
    let win = app.window_rect();
    let draw = app.draw();

    simulation.render(&draw, simulation.s);

    draw.text(&format!("{:.2} fps", app.fps()).to_string())
        .font_size(15)
        .x_y(-win.wh()[0]/2.0 + 35.0, win.wh()[1]/2.0 - 10.0);

    let low_point = simulation.get_lowest_point() * 1000.0; // mm
    draw.text(&format!("Sag = {:.0} {:0>3.0} {:0>3.0} nm", low_point - (low_point % 1.0), ((low_point*1000.0) % 1000.0).abs(), (low_point*1000000.0 % 1000.0).abs()).to_string())
        .font_size(30)
        .width(win.wh()[0])
        .x_y(0.0, win.wh()[1]/2.0 - 50.0);

    draw.to_frame(app, &frame).unwrap();
}

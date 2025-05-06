use nannou::prelude::*;

mod node;
use node::Node;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Simulation {
    nodes: Vec<Node>,
    k: f32,  // total spring constant
    x0: f32  // rest length of springs
}

impl Simulation {
    fn new(node_list: Vec<Node>, spring_constant: f32, rest_length: f32) -> Simulation {
        Simulation { nodes: node_list, k: spring_constant, x0: rest_length }
    }
}

fn model(_app: &App) -> Simulation {
    Simulation::new(vec![Node::new(vec![-1.0, 0.0], 1.0), Node::new(vec![0.0, 0.0], 1.0), Node::new(vec![1.0, 0.0], 1.0)], 1.0, 0.0)
}

// `update` is like `event` except that the only event it triggers on is clock ticks
// Basically, it's a 60Hz update function.
fn update(_app: &App, _simulation: &mut Simulation, _update: Update) {}

fn view(app: &App, _simulation: &Simulation, _frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
}

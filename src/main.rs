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
    x0: f32,  // rest length of springs
    subdiv: usize,  // number of subdivisions (1 segment = 1 subdivision)
    dn: f32,  // display size for nodes
    ds: f32,  // display size for springs
}

impl Simulation {
    fn new(node_list: Vec<Node>, spring_constant: f32, rest_length: f32, node_diameter: f32, spring_thickness: f32) -> Simulation {
        let s = node_list.len() - 1;
        Simulation { nodes: node_list, k: spring_constant, x0: rest_length, subdiv: s, dn: node_diameter, ds: spring_thickness }
    }
}

// Render
impl Simulation {
    fn render(&self, draw: &Draw, scale: f32) {
        draw.background().color(BLACK);

        for (i, node) in self.nodes.iter().enumerate() {
            draw.ellipse()
                .color(WHITE)
                .w(self.dn)
                .h(self.dn)
                .x_y(node.r[0]*scale, node.r[1]*scale);
            if i != (self.nodes.len() - 1) {
                draw.line()
                    .color(WHITE)
                    .start(pt2(self.nodes[i].r[0]*scale, self.nodes[i].r[1]*scale))
                    .end(pt2(self.nodes[i+1].r[0]*scale, self.nodes[i+1].r[1]*scale))
                    .weight(self.ds);
            }
        }
    }
}

fn model(_app: &App) -> Simulation {
    Simulation::new(vec![Node::new(vec![-1.0, 0.0], 1.0), Node::new(vec![0.0, 0.0], 1.0), Node::new(vec![1.0, 0.0], 1.0)], 1.0, 0.0, 20.0, 8.0)
}

// `update` is like `event` except that the only event it triggers on is clock ticks
// Basically, it's a 60Hz update function.
fn update(_app: &App, _simulation: &mut Simulation, _update: Update) {}

fn view(app: &App, simulation: &Simulation, frame: Frame) {
    let win = app.window_rect();
    let draw = app.draw();

    simulation.render(&draw, 100.0);

    draw.text(&format!("{:.2} fps", app.fps()).to_string())
        .font_size(15)
        .x_y(-win.wh()[0]/2.0 + 35.0, win.wh()[1]/2.0 - 10.0);

    draw.to_frame(app, &frame).unwrap();
}

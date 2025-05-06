use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

// `update` is like `event` except that the only event it triggers on is clock ticks
// Basically, it's a 60Hz update function.
fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, _frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
}

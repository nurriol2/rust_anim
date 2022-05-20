#![allow(dead_code)]

mod venn;

use nannou::prelude::*;
use venn::{VennCircle, Breathing};

struct Model {}


fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    // Try to keep 2:3 aspect ratio
    let _window = app.new_window()
                            .resizable(false)
                            .size(500, 750)
                            .view(view)
                            .build()
                            .unwrap();
    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw: Draw = app.draw();

    // Clear the canvas before each tick
    draw.background().color(BLACK);


    // Breathing circle at the origin
    let mut breathing_center: VennCircle = VennCircle{
        stroke_weight: 4.0,
        ..Default::default()
    };
    let rate: f32 = 4.0;
    let radius_min: f32 = 75.0;
    let radius_max: f32 = 100.0;
    breathing_center.breathe(&app, &draw, rate, radius_min, radius_max);

    
    draw.to_frame(app, &frame).unwrap();
}

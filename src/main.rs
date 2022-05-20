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
    let _window = app.new_window().size(500, 750).view(view).build().unwrap();
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

    // Huge, out of frame circle shrinking in

    let huge_start_radius: f32 = 500.0;
    let huge_velocity: f32 = 50.0;

    // Get window coordinates
    let win: Rect = app.window_rect();
    let circle_center: Vec2 = win.top_right();

    // Shrinking center coordinates
    let current_x = circle_center.x - app.time * huge_velocity;
    let current_y = circle_center.y - app.time * huge_velocity;
    // Shrinking radius
    let huge_radius = huge_start_radius - app.time * huge_velocity;

    // Draw the circle
    draw.ellipse()
        //.xy(circle_center)
        .x(current_x)
        .y(current_y)
        .radius(huge_radius)
        .stroke(GREEN)
        .stroke_weight(4.0)
        .no_fill();

    // END Huge shrinking in

    draw.to_frame(app, &frame).unwrap();
}

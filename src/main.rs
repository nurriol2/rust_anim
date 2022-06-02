#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod venn;

use nannou::prelude::*;
use venn::{Effects, VennCircle};


struct Model {
    circles: Vec<VennCircle>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    // Try to keep 2:3 aspect ratio
    let _window = app
        .new_window()
        .resizable(false)
        .size(500, 750)
        .view(view)
        .build()
        .unwrap();

    let win = app.window_rect();

    let a: VennCircle = VennCircle{
        center: win.top_right() * 0.9,
        initial_pos: win.top_right() * 0.9,
        angular_speed: 0.5, 
        breathing_rate: 50.0,
        ..Default::default()
    };

    let b: VennCircle = VennCircle{
        center: Vec2::new(0.0, win.top() * 0.85),
        initial_pos: Vec2::new(0.0, win.top() * 0.85),
        stroke_color: LIGHTSKYBLUE,
        angular_speed: 1.0, 
        breathing_rate: 35.0,
        ..Default::default()
    };
    
    let c: VennCircle = VennCircle{
        center: Vec2::new(win.right() * 0.1, win.top() * 0.70),
        initial_pos: Vec2::new(win.right() * 0.1, win.top() * 0.70),
        stroke_color: INDIGO,
        angular_speed: 0.75, 
        breathing_rate: 40.0,
        ..Default::default()
    };


    let d: VennCircle = VennCircle{
        center: win.bottom_left(),
        initial_pos: win.bottom_left() * 0.9,
        stroke_color: ORANGERED,
        angular_speed: 2.0, 
        breathing_rate: 5.0,
        ..Default::default()
    };


    let e: VennCircle = VennCircle{
        center: win.bottom_left(),
        initial_pos: win.bottom_left() * 0.9,
        stroke_color: PINK,
        angular_speed: 30.0, 
        breathing_rate: 25.0,
        ..Default::default()
    };

    let f: VennCircle = VennCircle{
        center: Vec2::new(win.right(), 5.0),
        initial_pos: Vec2::new(win.right(), 5.0),
        stroke_color: GOLD,
        angular_speed: 15.0, 
        breathing_rate: 10.0,
        ..Default::default()
    };

    let circles = vec![a, b, c, d, e, f];

    Model { circles }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw: Draw = app.draw();

    // Clear the canvas before each tick
    frame.clear(BLACK);

    // Draw a marker at the center
    draw.rect()
        .color(RED)
        .xy(Vec2::new(0.0, 0.0))
        .w(20.0)
        .h(20.0);

    // Draw the circles
    for circle in model.circles.iter() {
        circle.paint_to(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let origin = Vec2::new(0.0, 0.0);
    let x_axis = Vec2::new(1.0, 0.0);


    for circle in model.circles.iter_mut(){
        circle.breathe(app, 10.0, 20.0);
    }

    for circle in model.circles.iter_mut(){

        let t: f32 = app.elapsed_frames() as f32 / 600.0;

        let max_ampl = circle.initial_pos.distance(origin);
        
        let phi = circle.initial_pos.angle();
        let amplitude = ampl_t(max_ampl, 20.0, 10.0, t);
        spiral_to_orbit(circle, amplitude, phi, t);

    }
}

fn ampl_t(max_ampl: f32, min_ampl: f32, t_orbit: f32, t: f32) -> f32 {
    if t < t_orbit {
        let slope = (min_ampl - max_ampl) / t_orbit;
        (slope * t) + max_ampl
    } else {
        min_ampl
    }
}

fn spiral_to_orbit(c: &mut VennCircle, amplitude: f32, phi: f32, t: f32){
    let x = amplitude * ((c.angular_speed * t) + phi).cos();
    let y = amplitude * ((c.angular_speed * t) + phi).sin();

    c.center = Vec2::new(x, y);
}

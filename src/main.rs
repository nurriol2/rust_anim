#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod venn;

use nannou::prelude::*;
use venn::{Breathing, VennCircle};

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

    let dist_: f32 = 200.0;

    let twelve: VennCircle = VennCircle{
        center: Vec2::new(0.0, dist_), 
        initial_position: Vec2::new(0.0, dist_), 
        ..Default::default()
    };

    let one: VennCircle = VennCircle { 
        center: Vec2::new(dist_, dist_,),
        initial_position: Vec2::new(dist_, dist_,),
        speed: 5.0,
        ..Default::default()
    };

    let _six: VennCircle = VennCircle {
        center: Vec2::new(0.0, -dist_),
        ..Default::default()
    };

    let circles: Vec<VennCircle> = vec![twelve, one];

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
    //let dt: f32 = app.elapsed_frames() as f32 / 1200.0;
    let dt: f32 = app.elapsed_frames() as f32 / 2400.0;


    // Compound motion hack
    for circle in model.circles.iter_mut(){
        
        // Linear motion parameters
        let origin = Vec2::new(0., 0.);
        let speed = 2.0;

        // Breathing parameters
        let rate: f32 = 4.0;
        let radius_min: f32 = 15.0;
        let radius_max: f32 = 20.0;

        if circle.center.distance(origin) > 100.0{
            circle.line_to_origin(speed);
        } else {
            let phi = -circle.initial_position.angle();
            circle.orbit(99.0, dt, phi);
            circle.breathe(app, rate, radius_min, radius_max)
        }

        
    }
}

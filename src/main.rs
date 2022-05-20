#![allow(dead_code)]

mod venn;

use nannou::prelude::*;
use venn::{VennCircle, Breathing};

struct Model {
    satellites: Vec<VennCircle>,
}


fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    // Try to keep 2:3 aspect ratio
    let _window = app.new_window()
                            .resizable(false)
                            .size(500, 750)
                            .view(view)
                            .build()
                            .unwrap();


    /*
    This circle should breathe and remain stationary
    */
    let sat_a: VennCircle = VennCircle::default();
    // This circle should move while breathing
    let sat_b: VennCircle = VennCircle{center: Vec2::new(50.0, -50.0),
                                        velocity: Vec2::new(0.1, 0.1),
                                        ..Default::default()};
    
    let sats: Vec<VennCircle> = vec![sat_a, sat_b];

    Model {
        satellites: sats,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw: Draw = app.draw();

    // Clear the canvas before each tick
    draw.background().color(BLACK);
    
    // Draw the satellites 
    for satellite in model.satellites.iter(){
        satellite.paint_to(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn update(app: &App, model: &mut Model, _update: Update){
    
    let dt: f32 = app.elapsed_frames() as f32 / 60.0;
    
    // Make breathing satellites
    let rate: f32 = 4.0;
    let radius_min: f32 = 75.0;
    let radius_max: f32 = 100.0;
    for satellite in model.satellites.iter_mut(){
        satellite.breathe(app, rate, radius_min, radius_max);
    }

    // Move one of the satellites wrt the origin
    let mover: &mut VennCircle = &mut model.satellites[1];
    mover.update_position(dt);
    mover.update_velocity(dt);


}
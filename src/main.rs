#![allow(dead_code)]
#![allow(unused_variables)]

mod venn;

use nannou::prelude::*;
use venn::{Breathing, VennCircle};

struct Model {
    satellites: Vec<VennCircle>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    // Try to keep 2:3 aspect ratio
    let _window = app
        .new_window()
        //.resizable(false)
        .size(500, 750)
        .view(view)
        .build()
        .unwrap();

    // Starting distance of the initial satellite configuration
    let starting_distance: f32 = 125.0;
    let stroke_weight: f32 = 3.0;

    // Initial positions
    let north: Vec2 = Vec2::new(0.0, starting_distance);
    let east: Vec2 = Vec2::new(starting_distance, 0.0);
    let south: Vec2 = -1.0 * north;
    let west: Vec2 = -1.0 * east;

    // Initial velocity
    let right: Vec2 = Vec2::new(1.0, 0.0);
    let left: Vec2 = -1.0 * right;
    let up: Vec2 = Vec2::new(0.0, 1.0);
    let down: Vec2 = -1.0 * up;

    // Set up a satellite at each cardinal direction
    let north_sat: VennCircle = VennCircle {
        center: north,
        stroke_weight,
        velocity: left,
        ..Default::default()
    };

    let south_sat: VennCircle = VennCircle {
        center: south,
        stroke_weight,
        velocity: right,
        ..Default::default()
    };

    let east_sat: VennCircle = VennCircle {
        center: east,
        stroke_weight,
        velocity: up,
        ..Default::default()
    };

    let west_sat: VennCircle = VennCircle {
        center: west,
        stroke_weight,
        velocity: down,
        ..Default::default()
    };

    //let sats: Vec<VennCircle> = vec![north_sat, south_sat, east_sat, west_sat];
    let sats = vec![north_sat];

    Model { satellites: sats }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw: Draw = app.draw();

    // Clear the canvas before each tick
    draw.background().color(BLACK);

    // Draw a marker at the center
    draw.rect()
        .color(RED)
        .xy(Vec2::new(0.0, 0.0))
        .w(20.0)
        .h(20.0);

    // Draw the satellites
    for satellite in model.satellites.iter() {
        satellite.paint_to(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let dt: f32 = app.elapsed_frames() as f32 / 1200.0;

    // Make breathing satellites
    let rate: f32 = 4.0;
    let radius_min: f32 = 15.0;
    let radius_max: f32 = 20.0;
    for satellite in model.satellites.iter_mut() {
        satellite.breathe(app, rate, radius_min, radius_max);
    }

    // Move the satellites
    for satellite in model.satellites.iter_mut(){
        satellite.update_position(dt);
        satellite.update_velocity(dt);
    }
    
    // HACK:  Artificially set a speed limit
    for satellite in model.satellites.iter_mut(){
        if (dt as i32 != 0) && (dt as i32) % 15 == 0{
            satellite.velocity *= 0.1;
            println!("Slowing down!")
        } 
    }

}

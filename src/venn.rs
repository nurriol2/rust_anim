use nannou::prelude::*;

#[derive(Debug)]
pub struct VennCircle {
    pub center: (f32, f32),
    pub radius: f32,
    pub stroke_weight: f32,
    pub stroke_color: rgb::Rgb,
}

impl VennCircle {
    // TODO:  Rewrite or remove. This is annoying to use
    pub fn new(center: (f32, f32), radius: f32, stroke_weight: f32, stroke_color: rgb::Rgb) -> VennCircle {
        VennCircle {
            center,
            radius,
            stroke_weight,
            stroke_color,
        }
    }

    pub fn update_radius(&mut self, new_radius: f32){
        self.radius = new_radius;
    }

    pub fn paint_to(&self, draw: &Draw) {
        draw.ellipse()
            .x(self.center.0)
            .y(self.center.1)
            .radius(self.radius)
            .stroke(self.stroke_color)
            .stroke_weight(self.stroke_weight)
            .no_fill();
    }
}

// pub trait Breathing {
//     // Oscillate the circumfernce of a circle to get a breathing effect
//     fn breathe(&mut self, app: &App, draw: &Draw, rate: f32, radius_min: f32, radius_max: f32);
// }

pub trait Breathing {
    // Oscillate the circumfernce of a circle to get a breathing effect
    fn breathe(&mut self, app: &App, rate: f32, radius_min: f32, radius_max: f32);
}


impl Default for VennCircle{
    fn default() -> VennCircle{
        VennCircle {
            center: (0.0, 0.0),
            radius: 40.0,
            stroke_weight: 15.0,
            stroke_color: rgb::Rgb::new(0.0, 255.0, 0.0),
        }        
    }
}

// impl Breathing for VennCircle{
//     fn breathe(&mut self, app: &App, draw: &Draw, rate: f32, radius_min: f32, radius_max: f32){
//         let time: f32 = app.elapsed_frames() as f32 / 60.0;
//         let oscillation: f32 = (time * rate).sin();
//         let current_radius = map_range(oscillation, -1.0, 1.0, radius_min, radius_max);
//         self.update_radius(current_radius);
//         self.paint_to(draw);
//     }
// }

impl Breathing for VennCircle{
    fn breathe(&mut self, app: &App, rate: f32, radius_min: f32, radius_max: f32){
        let time: f32 = app.elapsed_frames() as f32 / 60.0;
        let oscillation: f32 = (time * rate).sin();
        let current_radius = map_range(oscillation, -1.0, 1.0, radius_min, radius_max);
        self.update_radius(current_radius);
    }
}
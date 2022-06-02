use nannou::prelude::*;
use nannou::color::named;

#[derive(Debug)]
pub struct VennCircle {
    pub center: Vec2,
    pub initial_pos: Vec2,
    pub radius: f32,
    pub stroke_weight: f32,
    pub stroke_color: Srgb<u8>,
    pub angular_speed: f32,
    pub breathing_rate: f32,
}

impl VennCircle {
    pub fn update_radius(&mut self, new_radius: f32) {
        self.radius = new_radius;
    }

    pub fn paint_to(&self, draw: &Draw) {
        draw.ellipse()
            .x(self.center.x)
            .y(self.center.y)
            .radius(self.radius)
            .stroke(self.stroke_color)
            .stroke_weight(self.stroke_weight)
            .no_fill();
    }
}

impl Default for VennCircle {
    fn default() -> Self {
        let origin = Vec2::new(0.0, 0.0);

        Self {
            center: origin,
            initial_pos: origin,
            radius: 30.0,
            stroke_weight: 4.0,
            //stroke_color: grn,
            stroke_color: GREEN,
            angular_speed: 5.0,
            breathing_rate: 5.0,
        }
    }
}

pub trait Effects {
    // Oscillate the radius of a circle to get a breathing effect
    fn breathe(&mut self, app: &App, radius_min: f32, radius_max: f32);
}



impl Effects for VennCircle {
    fn breathe(&mut self, app: &App, radius_min: f32, radius_max: f32) {
        let time: f32 = app.elapsed_frames() as f32 / 60.0;
        let oscillation: f32 = (time * self.breathing_rate).sin();
        let current_radius = map_range(oscillation, -1.0, 1.0, radius_min, radius_max);
        self.update_radius(current_radius);
    }
}

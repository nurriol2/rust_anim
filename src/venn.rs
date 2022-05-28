use nannou::prelude::*;

#[derive(Debug)]
pub struct VennCircle {
    // TODO:  Are all these pubs for the fields neccessary? I think so.
    pub center: Vec2,
    pub initial_position: Vec2,
    pub radius: f32,
    pub stroke_weight: f32,
    pub stroke_color: rgb::Rgb, // HACK:  Color is annoying to work with so this is a workaround
    pub speed: f32,
}

impl VennCircle {
    pub fn update_radius(&mut self, new_radius: f32) {
        self.radius = new_radius;
    }

    pub fn line_to_origin(&mut self, speed: f32){
        let origin: Vec2 = Vec2::new(0., 0.);
        let direction_heading = (origin - self.center).normalize_or_zero();
        self.center += speed * direction_heading;
    }

    pub fn orbit(&mut self, radius: f32, dt: f32, phi: f32){
        let x = radius * ((self.speed * dt) - phi).cos();
        let y = radius * ((self.speed * dt) - phi).sin();
        self.center = Vec2::new(x, y);
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
    fn default() -> VennCircle {
        VennCircle {
            center: Vec2::new(0.0, 100.0),
            initial_position: Vec2::new(0.0, 100.0),
            radius: 40.0,
            stroke_weight: 3.0,
            stroke_color: rgb::Rgb::new(0.0, 255.0, 0.0),
            speed: 2.0,
        }
    }
}

pub trait Breathing {
    // Oscillate the radius of a circle to get a breathing effect
    fn breathe(&mut self, app: &App, rate: f32, radius_min: f32, radius_max: f32);
}

impl Breathing for VennCircle {
    fn breathe(&mut self, app: &App, rate: f32, radius_min: f32, radius_max: f32) {
        let time: f32 = app.elapsed_frames() as f32 / 60.0;
        let oscillation: f32 = (time * rate).sin();
        let current_radius = map_range(oscillation, -1.0, 1.0, radius_min, radius_max);
        self.update_radius(current_radius);
    }
}
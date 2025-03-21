pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub theta: f64,
    pub rot_accel: f64,
    pub max_rot_velocity: f64,
    pub velocity: f64,
    pub max_velocity: f64,
    pub accel_factor: f64
}

use std::default::Default;
impl Default for Camera {
    fn default() -> Self {
        Self {
            x: 20.0,
            y: 20.0,
            height: 200.0, // What's a good starting height, and where in this codebase should it go?
            theta: -2.25,
            rot_accel: 0.0,
            max_rot_velocity: 0.075,
            velocity: 2.0, 
            max_velocity: 10.0,
            accel_factor: 0.0,
        }
    }
}

use minifb::{Window, Key};
impl Camera {
    pub fn new() -> Self { Self::default() }

    pub fn react_to_inputs(&mut self, window: &Window) {
        // Accelerate
        if window.is_key_down(Key::W){
            self.accel_factor += 0.02;
            self.accel_factor = self.accel_factor.min(6.0);
        } else if window.is_key_down(Key::S){
            self.accel_factor -= 0.03;
            self.accel_factor = self.accel_factor.max(0.0);
        }
        self.velocity = self.max_velocity / (1.0 + (-0.5 * (self.accel_factor - 4.0)).exp());

        // Yaw
        if window.is_key_down(Key::A){
            self.rot_accel += 0.025;
            self.rot_accel = self.rot_accel.min(1.0);
        }
        if window.is_key_down(Key::D){
            self.rot_accel -= 0.025;
            self.rot_accel = self.rot_accel.max(-1.0);
        }
        self.theta += self.max_rot_velocity / (1.0 + (-0.5 * (self.rot_accel - 0.0)).exp()) - self.max_rot_velocity / 2.0;
        // Note the unintuitive relation between coordinates and trig functions here. Switch them for disorientation!
        self.y -= self.theta.cos() * self.velocity;
        self.x -= self.theta.sin() * self.velocity;

        // Change height (TODO: vertical acceleration)
        if window.is_key_down(Key::R){
            self.height += 1.0;
        }
        if window.is_key_down(Key::F){
            self.height -= 1.0;
        }
    }
}
use crate::{camera::Camera, config::Config};
use minifb::Window;

pub struct Game {
    pub camera: Camera,
    pub config: Config,
    pub window: Window,
    pub fb: Vec<u32>
}
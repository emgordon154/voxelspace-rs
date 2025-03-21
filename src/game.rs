use crate::{
    camera::Camera,
    config::Config,
    load_maps::{self, TerrainPyramids},
    render,
    window
};
use minifb::{Window, Key};
use std::time::Instant;

pub struct Game {
    pub camera: Camera,
    pub window: Window,
    pub frame_buf: Vec<u32>,
    pub y_buf: Vec<usize>,
    pub current_map_pyrs: TerrainPyramids,
    pub config: Config,
}

use std::default::Default;
impl Default for Game {
    fn default() -> Self {
        let config = Config::default();
        Self {
            camera: Camera::new(),
            window: window::open(&config),
            frame_buf: vec![0; config.screen_height * config.screen_width],
            y_buf: vec![config.screen_height - 1; config.screen_width],
            current_map_pyrs: load_maps::load_maps("D5.png", "C5W.png").unwrap(),
            config,
        }
    }
}

impl Game {
    pub fn new() -> Self { Game::default() }

    pub fn run(self) {
        let Self {
            mut camera,
            mut window,
            mut frame_buf,
            mut y_buf,
            current_map_pyrs,
            config,
        } = self;
        let mut frame_counter = 0;
        let mut total_rendering_time: u128  = 0;
        while window.is_open() && !window.is_key_down(Key::Escape) {
            frame_counter += 1;
            let before_render_timestamp = Instant::now();
            camera.react_to_inputs(&window);
            render::render(&mut camera, &config, &mut frame_buf, &mut y_buf, &current_map_pyrs);
            window::display_fps(&mut window, &config, &before_render_timestamp, &mut total_rendering_time, &frame_counter);
            window
                .update_with_buffer(&frame_buf, config.screen_width, config.screen_height)
                .unwrap();
        }
    }
}
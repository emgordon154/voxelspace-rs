use crate::{
    camera::Camera,
    config::Config,
    load_maps::{self, TerrainPyramids},
    render,
    window::{self, display_fps}
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

// this feels like a weird and very wrong file to put this definition in
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Game {
    pub fn new() -> Self {
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
            render::render(&mut camera, &mut window, &config, &mut frame_buf, &mut y_buf, &current_map_pyrs);
            display_fps(&mut window, &config, &before_render_timestamp, &mut total_rendering_time, &mut frame_counter);
            // What can/should I do as far as error handling here? I don't see a compelling reason not to just panic.
            window
                .update_with_buffer(&frame_buf, config.screen_width, config.screen_height)
                .unwrap();
        }
    }
}
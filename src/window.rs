use minifb::{Window, WindowOptions};
use crate::config::Config;

pub fn open(config: &Config) -> Window {
    let mut window = Window::new(
        "Test - ESC to exit",
        config.screen_width,
        config.screen_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    window.set_target_fps(config.max_fps);
    window
}

use std::time::Instant;
pub fn display_fps(
    window: &mut Window,
    config: &Config,
    before_render_timestamp: &Instant, 
    total_rendering_time: &mut u128,
    frame_counter: &u128,
) {
    let perf_micros = before_render_timestamp.elapsed().as_micros();
    *total_rendering_time += perf_micros;
    let perf_micros_avg = *total_rendering_time / *frame_counter;
    window.set_title(&format!(
        "Time: {:.2} ms, FPS: {:.1} (capped at {}); Avg: {:.1}",
        perf_micros as f64 / 1000.0,
        1e6 / perf_micros as f64,
        config.max_fps,
        1e6 / perf_micros_avg as f64
    ));
}
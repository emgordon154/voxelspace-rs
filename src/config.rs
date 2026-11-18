pub struct Config {
    pub screen_width: usize,
    pub screen_height: usize,
    pub max_fps: usize,
    pub base_fov: f64, // TODO: Should this be here or on the camera?
    pub speed_fov_adjust: f64,
    pub draw_distance: u64, // TODO: Should this necessarily be a piece of the game's global state?
    pub lod_1_distance: f64, // distance at which to step up a layer of the pyramid
    pub scale_height: f64, // TODO: This feels maybe out-of-place here.
    pub horizon: f64, // So does this.
    pub render_thread_quant: usize,
}

use std::default::Default;
impl Default for Config {
    fn default() -> Self {
        let screen_width = 1080;
        let screen_height = 720;
        Self {
            screen_width,
            screen_height,
            max_fps: 60,
            base_fov: 90.0,
            speed_fov_adjust: 15.0,
            draw_distance: 1800,
            lod_1_distance: 400.0,
            scale_height: 240.0,
            horizon: screen_height as f64 / 3.0,
            render_thread_quant: 3,
        }
    }
}
pub struct Config {
    pub screen_width: usize,
    pub screen_height: usize,
    pub max_fps: usize,
    pub fov: f64, // Should this be here or on the camera?
    pub draw_distance: u64, // Should this necessarily be a piece of the game's global state?
    pub scale_height: f64, // This feel maybe out-of-place here.
    pub horizon: f64, // So does this.
}

// impl Config {
//     fn new(screen_height: u64, screen_width: u64, draw_distance: u64) -> Self {

//     }
// }

use std::default::Default;
impl Default for Config {
    fn default() -> Self {
        let screen_width = 1080;
        let screen_height = 720;
        Self {
            screen_width,
            screen_height,
            max_fps: 60,
            fov: 90.0,
            draw_distance: 400,
            scale_height: 240.0,
            horizon: screen_height as f64 / 3.0,
        }
    }
}
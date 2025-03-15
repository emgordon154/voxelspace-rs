pub struct Config {
    pub screen_height: usize,
    pub screen_width: usize,
    pub draw_distance: u64
}

// impl Config {
//     fn new(screen_height: u64, screen_width: u64, draw_distance: u64) -> Self {

//     }
// }

use std::default::Default;
impl Default for Config {
    fn default() -> Self {
        Self {
            screen_height: 540,
            screen_width: 360,
            draw_distance: 400
        }
    }
}
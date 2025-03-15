// use minifb::{Key, Window, WindowOptions};
// use flightsim::{
//     camera::Camera,
//     // game::Game
// };
use flightsim::dirtily;

fn main() {
    dirtily::do_everything();
    // let config = flightsim::config::Config::default();
    // let mut fb: Vec<u32> = vec![0; config.screen_height * config.screen_width];

    // let mut window = Window::new(
    //     "Test - ESC to exit",
    //     config.screen_width,
    //     config.screen_height,
    //     WindowOptions::default(),
    // )
    // .unwrap_or_else(|e| panic!("{}", e));

    // window.set_target_fps(60);

    // let mut camera = Camera::new();

    // // let game = Game {
    // //     camera: Camera::new(),
    // //     config,
    // //     window,
    // //     fb
    // // };



    

    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     for i in fb.iter_mut() {
    //         *i = 0; // write something more funny here!
    //     }

    //     // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    //     window
    //         .update_with_buffer(&fb, config.screen_width, config.screen_height)
    //         .unwrap();
    // }
}
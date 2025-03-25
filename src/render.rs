use core::f64;
use std::sync::RwLock;

use minifb::Window;

use crate::{
    camera::Camera,
    config::Config,
    load_maps::TerrainPyramids
};

pub struct Point {
    pub x: f64,
    pub y: f64
}

static SKY_BLUE: u32 = 0x000080E0;

pub fn render_single_threaded(
    cam: &mut Camera, 
    config: &Config,
    frame_buf: &mut [u32],
    y_buf: &mut [usize],
    current_map_pyrs: &TerrainPyramids
) {
    let TerrainPyramids {
        heightmap_pyr,
        colormap_pyr
    } = current_map_pyrs;

    // PROBLEM!! C5W.png and D5.png are mismatched sizes!!
    let cmap = colormap_pyr.levels[0].as_rgb8().unwrap();
    let hmap = heightmap_pyr.levels[0].as_luma8().unwrap(); 
    let get_color_pixel = |p: &Point| -> u32 {
        let [r, g, b] = 
        cmap[(p.x as u32 % cmap.width(), p.y as u32 % cmap.height())].0.map(|byte| byte as u32);
        //     cmap[(p.x.rem_euclid(cmap.width() as f64) as u32, p.y.rem_euclid(cmap.height() as f64) as u32 )].0.map(|byte| byte as u32);
        (r << 16) | (g << 8) | b
    };
    let get_height_pixel = |p: &Point| -> usize {
        // NASTY HACK to compensate for C5W.png and D5.png being mismatched sizes!!
        hmap[(p.x as u32 / 2 % hmap.width(), p.y as u32 / 2 % hmap.height())].0[0] as usize
        //     hmap[((p.x / 2.0).rem_euclid(hmap.width() as f64) as u32, (p.y / 2.0).rem_euclid(hmap.height() as f64) as u32 )].0[0] as usize
    };
    
    let fov = config.base_fov - config.speed_fov_adjust * (cam.velocity / cam.max_velocity);

    let fov_adjust = (fov - 90.0) * f64::consts::PI / 180.0;
    let theta_left = cam.theta + fov_adjust / 2.0;
    let sintheta_left = theta_left.sin();
    let costheta_left = theta_left.cos();
    let theta_right = cam.theta - fov_adjust / 2.0;
    let sintheta_right = theta_right.sin();
    let costheta_right = theta_right.cos();

    
    frame_buf.fill(SKY_BLUE);
    y_buf.fill(config.screen_height - 1);

    // where to cut the frustrum? 20?
    let mut z = 20.0;
    let mut dz = 1.0;
    while z < config.draw_distance as f64 {
        let mut pleft = Point{
            x: -costheta_left * z - sintheta_left * z + cam.x,
            y: sintheta_left * z - costheta_left * z + cam.y
        };
        let pright = Point{
            x: costheta_right * z - sintheta_right * z + cam.x,
            y: -sintheta_right * z - costheta_right * z + cam.y
        };
        
        let dx = (pright.x - pleft.x) / config.screen_height as f64;
        let dy = (pright.y - pleft.y) / config.screen_width as f64;
        for i in 0 .. config.screen_width {
            let color = get_color_pixel(&pleft);
            let hmap_pixel = get_height_pixel(&pleft);
            let height_on_screen = ((cam.height - hmap_pixel as f64) / z * config.scale_height + config.horizon) as usize;
            if height_on_screen < y_buf[i] {
                for j in height_on_screen .. y_buf[i] {
                    frame_buf[config.screen_width * j + i] = color;
                }
                y_buf[i] = height_on_screen;
            }
            pleft.x += dx;
            pleft.y += dy;
        }
        z += dz;
        // TODO: Mipmapping, so I can do LOD without horrific aliasing artifacts
        dz += 0.0;
    }
}

use std::thread;
// use std::sync::Barrier;
pub fn render_multi_threaded(
    cam: &mut Camera, 
    config: &Config,
    sideways_frame_buf: &mut [u32],
    y_buf: &mut [usize],
    current_map_pyrs: &TerrainPyramids
) {
    let TerrainPyramids {
        heightmap_pyr,
        colormap_pyr
    } = current_map_pyrs;
    

    // PROBLEM!! C5W.png and D5.png are mismatched sizes!!
    let cmap = colormap_pyr.levels[0].as_rgb8().unwrap();
    let hmap = heightmap_pyr.levels[0].as_luma8().unwrap(); 
    let get_color_pixel = |p: &Point| -> u32 {
        let [r, g, b] = 
        cmap[(p.x as u32 % cmap.width(), p.y as u32 % cmap.height())].0.map(|byte| byte as u32);
        //     cmap[(p.x.rem_euclid(cmap.width() as f64) as u32, p.y.rem_euclid(cmap.height() as f64) as u32 )].0.map(|byte| byte as u32);
        (r << 16) | (g << 8) | b
    };
    let get_height_pixel = |p: &Point| -> usize {
        // NASTY HACK to compensate for C5W.png and D5.png being mismatched sizes!!
        hmap[(p.x as u32 / 2 % hmap.width(), p.y as u32 / 2 % hmap.height())].0[0] as usize
        //     hmap[((p.x / 2.0).rem_euclid(hmap.width() as f64) as u32, (p.y / 2.0).rem_euclid(hmap.height() as f64) as u32 )].0[0] as usize
    };
    
    let fov_degrees = config.base_fov - config.speed_fov_adjust * (cam.velocity / cam.max_velocity);
    let fov_radians = fov_degrees * f64::consts::PI / 180.0;
    let theta_right = cam.theta + fov_radians / 2.0;
    let cam_x = cam.x;
    let cam_y = cam.y;
    let cam_height = cam.height;

    
    sideways_frame_buf.fill(SKY_BLUE);
    y_buf.fill(config.screen_height - 1);

    let screen_slice_pixelwidth = config.screen_width / config.render_thread_quant;
    let screen_slice_radians = fov_radians / config.render_thread_quant as f64;

    thread::scope(|s| {
        let mut y_buf_chunks = y_buf.chunks_exact_mut(screen_slice_pixelwidth);
        let mut frame_buf_chunks = sideways_frame_buf.chunks_exact_mut(screen_slice_pixelwidth * config.screen_height);
        for t in 0..config.render_thread_quant {
            let y_buf_chunk = y_buf_chunks.next().unwrap();
            let frame_buf_chunk = frame_buf_chunks.next().unwrap();

            s.spawn(move|| {
                let slice_theta_right = theta_right - screen_slice_radians * t as f64;
                let slice_theta_left = slice_theta_right + screen_slice_radians;
                let d_theta = (slice_theta_right - slice_theta_left) / screen_slice_pixelwidth as f64;
                let mut theta = slice_theta_right;
                for x_on_slice in 0..screen_slice_pixelwidth {
                    let sintheta = theta.sin();
                    let costheta = theta.cos();
                    let mut z = 20.0;
                    let mut dz = 1.0;
                    while z < config.draw_distance as f64 {
                        let p = Point {
                            x: -sintheta * z + cam_x,
                            y: -costheta * z + cam_y
                        };
                        let color = get_color_pixel(&p);
                        let hmap_pixel = get_height_pixel(&p);
                        let height_on_screen = ((cam_height - hmap_pixel as f64) / z * config.scale_height + config.horizon) as usize;
                        if height_on_screen < y_buf_chunk[x_on_slice] {
                            for j in height_on_screen .. y_buf_chunk[x_on_slice] {
                                frame_buf_chunk[x_on_slice * config.screen_height + j] = color;
                            }
                            y_buf_chunk[x_on_slice] = height_on_screen;
                        }
                        z += dz;
                        dz += 0.0;
                    }
                    theta += d_theta;
                }
            });
        }
    });
}


// Currently somewhat broken: Camera angle is off, and there's a weird waviness going on
pub fn render_multi_threaded_v2(
    cam: &mut Camera, 
    config: &Config,
    sideways_frame_buf: &mut [u32],
    y_buf: &mut [usize],
    current_map_pyrs: &TerrainPyramids
) {
    let TerrainPyramids {
        heightmap_pyr,
        colormap_pyr
    } = current_map_pyrs;

    // PROBLEM!! C5W.png and D5.png are mismatched sizes!!
    let cmap = colormap_pyr.levels[0].as_rgb8().unwrap();
    let hmap = heightmap_pyr.levels[0].as_luma8().unwrap(); 
    let get_color_pixel = |p: &Point| -> u32 {
        let [r, g, b] = 
        cmap[(p.x as u32 % cmap.width(), p.y as u32 % cmap.height())].0.map(|byte| byte as u32);
        //     cmap[(p.x.rem_euclid(cmap.width() as f64) as u32, p.y.rem_euclid(cmap.height() as f64) as u32 )].0.map(|byte| byte as u32);
        (r << 16) | (g << 8) | b
    };
    let get_height_pixel = |p: &Point| -> usize {
        // NASTY HACK to compensate for C5W.png and D5.png being mismatched sizes!!
        hmap[(p.x as u32 / 2 % hmap.width(), p.y as u32 / 2 % hmap.height())].0[0] as usize
        //     hmap[((p.x / 2.0).rem_euclid(hmap.width() as f64) as u32, (p.y / 2.0).rem_euclid(hmap.height() as f64) as u32 )].0[0] as usize
    };
    
    let fov_degrees = config.base_fov - config.speed_fov_adjust * (cam.velocity / cam.max_velocity);
    let fov_radians = fov_degrees * f64::consts::PI / 180.0;
    // let theta_left = cam.theta - fov_radians / 2.0;
    let theta_right = cam.theta + fov_radians / 2.0;
    let cam_x = cam.x;
    let cam_y = cam.y;
    let cam_height = cam.height;

    let screen_slice_pixelwidth = config.screen_width / config.render_thread_quant;
    let screen_slice_radians = fov_radians / config.render_thread_quant as f64;

    thread::scope(|s| {
        let mut y_buf_chunks = y_buf.chunks_exact_mut(screen_slice_pixelwidth);
        let mut frame_buf_chunks = sideways_frame_buf.chunks_exact_mut(screen_slice_pixelwidth * config.screen_height);
        for t in 0..config.render_thread_quant {
            let y_buf_chunk = y_buf_chunks.next().unwrap();
            let frame_buf_chunk = frame_buf_chunks.next().unwrap();
            
            s.spawn(move|| {
                frame_buf_chunk.fill(SKY_BLUE);
                y_buf_chunk.fill(config.screen_height - 1);
                let slice_theta_right = theta_right - screen_slice_radians * t as f64;
                let sin_right = slice_theta_right.sin();
                let cos_right = slice_theta_right.cos();
                let slice_theta_left = slice_theta_right + screen_slice_radians;
                let sin_left = slice_theta_left.sin();
                let cos_left = slice_theta_left.cos();
                let mut z = 20.0;
                let mut dz = 1.0;
                while z < config.draw_distance as f64 {
                    let mut p_left = Point { x: -sin_left * z + cam_x,  y: -cos_left * z + cam_y  };
                    let p_right = Point { x: -sin_right * z + cam_x,  y: -cos_right * z + cam_y  };
                    let dx = (p_right.x - p_left.x) / screen_slice_pixelwidth as f64;
                    let dy = (p_right.y - p_left.y) / screen_slice_pixelwidth as f64;
                    for x_on_slice in 0..screen_slice_pixelwidth {
                        let color = get_color_pixel(&p_left);
                        let hmap_pixel = get_height_pixel(&p_left);
                        let height_on_screen = ((cam_height - hmap_pixel as f64) / z * config.scale_height + config.horizon) as usize;
                        if height_on_screen < y_buf_chunk[x_on_slice] {
                            for j in height_on_screen .. y_buf_chunk[x_on_slice] {
                                frame_buf_chunk[x_on_slice * config.screen_height + j] = color;
                            }
                            y_buf_chunk[x_on_slice] = height_on_screen;
                        }
                        p_left.x += dx;
                        p_left.y += dy;
                    }
                    z += dz;
                    dz += 0.0;
                }
            });
        }
    });
}

use std::sync::Arc;
use std::time::Instant;
use minifb::Key;
use crate::window;
pub fn render_v3(
    cam: &mut Camera, 
    config: &Config,
    frame_buf: &mut [u32],
    sideways_frame_buf: &mut [u32],
    y_buf: &mut [usize],
    current_map_pyrs: &TerrainPyramids,
    mut window: &mut Window,
    mut frame_counter: u128,
    mut total_rendering_time: u128
) {
    let TerrainPyramids {
        heightmap_pyr,
        colormap_pyr
    } = current_map_pyrs;
    // PROBLEM!! C5W.png and D5.png are mismatched sizes!!
    let cmap = colormap_pyr.levels[0].as_rgb8().unwrap();
    let hmap = heightmap_pyr.levels[0].as_luma8().unwrap(); 
    let get_color_pixel = |p: &Point| -> u32 {
        let [r, g, b] = 
        cmap[(p.x as u32 % cmap.width(), p.y as u32 % cmap.height())].0.map(|byte| byte as u32);
        //     cmap[(p.x.rem_euclid(cmap.width() as f64) as u32, p.y.rem_euclid(cmap.height() as f64) as u32 )].0.map(|byte| byte as u32);
        (r << 16) | (g << 8) | b
    };
    let get_height_pixel = |p: &Point| -> usize {
        // NASTY HACK to compensate for C5W.png and D5.png being mismatched sizes!!
        hmap[(p.x as u32 / 2 % hmap.width(), p.y as u32 / 2 % hmap.height())].0[0] as usize
        //     hmap[((p.x / 2.0).rem_euclid(hmap.width() as f64) as u32, (p.y / 2.0).rem_euclid(hmap.height() as f64) as u32 )].0[0] as usize
    };
    let fov_degrees = config.base_fov - config.speed_fov_adjust * (cam.velocity / cam.max_velocity);
    let fov_radians = fov_degrees * f64::consts::PI / 180.0;
    // let theta_left = cam.theta - fov_radians / 2.0;
    let mut theta_right = cam.theta + fov_radians / 2.0;
    let mut cam_x = cam.x;
    let mut cam_y = cam.y;
    let mut cam_height = cam.height;

    let screen_slice_pixelwidth = config.screen_width / config.render_thread_quant;
    let screen_slice_radians = fov_radians / config.render_thread_quant as f64;
    

    let frame_buf_lock = Arc::new(RwLock::new(frame_buf));
    // let mut cam_lock = Arc::new(RwLock::new(cam));
    // let before_render_timestamp = Instant::now();

    thread::scope(|s| {

        let mut y_buf_chunks = y_buf.chunks_exact_mut(screen_slice_pixelwidth);
        let mut frame_buf_chunks = sideways_frame_buf.chunks_exact_mut(screen_slice_pixelwidth * config.screen_height);
        
        for t in 0..config.render_thread_quant {
            let y_buf_chunk = y_buf_chunks.next().unwrap();
            let frame_buf_chunk = frame_buf_chunks.next().unwrap();
            let frame_buf_lock = frame_buf_lock.clone();
            s.spawn(move|| {
                loop {
                    frame_buf_chunk.fill(SKY_BLUE);
                    y_buf_chunk.fill(config.screen_height - 1);
                    let slice_theta_right = theta_right - screen_slice_radians * t as f64;
                    let sin_right = slice_theta_right.sin();
                    let cos_right = slice_theta_right.cos();
                    let slice_theta_left = slice_theta_right + screen_slice_radians;
                    let sin_left = slice_theta_left.sin();
                    let cos_left = slice_theta_left.cos();
                    let mut z = 20.0;
                    let mut dz = 1.0;
                    while z < config.draw_distance as f64 {
                        let mut p_left = Point { x: -sin_left * z + cam_x,  y: -cos_left * z + cam_y  };
                        let p_right = Point { x: -sin_right * z + cam_x,  y: -cos_right * z + cam_y  };
                        let dx = (p_right.x - p_left.x) / screen_slice_pixelwidth as f64;
                        let dy = (p_right.y - p_left.y) / screen_slice_pixelwidth as f64;
                        for x_on_slice in 0..screen_slice_pixelwidth {
                            let color = get_color_pixel(&p_left);
                            let hmap_pixel = get_height_pixel(&p_left);
                            let height_on_screen = ((cam_height - hmap_pixel as f64) / z * config.scale_height + config.horizon) as usize;
                            if height_on_screen < y_buf_chunk[x_on_slice] {
                                for j in height_on_screen .. y_buf_chunk[x_on_slice] {
                                    frame_buf_chunk[x_on_slice * config.screen_height + j] = color;
                                }
                                y_buf_chunk[x_on_slice] = height_on_screen;
                            }
                            p_left.x += dx;
                            p_left.y += dy;
                        }
                        z += dz;
                        dz += 0.0;
                    }
                    
                    let mut frame_buf = frame_buf_lock.write().unwrap();
                    for y in 0..config.screen_height {
                        for x in 0..screen_slice_pixelwidth {
                            frame_buf[ y * config.screen_width + (t * screen_slice_pixelwidth + x) ] =
                                frame_buf_chunk[ x * config.screen_height + y];
                        }
                    }
                }
            });
        }
        while window.is_open() && !window.is_key_down(Key::Escape) {
            // let cam = cam_lock.read().unwrap();
                    // let theta_left = cam.theta - fov_radians / 2.0;
            let before_render_timestamp = Instant::now();
            cam.react_to_inputs(&window);
            theta_right = cam.theta + fov_radians / 2.0;
            cam_x = cam.x;
            cam_y = cam.y;
            cam_height = cam.height;
            frame_counter += 1;
            let frame_buf = frame_buf_lock.read().unwrap();
            window::display_fps(&mut window, &config, &before_render_timestamp, &mut total_rendering_time, &frame_counter);
            window
                .update_with_buffer(&frame_buf, config.screen_width, config.screen_height)
                .unwrap();
        }
    });
}

use core::f64;
use crate::{
    camera::Camera,
    config::Config,
    load_maps::TerrainPyramids
};
use std::thread;

pub struct Point {
    pub x: f64,
    pub y: f64
}

static SKY_BLUE: u32 = 0x000080E0;

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
    

    // PROBLEM: C5W.png and D5.png are mismatched sizes
    let cmap = colormap_pyr.levels[0].as_rgb8().unwrap();
    let hmap = heightmap_pyr.levels[0].as_luma8().unwrap(); 
    let get_color_pixel = |p: &Point| -> u32 {
        let [r, g, b] = 
        cmap[(p.x as u32 % cmap.width(), p.y as u32 % cmap.height())].0.map(|byte| byte as u32);
        (r << 16) | (g << 8) | b
    };
    let get_height_pixel = |p: &Point| -> usize {
        // Nasty hack to compensate for C5W.png and D5.png being mismatched sizes
        hmap[(p.x as u32 / 2 % hmap.width(), p.y as u32 / 2 % hmap.height())].0[0] as usize
    };

    // Doing this instead fixes the rendering bug at negative coordinates, but causes major slowdown
        // let get_color_pixel = |p: &Point| -> u32 {
        //     let [r, g, b] = 
        //     cmap[(p.x.rem_euclid(cmap.width() as f64) as u32, p.y.rem_euclid(cmap.height() as f64) as u32 )].0.map(|byte| byte as u32);
        //     (r << 16) | (g << 8) | b
        // };
        // let get_height_pixel = |p: &Point| -> usize {
        //     hmap[((p.x / 2.0).rem_euclid(hmap.width() as f64) as u32, (p.y / 2.0).rem_euclid(hmap.height() as f64) as u32 )].0[0] as usize
        // };
    
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
                        // TODO: Implement mipmapping
                        dz += 0.0;
                    }
                    theta += d_theta;
                }
            });
        }
    });
}
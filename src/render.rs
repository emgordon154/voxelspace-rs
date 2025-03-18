use core::f64;

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

pub fn render(
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
    
    let fov_adjust = (config.fov - 90.0) * f64::consts::PI / 90.0;
    let phi_left = cam.phi - fov_adjust / 2.0;
    let sinphi_left = phi_left.sin();
    let cosphi_left = phi_left.cos();
    let phi_right = cam.phi + fov_adjust / 2.0;
    let sinphi_right = phi_right.sin();
    let cosphi_right = phi_right.cos();

    
    frame_buf.fill(SKY_BLUE);
    y_buf.fill(config.screen_height - 1);

    // where to cut the frustrum? 20?
    let mut z = 20.0;
    let mut dz = 1.0;
    while z < config.draw_distance as f64 {
        let mut pleft = Point{
            x: -cosphi_left * z - sinphi_left * z + cam.x,
            y: sinphi_left * z - cosphi_left * z + cam.y
        };
        let pright = Point{
            x: cosphi_right * z - sinphi_right * z + cam.x,
            y: -sinphi_right * z - cosphi_right * z + cam.y
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
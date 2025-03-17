// This file is filthy!!!
// It's my successful first attempt at making this whole thing work,
// by doing it as hastily and slapdash as possible.
#![deprecated]

use minifb::{Key, Window, WindowOptions};
use std::fs::File;


fn load_heightmap() -> Vec<u8> {
    let decoder = png::Decoder::new(File::open("maps/height/D5.png").unwrap()); 
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut buf).unwrap();
    return buf;
}

// pub fn load_colormap() -> Vec<[u8; 3]> {
//     // let decoder = png::Decoder::new(File::open("maps/C5W.png").unwrap()); 
//     // let mut reader = decoder.read_info().unwrap();
//     // let mut buf = vec![0; reader.output_buffer_size()];
//     // reader.next_frame(&mut buf).unwrap();
//     // return buf;
//     let buf = lodepng::decode24_file("maps/C5W.png").expect("failed to load heightmap").buffer;
//     // panic!("{}",buf.len());
//     return buf.iter().map(|rgb|[rgb.r, rgb.g, rgb.b]).collect()
// }

fn load_colormap() -> Vec<u32> {
    let buf = lodepng::decode24_file("maps/color/C5W.png").expect("failed to load heightmap").buffer;
    return buf.iter().map(|rgb|((rgb.r as u32) << 16) | ((rgb.g as u32) << 8) | rgb.b as u32).collect()
}

// ((color[0] as u32) << 16) | ((color[1] as u32) << 8) | color[2] as u32;


static WIDTH: usize = 960;
static HEIGHT: usize = 540;

struct Point {
    x: f32,
    y: f32
}

pub fn do_everything() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut y_buf = vec![HEIGHT - 1; WIDTH];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    let hmap = load_heightmap();
    let cmap = load_colormap();
    // panic!("{}", cmap.len()/1024);

    // player position
    let mut p = Point {x: 512.0, y: 512.0}; // what's the best way to keep these from going negative?
    // player height
    let mut height = 250.0;
    // player angle
    let mut phi: f32 = -2.5;

    let max_rot_velocity = 0.025;

    // let min_rot_velocity = -0.05;

    let mut velocity: f32;

    // let mut vert_velocity: f32;

    let max_velocity: f32 = 5.0;
    let mut accel_factor: f32 = 0.0;

    let mut rot_accel: f32 = 0.0;

    // how tf does this work?!
    let mut color_param: f32 = 2.0;


    // max view distance
    let distance = 600;
    // ?
    // this is only mutable for testing out weird eye-murdering visual glitch shit
    // let mut scale_height = 180.0;
    let scale_height = 180.0;
    // ?
    let horizon = (HEIGHT / 3) as f32;
    

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(0x000080E0);

        let sinphi = phi.sin();
        let cosphi = phi.cos();

        // Accelerate
        if window.is_key_down(Key::W){
            accel_factor += 0.02;
            accel_factor = accel_factor.min(6.0);
        } else if window.is_key_down(Key::S){
            accel_factor -= 0.03;
            accel_factor = accel_factor.max(0.0);
        }
        velocity = max_velocity / (1.0 + (-0.5 * (accel_factor - 4.0)).exp());
        p.y -= cosphi * velocity;
        p.x -= sinphi * velocity;

        // Yaw left
        if window.is_key_down(Key::A){
            rot_accel += 0.025;
            rot_accel = rot_accel.min(1.0);
        }
        if window.is_key_down(Key::D){
            rot_accel -= 0.025;
            rot_accel = rot_accel.max(-1.0);
        }

        // Change height (TODO: implement accerlation)
        if window.is_key_down(Key::R){
            height += 1.0;
        }
        if window.is_key_down(Key::F){
            height -= 1.0;
        }

        // Change color param
        if window.is_key_down(Key::O){
            color_param = color_param.powf(0.67);
        }
        if window.is_key_down(Key::P){
            color_param = color_param.powf(1.33);
        }

        phi += max_rot_velocity / (1.0 + (-0.5 * (rot_accel - 0.0)).exp()) - max_rot_velocity / 2.0;

        y_buf.fill(HEIGHT - 1);

        // where to cut the frustrum? 20?
        // let mut z=20;
        // let mut z = 20.0;
        let mut z= 20.0;
        // let mut dz: f32 = 1.0;
        let dz: f32 = 1.0;
        // for z in 20 .. distance as u32+1 {
        while z < distance as f32 {
            // let z = z as f32;
            let mut pleft = Point{
                x: -cosphi * z - sinphi * z + p.x,
                y: sinphi * z - cosphi * z + p.y
            };
            let pright = Point{
                x: cosphi * z - sinphi * z + p.x,
                y: -sinphi * z - cosphi * z + p.y
            };
            
            let dx = (pright.x - pleft.x) / WIDTH as f32;
            let dy = (pright.y - pleft.y) / WIDTH as f32;
            for i in 0 .. WIDTH {
                // let color = from_u8_rgb(cmap[1024 * (pleft.y as usize * 2%1024) + pleft.x as usize*2%1024]);
                let color = cmap[1024 * (pleft.y as usize * 2%1024) + pleft.x as usize*2%1024];
                let height_on_screen = ((height - hmap[512 * (pleft.y % 512.0) as usize + pleft.x as usize%512] as f32) / z * scale_height + horizon) as usize;
                if height_on_screen < y_buf[i] {
                    
                    
                    for j in height_on_screen .. y_buf[i] {
                        // why do this work over and over again?!
                        // buffer[WIDTH * j + i] = ((color[0] as u32) << 16) | ((color[1] as u32) << 8) | color[2] as u32;

                        buffer[WIDTH * j + i] = color;

                        // cool color palette
                        // buffer[WIDTH * j + i] = ((color as f32) / (distance as f32 - 50.0).max(1.0).powf(0.05)) as u32;

                        // glitchy af!!!
                        // buffer[WIDTH * j + i] = ((color as f32) / (1.0 + ((distance + j) as f32 - 200.0).max(0.0) / 250.0)) as u32;
                        // buffer[WIDTH * j + i] = ((color as f32) / (1.0 + ((distance + j) as f32 - 200.0).max(0.0) / color_param)) as u32;
                    }
                    y_buf[i] = height_on_screen;

                }

                // // draw_vertical_line(i, height_on_screen, HEIGHT, cmap[(1024 * (pleft.y as usize * 2) + pleft.x as usize*2) % (1024 * 1024)]);
                // let color = from_u8_rgb(cmap[1024 * (pleft.y as usize * 2%1024) + pleft.x as usize*2%1024]);
                // for j in height_on_screen .. HEIGHT {
                //     buffer[WIDTH * j + i] = color;
                // }


                pleft.x += dx;
                pleft.y += dy;
            }

            z += dz;
            // dz += 0.02;

            // I wonder how bad it is to have a division here
            // certainly seems unnecessary
            // dz += z / 10000.0;
            // dz += z / 10.0;


            // dz += 0.2;
            // scale_height = (1.0 + scale_height).powf(1.02) % 300.0;
            // why does even a tiny increment COMPLETELY fuck shit up?
            // scale_height = (0.03 + scale_height) % 300.0;
            // because it's carving the terrain down rather than sculpting it up :|
            // scale_height = (0.0003 + scale_height) % 300.0;
            // why does this seemingly never stop?
            // scale_height = (scale_height - 0.0003) % 300.0;
            // scale_height = (scale_height - 0.03) % 300.0;
            // if z > 300.0 {dz += 0.03;};
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
        // dbg!("frame");
    }
}

// #[inline]
// fn from_u8_rgb(rgb: u8) -> u32 {
//     let rgb = rgb as u32;
//     let (r, g, b) = ((rgb & 0b110000) >> 6, (rgb & 0b00110000) >> 4, (rgb & 0b00001100) >> 2);
//     (r << 22) | (g << 14) | (b << 6)
// }

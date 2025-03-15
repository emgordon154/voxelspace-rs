#![allow(unused)]

use std::fs::File;

pub struct Map {
    pub map: Vec<u8>,
    pub height: usize,
    pub width: usize
}

pub fn load_heightmap() -> Vec<u8> {
    let decoder = png::Decoder::new(File::open("maps/D5.png").unwrap()); 
    let mut reader = decoder.read_info().unwrap();
    println!("Heightmap info {:#?}", reader.info());
    let mut buf = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut buf).unwrap();
    return buf;
}

pub fn load_colormap() -> Vec<u32> {
    let buf = lodepng::decode24_file("maps/C5W.png").expect("failed to load heightmap").buffer;
    return buf.iter().map(|rgb|((rgb.r as u32) << 16) | ((rgb.g as u32) << 8) | rgb.b as u32).collect()
}



// fn load_heightmap_v1() -> Map {
//     // The decoder is a build for reader and can be used to set various decoding options
//     // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
//     let decoder = png::Decoder::new(File::open("maps/D5.png").unwrap()); 
//     let mut reader = decoder.read_info().unwrap();
//     let height = reader.info().height as usize;
//     let width = reader.info().width as usize;
//     // println!("{:?}", reader.info());
//     // Allocate the output buffer.
//     let mut buf = vec![0; reader.output_buffer_size()];
//     // Read the next frame. An APNG might contain multiple frames.
//     let info = reader.next_frame(&mut buf).unwrap();
//     // Grab the bytes of the image.
//     // let bytes = &buf[..info.buffer_size()];

//     // D5.png is 512 x 512, 8bit grayscale
//     return Map {
//         map: buf,
//         height,
//         width
//     }

// }


// fn load_colormap_v1() -> Map {
//     let decoder = png::Decoder::new(File::open("maps/C5W.png").unwrap()); // C5W.png is 1024 x 1024, 8bit Indexed color_type
//     let mut reader = decoder.read_info().unwrap();
//     let height = reader.info().height as usize;
//     let width = reader.info().width as usize;
//     let mut buf = vec![0; reader.output_buffer_size()];
//     let info = reader.next_frame(&mut buf).unwrap();
//     return Map {
//         map: buf,
//         height,
//         width
//     }
// }
use image::{ImageReader, DynamicImage, ImageResult};

fn load_heightmap(map_name: &str) -> ImageResult<DynamicImage> {
    let path = String::from("maps/height/") + map_name;
    let img = ImageReader::open(path)?.with_guessed_format()?.decode()?;
    Ok(img)
}

fn load_colormap(map_name: &str) -> ImageResult<DynamicImage> {
    let path = String::from("maps/color/") + map_name;
    let img = ImageReader::open(path)?.with_guessed_format()?.decode()?;
    Ok(img)
}

use image_pyramid::*;

fn pyramidize(map: DynamicImage) -> Result<ImagePyramid, image_pyramid::ImagePyramidError> {
    let params = ImagePyramidParams {
        scale_factor: UnitIntervalValue::new(1.0 / 2.0).unwrap(),
        pyramid_type: ImagePyramidType::Lowpass,
        smoothing_type: SmoothingType::Gaussian
    };

    ImagePyramid::create(&map, Some(&params))
}

pub struct TerrainPyramids {
    pub heightmap_pyr: ImagePyramid,
    pub colormap_pyr: ImagePyramid
}

use std::error::Error;

pub fn load_maps(heightmap_name: &str, colormap_name: &str) -> Result<TerrainPyramids, Box<dyn Error>> {
    let heightmap = load_heightmap(heightmap_name)?;
    let heightmap_pyr = pyramidize(heightmap)?;
    let colormap = load_colormap(colormap_name)?;
    let colormap_pyr = pyramidize(colormap)?;
    Ok( TerrainPyramids {
        heightmap_pyr,
        colormap_pyr
    })
}
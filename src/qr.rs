use std::path::PathBuf;

use image;
use rqrr;

pub fn extract(path: PathBuf) -> Vec<String> {
    // Load on image to search, convert it to grayscale
    let img = image::open(path).unwrap().to_luma8();
    // Prepare for detection
    let mut img = rqrr::PreparedImage::prepare(img);
    // Search for grids, without decoding
    let grids = img.detect_grids();

    grids
        .into_iter()
        .map(|grid| {
            let (_meta, content) = grid.decode().unwrap();
            content
        })
        .collect()
}

mod n_space_util;
mod worley_noise;

// use std::env;
use image::GrayImage;
use chrono::{DateTime, Utc};



fn main() {
    let mut img = GrayImage::new(1024, 1024);
    img = worley_noise::do_frag(img);

    let now: DateTime<Utc> = Utc::now();
    let _ = img.save(format!("output/test{}.jpg", now.timestamp()));
    let _ = img.save("frame.jpg");
    println!("Done! Image output!");
}

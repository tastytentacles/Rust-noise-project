mod n_space_util;
use n_space_util::*;

use rand::Rng;
use std::time::Instant;
use std::thread::spawn;
use chrono::{DateTime, Utc};
use image::{GrayImage, Luma};
use vecmath::{Vector2, vec2_sub, vec2_add, vec2_len};


const NULL_PPOINT: Vector2<f32> = [-2.0, -2.0];

const WIDTH_SEGS: usize = 8;
const POINT_SIZE: usize = 128;
const POINT_EDGE_SIZE: usize = 8;
const POINT_RAD: f32 = 0.05;
const POINT_SHRINK: f32 = 8.0;



// fragment code
fn frag(uv: Vector2<f32>, points: [Vector2<f32>; POINT_SIZE]) -> f32 {
    let mut rtn: f32 = 1.0;
    
    for n in 0..POINT_SIZE {
        rtn = f32::min(vec2_len(vec2_sub(uv, points[n])) * POINT_SHRINK, rtn);
    }

    return 1.0 - rtn;
}

// populate data and apply fragment shader
fn do_frag(mut img: GrayImage) -> GrayImage {
    let dtime = Instant::now();
    println!("{}", "Starting fragment shader!");

    let mut rng = rand::thread_rng();
    let mut points: [Vector2<f32>; POINT_SIZE] = [NULL_PPOINT; POINT_SIZE];
    

    // edge point cycle top / bottom
    for n in 0..POINT_EDGE_SIZE {
        let mut spec_point: Vector2<f32> = [rng.gen(), 0.0];

        let mut tries: u8 = 32;
        let mut fail_falg: bool = false;
        while tries > 0 {
            for nn in 0..n {
                if vec2_len(vec2_sub(spec_point, points[nn])) < POINT_RAD * 2.0 {
                    fail_falg = true;
                }
            }
            
            if fail_falg {
                spec_point = [rng.gen(), 0.0];
                fail_falg = false;
                tries -= 1;
            } else {
                points[n] = spec_point;
                tries = 0;
            }
        }
    }

    for n in POINT_EDGE_SIZE..POINT_EDGE_SIZE * 2 {
        points[n] = vec2_add(points[n - POINT_EDGE_SIZE], [0.0, 1.0]);
    }


    // edge point cycle left / right
    for n in POINT_EDGE_SIZE * 2..POINT_EDGE_SIZE * 3 {
        let mut spec_point: Vector2<f32> = [0.0, rng.gen()];

        let mut tries: u8 = 32;
        let mut fail_falg: bool = false;
        while tries > 0 {
            for nn in 0..n {
                if vec2_len(vec2_sub(spec_point, points[nn])) < POINT_RAD * 2.0 {
                    fail_falg = true;
                }
            }
            
            if fail_falg {
                spec_point = [0.0, rng.gen()];
                fail_falg = false;
                tries -= 1;
            } else {
                points[n] = spec_point;
                tries = 0;
            }
        }
    }

    for n in POINT_EDGE_SIZE * 3..POINT_EDGE_SIZE * 4 {
        points[n] = vec2_add(points[n - POINT_EDGE_SIZE], [1.0, 0.0]);
    }

    println!("{} {}", dtime.elapsed().as_secs_f32(), "Edge points plotted!");


    // center point cycle
    let mut skips: u32 = 0;
    for n in 0..POINT_SIZE {
        if points[n] == NULL_PPOINT {
            let mut spec_point: Vector2<f32> = [rng.gen(), rng.gen()];
        
            let mut tries: u8 = 32;
            let mut fail_falg: bool = false;
            while tries > 0 {
                for nn in 0..POINT_SIZE {
                    if n != nn {
                        if vec2_len(vec2_sub(spec_point, points[nn])) < POINT_RAD * 2.0 {
                            fail_falg = true;
                        }
                    }
                }
                
                if fail_falg {
                    spec_point = [rng.gen(), rng.gen()];
                    fail_falg = false;
                    tries -= 1;
                } else {
                    points[n] = spec_point;
                    tries = 0;
                }
            }
        } else {
            skips += 1;
        }
    }
    
    println!("{} Center Points Plotted! {} skips", dtime.elapsed().as_secs_f32(), skips);


    // break up image and spawn threads
    let thread_count = WIDTH_SEGS * WIDTH_SEGS;
    let mut threads = vec!();
    let size: Vector2<u32> = [
        img.width() / WIDTH_SEGS as u32,
        img.height() / WIDTH_SEGS as u32 ];

    for n in 0..thread_count {
        threads.push(spawn(move || {
            let mut frame = GrayImage::new(size[0], size[1]);

            for x in 0..size[0] {
                for y in 0..size[1] {
                    let n_space = n_space_unwrap(n, WIDTH_SEGS);
                    let uv: Vector2<f32> = [
                        (x + n_space[0] * size[0]) as f32 / (size[0] * WIDTH_SEGS as u32) as f32,
                        (y + n_space[1] * size[1]) as f32 / (size[1] * WIDTH_SEGS as u32) as f32
                    ];

                    frame.put_pixel(x, y, Luma([(255.0 * frag(uv, points)) as u8]));
                }
            }

            // ugly frame edge highlighting
            // for x in 0..frame.width() {
            //     for y in 0..frame.height() {
            //         if x < 2 || x > frame.width() - 2 || y < 2 || y > frame.width() - 2 {
            //             frame.put_pixel(x, y, Luma([0]));
            //         }
            //     }
            // }

            return (frame, n);
        }));
    }

    // stich image back together
    for n in threads {
        let mut thread_frame = n.join().unwrap();
        let n_sapce = n_space_unwrap(thread_frame.1, WIDTH_SEGS);
        image::imageops::overlay(
                &mut img,
                &mut thread_frame.0,
                n_sapce[0] as u32 * size[0],
                n_sapce[1] as u32 * size[1] );
    }

    println!("{} {}", dtime.elapsed().as_secs_f32(), "Frag applied!");
    return img;
}

fn main() {
    let mut img = GrayImage::new(1024, 1024);
    img = do_frag(img);

    let now: DateTime<Utc> = Utc::now();
    let _ = img.save(format!("output/test{}.jpg", now.timestamp()));
    let _ = img.save("frame.jpg");
    println!("Done! Image output!");
}

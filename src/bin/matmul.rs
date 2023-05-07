extern crate anyhow;

use anyhow::*;
use rand::{self, Rng};
use std::{
    env,
    sync::{Arc, Mutex},
    thread,
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Oops!");
    }

    let interval = args[1].parse::<usize>()?;
    let thread = args[2].parse::<usize>()?;

    assert!(interval % thread == 0);

    let iterations = interval * interval / thread;

    let mut handles = vec![];

    let circle_points = Arc::new(Mutex::new(0));
    let square_points = Arc::new(Mutex::new(0));

    // Spawn threads to calculate for us
    for _ in 0..thread {
        let circle_points = Arc::clone(&circle_points);
        let square_points = Arc::clone(&square_points);

        let handle = thread::spawn(move || {
            // TODO: See if these have to be moved or not
            let mut cp = circle_points.lock().unwrap();
            let mut sp = square_points.lock().unwrap();

            let mut rng = rand::thread_rng();

            (0..iterations).into_iter().for_each(|_| {
                let rand_x: f32 = rng.gen_range(-1.0..1.0);
                let rand_y: f32 = rng.gen_range(-1.0..1.0);

                let origin_dist: f32 = rand_y * rand_y + rand_x * rand_x;

                // NOTE: This block must be safe

                if origin_dist <= 1.0 {
                    *cp += 1;
                }

                *sp += 1;
            });
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let pi: f32 =
        (4.0 * *circle_points.lock().unwrap() as f32) / *square_points.lock().unwrap() as f32;

    println!("Rust pi is {:.6}", pi);

    Ok(())
}

#[test]
fn accuracy() {
    unimplemented!();
}

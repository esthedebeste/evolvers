use clap::Parser;
use gerald::{pick_parent, Fitness, Gerald};
use image::RgbImage;

use pool::Pool;

pub mod gerald;
pub mod pool;

impl Gerald for RgbImage {
    type Context = RgbImage;
    fn cross(a: &Self, b: &Self) -> Self {
        let mut new = RgbImage::new(a.width(), a.height());
        for (x, y, pixel) in new.enumerate_pixels_mut() {
            let new = pick_parent(a, b).get_pixel(x, y);
            if rand::random::<f64>() < 0.001 {
                *pixel = image::Rgb(
                    [
                        new[0] as i32 + (rand::random::<i8>() / 4) as i32,
                        new[1] as i32 + (rand::random::<i8>() / 4) as i32,
                        new[2] as i32 + (rand::random::<i8>() / 4) as i32,
                    ]
                    .map(|x| x as u8),
                );
            } else {
                *pixel = *new;
            }
        }
        new
    }

    fn fitness(&self, target: &RgbImage) -> Fitness {
        let mut distance = 0;
        for (x, y, pixel) in self.enumerate_pixels() {
            let target_pixel = target.get_pixel(x, y);
            let r = (pixel[0] as i32 - target_pixel[0] as i32).abs();
            let g = (pixel[1] as i32 - target_pixel[1] as i32).abs();
            let b = (pixel[2] as i32 - target_pixel[2] as i32).abs();
            distance += r + g + b;
        }
        Fitness::MAX - distance // higher is better so we invert the distance
    }
}

fn random_img(target: &RgbImage) -> RgbImage {
    let mut img = RgbImage::new(target.width(), target.height());
    for pixel in img.pixels_mut() {
        *pixel = image::Rgb([rand::random(), rand::random(), rand::random()]);
    }
    img
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// How many generations to run before saving curr.png
    #[arg(short, long, default_value_t = 100)]
    step_size: usize,

    /// How many geralds to keep in the gerald pool
    #[arg(short, long, default_value_t = 1_000)]
    pool_size: usize,

    /// Target image path
    #[arg(short, long, default_value = "target.png")]
    target_image: String,

    /// Current image path
    #[arg(short, long, default_value = "curr.png")]
    current_image: String,
}

fn main() {
    let args = Args::parse();
    let target = image::open(args.target_image)
        .expect("error when opening target image :(")
        .to_rgb8();
    let mut pool = Pool::new(|ctx, _| random_img(ctx), args.pool_size, target);
    let mut i = 0;
    loop {
        pool.run();
        if i % args.step_size == 0 {
            let best = pool.best();
            best.gerald.save(&args.current_image).unwrap();
            println!("{i}: {}", Fitness::MAX - best.fitness);
        }
        pool.cross();
        i += 1;
    }
}

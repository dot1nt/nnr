use image::GenericImageView;
use image::GenericImage;
use std::env::args;

pub struct Args {
    crop: bool,
    threshold: f32,
    filter: bool,
    input: String,
    output: String
}

impl Args {
    fn parse() -> Args {
        let args: Vec<String> = args().collect();

        if args.len() < 3 { usage(); std::process::exit(1) }

        let mut cmd_args = Args {
            crop: false,
            threshold: 0.5,
            filter: false,
            input: String::new(),
            output: String::new(),
        };

        cmd_args.output = args[args.len()-1].to_string();
        cmd_args.input = args[args.len()-2].to_string();

        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "-c" | "--crop" => cmd_args.crop= true,
                "-t" | "--threshold" => {
                    cmd_args.threshold = args[i+1].parse::<f32>().unwrap_or_else(|_| {
                        eprintln!("Threshold is not a floating point number");
                        usage();
                        std::process::exit(1)
                    })
                },
                "-f" | "--filter"    => cmd_args.filter = true,
                "-h" | "--help"      => { usage(); std::process::exit(0) },
                _=> ()
            }
        }

        cmd_args
    }
}


fn usage() {
    println!("
Usage:
nnr <options> <input> <output>

Options:
-c         |    Crop noise
-t <val>   |    Noise threshold; 0.0 to 1.0; Default: 0.5
-f         |    Filter noise with median filter
");
}

fn median_filter(img: &mut image::DynamicImage) {
    let size: i32 = 1;
    let median = (((size*2+1).pow(2)+1)/2-1) as usize;

    let (w, h) = img.dimensions();
    let mut new_img = img.clone();

    for x in 0..w {
        for y in 0..h {

            let mut window = Vec::new();

            for wx in -size..size+1 {
                for wy in -size..size+1 {
                    let mut px = (x as i32 + wx).abs() as u32;
                    let mut py = (y as i32 + wy).abs() as u32;

                    if px > w-1 { px = w-1 }
                    if py > h-1 { py = h-1 }

                    let pix = img.get_pixel(px , py);
                    window.push(pix[0])
                }
            }

            window.sort_unstable();
            let np = window[median];
            new_img.put_pixel(x, y, image::Rgba([np, np, np, 255]));
        }
    }

    *img = new_img;
}

fn get_noise_estimation(img: &image::DynamicImage) -> Vec<u32> {
    let (w, h) = img.dimensions();

    let mut noise = Vec::new();
    for y in 0..h {
        let mut sum: u32 =  0;

        for x in 0..w {
            let pix1 = if x == 0 {
                img.get_pixel(w-1, y).0[0] as i32
            } else {
                img.get_pixel(x-1, y).0[0] as i32
            };

            let pix2 = img.get_pixel(x, y).0[0] as i32;

            sum += (pix1 - pix2).abs() as u32
        }

        noise.push(sum)
    }

    noise
}

fn crop_noise(img: &mut image::DynamicImage, threshold: &f32) {
    let noise = get_noise_estimation(img);

    let (w, h) = img.dimensions();

    let noise_min = *noise.iter().min().unwrap() as f32;
    let noise_max = *noise.iter().max().unwrap() as f32;

    let threshold = (noise_min + threshold * (noise_max - noise_min)) as u32;

    let mut points = Vec::new(); // points where noise crosses threshold

    for y in 0..h {
        let p1 = noise[y as usize];

        let p2 = if y == 0 { 
            noise[noise.len() - 1] 
        } else {
            noise[(y - 1) as usize]
        };

        if p1 < threshold && p2 > threshold || p1 > threshold && p2 < threshold {
            points.push(y)
        }
    }

    if points.is_empty() { return }

    let top = points[0];
    let bottom = points[points.len()-1];

    *img = img.crop(0, top, w, bottom-top);
}

fn main() {
    let args = Args::parse();

    let mut img = image::open(&args.input).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1)
    });

    if args.filter {
        median_filter(&mut img)
    }

    if args.crop {
        crop_noise(&mut img, &args.threshold);
    }

    img.save(&args.output).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1)
    });
}
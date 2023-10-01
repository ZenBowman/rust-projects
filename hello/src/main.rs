use image::codecs::png::PngEncoder;
use image::ColorType;
use image::ImageEncoder;
use show_image::create_window;
use show_image::ImageInfo;
use show_image::ImageView;
use std::env;
use std::fs::File;
use std::str::FromStr;
use std::thread;
use std::time;

fn gcd(mut n: u16, mut m: u16) -> u16 {
    // Assert that both arguments are greater than zero.
    assert!(n > 0);
    assert!(m > 0);

    // Euclid's algorithm.
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    return n;
}

// Mutates the row of a 2d dimensional array stored in `pixels`.
// The array is stored in row-major order, so the index of column i in row n
// is n * row_width + i.
fn color_row(pixels: &mut [u8], row: u16, num_pixels_to_color_in_row: u16, row_width: u16) {
    let start_index: usize = (row * row_width).into();
    let end_index = start_index + (num_pixels_to_color_in_row * 10) as usize;
    let mut index = start_index;
    while index < end_index {
        pixels[index] = 255;
        index += 1;
    }
}

fn color_rows(pixels: &mut [u8], rows: &[u16], num_pixels_to_color_in_row: u16, row_width: u16) {
    for element in rows.iter() {
	color_row(pixels, *element, num_pixels_to_color_in_row, row_width);
    }
}

#[show_image::main]
fn main() {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(
            u16::from_str(&arg).expect("Unable to convert argument to unsigned 64 bit number"),
        );
    }

    if numbers.len() != 2 {
        eprintln!("Usage: hello X Y, where X and Y are u64 numbers");
        std::process::exit(1);
    }

    let gcd_result = gcd(numbers[0], numbers[1]);
    println!("Namaste, world!");
    println!(
        "The GCD of {} and {} is {}",
        numbers[0], numbers[1], gcd_result,
    );

    let row_width = std::cmp::max(numbers[0], numbers[1]) * 10;
    let row_height = 60;
    let mut pixels = vec![0; ((row_width * row_height) as usize).try_into().unwrap()];

    color_rows(&mut pixels, &[0,1], numbers[0], row_width);

    color_rows(&mut pixels, &[10,11], numbers[1], row_width);

    color_rows(&mut pixels, &[20,21], gcd_result, row_width);

    let output = File::create("outputgcd.png").unwrap();
    let encoder = PngEncoder::new(output);
    encoder
        .write_image(&pixels, row_width as u32, row_height as u32, ColorType::L8)
        .expect("Error writing png file");

    let image = ImageView::new(
        ImageInfo::mono8(row_width.into(), row_height.into()),
        &pixels,
    );
    let window = create_window("image", Default::default()).expect("Unable to create window");
    window
        .set_image("image-001", image)
        .expect("Unable to display image");

    thread::sleep(time::Duration::from_secs(10));
}

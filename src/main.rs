use std::path::Path;
use std::fs;
use image::{ImageBuffer ,RgbaImage,Rgba};

struct RgbaData {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

fn main() {

    let file_path = "next.txt";    
    println!("In file, {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file.");
    let contents_into_bytes = contents.into_bytes();

    let rgba_data_list: Vec<RgbaData> = contents_into_bytes
    .iter()
    .map(|&byte| byte_to_rgba(byte))
    .collect();

    let content_size = rgba_data_list.len();
    println!("Content size {content_size}");
    
    // image resolution
    let width = 256;
    let height = (content_size as f64 / width as f64).ceil() as u32;

    // Create ImageBuffer
    let mut img: RgbaImage = ImageBuffer::new(width, height);

    // Generate color
    let mut pixel_index = 0;
    for pixel in img.pixels_mut() {
        if pixel_index < content_size {
            let rgba_data = &rgba_data_list[pixel_index];
            *pixel = Rgba([rgba_data.r,rgba_data.g,rgba_data.b,rgba_data.a]);
            pixel_index += 1;
        }
        else {
            // Fill remaining pixels with black;
            *pixel = Rgba([0,0,0,255]);
        }
    }

    let output_filename = format!("{}.png", get_file_stem(file_path));
    img.save(&output_filename).expect("Failed to save image");
    println!("Image saved as: {}", output_filename);
}

fn byte_to_rgba(byte: u8) -> RgbaData {
    let binary_str = format!("{:08b}",byte);

    let r_bits = &binary_str[0..2];
    let g_bits = &binary_str[2..4];
    let b_bits = &binary_str[4..6];
    let a_bits = &binary_str[6..8];

    let r = binary_to_u8(r_bits) * 85;
    let g = binary_to_u8(g_bits) * 85;
    let b = binary_to_u8(b_bits) * 85;
    let a = binary_to_u8(a_bits) * 85;

    RgbaData { r, g, b, a }
}

fn binary_to_u8(binary_str: &str) -> u8 {
    u8::from_str_radix(binary_str, 2).unwrap()
}

fn get_file_stem(file_path: &str) -> String {
    Path::new(file_path)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}
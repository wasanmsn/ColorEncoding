use std::path::Path;
use std::fs;
use image::{ImageBuffer ,RgbaImage,Rgba,ImageReader};

const BYTES_PER_PIXEL: u8 = 4;
fn main() {
    let file_path = "next.txt";    
    encode(file_path);
    let image_path = "next.png";
    decode(image_path);
}

fn decode(image_path:&str) {
    let img = ImageReader::open(image_path)
    .expect("Failed to open image")
    .decode()
    .expect("Failed to decode image");

    let rgba_image = img.to_rgba8();

    let mut bytes:Vec<u8> = Vec::new();

    for pixel in rgba_image.pixels() {
        let [r, g, b, a] = pixel.0;

        // check for padding pixels
        if r == 0 && g == 0 && b == 0 && a == 255 {
            continue;
        }
        bytes.push(r);
        bytes.push(g);
        bytes.push(b);
        bytes.push(a);

    }

    let text = byte_to_character(bytes);
    println!("{text}");
}

fn encode(file_path: &str){
    println!("In file, {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file.");
    let contents_into_bytes = contents.into_bytes();

    let content_size = contents_into_bytes.len();
    println!("Content size {content_size}");
    
    // image resolution
    let width = 256;
    let height = ((content_size as f64 )/ (width as f64 * BYTES_PER_PIXEL as f64)).ceil() as u32;

    // Create ImageBuffer
    let mut img: RgbaImage = ImageBuffer::new(width, height);

    // Generate color
    let mut pixel_index = 0;
    for pixel in img.pixels_mut() {
        if pixel_index < content_size {
            // 1 pixel in image eqauls 4 characters
            let rgba_data: Vec<u8> = contents_into_bytes.get(pixel_index..pixel_index + 4)
            .unwrap_or(&[0,0,0,255])
            .to_vec();
            *pixel = Rgba([rgba_data[0],rgba_data[1],rgba_data[2],rgba_data[3]]);
            pixel_index += 4;
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

fn byte_to_character(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes).expect("Invalid UTF-8 sequence")
}

fn get_file_stem(file_path: &str) -> String {
    Path::new(file_path)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}
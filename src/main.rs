use std::path::Path;
use std::fs;
use image::{ImageBuffer ,RgbaImage,Rgba,ImageReader};

struct RgbaData {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

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

    let mut binary_string:Vec<String> = Vec::new();

    for pixel in rgba_image.pixels() {
        let [r, g, b, a] = pixel.0;

        // check for padding pixels
        if r == 0 && g == 0 && b == 0 && a == 255 {
            continue;
        }
        let binary = rgba_to_binary(r, g, b, a);
        binary_string.push(binary);
    }

    let mut bytes:Vec<u8> = Vec::new();

    for binary in binary_string {
        let byte = binary_to_u8(&binary);
        bytes.push(byte);
    }

    let text = byte_to_character(bytes);
    println!("{text}");
}

fn encode(file_path:&str) {
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

fn byte_to_character(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes).expect("Invalid UTF-8 sequence")
}

fn binary_to_u8(binary_str: &str) -> u8 {
    u8::from_str_radix(binary_str, 2).unwrap()
}

fn u8_to_binary(bits:u8) -> String {
    format!("{:02b}",bits)
}

fn rgba_to_binary(r:u8,g:u8,b:u8,a:u8) -> String {
    //rr000000 represent r position in byte
    let r_bits = u8_to_binary(r/85);
    //rrGG0000 represent G position in byte
    let g_bits = u8_to_binary(g/85);
    //rrGGbb00 represent b position in byte
    let b_bits = u8_to_binary(b/85);
    //rrGGbbAA represent A position in byte
    let a_bits = u8_to_binary(a/85);

    format!("{r_bits}{g_bits}{b_bits}{a_bits}")

}

fn get_file_stem(file_path: &str) -> String {
    Path::new(file_path)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}
# Text to Image Encoder

This Rust program reads a text file, converts its contents into binary data, maps the binary data to RGBA color values, and generates an image based on that data. Each character in the text file is represented as a pixel in the image.

## How It Works

- **Reading the File**: The program reads the contents of a text file (`next.txt` by default).
- **Converting to Binary**: Each character is converted into its binary representation (8 bits).
- **Mapping to RGBA**:
  - The 8 bits are divided into four 2-bit segments.
  - Each segment corresponds to one of the RGBA components (Red, Green, Blue, Alpha).
  - The 2-bit values (ranging from 0 to 3) are scaled to the 0-255 range by multiplying by 85.
- **Creating the Image**:
  - An image buffer is created with a fixed width (256 pixels).
  - The height is calculated based on the number of characters in the text file.
  - Each pixel in the image is assigned the RGBA color derived from the corresponding character.
- **Saving the Image**: The image is saved as a PNG file with the same name as the input file.

## Usage

1. **Prepare the Text File**:
   - Create a text file named `next.txt` in the same directory as the program.
   - Add the text content you want to encode into the image.

2. **Run the Program**:
   - Ensure you have Rust installed.
   - Build and run the program using Cargo:

     ```bash
     cargo run
     ```

3. **View the Output**:
   - After running, the program will generate an image file (`next.png`).
   - Open the image file using an image viewer to see the visual representation of your text.

## Customization

- **Input File**:
  - To use a different input file, modify the `file_path` variable in the `main` function:

    ```rust
    let file_path = "your_file.txt";
    ```

- **Image Dimensions**:
  - You can adjust the `width` variable to change the width of the output image.
  - The height is automatically calculated to accommodate all the pixels.

## Dependencies

- [image](https://crates.io/crates/image): For image creation and manipulation.

## Code Explanation

- **`RgbaData` Struct**: Holds the RGBA values for a pixel.
- **`byte_to_rgba` Function**: Converts a byte into `RgbaData` by mapping bits to color components.
- **`binary_to_u8` Function**: Converts a binary string to a `u8` integer.
- **`get_file_stem` Function**: Extracts the filename without extension from a file path.

## Example

Given the text file `next.txt` containing:


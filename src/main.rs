use std::fs::File;
use std::io::Write;
use base64::engine::general_purpose;
use base64::Engine as _; // To bring the Engine trait in scope

// Import the base64-encoded image string from the other file
mod encoded_image;

fn main() {
    // Get the base64-encoded image string from the constant
    let base64_str = encoded_image::BASE64_IMAGE;

    // Decode the base64 string into raw bytes
    let image_bytes = general_purpose::STANDARD
        .decode(base64_str)
        .expect("Failed to decode base64 string");

    // Create and write the raw image bytes to a JPG file
    let mut output_file = File::create("output_image.jpg")
        .expect("Failed to create output file");
    output_file
        .write_all(&image_bytes)
        .expect("Failed to write image to file");

    println!("Image written to output_image.jpg");
}

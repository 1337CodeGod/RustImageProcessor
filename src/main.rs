extern crate image;
use image::{DynamicImage, ImageBuffer, Rgba};
use std::fs;
use std::path::Path;


fn main() {
    // Set the path to the folder containing the images
    let path = Path::new(r"C:\Users\JM\Pictures\Test Images");

    // Read all files in the folder
    let files = fs::read_dir(path).unwrap();

    // Iterate over the files
    for file in files {
        let file = file.unwrap();

        // Check if the file is a JPG, JPEG, or PNG image
        let extension = file.path().extension().unwrap().to_str().unwrap().to_lowercase();
        if extension == "jpg" || extension == "jpeg" || extension == "png" {
            // Load the image
            let image = image::open(file.path()).unwrap();

            // Invert the image
            let inverted = invert_image(image);

            // Save the inverted image
            let output_path = file.path().with_extension("inverted.jpg");
            inverted.save(output_path).unwrap();
        }


    }
}

// Invert the colors of an image
fn invert_image(image: DynamicImage) -> DynamicImage {
    let image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = image.into_rgba8();
    let (width, height) = image_buffer.dimensions();
    let mut inverted_buffer = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let pixel = image_buffer.get_pixel(x, y);
            inverted_buffer.put_pixel(x, y, invert_pixel(pixel));
        }
    }

    DynamicImage::ImageRgba8(inverted_buffer)
}

// Invert a pixel
fn invert_pixel(pixel: &Rgba<u8>) -> Rgba<u8> {
    let red = 255 - pixel[0];
    let green = 255 - pixel[1];
    let blue = 255 - pixel[2];
    let alpha = pixel[3];

    Rgba([red, green, blue, alpha])
}




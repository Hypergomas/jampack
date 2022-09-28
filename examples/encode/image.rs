// This example demonstrates how to create a simple texture encoder using JamPack
// It cooks raw images into RGBA data

const INPUT_DIR: &str = "./res-src"; // It is recommended that you save
const OUT_DIR: &str   = "./res";     // to different directories.

fn main() {
    // Create out stove, adding in our recipes
    let stove = jampack::Stove::new()
        .unwrap()
        .with_recipe(image_recipe());
    
    // Tell our stove to cook the assets
    stove.cook(INPUT_DIR, OUT_DIR);
}

fn image_recipe() -> jampack::Recipe {
    fn bake(data: Vec<u8>) -> jampack::Jar {
        let image = image::load_from_memory(&data)
            .unwrap();
        let rgba = image.to_rgba8();
        let mut rgba_buffer: Vec<u8> = Vec::new();
        for pixel in rgba.pixels() {
            rgba_buffer.push(pixel[0]);
            rgba_buffer.push(pixel[1]);
            rgba_buffer.push(pixel[2]);
            rgba_buffer.push(pixel[3]);
        }

        jampack::Jar::new(
            128, // Jam category, representing an asset type
            0, // Jam type, a kind of sub-category
            rgba_buffer, // Our parsed RGBA data
        )
    }
    jampack::Recipe::new(
        bake, // The function that parses the input data
        vec!["png", "jpg", "jpeg", "bmp", "tiff"], // The formats supported by our recipe
    )
}
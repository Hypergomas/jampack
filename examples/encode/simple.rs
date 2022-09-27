// This example demonstrates how to create a simple asset passthrough using JamPack

const INPUT_DIR: &str = "./res-src"; // It is recommended that you save
const OUT_DIR: &str   = "./res";     // to different directories.

fn main() {
    // Create out stove, adding in our recipes
    let stove = jampack::Stove::new()
        .unwrap()
        .with_recipe(passthrough_recipe());
    
    // Tell our stove to cook the assets
    stove.cook(INPUT_DIR, OUT_DIR);
}

fn passthrough_recipe() -> jampack::Recipe {
    fn bake(data: Vec<u8>) -> jampack::Jar {
        jampack::Jar::new(
            128, // Jam category, representing an asset type
            0,    // Jam type, a kind of sub-category
            data,    // We don't want to do any data manipulation
        )
    }
    jampack::Recipe::new(
        bake, // The function that parses the input data
        vec!["bin"], // The formats supported by our recipe
    )
}
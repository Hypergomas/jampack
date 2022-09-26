fn main() {
    let stove = jampack::Stove::new()
        .unwrap()
        .with_recipe(image_recipe());
    
    stove.cook("./.testing/res-src", "./.testing/res");
}

fn image_recipe() -> jampack::Recipe {
    fn bake(data: Vec<u8>) -> jampack::Jar {
        jampack::Jar::new(128, 0, data)
    }
    jampack::Recipe::new(bake, vec!["png"])
}
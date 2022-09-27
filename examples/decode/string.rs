// This example demonstrates how to decode a string that has been converted to bytes using JamPack

const INPUT_DIR: &str = "./res";

fn main() {
    let my_string = jampack::open<String>(format!("{}/my_string", INPUT_DIR))
        .unwrap();
    println!("{:?}", my_string);
}

impl jampack::Jam for String {
    fn unjar(_ty: u8, data: Vec<u8>) -> jampack::Result<Self> {
        match Self::from_utf8(data) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.to_string()),
        }
    }
    fn jam_idx() -> u8 {
        128
    }
}
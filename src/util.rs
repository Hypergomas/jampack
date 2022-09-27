/// Converts a u8 to a 3-digit string, used for hashing
pub fn u8_to_string(x: u8) -> String {
    let base = x.to_string();
    match base.len() {
        1 => format!("00{}", base),
        2 => format!("0{}", base),
        3 => base,
        _ => String::new(),
    }
}
use std::fs;
use tracing::info;

pub fn banner() {
    match fs::read_to_string("src/banner.txt") {
        Ok(str) => {
            println!("{}", str);
            return;
        }
        Err(_) => print!(""),
    };

    match fs::read_to_string("src/banner/banner.txt") {
        Ok(str) => {
            println!("{}", str);
            return;
        }
        Err(_) => print!(""),
    };

    info!("banner not found")
}

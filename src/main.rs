use std::error::Error;
mod init;

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, World!");
    init::init()
}

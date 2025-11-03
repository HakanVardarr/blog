use blog::metadata::*;
use std::{fs, io::Read};

const FILE_PATH: &str = "rust-ile-blog-yapmak.md";

pub fn run() -> anyhow::Result<()> {
    let mut file = fs::File::open(FILE_PATH)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    if content.starts_with("---") {
        if let Some(position) = content[4..].find("---") {
            let metadata = Metadata::new(&content[4..position + 3])?;
            println!("{metadata:?}");
        }
    }

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("ERROR: {e}")
        }
    }
}

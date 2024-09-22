use std::error::Error;

use crate::utils::{decompress_file, get_path};

pub fn cat_file(directory: &str, file: &str) -> Result<(), Box<dyn Error>> {
    let decompressed_content = decompress_file(&get_path(directory, file))?;
    let mut null_index = None;
    for (index, value) in decompressed_content.as_bytes().iter().enumerate() {
        if *value == b'\n' {
            null_index = Some(index);
            break;
        }
    }

    match null_index {
        Some(i) => {
            println!("{}", decompressed_content[i + 1..].to_string());
        }
        None => panic!("error: not a valid header"),
    };

    Ok(())
}

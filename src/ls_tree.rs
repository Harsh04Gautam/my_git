use crate::utils::{decompress_file, get_path};
use std::error::Error;

pub fn ls_tree(directory: &str, file: &str) -> Result<(), Box<dyn Error>> {
    let decompressed_content = decompress_file(&get_path(directory, file))?;

    for entry in decompressed_content.lines() {
        if entry.starts_with("100644") {
            let file_name = entry.split(' ').last();
            match file_name {
                Some(file_name) => println!("{file_name}"),
                None => panic!("error: name missing"),
            }
        }
    }
    Ok(())
}

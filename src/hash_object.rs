use crate::utils::{compress_file, Type};

use std::error::Error;
use std::io::Write;

pub fn hash_object(file_path: &str) -> Result<(), Box<dyn Error>> {
    let compressed = compress_file(file_path, Type::Blob)?;
    compressed.object.create_dir()?;
    compressed.object.create_file()?.write(&compressed.data)?;
    Ok(())
}

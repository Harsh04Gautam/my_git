use crate::{
    defaults::BASE_DIR,
    utils::{compress_str, get_hash, write_recursive},
};
use std::{
    error::Error,
    fs,
    io::{Read, Write},
};

pub fn add() -> Result<(), Box<dyn Error>> {
    let tree_content = write_recursive(fs::read_dir(BASE_DIR)?, &mut String::new(), true)?;
    let size = tree_content.as_bytes().len();
    let mut uncompressed_content = String::new();
    let header = format!("040000\0tree\0{size}\0\n");

    uncompressed_content.push_str(&header);

    for line in tree_content.lines() {
        let file_info: Vec<&str> = line.split('\0').collect();
        let mode = &file_info[0];
        let object_type = &file_info[1];
        let hash = &file_info[2];
        let file_name = &file_info[3];
        let line = format!("{mode}\t{object_type}\t{hash}\t{file_name}\n");

        uncompressed_content.push_str(&line);
    }

    let object = get_hash(&uncompressed_content.as_bytes().to_vec());
    let compressed = compress_str(uncompressed_content.as_bytes().to_vec())?;

    let mut prev = Vec::new();

    fs::File::open(".my_git/index")?.read_to_end(&mut prev)?;

    if compressed == prev {
        println!("there is no change");
        return Ok(());
    }

    fs::File::create(".my_git/index")?.write_all(&compressed)?;

    object.create_dir()?;
    object.create_file()?.write(&compressed)?;

    Ok(())
}

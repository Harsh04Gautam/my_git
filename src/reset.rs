use crate::defaults::BASE_DIR;
use std::{error::Error, fs, io::Write, path::Path};

use crate::utils::decompress_file;

pub fn reset(path: &str) -> Result<(), Box<dyn Error>> {
    let commit = decompress_file(&format!(".my_git/objects/{}/{}", &path[..2], &path[2..]))?;
    let commit: Vec<&str> = commit.lines().collect();

    let file = decompress_file(&format!(
        ".my_git/objects/{}/{}",
        &commit[0][..2],
        &commit[0][2..]
    ))?;

    if !file.starts_with("040000") {
        eprintln!("not a valid tree");
        return Ok(());
    }

    fs::remove_dir_all(BASE_DIR)?;
    fs::create_dir(BASE_DIR)?;

    for line in file.lines().skip(1) {
        if line.starts_with("040000") {
            continue;
        }

        let info: Vec<&str> = line.split('\t').collect();
        let hash = &info[2];
        let path = &info[3];
        let path = Path::new(path);
        let dir = path.parent().unwrap();
        fs::create_dir_all(dir)?;
        let mut file = fs::File::create_new(path)?;
        let content = decompress_file(&format!(".my_git/objects/{}/{}", &hash[..2], &hash[2..]))?;
        let index = content.find('\n').unwrap();
        let content = &content[index + 1..];
        file.write_all(content.as_bytes())?;
    }

    let head = fs::read_to_string(".my_git/HEAD")?;
    fs::File::create(format!(".my_git/{}", head.trim()))?.write_all(path.as_bytes())?;

    Ok(())
}

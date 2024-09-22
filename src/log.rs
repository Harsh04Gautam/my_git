use std::{error::Error, fs};

use crate::utils::decompress_file;

pub fn log() -> Result<(), Box<dyn Error>> {
    let head = fs::read_to_string(".my_git/HEAD")?;
    let commit_path = fs::read_to_string(&format!(".my_git/{}", head.trim()))?;
    let commit_path = format!(
        ".my_git/objects/{}/{}",
        &commit_path[..2],
        &commit_path[2..]
    );

    let head: Vec<&str> = head.split('/').collect();
    print!("\nHEAD -> {}", head[2]);
    log_recursive(commit_path)?;

    return Ok(());
}

pub fn log_recursive(path: String) -> Result<(), Box<dyn Error>> {
    let commit = decompress_file(&path)?;
    let commit: Vec<&str> = commit.lines().collect();

    let commit_hash: Vec<&str> = path.split('/').skip(2).collect();
    let commit_hash = commit_hash.join("");

    if commit[1] == "" {
        println!("commit\t{}\nmessage\t{}\n\n", commit_hash, &commit[2]);
        return Ok(());
    }

    let parent = format!(".my_git/objects/{}/{}", &commit[1][..2], &commit[1][2..]);
    println!("commit\t{}\nmessage\t{}\n\n", commit_hash, &commit[2]);

    log_recursive(parent.to_string())
}

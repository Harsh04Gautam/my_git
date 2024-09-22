use std::{error::Error, fs, io::Write};

use crate::utils::{compress_str, decompress_file, files_changed, files_staged, get_hash};

pub fn commit(message: &str) -> Result<(), Box<dyn Error>> {
    let head = fs::read_to_string(".my_git/HEAD")?;
    let parent_path = fs::read_to_string(format!(".my_git/{}", head.trim()))?;

    let files_changed = files_changed()?;
    let files_staged = files_staged()?;

    if files_changed.len() == 0 && files_staged.len() == 0 {
        println!("nothing to commit, working tree clean");
    }

    if files_changed.len() != 0 {
        println!("Untracked files:");
        for file in files_changed.iter() {
            println!("\t{file}");
        }
    }

    if files_staged.len() == 0 && files_changed.len() != 0 {
        println!("nothing added to commit but untracked files present (use 'add' to track)");
        return Ok(());
    }

    let index = decompress_file(".my_git/index")?.as_bytes().to_vec();
    let tree = get_hash(&index);
    let commit_content = format!(
        "{}{}\n{parent_path}\n{message}\n",
        tree.directory, tree.file
    )
    .as_bytes()
    .to_vec();

    let commit = get_hash(&commit_content);
    let compress = compress_str(commit_content)?;

    commit.create_dir()?;
    commit.create_file()?.write_all(&compress)?;

    fs::File::create(format!(".my_git/{}", head.trim()))?
        .write_all(format!("{}{}", commit.directory, commit.file).as_bytes())?;

    Ok(())
}

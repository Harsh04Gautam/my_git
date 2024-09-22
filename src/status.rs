use crate::utils::{files_changed, files_staged};
use std::error::Error;

pub fn status() -> Result<(), Box<dyn Error>> {
    let changed = files_changed()?;

    if changed.len() == 0 {
        println!("nothing to commit, working tree clean");
    } else {
        println!("Untracked files:");
        for file in changed {
            println!("\t{file}");
        }
    }

    let staged = files_staged()?;

    if staged.len() == 0 {
        return Ok(());
    } else {
        println!("Staged files:");
        for file in staged {
            println!("\t{file}");
        }
    }

    Ok(())
}

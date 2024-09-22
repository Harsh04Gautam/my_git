use std::error::Error;
use std::fs;
use std::io::Write;

pub fn init() -> Result<(), Box<dyn Error>> {
    fs::create_dir(".my_git")?;
    fs::create_dir(".my_git/objects")?;
    fs::create_dir_all(".my_git/refs/heads")?;
    fs::File::create(".my_git/refs/heads/master")?;
    fs::File::create(".my_git/index")?;

    let mut file = fs::File::create(".my_git/HEAD")?;
    file.write("refs/heads/master\n".as_bytes())?;

    Ok(())
}

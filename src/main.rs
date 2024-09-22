use std::env;
use std::error::Error;

use my_git::add::add;
use my_git::cat_file::cat_file;
use my_git::commit::commit;
use my_git::hash_object::hash_object;
use my_git::init::init;
use my_git::log::log;
use my_git::ls_tree::ls_tree;
use my_git::reset::reset;
use my_git::status::status;
use my_git::write_tree::write_tree;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match &args[1][..] {
        "init" => init(),
        "cat-file" => cat_file(&args[2][..2], &args[2][2..]),
        "hash-object" => hash_object(&args[2]),
        "ls-tree" => ls_tree(&args[2][..2], &args[2][2..]),
        "write-tree" => write_tree(&args[2]),
        "status" => status(),
        "add" => add(),
        "commit" => commit(&args[2]),
        "log" => log(),
        "reset" => reset(&args[2]),
        _ => {
            println!("my_git: '{}' is not a my_git command", args[1]);
            Ok(())
        }
    }?;

    Ok(())
}

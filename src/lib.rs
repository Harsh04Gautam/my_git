pub mod add;
pub mod cat_file;
pub mod commit;
pub mod hash_object;
pub mod init;
pub mod log;
pub mod reset;
pub mod status;
pub mod utils;
pub mod write_tree;
pub mod defaults {
    pub const BASE_DIR: &str = "./being_tracked";
}

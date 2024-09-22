use flate2::{bufread::ZlibDecoder, write::ZlibEncoder, Compression};
use sha2::{Digest, Sha512_256};
use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::{ErrorKind, Read, Write},
};

use crate::defaults::BASE_DIR;

pub enum Type {
    Blob,
    Tree,
}

pub struct Object {
    pub directory: String,
    pub file: String,
}

impl Object {
    pub fn get_directory_path(&self) -> String {
        format!(".my_git/objects/{}/", self.directory)
    }

    pub fn get_file_path(&self) -> String {
        format!(".my_git/objects/{}/{}", self.directory, self.file)
    }

    pub fn file_exist(&self) -> bool {
        match fs::File::open(&self.get_file_path()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn create_dir(&self) -> Result<(), Box<dyn Error>> {
        match fs::create_dir(&self.get_directory_path()) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => Ok(()),
                _ => Err(Box::new(e)),
            },
        }
    }

    pub fn create_file(&self) -> Result<fs::File, Box<dyn Error>> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&self.get_file_path())?;
        Ok(file)
    }
}

pub struct Compressed {
    pub data: Vec<u8>,
    pub object: Object, // pub directory: String,
    pub file_type: Type,
}

pub fn get_hash(data: &Vec<u8>) -> Object {
    let mut hasher = Sha512_256::new();
    hasher.update(data);
    let hash = format!("{:x}", hasher.finalize());

    let directory = hash[..2].to_string();
    let file = hash[2..].to_string();
    Object { directory, file }
}

pub fn compress_file(path: &str, file_type: Type) -> Result<Compressed, Box<dyn Error>> {
    let mut uncompressed_data = Vec::from(format!("100644\0blob\0{path}\n",).as_bytes());

    fs::File::open(path)?.read_to_end(&mut uncompressed_data)?;

    let object = get_hash(&uncompressed_data);
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());

    e.write_all(&uncompressed_data)?;

    Ok(Compressed {
        data: e.finish()?,
        object,
        file_type,
    })
}

pub fn compress_str(data: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());

    e.write_all(&data)?;

    Ok(e.finish()?)
}

pub fn decompress_file(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = fs::File::open(path)?;
    let mut compressed_data: Vec<u8> = vec![];
    file.read_to_end(&mut compressed_data)?;

    Ok(decompress_str(compressed_data)?)
}

pub fn decompress_str(data: Vec<u8>) -> Result<String, Box<dyn Error>> {
    let mut decompressed_content = String::new();
    let mut zlib_decoder = ZlibDecoder::new(data.as_slice());

    zlib_decoder.read_to_string(&mut decompressed_content)?;

    Ok(decompressed_content)
}

pub fn get_path(directory: &str, file: &str) -> String {
    format!(".my_git/objects/{directory}/{file}")
}

pub fn write_recursive(
    directory: fs::ReadDir,
    tree_content: &mut String,
    write: bool,
) -> Result<String, Box<dyn Error>> {
    for dir_object in directory {
        let dir_object = dir_object?;
        let file_type = dir_object.file_type()?;
        let file_path = dir_object.path().display().to_string();

        if file_type.is_dir() {
            let dir_objects = dir_object.path().read_dir()?;

            write_recursive(dir_objects, tree_content, write)?;

            let object = get_hash(&tree_content.as_bytes().to_vec());
            let entry = format!(
                "040000\0tree\0{}{}\0{file_path}\n",
                object.directory, object.file
            );

            tree_content.push_str(&entry);
        } else {
            let mut file_content = Vec::new();

            fs::File::open(dir_object.path())?.read_to_end(&mut file_content)?;

            let compressed = compress_file(&file_path, Type::Blob)?;
            let entry = format!(
                "100644\0blob\0{}{}\0{file_path}\n",
                compressed.object.directory, compressed.object.file
            );

            if write {
                compressed.object.create_dir()?;
                compressed
                    .object
                    .create_file()?
                    .write_all(&compressed.data)?;
            }

            tree_content.push_str(&entry);
        }
    }
    Ok(tree_content.to_string())
}

pub fn files_changed() -> Result<Vec<String>, Box<dyn Error>> {
    let dir = fs::read_dir(BASE_DIR)?;
    let content = write_recursive(dir, &mut String::new(), false)?;

    let mut files_changed = Vec::new();

    for line in content.lines() {
        let file_info: Vec<&str> = line.split('\0').collect();
        let object_type = &file_info[1];
        let hash = &file_info[2];
        let file_path = &file_info[3];
        let object = Object {
            directory: hash[..2].to_string(),
            file: hash[2..].to_string(),
        };

        if object_type.starts_with("blob") && !object.file_exist() {
            files_changed.push(file_path.to_string());
        };
    }

    Ok(files_changed)
}

pub fn files_staged() -> Result<Vec<String>, Box<dyn Error>> {
    let index = decompress_file(".my_git/index")?;

    let head = fs::read_to_string(".my_git/HEAD")?;
    let head = head.trim();

    if index == "" {
        return Ok(Vec::new());
    }

    let head = fs::read_to_string(format!(".my_git/{head}"))?;
    if head == "" {
        let mut staged_files = Vec::new();
        for line in index.lines() {
            if line.contains("blob") {
                let file_info: Vec<&str> = line.split('\t').collect();
                staged_files.push(file_info[3].to_string());
            }
        }
        return Ok(staged_files);
    }

    let commit_path = format!(".my_git/objects/{}/{}", &head[..2], &head[2..]);
    let last_commit = decompress_file(&commit_path)?;
    let last_commit: Vec<&str> = last_commit.lines().collect();
    let tree = &last_commit[0];
    let commited_files =
        decompress_file(&format!(".my_git/objects/{}/{}", &tree[..2], &tree[2..]))?;
    let mut files_staged = Vec::new();

    for line in index.lines() {
        if !commited_files.contains(line) && line.contains("blob") {
            let file_info: Vec<&str> = line.split('\t').collect();
            files_staged.push(file_info[3].to_string());
        }
    }

    Ok(files_staged)
}

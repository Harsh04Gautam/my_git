# My Git

**My Git** is a minimal, Git-like version control system implemented in Rust. It allows users to perform operations on the "being_tracked" directory, such as initializing a repository, adding files, committing changes, viewing the log, and more.

## Features

- **`init`**: Initialize a new My Git repository.
- **`cat-file <hash>`**: Display the contents of a file in the object database.
- **`hash-object <file>`**: Compute the SHA-2 hash of the contents of a file.
- **`write-tree`**: Write the current index as a tree object.
- **`status`**: Show the current state of the repository.
- **`add`**: Add file contents to the index.
- **`commit <message>`**: Record changes to the repository with a commit message.
- **`log`**: Show the commit history.
- **`reset <hash>`**: Reset the current HEAD to the specified commit.

## Installation

To build and run **My Git**, make sure you have [Rust](https://www.rust-lang.org/) installed. Clone the repository and run:

```bash
git clone <your-repo-url>
cd my_git
cargo build --release
cp target/release/my_git ./
```

## Usage

You can use the **My Git** commands as follows:

```bash
my_git init
my_git add
my_git commit "Initial commit"
my_git log
my_git reset <commit-hash>
```

## Dependencies

This project uses two dependence for file compression and for generating hash.

- `flate2 = "1.0.33"`
- `sha2 = "0.10.8"`

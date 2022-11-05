use std::path::{Path, PathBuf};
use std::io;

#[cfg(target_os = "unix")]
use std::os::unix::fs::symlink;

#[cfg(target_os = "unix")]
fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()>{
    symlink(original, link);
}

#[cfg(target_os = "windows")]
use std::os::windows::fs::symlink_file;


#[cfg(target_os = "windows")]
fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()>{
    symlink_file(original, link)
}


pub struct Input {
    path: PathBuf
}

impl Input {
    pub fn new(path: PathBuf) -> Self {
        Input{
            path
        }
    }
    pub fn add_link(&self, at : &Path) -> io::Result<()>{
        create_symlink(&self.path, at.to_path_buf().join(self.path.file_name().unwrap()))
    }
}
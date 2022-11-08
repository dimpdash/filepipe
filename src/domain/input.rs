use std::os::unix::thread;
use std::path::{Path, PathBuf};
use std::io;

// #[cfg(target_os = "unix")]
use std::os::unix::fs::symlink;
use std::thread::sleep;
use std::time::Duration;

use mockall::automock;

// #[cfg(target_os = "unix")]
fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()>{
    let r = symlink(original, link);
    return r;
}


pub struct Input {
    path: PathBuf
}

#[automock]
impl Input {
    pub fn new(path: PathBuf) -> Result<Self, ()> {
        if path.is_dir() {
            return Err(());
        }

        Ok(
            Input{
                path
            }
        )
        
    }
    pub fn add_link(&self, at : &Path) -> io::Result<()> {
        create_symlink(&self.path, at.to_path_buf().join(self.path.file_name().unwrap()))
    }
}


#[cfg(test)]
mod tests {
        use std::{path::{PathBuf, Path}, fs::{self}};

        use crate::domain::input::Input;

        fn get_input_artefacts() -> &'static Path {
            Path::new("./tests/artefacts/generated/input")
        }

        use std::{io};

        use thiserror::Error;

        #[derive(Error, Debug)]
        pub enum TestError {
            #[error("Io Error")]
            IoError(#[from] io::Error),
        }

        #[test]
        fn add_symlink_success() {

            use std::{path::Path, os::unix::fs::symlink};

            
            let dir = PathBuf::from(get_input_artefacts().join("add_sym_link_success"));
            let input = Input::new(dir.clone().join("input").join("file.txt")).unwrap();
            let input_link = dir.clone().join("file.txt");

            let _ = fs::remove_file(&input_link);

            let result = (|| -> Result<(), TestError> {
                input.add_link(&dir).map_err(|e| TestError::IoError(e));
                input_link.read_link().map_err(|e| TestError::IoError(e))?;
                return Ok(());
            })();

            let _ = fs::remove_file(&input_link);
            
            result.unwrap();
        }
                
    #[test]
    fn add_sym_link_not_exist() {
        let dir = PathBuf::from(get_input_artefacts().join("add_sym_link_not_exist").join("file.txt"));
        println!("{}",dir.display());
        let input = Input::new(dir.clone().join("file.txt")).unwrap();

        if let Ok(_) = input.add_link(&dir) {
            panic!("File exists");
        }

    }
}
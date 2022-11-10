use std::io;
use std::os::unix::thread;
use std::path::{Path, PathBuf};

// #[cfg(target_os = "unix")]
use std::os::unix::fs::symlink;
use std::thread::sleep;
use std::time::Duration;

use mockall::automock;

pub mod inputs;
pub mod regex_file;
pub mod single_file;

// #[automock]

pub trait Input {}

#[cfg(test)]
mod tests {}

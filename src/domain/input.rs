// #[cfg(target_os = "unix")]

pub mod input_list;
pub mod inputs;
pub mod regex_file;
pub mod single_file;

// #[automock]

#[typetag::serde(tag = "type")]
pub trait Input: Send + Sync {
    fn get_file_name(&self) -> String;
}

#[cfg(test)]
mod tests {}





// #[cfg(target_os = "unix")]






pub mod inputs;
pub mod regex_file;
pub mod single_file;

// #[automock]

pub trait Input {}

#[cfg(test)]
mod tests {}

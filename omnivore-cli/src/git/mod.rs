pub mod command;
pub mod filter;
pub mod output;
pub mod source;
pub mod utils;

pub use command::{execute_git_command, GitArgs};

// Re-export utility functions for tests
#[cfg(test)]
pub use utils::{is_text_file, parse_size_string};
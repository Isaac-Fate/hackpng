pub mod chunk;
pub mod chunk_type;
pub mod png;
pub mod cli;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

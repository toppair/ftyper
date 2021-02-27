use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
// pub type Result<T> = std::result::Result<T, dyn Error>;

use std::fs;
use std::str::FromStr;

pub use simple_error::{SimpleError as Error, SimpleResult as Result};

pub fn read_input(path: &str) -> Result<String> {
    fs::read_to_string(path).map_err(|err| Error::from(err))
}

pub fn parse<F>(s: &str) -> Result<F>
where
    F: FromStr,
    <F as FromStr>::Err: std::error::Error,
{
    Ok(try_with!(s.parse::<F>(), "failed to parse {}", s))
}

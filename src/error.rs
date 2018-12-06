use regex::Regex;
use std::str::FromStr;

pub use simple_error::{
    bail, require_with, try_with, SimpleError as Error, SimpleResult as Result,
};

pub fn parse<F>(s: &str) -> Result<F>
where
    F: FromStr,
    <F as FromStr>::Err: std::error::Error,
{
    Ok(try_with!(s.parse::<F>(), "failed to parse {}", s))
}

pub fn re(s: &str) -> Result<Regex> {
    Ok(try_with!(Regex::new(s), "failed to compile regex: {}", s))
}

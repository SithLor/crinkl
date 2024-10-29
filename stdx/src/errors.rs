// src/errors.rs

use std::fmt;

/// A simple custom error type for demonstration purposes.
#[derive(Debug)]
pub struct MyError {
    details: String,
}

impl MyError {
    pub fn new(msg: &str) -> MyError {
        MyError { details: msg.to_string() }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for MyError {}

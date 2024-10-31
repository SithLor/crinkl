
pub mod crc;       // A utilities module.
pub mod errors;      // An error handling module.
pub mod prelude;     // A prelude module for common imports.

// Re-exports
pub use errors::*;
pub use prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}

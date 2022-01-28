use std::fmt::Display;

use log::error;

/// A trait that tries to unwrap a data, and logs then panic if this data does
/// not exist.
pub trait LogExpect<T> {

    /// Tries to unwrap data, and logs then panic if this data does not exist.
    fn log_expect(self, message: &str) -> T;

}

impl <T> LogExpect<T> for Option<T> {
    fn log_expect(self, message: &str) -> T {
        match self {
            Some(x) => x,
            None => {
                error!("{}: None value found.", message);
                panic!();
            },
        }
    }
}

impl <T, E: Display> LogExpect<T> for Result<T, E> {
    fn log_expect(self, message: &str) -> T {
        match self {
            Ok(x) => x,
            Err(error) => {
                error!("{}: {}", message, error);
                panic!();
            },
        }
    }
}
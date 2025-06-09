use std::error::Error;

use openssl::error::ErrorStack;

#[derive(Debug)]
pub enum CryptError {
    Unknown(String),
    UnlockRequired,
}

impl Default for CryptError {
    fn default() -> Self {
        CryptError::Unknown(String::from("Unknown error"))
    }
}

impl From<Box<dyn Error>> for CryptError {
    fn from(value: Box<dyn Error>) -> Self {
        CryptError::Unknown(value.to_string())
    }
}

impl From<ErrorStack> for CryptError {
    fn from(value: ErrorStack) -> Self {
        let error_string =
            value
                .errors()
                .iter()
                .fold(String::from("Unexpected error -> "), |acc, val| {
                    acc + &format!(
                        "({}): {}",
                        val.reason_code(),
                        val.reason().unwrap_or("Unkown")
                    )
                });

        CryptError::Unknown(error_string)
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    OtherError(#[from] std::io::Error),

    #[error(transparent)]
    FooError(#[from] std::string::FromUtf8Error),

    #[error("")]
    NotFoundError { cause: String },
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#![deny(warnings)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
use thiserror::Error;
use async_trait::async_trait;

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

// struct Run {
//     x: i32
// }
// impl std::futures::Future for Run {
//
// }

#[async_trait]
pub trait RF<I> {
    //type Input;
    type Output;
    async fn run(self, input: I) -> Self::Output;

    // fn and<X>(self,next:RF<X>) -> And<I,X> {
    //
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use minicbor::Encode;

#[derive(Clone, Debug, Encode)]
pub enum Error {
    #[n(0)]
    KelkError(#[n(0)] i32),
    #[n(1)]
    InvalidMsg,
    #[n(2)]
    InsufficientAmount,
}

// impl From<kelk::error::Error> for Error {
//     fn from(error: kelk::error::Error) -> Self {
//         Error::KelkError(error.code)
//     }
// }

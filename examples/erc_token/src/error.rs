use minicbor::Encode;

#[derive(Clone, Debug, Encode)]
pub enum TokenError {
    #[n(0)]
    KelkError(#[n(0)] i32),
    #[n(1)]
    InvalidMsg,
    #[n(2)]
    InsufficientAmount,
}

impl From<kelk_env::error::HostError> for TokenError {
    fn from(error: kelk_env::error::HostError) -> Self {
        TokenError::KelkError(error.code)
    }
}

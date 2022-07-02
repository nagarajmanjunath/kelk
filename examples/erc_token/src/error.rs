use minicbor::Encode;

#[derive(Clone, Debug, Encode)]
pub enum TokenError {
    #[n(0)]
    KelkError,
    #[n(1)]
    InsufficientAmount,
}

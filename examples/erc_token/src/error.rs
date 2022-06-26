use minicbor::Encode;

#[derive(Clone, Debug, Encode)]
pub enum CalcError {
    #[n(0)]
    KelkError,
    #[n(1)]
    DivByZero,
}


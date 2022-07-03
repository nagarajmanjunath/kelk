use kelk_lib::alloc::vec::Vec;
use minicbor::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode)]
pub enum ProcMsg {
    #[n(4)]
    Transfer {
        #[n(0)]
        from: [u8; 4],
        #[n(1)]
        to: [u8; 4],
        #[n(2)]
        amount: i64,
    },
}

#[derive(Clone, Debug, Encode, Decode)]
pub enum QueryMsg {
    #[n(0)]
    LastResult,
}

#[derive(Clone, Debug, Encode, Decode)]
pub enum QueryRsp {
    #[n(0)]
    Result {
        #[n(0)]
        res: i32,
    },
}

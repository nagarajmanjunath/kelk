use super::*;
use kelk_env::context::Context;
use kelk_env::mock::mock_context;
use kelk_lib::collections::bst::tree::StorageBST;

fn setup(ctx: Context) {
    let msg = InstansiteMsg {
        owner: [0; 4],
        name: "test-erc20".to_string(),
        symbol: "@".to_string(),
        total_supply: 2000,
    };
    assert!(instantiate(ctx, msg).is_ok());
}

#[test]
fn test_instantiate() {
    let ctx = mock_context(1024 * 1024);
    setup(ctx.as_ref());
    // assert_eq!(name(ctx.as_ref()).unwrap(), "test-erc20".to_string());
}

#[test]
fn test_transfer() {
    let ctx = mock_context(1024 * 1024);
    setup(ctx.as_ref());
    let addr_1 = [1; 4];
    let addr_2 = [2; 4];
    assert!(transfer(ctx.as_ref(), addr_1, 10).is_ok());
    assert!(transfer_from(ctx.as_ref(), addr_1, addr_2, 20).is_err());
    assert!(transfer_from(ctx.as_ref(), addr_1, addr_2, 5).is_ok());
}

// #[test]
// fn test_contract_name_with_64_charater() {
//     let ctx = mock_context(1024 * 1024);
//     let _bst: StorageBST<[u8; 4], Vec<u8>> =
//         StorageBST::create(ctx.as_ref().storage, 0, 1024).unwrap();
//     let from_address = [1; 4];
//     let name = String::from("rpstlnmmhwrngtfsvtzsvbichuhkkvmwdyggxltvxbjykgkjhgodelwehgodxjos")
//         .as_bytes()
//         .to_vec();
//     let response_1 = contract_name(ctx.as_ref(), from_address, name);
//     assert!(response_1.is_ok());
// }

// #[test]
// fn test_contract_name_with_greater_64_charater() {
//     let ctx = mock_context(1024 * 1024);
//     let _bst: StorageBST<[u8; 4], Vec<u8>> =
//         StorageBST::create(ctx.as_ref().storage, 0, 1024).unwrap();
//     let from_address = [1; 4];
//     let name =
//         String::from("rpstlnmmhwrngtfsvtzsvbichuhkkvmwdyggxltvxbjykgkjhgodelwehgodxjos-----")
//             .as_bytes()
//             .to_vec();
//     let response_1 = contract_name(ctx.as_ref(), from_address, name);
//     assert!(response_1.is_ok());
// }

use super::*;
use kelk_env::mock::mock_context;
use kelk_lib::collections::bst::tree::StorageBST;

#[test]
fn test_transfer() {
    let ctx = mock_context(1024 * 1024);
    let mut bst: StorageBST<[u8; 4], i64> = StorageBST::create(ctx.as_ref().api, 0, 1024).unwrap();

    let sender = [1; 4];
    let receiver = [2; 4];

    let response_1 = transfer(ctx.as_ref(), sender, receiver, 10);
    assert!(response_1.is_err());
    bst.insert(sender, 11).unwrap();
    let response_2 = transfer(ctx.as_ref(), sender, receiver, 10);
    assert!(response_2.is_ok());
    assert_eq!(bst.find(&sender).unwrap(), Some(1));
    assert_eq!(bst.find(&receiver).unwrap(), Some(10));
}

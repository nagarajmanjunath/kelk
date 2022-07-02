use super::*;
use kelk_env::mock::mock_context;
use kelk_lib::collections::bst::tree::StorageBST;

#[test]
fn test_transfer() {
    let ctx = mock_context(1024 * 1024);
    let mut bst: StorageBST<Vec<u8>, i64> = StorageBST::create(ctx.as_ref().api, 0, 1024).unwrap();
    let response_1 = transfer(
        ctx.as_ref(),
        "alice".as_bytes().to_vec(),
        "bob".as_bytes().to_vec(),
        10,
    );
    assert!(response_1.is_err());
    bst.insert("alice".as_bytes().to_vec(), 11).unwrap();
    let response_2 = transfer(
        ctx.as_ref(),
        "alice".as_bytes().to_vec(),
        "bob".as_bytes().to_vec(),
        10,
    );
    assert!(response_2.is_ok());
    assert_eq!(bst.find(&"alice".as_bytes().to_vec()).unwrap(), Some(1));
    assert_eq!(bst.find(&"bob".as_bytes().to_vec()).unwrap(), Some(10));
}

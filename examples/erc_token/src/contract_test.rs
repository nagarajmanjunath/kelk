use super::*;
use kelk_env::mock::mock_context;

#[test]
fn test_transfer() {
    let ctx = mock_context(10);
    div(ctx.as_ref(), 4, 2).unwrap();
    let res = query_result(ctx.as_ref()).unwrap();
    assert_eq!(res, 2);

    assert!(div(ctx.as_ref(), 4, 0).is_err());
}

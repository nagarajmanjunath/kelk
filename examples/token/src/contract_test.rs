use super::*;
use kelk::context::mock_context;
use kelk::context::Context;


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
    assert_eq!(name(ctx.as_ref()).unwrap(), "test-erc20".to_string());
    assert_eq!(symbol(ctx.as_ref()).unwrap(), "@".to_string());
    assert_eq!(balance(ctx.as_ref(), [0; 4]).unwrap(), 2000);
    assert_eq!(total_supply(ctx.as_ref()).unwrap(), 2000);
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
    assert_eq!(balance(ctx.as_ref(), addr_1).unwrap(), 5);
    assert_eq!(balance(ctx.as_ref(), addr_2).unwrap(), 5);
}

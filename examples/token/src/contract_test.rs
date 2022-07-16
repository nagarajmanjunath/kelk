use super::*;
use kelk::context::Context;
use kelk::mock::{mock_context, MockContext};

fn setup(ctx: &mut MockContext) -> Address {
    let owner = ctx.mocked_blockchain().generate_new_address();
    let msg = InstansiteMsg {
        owner: owner.clone(),
        name: "test-erc20".to_string(),
        symbol: "@".to_string(),
        total_supply: 2000,
    };
    assert!(instantiate(ctx.as_ref(), msg).is_ok());
    owner
}

#[test]
fn test_instantiate() {
    let mut ctx = mock_context(1024 * 1024);
    let owner = setup(&mut ctx);
    assert_eq!(name(ctx.as_ref()).unwrap(), "test-erc20".to_string());
    assert_eq!(symbol(ctx.as_ref()).unwrap(), "@".to_string());
    assert_eq!(balance(ctx.as_ref(), owner).unwrap(), 2000);
    assert_eq!(total_supply(ctx.as_ref()).unwrap(), 2000);
}

#[test]
fn test_transfer() {
    let mut ctx = mock_context(1024 * 1024);
    let owner = setup(&mut ctx);
    let addr_1 = ctx.mocked_blockchain().generate_new_address();
    let addr_2 = ctx.mocked_blockchain().generate_new_address();
    assert!(transfer(ctx.as_ref(), addr_1.clone(), 10).is_ok());
    assert!(transfer_from(ctx.as_ref(), addr_1.clone(), addr_2.clone(), 20).is_err());
    assert!(transfer_from(ctx.as_ref(), addr_1.clone(), addr_2.clone(), 5).is_ok());
    assert_eq!(balance(ctx.as_ref(), addr_1).unwrap(), 5);
    assert_eq!(balance(ctx.as_ref(), addr_2).unwrap(), 5);
}

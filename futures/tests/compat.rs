#![cfg(feature = "compat")]

use futures::compat::Future01CompatExt;

#[test]
fn can_use_01_futures_in_a_03_future() {
    // Create a simple futures 0.1 future that resolves immediately.
    let f = futures_01::future::ok::<i32, ()>(42).compat();

    let result = futures::executor::block_on(f);
    assert_eq!(result, Ok(42));
}

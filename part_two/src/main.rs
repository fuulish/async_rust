use futures::Future;
use std::task::Context;

async fn test() {
    println!("Hello, world!");
}
fn main() {
    let noop = futures::task::noop_waker();
    let mut context = Context::from_waker(&noop);

    // XXX: why can't this be just
    // test().poll(&context); // or unpin...
    let mut pinned = std::pin::pin!(test());
    pinned.as_mut().poll(&mut context);
}

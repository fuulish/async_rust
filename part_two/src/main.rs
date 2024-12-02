use futures::Future;
use std::task::Context;

async fn test() -> std::sync::Arc<i32> {
    let a = std::sync::Arc::new(15);
    std::sync::Arc::clone(&a)
}

fn main() {
    let noop = futures::task::noop_waker();
    let mut context = Context::from_waker(&noop);

    // XXX: why can't this be just
    // test().poll(&context); // or unpin...
    let mut pinned = std::pin::pin!(test());
    match pinned.as_mut().poll(&mut context) {
        std::task::Poll::Ready(v) => println!("{:x}", *v),
        std::task::Poll::Pending => unreachable!("this should not happen"),
    }
}

use futures::future;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

// XXX: if this were an async function, it wouldn't make any progress
//      because it wouldn't be called to make progress
fn foo(n: u64) -> Foo {
    let started = false;
    let duration = Duration::from_secs(1);
    let sleep = Box::pin(tokio::time::sleep(duration));
    Foo { n, started, sleep }
}

struct Foo {
    n: u64,
    // make sure that we're only printing the starting message once
    //  `started` turns Foo into state machine
    //  if there were more states than started/stopped, we'd need more state representations/enum
    started: bool,
    sleep: Pin<Box<tokio::time::Sleep>>,
}

impl Future for Foo {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<Self::Output> {
        if !self.started {
            println!("start {}", self.n);
            self.started = true;
        }
        // We trust the underlying futures that they are reporting Poll::Pending appropriately
        // ...and don't waste time, but rather return quickly
        if self.sleep.as_mut().poll(context).is_pending() {
            return Poll::Pending;
        }
        println!("end {}", self.n);
        Poll::Ready(())
    }
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for n in 1..=10 {
        futures.push(foo(n));
    }
    let joined_future = future::join_all(futures);
    joined_future.await;
}

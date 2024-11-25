use futures::future;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

fn sleep(duration: Duration) -> Sleep {
    let wake_time = Instant::now() + duration;
    Sleep { wake_time }
}

struct Sleep {
    wake_time: Instant,
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context) -> Poll<()> {
        if Instant::now() >= self.wake_time {
            Poll::Ready(())
        } else {
            // for 10 futures, this still works without scheduling wakers
            // context.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

async fn foo(n: u64) {
    println!("start {n}");
    sleep(Duration::from_secs(1)).await;
    println!("end {n}");
}

fn main() {
    let mut futures = Vec::new();
    for n in 1..=1000 {
        futures.push(foo(n));
    }
    // join_all creates its own wakers in order not to poll all individual futures
    // when pushing the total number of futures up, they won't all get woken up individually
    //
    let mut joined_future = Box::pin(future::join_all(futures));
    let waker = futures::task::noop_waker();
    let mut context = Context::from_waker(&waker);
    // this is polling all the time...
    while joined_future.as_mut().poll(&mut context).is_pending() {
        // Busy loop!
    }
}

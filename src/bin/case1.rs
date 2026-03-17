use std::time::Duration;

use futures::executor::block_on;
use smol::Timer;

async fn case1() {
    println!("hello");
    println!("start waiting 6s!!");
    let timer = Timer::after(Duration::from_secs(6));
    // let/cc
    // let waker = Context::from_context();
    // loop {
    //     match timer.poll(waker) {
    //         Ready => return Ready
    //         Pending => return Pending
    //     }
    // }
    timer.await;
    println!("done waiting 6s!!");
    println!("world");
}

#[tokio::main]
async fn main() {
    let future = case1();

    future.await
    // future.poll();
    // block_on(future)
}

use std::time::Duration;

use futures::executor::block_on;
use smol::Timer;

// TODO: dump this MIR
async fn case1() {
    println!("hello");
    println!("start waiting 6s!!");
    let timer = Timer::after(Duration::from_secs(6));
    let timer2 = Timer::after(Duration::from_secs(6));
    let mut x = 42;
    let y = &mut x;
    timer.await;
    *y = 43;
    timer2.await;
    println!("done waiting 6s!!");
    println!("world {}", x);
}

#[tokio::main]
async fn main() {
    let future = case1();

    future.await
    // future.poll();
}

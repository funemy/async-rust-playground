use std::time::Duration;

use futures::executor::block_on;
use smol::Timer;

async fn case2(x: &mut i32) {
    println!("hello");
    println!("start waiting 6s!!");
    let timer = Timer::after(Duration::from_secs(6));
    let timer2 = Timer::after(Duration::from_secs(6));
    let y = &mut (*x);
    timer.await;
    *y = 43;
    timer2.await;
    println!("done waiting 6s!!");
    println!("world {}", x);
}

#[tokio::main]
async fn main() {
    let mut x = 42;
    let future = case2(&mut x);

    future.await
    // future.poll();
}

use futures::executor::block_on;

// simplest async function, an "async" version of a sync function
async fn case0() -> () {
    println!("hello");
    println!("world");
    ()
}

#[tokio::main]
async fn main() {
    let simplest_future = case0();
    simplest_future.await;
    // NOTE: the next line will raise error b/c await consumes a future.
    // simplest_future.await;

    // NOTE: poll will need to take a waker to actually be called
    // simplest_future.poll(...);

    // NOTE: use block_on if you want main to be a normal function
    // #[tokio::main] essentially wrap the async main function in a `block_on`
    // block_on(simplest_future)
}

use std::{cell::{Ref, UnsafeCell}, time::Duration};

use futures::executor::block_on;
use smol::{Timer, future::FutureExt};

#[derive(Debug)]
enum Case1BodyState {
    Start,
    Await1,
    Await2,
    Complete,
}

struct Case1Body {
    state: Case1BodyState,
    timer: Option<Timer>,
    timer2: Option<Timer>,
    x: Option<i32>,
    y: Option<*mut i32>,
}

impl Case1Body {
    fn new() -> Self {
        Case1Body {
            state: Case1BodyState::Start,
            timer: None,
            timer2: None,
            x: None,
            y: None,
        }
    }
}

impl Future for Case1Body {
    type Output = ();

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        loop {
            match self.state {
                Case1BodyState::Start => {
                    println!("hello");
                    println!("start waiting 6s!!");
                    let mut self_mut = self.as_mut();
                    self_mut.timer = Some(Timer::after(Duration::from_secs(6)));
                    self_mut.timer2 = Some(Timer::after(Duration::from_secs(6)));
                    self_mut.x = Some(42);
                    self_mut.y = Some(self_mut.x.as_mut().unwrap() as *mut i32);
                    self_mut.state = Case1BodyState::Await1;
                }

                Case1BodyState::Await1 => {
                    let mut self_mut = self.as_mut();
                    let poll_res = self_mut.timer.as_mut().unwrap().poll(cx);
                    match poll_res {
                        std::task::Poll::Ready(_) => {
                            unsafe { *self_mut.y.unwrap() = 43 }
                            self_mut.state = Case1BodyState::Await2;
                        },
                        std::task::Poll::Pending => {
                            return std::task::Poll::Pending
                        }
                    }
                }

                Case1BodyState::Await2 => {
                    let mut self_mut = self.as_mut();
                    let poll_res = self_mut.timer2.as_mut().unwrap().poll(cx);
                    match poll_res {
                        std::task::Poll::Ready(_) => {
                            println!("done waiting 6s!!");
                            let self_ref = self.as_ref();
                            println!("world {}", self_ref.x.unwrap());
                            let mut self_mut = self.as_mut();
                            self_mut.state = Case1BodyState::Complete;
                        },
                        std::task::Poll::Pending => return std::task::Poll::Pending
                    }
                },

                Case1BodyState::Complete => return std::task::Poll::Ready(()),
            }
        }
    }
}

// now the original `case1` function is just constructing the state machine above
async fn case1() {
    let fut = Case1Body::new();
    fut.await
}

#[tokio::main]
async fn main() {
    let future = case1();

    future.await
    // future.poll();
}

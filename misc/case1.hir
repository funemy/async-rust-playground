#[attr = MacroUse {arguments: UseAll}]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use std::time::Duration;

use futures::executor::block_on;
use smol::Timer;

// TODO: dump this MIR
async fn case1()
    ->
        /*impl Trait*/ (|mut _task_context: ResumeTy|
    ({
        ({
            let _t =
                ({


                    // future.poll();
                    ({
                        ((::std::io::_print as
                                for<'a> fn(Arguments<'a>) {_print})(((format_arguments::from_str
                                    as
                                    fn(&'static str) -> Arguments<'_> {Arguments::<'_>::from_str})(("hello\n"
                                    as &str)) as Arguments<'_>)) as ());
                    } as ());
                    ({
                        ((::std::io::_print as
                                for<'a> fn(Arguments<'a>) {_print})(((format_arguments::from_str
                                    as
                                    fn(&'static str) -> Arguments<'_> {Arguments::<'_>::from_str})(("start waiting 6s!!\n"
                                    as &str)) as Arguments<'_>)) as ());
                    } as ());
                    let timer =
                        ((Timer::after as
                                fn(Duration) -> Timer {Timer::after})(((Duration::from_secs
                                    as fn(u64) -> Duration {Duration::from_secs})((6 as u64)) as
                                Duration)) as Timer);
                    let timer2 =
                        ((Timer::after as
                                fn(Duration) -> Timer {Timer::after})(((Duration::from_secs
                                    as fn(u64) -> Duration {Duration::from_secs})((6 as u64)) as
                                Duration)) as Timer);
                    let mut x = (42 as i32);
                    let y = (&mut (x as i32) as &mut i32);
                    (match ((into_future as
                                fn(Timer) -> <Timer as std::future::IntoFuture>::IntoFuture {<Timer as std::future::IntoFuture>::into_future})((timer
                                as Timer)) as Timer) {
                        mut __awaitee =>
                            (loop {
                                (match (unsafe {
                                        ((poll as
                                                for<'a, 'b, 'c> fn(Pin<&'a mut Timer>, &'b mut Context<'c>) -> Poll<<Timer as futures::Future>::Output> {<Timer as futures::Future>::poll})(((new_unchecked
                                                    as
                                                    unsafe fn(&mut Timer) -> Pin<&mut Timer> {Pin::<&mut Timer>::new_unchecked})((&mut (__awaitee
                                                        as Timer) as &mut Timer)) as Pin<&mut Timer>),
                                            ((get_context as
                                                    unsafe fn(ResumeTy) -> &mut Context<'_> {get_context::<'_, '_>})((_task_context
                                                    as ResumeTy)) as &mut Context<'_>)) as
                                            Poll<std::time::Instant>)
                                    } as Poll<std::time::Instant>) {
                                    Ready {  0: result } =>
                                        (break (result as std::time::Instant) as !),
                                    Pending {} => { }
                                } as ())
                                ((_task_context as ResumeTy) =
                                    ((yield (() as ()) as ResumeTy)) as ());
                            } as std::time::Instant),
                    } as std::time::Instant);
                    ((*(y as &mut i32) as i32) = (43 as i32) as ());
                    (match ((into_future as
                                fn(Timer) -> <Timer as std::future::IntoFuture>::IntoFuture {<Timer as std::future::IntoFuture>::into_future})((timer2
                                as Timer)) as Timer) {
                        mut __awaitee =>
                            (loop {
                                (match (unsafe {
                                        ((poll as
                                                for<'a, 'b, 'c> fn(Pin<&'a mut Timer>, &'b mut Context<'c>) -> Poll<<Timer as futures::Future>::Output> {<Timer as futures::Future>::poll})(((new_unchecked
                                                    as
                                                    unsafe fn(&mut Timer) -> Pin<&mut Timer> {Pin::<&mut Timer>::new_unchecked})((&mut (__awaitee
                                                        as Timer) as &mut Timer)) as Pin<&mut Timer>),
                                            ((get_context as
                                                    unsafe fn(ResumeTy) -> &mut Context<'_> {get_context::<'_, '_>})((_task_context
                                                    as ResumeTy)) as &mut Context<'_>)) as
                                            Poll<std::time::Instant>)
                                    } as Poll<std::time::Instant>) {
                                    Ready {  0: result } =>
                                        (break (result as std::time::Instant) as !),
                                    Pending {} => { }
                                } as ())
                                ((_task_context as ResumeTy) =
                                    ((yield (() as ()) as ResumeTy)) as ());
                            } as std::time::Instant),
                    } as std::time::Instant);
                    ({
                        ((::std::io::_print as
                                for<'a> fn(Arguments<'a>) {_print})(((format_arguments::from_str
                                    as
                                    fn(&'static str) -> Arguments<'_> {Arguments::<'_>::from_str})(("done waiting 6s!!\n"
                                    as &str)) as Arguments<'_>)) as ());
                    } as ());
                    ({
                        ((::std::io::_print as
                                for<'a> fn(Arguments<'a>) {_print})(({
                                super let args = (((&(x as i32) as &i32),) as (&i32,));
                                super let args =
                                    ([((format_argument::new_display as
                                                    for<'a> fn(&'a i32) -> core::fmt::rt::Argument<'a> {core::fmt::rt::Argument::<'_>::new_display::<i32>})(((args
                                                        as (&i32,)).0 as &i32)) as core::fmt::rt::Argument<'_>)] as
                                        [core::fmt::rt::Argument<'_>; 1]);
                                (unsafe {
                                    ((format_arguments::new as
                                            unsafe fn(&[u8; 11], &[core::fmt::rt::Argument<'_>; 1]) -> Arguments<'_> {Arguments::<'_>::new::<11, 1>})((b"\x06world \xc0\x01\n\x00"
                                            as &[u8; 11]),
                                        (&(args as [core::fmt::rt::Argument<'_>; 1]) as
                                            &[core::fmt::rt::Argument<'_>; 1])) as Arguments<'_>)
                                } as Arguments<'_>)
                            } as Arguments<'_>)) as ());
                    } as ());
                } as ());
            _t
        } as ())
    } as ()) as {async fn body of case1()})
fn main() ({
    let body =
        (|mut _task_context: ResumeTy|
            ({
                let future =
                    ((case1 as
                            fn() -> impl futures::Future<Output = ()> {case1})() as
                        impl futures::Future<Output = ()>);
                (match ((into_future as
                            fn(impl futures::Future<Output = ()>) -> <impl futures::Future<Output = ()> as std::future::IntoFuture>::IntoFuture {<impl futures::Future<Output = ()> as std::future::IntoFuture>::into_future})((future
                            as impl futures::Future<Output = ()>)) as
                        impl futures::Future<Output = ()>) {
                    mut __awaitee =>
                        (loop {
                            (match (unsafe {
                                    ((poll as
                                            for<'a, 'b, 'c> fn(Pin<&'a mut impl futures::Future<Output = ()>>, &'b mut Context<'c>) -> Poll<<impl futures::Future<Output = ()> as futures::Future>::Output> {<impl futures::Future<Output = ()> as futures::Future>::poll})(((new_unchecked
                                                as
                                                unsafe fn(&mut impl futures::Future<Output = ()>) -> Pin<&mut impl futures::Future<Output = ()>> {Pin::<&mut impl futures::Future<Output = ()>>::new_unchecked})((&mut (__awaitee
                                                    as impl futures::Future<Output = ()>) as
                                                &mut impl futures::Future<Output = ()>)) as
                                            Pin<&mut impl futures::Future<Output = ()>>),
                                        ((get_context as
                                                unsafe fn(ResumeTy) -> &mut Context<'_> {get_context::<'_, '_>})((_task_context
                                                as ResumeTy)) as &mut Context<'_>)) as Poll<()>)
                                } as Poll<()>) {
                                Ready {  0: result } => (break (result as ()) as !),
                                Pending {} => { }
                            } as ())
                            ((_task_context as ResumeTy) =
                                ((yield (() as ()) as ResumeTy)) as ());
                        } as ()),
                } as ())
            } as ()) as {async block@src/bin/case1.rs:21:1: 21:15});
    let body =
        ({
            (if (false as bool) {
                let _: &dyn ::core::future::Future<Output = ()> =
                    (&(body as {async block@src/bin/case1.rs:21:1: 21:15}) as
                        &{async block@src/bin/case1.rs:21:1: 21:15});
            } as ())
            (body as {async block@src/bin/case1.rs:21:1: 21:15})
        } as {async block@src/bin/case1.rs:21:1: 21:15});
    #[<cfg_trace>(all())]
    #[allow(clippy :: expect_used, clippy :: diverging_sub_expression, clippy
    :: needless_return, clippy :: unwrap_in_result)]
    ({
        use tokio::runtime::Builder;
        (return ((((((Builder::new_multi_thread as
                                    fn() -> tokio::runtime::Builder {tokio::runtime::Builder::new_multi_thread})()
                                as tokio::runtime::Builder).enable_all() as
                            &mut tokio::runtime::Builder).build() as
                        Result<Runtime, std::io::Error>).expect(("Failed building the Runtime"
                        as &str)) as
                    Runtime).block_on((body as
                    {async block@src/bin/case1.rs:21:1: 21:15})) as ()) as !);
    } as !)
} as ())

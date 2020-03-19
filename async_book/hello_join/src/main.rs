use async_std::task::sleep;
use futures::{executor::block_on, join};
use std::time::Duration;

async fn func1() {
    println!("This is func1");
    sleep(Duration::from_micros(250)).await;
}

async fn func2() {
    println!("This is func2");
}

async fn func3() {
    println!("This is func3");
}

async fn func1_and_func2() {
    // These must happen synchronously, but func3 can
    // run at the same time.
    func1().await;
    func2().await;
}

async fn async_main() {
    let f1 = func1_and_func2();
    let f2 = func3();

    join!(f1, f2);
}

fn main() {
    block_on(async_main());
}

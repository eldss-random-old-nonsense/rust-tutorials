use futures::{executor::block_on, future, select};

async fn count() {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break,
            default => unreachable!(),
        };
    }

    println!("{}", total);
}

fn main() {
    block_on(count());
}

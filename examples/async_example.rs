use rate_guard::{SlidingWindow, TokenBucket};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let burst = SlidingWindow::new(5, Duration::from_millis(500));
    let steady = TokenBucket::new(2, Duration::from_millis(250));

    let mut handled = 0;
    let mut rejected = 0;

    for i in 0..10 {
        if burst.try_acquire() && steady.try_acquire() {
            handled += 1;
            println!("async example: request {i} handled");
        } else {
            rejected += 1;
            println!("async example: request {i} rate limited");
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    println!(
        "async example: handled {handled} and rejected {rejected} of 10 requests"
    );
}
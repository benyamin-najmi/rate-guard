use rate_guard::{FixedWindow, SlidingWindow, TokenBucket};
use std::thread;
use std::time::Duration;

fn main() {
    let bucket = TokenBucket::new(3, Duration::from_secs(1));

    let accepted = (0..5).filter(|_| bucket.try_acquire()).count();
    println!("TokenBucket: accepted {accepted} of 5 burst requests (capacity 3)");

    thread::sleep(Duration::from_millis(1100));
    println!(
        "TokenBucket: {}/3 tokens available after a 1.1s refill",
        bucket.available()
    );

    let window = FixedWindow::new(10, Duration::from_secs(60));
    let granted = (0..12).filter(|_| window.try_acquire()).count();
    println!("FixedWindow: granted {granted} of 12 requests (limit 10)");

    let sliding = SlidingWindow::new(4, Duration::from_secs(10));
    let passed = (0..6).filter(|_| sliding.try_acquire()).count();
    println!("SlidingWindow: passed {passed} of 6 requests (limit 4)");
}
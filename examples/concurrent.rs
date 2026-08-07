use rate_guard::FixedWindow;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let limiter = Arc::new(FixedWindow::new(50, Duration::from_secs(1)));

    let mut handles = Vec::new();
    for _ in 0..8 {
        let limiter = Arc::clone(&limiter);
        handles.push(thread::spawn(move || {
            let mut accepted = 0;
            for _ in 0..25 {
                if limiter.try_acquire() {
                    accepted += 1;
                }
            }
            accepted
        }));
    }

    let total: usize = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum();

    println!(
        "concurrent example: granted {total} of 200 requests across 8 threads (limit 50)"
    );
}
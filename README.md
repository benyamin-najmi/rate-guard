# rate-guard

A lightweight, flexible, and thread-safe rate limiter library for Rust.

## Features

- **Multiple algorithms**: Token Bucket, Fixed Window, Sliding Window
- **Async & Sync support**: Works seamlessly with async runtimes like Tokio
- **Thread-safe**: Built with `Arc` and `Mutex` for concurrent environments
- **Easy to use**: Simple API for quick integration
- **Zero-cost abstractions**: Minimal overhead

## Installation

Add this to your `Cargo.toml`:
```toml
[dependencies]
rate-guard = "0.1.0"

## Quick Start

### Token Bucket Example

rust
use rate_guard::TokenBucket;
use std::time::Duration;

fn main() {
// Create a rate limiter: 10 requests per second
let limiter = TokenBucket::new(10, Duration::from_secs(1));

if limiter.try_acquire() {
println!("Request allowed!");
} else {
println!("Rate limit exceeded!");
}
}

### Async Example with Tokio

rust
use rate_guard::TokenBucket;
use std::time::Duration;

#[tokio::main]
async fn main() {
let limiter = TokenBucket::new(5, Duration::from_secs(1));

for i in 0..10 {
if limiter.try_acquire() {
println!("Request {} allowed", i);
} else {
println!("Request {} blocked", i);
}
tokio::time::sleep(Duration::from_millis(200)).await;
}
}

## Algorithms

### 1. Token Bucket
Allows bursts while maintaining average rate.

rust
let limiter = TokenBucket::new(capacity, refill_period);

### 2. Fixed Window
Simple counter reset at fixed intervals.

rust
let limiter = FixedWindow::new(max_requests, window_duration);

### 3. Sliding Window
More accurate than fixed window, prevents burst at boundaries.

rust
let limiter = SlidingWindow::new(max_requests, window_duration);

## API Reference

### `TokenBucket::new(capacity: u32, refill_period: Duration)`
Creates a new token bucket rate limiter.

### `try_acquire() -> bool`
Attempts to acquire a token. Returns `true` if allowed, `false` if rate limit exceeded.

### `acquire_blocking()`
Blocks until a token is available (sync only).

### `acquire().await`
Waits asynchronously until a token is available (async only).

## Use Cases

- **API rate limiting**: Protect your endpoints from abuse
- **Network request throttling**: Control outbound API calls
- **Resource management**: Limit access to shared resources
- **Backpressure handling**: Smooth traffic spikes

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Made with ❤️ in Rust**

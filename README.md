# rate-guard

A lightweight, thread-safe rate limiting library for Rust with support for multiple algorithms.

## Features

- **Multiple algorithms**: Token Bucket, Fixed Window, Sliding Window
- **Thread-safe**: Built with atomic operations and efficient locking
- **Async support**: Works with Tokio and other async runtimes
- **Minimal dependencies**: relies on the small, fast `parking_lot` for locking (plus optional async support)
- **Simple API**: Easy integration into existing projects

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
rate-guard = "0.1.0"

# For async support
rate-guard = { version = "0.1.0", features = ["async"] }
```

## Quick Start

### Token Bucket

```rust
use rate_guard::TokenBucket;
use std::time::Duration;

fn main() {
    let limiter = TokenBucket::new(10, Duration::from_secs(1));
    
    if limiter.try_acquire() {
        println!("Request allowed");
    } else {
        println!("Rate limit exceeded");
    }
}
```

### Fixed Window

```rust
use rate_guard::FixedWindow;
use std::time::Duration;

let limiter = FixedWindow::new(100, Duration::from_secs(60));

if limiter.try_acquire() {
    // Process request
}
```

### Sliding Window

```rust
use rate_guard::SlidingWindow;
use std::time::Duration;

let limiter = SlidingWindow::new(50, Duration::from_secs(30));

if limiter.try_acquire() {
    // Handle request
}
```

### Validated Construction & Errors

Each limiter also offers a fallible `try_new` constructor that rejects invalid
configuration (zero capacity, limit, or window) instead of silently clamping:

```rust
use rate_guard::{FixedWindow, RateLimitError};
use std::time::Duration;

match FixedWindow::try_new(100, Duration::from_secs(60)) {
    Ok(limiter) => {
        if limiter.try_acquire() {
            // Process request
        }
    }
    Err(RateLimitError::ZeroWindow) => unreachable!(),
    Err(_) => println!("invalid configuration"),
}
```

### Async Usage

```rust
use rate_guard::TokenBucket;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let limiter = TokenBucket::new(5, Duration::from_secs(1));
    
    for i in 0..10 {
        if limiter.try_acquire() {
            println!("Request {} processed", i);
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
}
```

## Algorithms

### Token Bucket
Allows burst traffic while maintaining average rate. Tokens refill continuously.

### Fixed Window
Counts requests in fixed time windows. Simple but can allow bursts at window boundaries.

### Sliding Window
Tracks individual request timestamps. More accurate than fixed window but slightly higher memory overhead.

## Use Cases

- API rate limiting
- Request throttling
- Resource access control
- Traffic shaping
- Backpressure management

## Performance

All algorithms are designed for high-performance concurrent access with minimal contention.

## License

MIT

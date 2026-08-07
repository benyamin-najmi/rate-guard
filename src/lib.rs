//! A lightweight, thread-safe rate limiting library for Rust.
//!
//! `rate-guard` provides three well-known rate limiting algorithms behind a
//! single, minimal API:
//!
//! - [`TokenBucket`]: allows short bursts while enforcing a steady average rate.
//! - [`FixedWindow`]: the fastest implementation, at the cost of allowing
//!   bursts at window boundaries.
//! - [`SlidingWindow`]: the most accurate, at the cost of extra memory.
//!
//! Every limiter exposes a blocking `try_acquire()` method that returns
//! `true` when a request may proceed and `false` when it should be rejected.
//! All of them are safe to share across threads.
//!
//! ```
//! use rate_guard::TokenBucket;
//! use std::time::Duration;
//!
//! let limiter = TokenBucket::new(10, Duration::from_secs(1));
//! assert!(limiter.try_acquire());
//! ```

pub mod error;
pub mod fixed_window;
pub mod sliding_window;
pub mod token_bucket;

pub use error::RateLimitError;
pub use fixed_window::FixedWindow;
pub use sliding_window::SlidingWindow;
pub use token_bucket::TokenBucket;
use rate_guard::{FixedWindow, RateLimitError, SlidingWindow, TokenBucket};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn token_bucket_limits_burst() {
    let bucket = TokenBucket::new(5, Duration::from_secs(1));
    let accepted = (0..10).filter(|_| bucket.try_acquire()).count();
    assert_eq!(accepted, 5);
}

#[test]
fn token_bucket_refills_and_respects_capacity() {
    let bucket = TokenBucket::new(2, Duration::from_millis(100));
    assert!(bucket.try_acquire());
    assert!(bucket.try_acquire());
    assert!(!bucket.try_acquire());

    thread::sleep(Duration::from_millis(120));
    assert!(bucket.try_acquire());
    assert!(!bucket.try_acquire());
    assert!(bucket.available() <= 1);
}

#[test]
fn fixed_window_limits_burst() {
    let window = FixedWindow::new(4, Duration::from_secs(60));
    let accepted = (0..20).filter(|_| window.try_acquire()).count();
    assert_eq!(accepted, 4);
}

#[test]
fn fixed_window_resets_after_window() {
    let window = FixedWindow::new(2, Duration::from_millis(50));
    assert!(window.try_acquire());
    assert!(window.try_acquire());
    assert!(!window.try_acquire());

    thread::sleep(Duration::from_millis(70));
    assert!(window.try_acquire());
}

#[test]
fn sliding_window_limits_burst() {
    let window = SlidingWindow::new(3, Duration::from_secs(60));
    let accepted = (0..10).filter(|_| window.try_acquire()).count();
    assert_eq!(accepted, 3);
    assert_eq!(window.count(), 3);
}

#[test]
fn sliding_window_expires_old_timestamps() {
    let window = SlidingWindow::new(1, Duration::from_millis(50));
    assert!(window.try_acquire());
    assert!(!window.try_acquire());

    thread::sleep(Duration::from_millis(70));
    assert!(window.try_acquire());
}

#[test]
fn token_bucket_is_thread_safe() {
    let bucket = Arc::new(TokenBucket::new(1000, Duration::from_secs(60)));
    let mut handles = Vec::new();
    for _ in 0..8 {
        let bucket = Arc::clone(&bucket);
        handles.push(thread::spawn(move || {
            (0..150).filter(|_| bucket.try_acquire()).count()
        }));
    }
    let total: usize = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum();
    assert_eq!(total, 1000);
}

#[test]
fn fixed_window_is_thread_safe() {
    let window = Arc::new(FixedWindow::new(100, Duration::from_secs(60)));
    let mut handles = Vec::new();
    for _ in 0..8 {
        let window = Arc::clone(&window);
        handles.push(thread::spawn(move || {
            (0..20).filter(|_| window.try_acquire()).count()
        }));
    }
    let total: usize = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum();
    assert_eq!(total, 100);
}

#[test]
fn sliding_window_is_thread_safe() {
    let window = Arc::new(SlidingWindow::new(64, Duration::from_secs(60)));
    let mut handles = Vec::new();
    for _ in 0..8 {
        let window = Arc::clone(&window);
        handles.push(thread::spawn(move || {
            (0..10).filter(|_| window.try_acquire()).count()
        }));
    }
    let total: usize = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum();
    assert_eq!(total, 64);
}

#[test]
fn constructors_validate_invalid_configuration() {
    assert!(matches!(
        TokenBucket::try_new(0, Duration::from_secs(1)),
        Err(RateLimitError::ZeroCapacity)
    ));
    assert!(matches!(
        FixedWindow::try_new(0, Duration::from_secs(1)),
        Err(RateLimitError::ZeroLimit)
    ));
    assert!(matches!(
        SlidingWindow::try_new(10, Duration::ZERO),
        Err(RateLimitError::ZeroWindow)
    ));
    assert!(matches!(
        TokenBucket::try_new(10, Duration::ZERO),
        Err(RateLimitError::InvalidRefillRate)
    ));
}
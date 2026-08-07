use crate::error::RateLimitError;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

pub struct FixedWindow {
    limit: u64,
    window_nanos: u64,
    count: AtomicU64,
    window_start: AtomicU64,
}

impl FixedWindow {
    pub fn new(limit: u64, window: Duration) -> Self {
        let window = if window.is_zero() {
            Duration::from_nanos(1)
        } else {
            window
        };
        Self {
            limit,
            window_nanos: window.as_nanos() as u64,
            count: AtomicU64::new(0),
            window_start: AtomicU64::new(monotonic_nanos()),
        }
    }

    pub fn try_new(limit: u64, window: Duration) -> Result<Self, RateLimitError> {
        if limit == 0 {
            return Err(RateLimitError::ZeroLimit);
        }
        if window.is_zero() {
            return Err(RateLimitError::ZeroWindow);
        }
        Ok(Self::new(limit, window))
    }

    pub fn try_acquire(&self) -> bool {
        let now = monotonic_nanos();
        loop {
            let start = self.window_start.load(Ordering::Acquire);
            let elapsed = now.saturating_sub(start);

            if elapsed >= self.window_nanos {
                match self.window_start.compare_exchange(
                    start,
                    now,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                ) {
                    Ok(_) => {
                        self.count.store(1, Ordering::Relaxed);
                        return true;
                    }
                    Err(_) => continue,
                }
            }

            let count = self.count.fetch_add(1, Ordering::AcqRel);

            if self.window_start.load(Ordering::Acquire) == start {
                return count < self.limit;
            }

            self.count.fetch_sub(1, Ordering::Relaxed);
        }
    }

    pub fn remaining(&self) -> u64 {
        let count = self.count.load(Ordering::Relaxed);
        self.limit.saturating_sub(count)
    }

    pub fn limit(&self) -> u64 {
        self.limit
    }
}

fn monotonic_nanos() -> u64 {
    static BASE: OnceLock<Instant> = OnceLock::new();
    let base = BASE.get_or_init(Instant::now);
    base.elapsed().as_nanos() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limits_burst_within_window() {
        let window = FixedWindow::new(4, Duration::from_secs(60));
        let accepted = (0..12).filter(|_| window.try_acquire()).count();
        assert_eq!(accepted, 4);
        assert_eq!(window.remaining(), 0);
    }

    #[test]
    fn resets_after_window_elapses() {
        let window = FixedWindow::new(1, Duration::from_millis(50));
        assert!(window.try_acquire());
        assert!(!window.try_acquire());
        std::thread::sleep(Duration::from_millis(70));
        assert!(window.try_acquire());
    }

    #[test]
    fn rejects_invalid_configuration() {
        assert!(matches!(
            FixedWindow::try_new(0, Duration::from_secs(1)),
            Err(RateLimitError::ZeroLimit)
        ));
        assert!(matches!(
            FixedWindow::try_new(10, Duration::ZERO),
            Err(RateLimitError::ZeroWindow)
        ));
    }
}
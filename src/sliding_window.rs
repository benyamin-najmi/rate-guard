use crate::error::RateLimitError;
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct SlidingWindow {
    limit: usize,
    window: Duration,
    timestamps: Mutex<VecDeque<Instant>>,
}

impl SlidingWindow {
    pub fn new(limit: usize, window: Duration) -> Self {
        let window = if window.is_zero() {
            Duration::from_nanos(1)
        } else {
            window
        };
        Self {
            limit,
            window,
            timestamps: Mutex::new(VecDeque::with_capacity(limit)),
        }
    }

    pub fn try_new(limit: usize, window: Duration) -> Result<Self, RateLimitError> {
        if limit == 0 {
            return Err(RateLimitError::ZeroLimit);
        }
        if window.is_zero() {
            return Err(RateLimitError::ZeroWindow);
        }
        Ok(Self::new(limit, window))
    }

    pub fn try_acquire(&self) -> bool {
        let now = Instant::now();
        let mut timestamps = self.timestamps.lock();
        self.prune(&mut timestamps, now);
        if timestamps.len() < self.limit {
            timestamps.push_back(now);
            true
        } else {
            false
        }
    }

    pub fn count(&self) -> usize {
        let now = Instant::now();
        let mut timestamps = self.timestamps.lock();
        self.prune(&mut timestamps, now);
        timestamps.len()
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    fn prune(&self, timestamps: &mut VecDeque<Instant>, now: Instant) {
        let cutoff = now - self.window;
        while timestamps.front().is_some_and(|t| *t <= cutoff) {
            timestamps.pop_front();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limits_burst() {
        let window = SlidingWindow::new(3, Duration::from_secs(60));
        let accepted = (0..8).filter(|_| window.try_acquire()).count();
        assert_eq!(accepted, 3);
        assert_eq!(window.count(), 3);
    }

    #[test]
    fn expires_old_timestamps() {
        let window = SlidingWindow::new(2, Duration::from_millis(50));
        assert!(window.try_acquire());
        std::thread::sleep(Duration::from_millis(60));
        assert!(window.try_acquire());
        assert!(window.try_acquire());
        assert!(!window.try_acquire());
    }

    #[test]
    fn rejects_invalid_configuration() {
        assert!(matches!(
            SlidingWindow::try_new(0, Duration::from_secs(1)),
            Err(RateLimitError::ZeroLimit)
        ));
        assert!(matches!(
            SlidingWindow::try_new(10, Duration::ZERO),
            Err(RateLimitError::ZeroWindow)
        ));
    }
}
use crate::error::RateLimitError;
use parking_lot::Mutex;
use std::fmt;
use std::time::{Duration, Instant};

pub struct TokenBucket {
    capacity: u64,
    refill_per_token: Duration,
    state: Mutex<BucketState>,
}

struct BucketState {
    tokens: f64,
    last_refill: Instant,
}

impl TokenBucket {
    pub fn new(capacity: u64, refill_per_token: Duration) -> Self {
        let refill = if refill_per_token.is_zero() {
            Duration::from_nanos(1)
        } else {
            refill_per_token
        };
        Self {
            capacity,
            refill_per_token: refill,
            state: Mutex::new(BucketState {
                tokens: capacity as f64,
                last_refill: Instant::now(),
            }),
        }
    }

    pub fn try_new(capacity: u64, refill_per_token: Duration) -> Result<Self, RateLimitError> {
        if capacity == 0 {
            return Err(RateLimitError::ZeroCapacity);
        }
        if refill_per_token.is_zero() {
            return Err(RateLimitError::InvalidRefillRate);
        }
        Ok(Self::new(capacity, refill_per_token))
    }

    pub fn try_acquire(&self) -> bool {
        let mut state = self.state.lock();
        let now = Instant::now();
        self.refill(&mut state, now);
        if state.tokens >= 1.0 {
            state.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    pub fn available(&self) -> u64 {
        let mut state = self.state.lock();
        let now = Instant::now();
        self.refill(&mut state, now);
        state.tokens.floor() as u64
    }

    pub fn capacity(&self) -> u64 {
        self.capacity
    }

    fn refill(&self, state: &mut BucketState, now: Instant) {
        let elapsed = now.duration_since(state.last_refill);
        let added = elapsed.as_secs_f64() / self.refill_per_token.as_secs_f64();
        let total = state.tokens + added;
        state.tokens = total.min(self.capacity as f64);
        state.last_refill = now;
    }
}

impl fmt::Debug for TokenBucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenBucket")
            .field("capacity", &self.capacity)
            .field("refill_per_token", &self.refill_per_token)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_full_and_allows_up_to_capacity() {
        let bucket = TokenBucket::new(5, Duration::from_secs(1));
        let accepted = (0..8).filter(|_| bucket.try_acquire()).count();
        assert_eq!(accepted, 5);
        assert_eq!(bucket.available(), 0);
    }

    #[test]
    fn refills_over_time() {
        let bucket = TokenBucket::new(2, Duration::from_millis(100));
        assert!(bucket.try_acquire());
        assert!(bucket.try_acquire());
        assert!(!bucket.try_acquire());
        std::thread::sleep(Duration::from_millis(120));
        assert!(bucket.try_acquire());
    }

    #[test]
    fn never_exceeds_capacity() {
        let bucket = TokenBucket::new(4, Duration::from_millis(1));
        std::thread::sleep(Duration::from_millis(30));
        assert_eq!(bucket.available(), 4);
    }

    #[test]
    fn zero_capacity_denies_everything_infallibly() {
        let bucket = TokenBucket::new(0, Duration::from_secs(1));
        for _ in 0..10 {
            assert!(!bucket.try_acquire());
        }
        assert_eq!(bucket.available(), 0);
    }

    #[test]
    fn zero_capacity_rejected_by_try_new() {
        assert!(matches!(
            TokenBucket::try_new(0, Duration::from_secs(1)),
            Err(RateLimitError::ZeroCapacity)
        ));
        assert!(matches!(
            TokenBucket::try_new(5, Duration::ZERO),
            Err(RateLimitError::InvalidRefillRate)
        ));
    }
}
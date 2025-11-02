/// Clock abstraction for testability
/// This allows us to mock time in tests while following clippy best practices
use std::time::{SystemTime, UNIX_EPOCH};

pub trait Clock {
    /// Get the current Unix timestamp in seconds
    fn now_seconds(&self) -> u64;
}

/// System clock implementation using real system time
#[derive(Debug, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_seconds(&self) -> u64 {
        #[allow(clippy::disallowed_methods)]
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time is before UNIX epoch")
            .as_secs()
    }
}

#[cfg(test)]
pub struct MockClock {
    pub timestamp: u64,
}

#[cfg(test)]
impl Clock for MockClock {
    fn now_seconds(&self) -> u64 {
        self.timestamp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_clock_returns_reasonable_time() {
        let clock = SystemClock;
        let now = clock.now_seconds();

        // Should be after 2020-01-01 (timestamp: 1577836800)
        // and before 2100-01-01 (timestamp: 4102444800)
        assert!(now > 1_577_836_800);
        assert!(now < 4_102_444_800);
    }

    #[test]
    fn test_mock_clock_returns_set_time() {
        let clock = MockClock {
            timestamp: 1_234_567_890,
        };
        assert_eq!(clock.now_seconds(), 1_234_567_890);
    }
}

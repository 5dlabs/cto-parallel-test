//! Clock abstraction for testable time operations
//!
//! This module provides a clock trait to abstract time operations,
//! making JWT token creation testable while avoiding direct `SystemTime::now()` calls.

use std::time::{SystemTime, UNIX_EPOCH};

/// Trait for obtaining current time (for testability)
pub trait Clock {
    /// Returns the current time as seconds since Unix epoch
    fn now(&self) -> u64;
}

/// Production clock implementation using system time
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    #[allow(clippy::disallowed_methods)] // This is the one place SystemTime::now is allowed
    fn now(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
}

#[cfg(test)]
pub mod test_helpers {
    use super::*;

    /// Mock clock for testing with fixed time
    #[derive(Debug, Clone, Copy)]
    pub struct MockClock {
        pub timestamp: u64,
    }

    impl MockClock {
        #[must_use]
        pub const fn new(timestamp: u64) -> Self {
            Self { timestamp }
        }
    }

    impl Clock for MockClock {
        fn now(&self) -> u64 {
            self.timestamp
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_clock_returns_reasonable_time() {
        let clock = SystemClock;
        let now = clock.now();

        // Time should be after 2020-01-01 and before 2100-01-01
        assert!(now > 1_577_836_800); // 2020-01-01
        assert!(now < 4_102_444_800); // 2100-01-01
    }

    #[test]
    fn test_mock_clock_returns_fixed_time() {
        let clock = test_helpers::MockClock::new(1_234_567_890);
        assert_eq!(clock.now(), 1_234_567_890);
    }
}

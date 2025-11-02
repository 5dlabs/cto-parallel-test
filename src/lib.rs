#![forbid(unsafe_code)]

pub mod auth;

// Shared test utilities
#[cfg(test)]
pub mod test_support {
    use std::sync::{Mutex, OnceLock};

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    /// Returns a global mutex guard used to serialize environment
    /// variable mutations across tests.
    ///
    /// # Panics
    /// Panics only if the global mutex is poisoned, which would
    /// indicate a previous test panicked while holding the lock.
    pub fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        ENV_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("env lock")
    }
}

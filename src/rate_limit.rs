#[cfg(feature = "server")]
use std::collections::HashMap;
#[cfg(feature = "server")]
use std::sync::LazyLock;
#[cfg(feature = "server")]
use std::sync::Mutex;
#[cfg(feature = "server")]
use std::time::Duration;
#[cfg(feature = "server")]
use std::time::Instant;

#[cfg(feature = "server")]
struct RateLimitEntry {
    count: u32,
    window_start: Instant,
}

#[cfg(feature = "server")]
static CONTACT_LIMITS: LazyLock<Mutex<HashMap<String, RateLimitEntry>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[cfg(feature = "server")]
static CLEANUP_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[cfg(feature = "server")]
/// Check if the given key is within the contact form rate limit.
/// Allows 5 requests per 5 minutes.
pub fn check_contact_rate_limit(key: &str) -> bool {
    let mut limits = CONTACT_LIMITS.lock().unwrap_or_else(|e| e.into_inner());
    let now = Instant::now();
    let window = Duration::from_secs(300);
    let max_requests = 5u32;

    // Periodic cleanup: sweep entries older than one window every ~64 calls
    let count = CLEANUP_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if count.is_multiple_of(64) {
        limits.retain(|_, entry| now.duration_since(entry.window_start) <= window);
    }

    match limits.get_mut(key) {
        Some(entry) => {
            if now.duration_since(entry.window_start) > window {
                entry.count = 1;
                entry.window_start = now;
                true
            } else if entry.count < max_requests {
                entry.count += 1;
                true
            } else {
                false
            }
        }
        None => {
            limits.insert(
                key.to_string(),
                RateLimitEntry {
                    count: 1,
                    window_start: now,
                },
            );
            true
        }
    }
}

#[cfg(not(feature = "server"))]
pub fn check_contact_rate_limit(_key: &str) -> bool {
    true
}

#[cfg(all(test, feature = "server"))]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn allows_first_request_for_new_key() {
        assert!(check_contact_rate_limit("allows-first-key"));
    }

    #[test]
    fn allows_up_to_five_consecutive_requests() {
        let key = "allows-five-explicit-key";

        assert!(check_contact_rate_limit(key));
        assert!(check_contact_rate_limit(key));
        assert!(check_contact_rate_limit(key));
        assert!(check_contact_rate_limit(key));
        assert!(check_contact_rate_limit(key));
        assert!(!check_contact_rate_limit(key));
    }

    #[test]
    fn rejects_sixth_request_within_window() {
        let key = "rejects-sixth-key";

        assert!(check_contact_rate_limit(key));
        assert!(check_contact_rate_limit(key));
        assert!(check_contact_rate_limit(key));
        assert!(check_contact_rate_limit(key));
        assert!(check_contact_rate_limit(key));
        assert!(!check_contact_rate_limit(key));
    }

    #[test]
    fn reopens_window_after_expiry() {
        let key = "reopens-window-key";

        assert!(check_contact_rate_limit(key));
        assert!(check_contact_rate_limit(key));
        assert!(check_contact_rate_limit(key));
        assert!(check_contact_rate_limit(key));
        assert!(check_contact_rate_limit(key));
        assert!(!check_contact_rate_limit(key));

        {
            let mut limits = CONTACT_LIMITS.lock().unwrap();
            limits.insert(
                key.to_string(),
                RateLimitEntry {
                    count: 5,
                    window_start: Instant::now() - Duration::from_secs(301),
                },
            );
        }

        assert!(check_contact_rate_limit(key));
        assert!(check_contact_rate_limit(key));
        assert!(check_contact_rate_limit(key));
        assert!(check_contact_rate_limit(key));
        assert!(check_contact_rate_limit(key));
        assert!(!check_contact_rate_limit(key));
    }

    #[test]
    fn isolates_rate_limits_per_key() {
        let key_a = "isolates-key-a";
        let key_b = "isolates-key-b";

        assert!(check_contact_rate_limit(key_a));
        assert!(check_contact_rate_limit(key_a));
        assert!(check_contact_rate_limit(key_a));
        assert!(check_contact_rate_limit(key_a));
        assert!(check_contact_rate_limit(key_a));
        assert!(!check_contact_rate_limit(key_a));

        assert!(check_contact_rate_limit(key_b));
    }

    #[test]
    /// Exception: thread spawn loop is unavoidable — the behavior under test
    /// IS concurrent access to the Mutex. A Vec of handles and sequential
    /// joins is the idiomatic safe concurrency pattern in std Rust.
    fn handles_concurrent_access_safely() {
        let key = "handles-concurrent-key";

        let mut handles = Vec::new();
        for _ in 0..10 {
            handles.push(thread::spawn(move || {
                check_contact_rate_limit(key);
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
}

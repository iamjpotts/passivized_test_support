use std::time::Duration;
use backoff::backoff::Backoff;

/// Wraps another retry policy.
///
/// Limits the number of attempts.
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
/// use backoff::backoff::Constant;
/// use passivized_test_support::retry::Limit;
///
/// let policy = Limit::new(
///     5,
///     Constant::new(Duration::from_secs(2))
/// );
/// ```
pub struct Limit {
    configured_remaining: usize,
    remaining: usize,
    policy: Box<dyn Backoff>
}

impl Limit {

    pub fn new<B: Backoff + 'static>(remaining: usize, policy: B) -> Self {
        Limit {
            configured_remaining: remaining,
            remaining,
            policy: Box::new(policy)
        }
    }

}

impl Backoff for Limit {

    fn reset(&mut self) {
        self.policy.reset();
        self.remaining = self.configured_remaining;
    }

    fn next_backoff(&mut self) -> Option<Duration> {
        if self.remaining > 0 {
            self.remaining = self.remaining - 1;
            self.policy.next_backoff()
        }
        else {
            None
        }
    }

}

#[cfg(test)]
mod test_limited_retry {
    use std::time::Duration;
    use backoff::backoff::{Backoff, Constant};
    use super::Limit;

    fn inner() -> Constant {
        Constant::new(Duration::from_secs(2))
    }

    #[test]
    fn test_terminates() {
        let mut policy = Limit::new(3, inner());

        let d = Duration::from_secs(2);

        assert_eq!(Some(d), policy.next_backoff());
        assert_eq!(Some(d), policy.next_backoff());
        assert_eq!(Some(d), policy.next_backoff());
        assert_eq!(None, policy.next_backoff());
    }

    #[test]
    fn test_resets() {
        let mut policy = Limit::new(2, inner());

        assert!(policy.next_backoff().is_some());
        assert!(policy.next_backoff().is_some());
        assert_eq!(None, policy.next_backoff());

        policy.reset();

        assert!(policy.next_backoff().is_some());
        assert!(policy.next_backoff().is_some());
        assert_eq!(None, policy.next_backoff());
    }
}
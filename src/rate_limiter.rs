use std::time::{Duration, Instant};
use tokio::time::sleep;

pub struct RateLimiter {
    last_check: Instant,
    delay: Duration,
}

impl RateLimiter {
    pub fn new(requests_per_second: u64) -> Self {
        RateLimiter {
            last_check: Instant::now(),
            delay: Duration::from_micros(1_000_000 / requests_per_second),
        }
    }

    pub async fn wait(&self) {
        let elapsed = self.last_check.elapsed();
        if elapsed < self.delay {
            sleep(self.delay - elapsed).await;
        }
    }
}
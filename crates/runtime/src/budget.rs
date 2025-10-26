use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use wadah_spec::Budgets;

#[derive(Debug, Clone)]
pub struct BudgetTracker {
    budgets: Option<Budgets>,
    state: Arc<Mutex<BudgetState>>,
}

#[derive(Debug)]
struct BudgetState {
    start_time: Instant,
    tokens_this_minute: u64,
    minute_start: Instant,
    usd_spent_today: f64,
    day_start: Instant,
}

impl BudgetTracker {
    pub fn new(budgets: Option<Budgets>) -> Self {
        let now = Instant::now();
        Self {
            budgets,
            state: Arc::new(Mutex::new(BudgetState {
                start_time: now,
                tokens_this_minute: 0,
                minute_start: now,
                usd_spent_today: 0.0,
                day_start: now,
            })),
        }
    }

    pub fn check_tokens(&self, tokens: u64) -> crate::Result<()> {
        if let Some(ref budgets) = self.budgets {
            if let Some(limit) = budgets.tokens_per_minute {
                let mut state = self.state.lock().unwrap();

                // Reset counter if a minute has passed
                if state.minute_start.elapsed() >= Duration::from_secs(60) {
                    state.tokens_this_minute = 0;
                    state.minute_start = Instant::now();
                }

                if state.tokens_this_minute + tokens > limit {
                    return Err(crate::RuntimeError::BudgetExceeded(format!(
                        "Token budget exceeded: {}/{} tokens/min",
                        state.tokens_this_minute + tokens,
                        limit
                    )));
                }

                state.tokens_this_minute += tokens;
            }
        }
        Ok(())
    }

    pub fn record_cost(&self, cost: f64) -> crate::Result<()> {
        if let Some(ref budgets) = self.budgets {
            if let Some(limit) = budgets.usd_per_day {
                let mut state = self.state.lock().unwrap();

                // Reset counter if a day has passed
                if state.day_start.elapsed() >= Duration::from_secs(86400) {
                    state.usd_spent_today = 0.0;
                    state.day_start = Instant::now();
                }

                if state.usd_spent_today + cost > limit {
                    return Err(crate::RuntimeError::BudgetExceeded(format!(
                        "Cost budget exceeded: ${:.2}/${:.2} per day",
                        state.usd_spent_today + cost,
                        limit
                    )));
                }

                state.usd_spent_today += cost;
            }
        }
        Ok(())
    }

    pub fn check_duration(&self) -> crate::Result<()> {
        if let Some(ref budgets) = self.budgets {
            if let Some(max_duration) = budgets.max_duration_secs {
                let state = self.state.lock().unwrap();
                let elapsed = state.start_time.elapsed().as_secs();

                if elapsed > max_duration {
                    return Err(crate::RuntimeError::BudgetExceeded(format!(
                        "Duration budget exceeded: {}s/{}s",
                        elapsed, max_duration
                    )));
                }
            }
        }
        Ok(())
    }

    pub fn get_stats(&self) -> BudgetStats {
        let state = self.state.lock().unwrap();
        BudgetStats {
            elapsed_secs: state.start_time.elapsed().as_secs(),
            tokens_this_minute: state.tokens_this_minute,
            usd_spent_today: state.usd_spent_today,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BudgetStats {
    pub elapsed_secs: u64,
    pub tokens_this_minute: u64,
    pub usd_spent_today: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_budget_tracker() {
        let budgets = Budgets {
            tokens_per_minute: Some(1000),
            usd_per_day: Some(10.0),
            max_duration_secs: Some(3600),
        };

        let tracker = BudgetTracker::new(Some(budgets));

        // Should succeed
        assert!(tracker.check_tokens(500).is_ok());

        // Should succeed
        assert!(tracker.check_tokens(400).is_ok());

        // Should fail - exceeds limit
        assert!(tracker.check_tokens(200).is_err());
    }
}

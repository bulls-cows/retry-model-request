//! Statistics collection module

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use serde::Serialize;

/// Statistics snapshot
#[derive(Debug, Clone, Serialize, Default)]
pub struct StatsSnapshot {
    /// Total requests processed
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Total retries performed
    pub total_retries: u64,
    /// Success rate (percentage)
    pub success_rate: f64,
}

/// Statistics collector
pub struct StatsCollector {
    total_requests: AtomicU64,
    successful_requests: AtomicU64,
    failed_requests: AtomicU64,
    total_retries: AtomicU64,
}

impl StatsCollector {
    /// Create a new stats collector
    pub fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            successful_requests: AtomicU64::new(0),
            failed_requests: AtomicU64::new(0),
            total_retries: AtomicU64::new(0),
        }
    }

    /// Record a successful request
    pub fn record_success(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.successful_requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a failed request
    pub fn record_failure(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.failed_requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a retry attempt
    pub fn record_retry(&self) {
        self.total_retries.fetch_add(1, Ordering::Relaxed);
    }

    /// Get current statistics snapshot
    pub fn get_snapshot(&self) -> StatsSnapshot {
        let total = self.total_requests.load(Ordering::Relaxed);
        let successful = self.successful_requests.load(Ordering::Relaxed);
        let failed = self.failed_requests.load(Ordering::Relaxed);
        let retries = self.total_retries.load(Ordering::Relaxed);

        let success_rate = if total > 0 {
            (successful as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        StatsSnapshot {
            total_requests: total,
            successful_requests: successful,
            failed_requests: failed,
            total_retries: retries,
            success_rate,
        }
    }

    /// Reset all statistics
    pub fn reset(&self) {
        self.total_requests.store(0, Ordering::Relaxed);
        self.successful_requests.store(0, Ordering::Relaxed);
        self.failed_requests.store(0, Ordering::Relaxed);
        self.total_retries.store(0, Ordering::Relaxed);
    }
}

impl Default for StatsCollector {
    fn default() -> Self {
        Self::new()
    }
}
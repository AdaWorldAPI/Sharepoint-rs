//! QScore Calculator — Quality of Service scoring for SharePoint / Graph operations.
//!
//! Transcode of QoS/QScoreCalculator.cs (ST-17)

#![warn(missing_docs)]
#![forbid(unsafe_code)]

use std::time::Duration;

#[derive(Clone, Debug)]
pub struct QScoreWeights {
    pub latency_weight: f64,
    pub throughput_weight: f64,
    pub throttling_weight: f64,
    pub error_rate_weight: f64,
    pub priority_weight: f64,
}

impl Default for QScoreWeights {
    fn default() -> Self {
        Self {
            latency_weight: 0.35,
            throughput_weight: 0.25,
            throttling_weight: 0.20,
            error_rate_weight: 0.15,
            priority_weight: 0.05,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct QScore {
    pub score: f64,                    // 0.0 (bad) → 1.0 (excellent)
    pub latency_ms: u64,
    pub throughput_items_per_sec: f64,
    pub throttling_ratio: f64,         // 0.0 = no throttling, 1.0 = fully throttled
    pub error_rate: f64,
    pub priority_boost: f64,
    pub timestamp_ms: u64,
}

#[derive(Clone, Debug)]
pub struct QScoreCalculator {
    weights: QScoreWeights,
}

impl QScoreCalculator {
    pub fn new(weights: Option<QScoreWeights>) -> Self {
        Self {
            weights: weights.unwrap_or_default(),
        }
    }

    /// Compute QoS score for an operation.
    #[tracing::instrument(skip(self), fields(latency_ms, throughput, throttling))]
    pub fn calculate(
        &self,
        latency: Duration,
        items_processed: u32,
        duration_sec: f64,
        throttling_events: u32,
        total_requests: u32,
        error_count: u32,
        priority: f64, // 0.0-1.0 business priority
    ) -> QScore {
        let latency_ms = latency.as_millis() as u64;
        let throughput = if duration_sec > 0.0 {
            items_processed as f64 / duration_sec
        } else {
            0.0
        };

        let throttling_ratio = if total_requests > 0 {
            throttling_events as f64 / total_requests as f64
        } else {
            0.0
        };

        let error_rate = if total_requests > 0 {
            error_count as f64 / total_requests as f64
        } else {
            0.0
        };

        // Normalize individual components (higher = better)
        let latency_score = (-(latency_ms as f64) / 5000.0).exp(); // decays after ~2-3s
        let throughput_score = (throughput / 50.0).min(1.0);       // target 50 items/sec
        let throttling_score = 1.0 - throttling_ratio.clamp(0.0, 1.0);
        let error_score = (1.0 - error_rate).clamp(0.0, 1.0);

        let score = self.weights.latency_weight * latency_score +
                    self.weights.throughput_weight * throughput_score +
                    self.weights.throttling_weight * throttling_score +
                    self.weights.error_rate_weight * error_score +
                    self.weights.priority_weight * priority;

        QScore {
            score: score.clamp(0.0, 1.0),
            latency_ms,
            throughput_items_per_sec: throughput,
            throttling_ratio,
            error_rate,
            priority_boost: priority,
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthy_operation() {
        let calc = QScoreCalculator::new(None);
        let score = calc.calculate(
            Duration::from_millis(120),
            1000,
            8.0,
            0,
            1000,
            2,
            0.9,
        );
        assert!(score.score > 0.85);
    }

    #[test]
    fn test_throttled_operation() {
        let calc = QScoreCalculator::new(None);
        let score = calc.calculate(
            Duration::from_millis(4500),
            200,
            30.0,
            45,
            100,
            15,
            0.4,
        );
        assert!(score.score < 0.45);
    }
}
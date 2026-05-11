//! Lane Scheduler — QoS lane scheduling and prioritization (ST-06)

use crate::qscore::QScore;
use std::collections::HashMap;

/// Lane priority levels
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LanePriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
    Background = 4,
}

impl LanePriority {
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s >= 0.85 => LanePriority::Critical,
            s if s >= 0.70 => LanePriority::High,
            s if s >= 0.50 => LanePriority::Normal,
            s if s >= 0.30 => LanePriority::Low,
            _ => LanePriority::Background,
        }
    }
}

/// Simple lane scheduler
#[derive(Debug, Default)]
pub struct LaneScheduler {
    lanes: HashMap<LanePriority, Vec<String>>,
}

impl LaneScheduler {
    pub fn new() -> Self {
        Self {
            lanes: HashMap::new(),
        }
    }

    pub fn schedule(&mut self, operation_id: String, score: &QScore) {
        let priority = LanePriority::from_score(score.score);
        self.lanes.entry(priority).or_default().push(operation_id);
    }

    pub fn next(&mut self) -> Option<String> {
        for prio in [
            LanePriority::Critical,
            LanePriority::High,
            LanePriority::Normal,
            LanePriority::Low,
            LanePriority::Background,
        ] {
            if let Some(queue) = self.lanes.get_mut(&prio) {
                if !queue.is_empty() {
                    return Some(queue.remove(0));
                }
            }
        }
        None
    }
}
use chrono::{DateTime, Duration, Utc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpkiPinset {
    pub current: Vec<String>,
    pub previous: Vec<String>,
    pub rotated_at: Option<DateTime<Utc>>,
    pub rotation_window_days: Option<u32>,
}

impl SpkiPinset {
    pub fn is_allowed(&self, presented_hash: &str, now: DateTime<Utc>) -> bool {
        if self
            .current
            .iter()
            .any(|hash| hash == presented_hash)
        {
            return true;
        }

        if self
            .previous
            .iter()
            .any(|hash| hash == presented_hash)
            && self.rotation_window_open(now)
        {
            return true;
        }

        false
    }

    fn rotation_window_open(&self, now: DateTime<Utc>) -> bool {
        let rotated_at = match self.rotated_at {
            Some(value) => value,
            None => return false,
        };

        let window_days = match self.rotation_window_days {
            Some(value) => value,
            None => return false,
        };

        let deadline = rotated_at + Duration::days(window_days as i64);
        now <= deadline
    }
}

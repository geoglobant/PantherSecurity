#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RiskScore(u32);

impl RiskScore {
    pub fn new(score: u32) -> Self {
        Self(score.min(100))
    }

    pub fn value(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    pub category: String,
    pub severity: Severity,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

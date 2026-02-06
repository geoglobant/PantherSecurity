use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Report {
    pub report_id: String,
    pub app_id: String,
    pub env: String,
    pub source: String,
    pub findings: Vec<Finding>,
}

impl Report {
    pub fn empty() -> Self {
        Self {
            report_id: "report_000".to_string(),
            app_id: "unknown".to_string(),
            env: "local".to_string(),
            source: "cli".to_string(),
            findings: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Finding {
    pub category: String,
    pub severity: Severity,
    pub details: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

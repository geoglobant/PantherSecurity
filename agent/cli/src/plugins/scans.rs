use crate::domain::report::{Finding, Severity};
use crate::ports::CheckPlugin;

pub struct PerimeterScan;

impl CheckPlugin for PerimeterScan {
    fn name(&self) -> &'static str {
        "perimeter"
    }

    fn run(&self) -> Vec<Finding> {
        Vec::new()
    }
}

pub struct RateLimitScan;

impl CheckPlugin for RateLimitScan {
    fn name(&self) -> &'static str {
        "rate-limit"
    }

    fn run(&self) -> Vec<Finding> {
        Vec::new()
    }
}

pub struct AuthzScan;

impl CheckPlugin for AuthzScan {
    fn name(&self) -> &'static str {
        "authz"
    }

    fn run(&self) -> Vec<Finding> {
        Vec::new()
    }
}

pub struct MobileBuildScan;

impl CheckPlugin for MobileBuildScan {
    fn name(&self) -> &'static str {
        "mobile-build"
    }

    fn run(&self) -> Vec<Finding> {
        Vec::new()
    }
}

pub fn stub_finding(category: &str) -> Finding {
    Finding {
        category: category.to_string(),
        severity: Severity::Low,
        details: Some("stub".to_string()),
    }
}

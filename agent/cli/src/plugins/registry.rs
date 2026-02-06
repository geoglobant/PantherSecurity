use crate::ports::CheckPlugin;
use crate::plugins::scans::{AuthzScan, MobileBuildScan, PerimeterScan, RateLimitScan};

pub fn builtin_plugins() -> Vec<Box<dyn CheckPlugin>> {
    vec![
        Box::new(PerimeterScan),
        Box::new(RateLimitScan),
        Box::new(AuthzScan),
        Box::new(MobileBuildScan),
    ]
}

pub fn plugin_by_name(name: &str) -> Option<Box<dyn CheckPlugin>> {
    match name {
        "perimeter" => Some(Box::new(PerimeterScan)),
        "rate-limit" => Some(Box::new(RateLimitScan)),
        "authz" => Some(Box::new(AuthzScan)),
        "mobile-build" => Some(Box::new(MobileBuildScan)),
        _ => None,
    }
}

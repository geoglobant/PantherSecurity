//! Rust core for Mobile AppSec Platform (hexagonal architecture).

pub mod domain;
pub mod ports;
pub mod app;
pub mod adapters;
pub mod sdk;

pub use app::core_service::CoreService;
pub use domain::policy::{Decision, PolicyEngine, PolicyRule, PolicySet};
pub use domain::risk::{Finding, RiskScore};
pub use domain::pinning::SpkiPinset;
pub use adapters::http::{HttpConfig, HttpTelemetryClient};
pub use sdk::{PinningConfig, Sdk, SdkConfig};
pub use domain::telemetry::{
    ActionContext, AttestationProvider, AttestationResult, AttestationStatus, DeviceInfo,
    IntegritySignals, SessionInfo, TelemetryAuth, TelemetryEnvelope, TelemetryEvent,
};

use crate::domain::policy::PolicySet;
use crate::domain::risk::{Finding, RiskScore};
use crate::domain::telemetry::{
    AttestationResult, IntegritySignals, Platform, TelemetryEnvelope,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortError {
    pub message: String,
}

impl PortError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

pub trait TelemetrySink {
    fn send(&self, envelope: &TelemetryEnvelope) -> Result<(), PortError>;
}

pub trait PolicyStore {
    fn get_policy(
        &self,
        app_id: &str,
        app_version: &str,
        env: &str,
        platform: Platform,
    ) -> Result<PolicySet, PortError>;
}

pub trait Clock {
    fn now(&self) -> String;
}

pub trait CryptoSigner {
    fn sign(&self, payload: &[u8]) -> Result<String, PortError>;
}

pub trait RiskScorer {
    fn score(
        &self,
        signals: &IntegritySignals,
        attestation: Option<&AttestationResult>,
        findings: &[Finding],
    ) -> RiskScore;
}

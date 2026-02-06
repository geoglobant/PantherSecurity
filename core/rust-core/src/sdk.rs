use chrono::{DateTime, Utc};

use crate::adapters::http::{HttpConfig, HttpTelemetryClient};
use crate::domain::pinning::SpkiPinset;
use crate::domain::policy::{Decision, PolicySet};
use crate::domain::risk::{Finding, RiskScore};
use crate::domain::telemetry::{
    ActionContext, AttestationResult, DeviceInfo, IntegritySignals, Platform, SessionInfo,
    TelemetryAuth, TelemetryEnvelope, TelemetryEvent,
};
use crate::ports::{Clock, CryptoSigner, PortError, RiskScorer};
use crate::CoreService;

#[derive(Clone, Debug)]
pub struct PinningConfig {
    pub current_spki_hashes: Vec<String>,
    pub previous_spki_hashes: Vec<String>,
    pub rotated_at: Option<String>,
    pub rotation_window_days: Option<u32>,
}

impl PinningConfig {
    pub fn to_pinset(&self) -> SpkiPinset {
        SpkiPinset {
            current: self.current_spki_hashes.clone(),
            previous: self.previous_spki_hashes.clone(),
            rotated_at: self
                .rotated_at
                .as_ref()
                .and_then(|value| DateTime::parse_from_rfc3339(value).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            rotation_window_days: self.rotation_window_days,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SdkConfig {
    pub app_id: String,
    pub app_version: String,
    pub env: String,
    pub platform: Platform,
    pub base_url: String,
    pub api_token: Option<String>,
    pub device_info: DeviceInfo,
    pub pinning: Option<PinningConfig>,
}

pub struct Sdk {
    config: SdkConfig,
    core: CoreService<HttpTelemetryClient, HttpTelemetryClient, SystemClock, NoopSigner, SimpleRiskScorer>,
}

impl Sdk {
    pub fn new(config: SdkConfig) -> Result<Self, PortError> {
        let http = HttpTelemetryClient::new(HttpConfig {
            base_url: config.base_url.clone(),
            api_token: config.api_token.clone(),
        })?;

        let core = CoreService::new(
            http.clone(),
            http,
            SystemClock,
            NoopSigner,
            SimpleRiskScorer,
        );

        Ok(Self { config, core })
    }

    pub fn fetch_policy(&self) -> Result<PolicySet, PortError> {
        self.core.fetch_policy(
            &self.config.app_id,
            &self.config.app_version,
            &self.config.env,
            self.config.platform.clone(),
        )
    }

    pub fn emit_event(
        &self,
        action: ActionContext,
        signals: IntegritySignals,
        attestation: Option<AttestationResult>,
        session: Option<SessionInfo>,
    ) -> Result<TelemetryEnvelope, PortError> {
        let event = TelemetryEvent {
            event_id: uuid(),
            app_id: self.config.app_id.clone(),
            app_version: self.config.app_version.clone(),
            env: self.config.env.clone(),
            device: self.config.device_info.clone(),
            session,
            signals,
            attestation,
            action,
            timestamp: None,
            signature: None,
        };

        let auth = TelemetryAuth {
            api_token: self.config.api_token.clone(),
        };

        self.core.emit_telemetry(event, auth)
    }

    pub fn decide_action(
        &self,
        policy: &PolicySet,
        action: &ActionContext,
        signals: &IntegritySignals,
        attestation: Option<&AttestationResult>,
        findings: &[Finding],
    ) -> Decision {
        self.core
            .decide_action(policy, action, signals, attestation, findings)
    }

    pub fn baseline_signals() -> IntegritySignals {
        IntegritySignals {
            jailbreak: false,
            root: false,
            debugger: false,
            hooking: false,
            proxy_detected: false,
        }
    }

    pub fn validate_pinning(&self, presented_spki_hash: &str) -> bool {
        let config = match &self.config.pinning {
            Some(value) => value,
            None => return true,
        };
        let pinset = config.to_pinset();
        pinset.is_allowed(presented_spki_hash, Utc::now())
    }
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> String {
        Utc::now().to_rfc3339()
    }
}

pub struct NoopSigner;

impl CryptoSigner for NoopSigner {
    fn sign(&self, _payload: &[u8]) -> Result<String, PortError> {
        Ok("stub-signature".to_string())
    }
}

pub struct SimpleRiskScorer;

impl RiskScorer for SimpleRiskScorer {
    fn score(
        &self,
        signals: &IntegritySignals,
        attestation: Option<&AttestationResult>,
        findings: &[Finding],
    ) -> RiskScore {
        let mut score = 0u32;
        if signals.jailbreak || signals.root {
            score += 40;
        }
        if signals.debugger || signals.hooking {
            score += 30;
        }
        if signals.proxy_detected {
            score += 20;
        }
        if let Some(att) = attestation {
            if matches!(att.status, crate::domain::telemetry::AttestationStatus::Fail) {
                score += 30;
            }
        }
        score += findings.len().saturating_mul(5) as u32;
        RiskScore::new(score)
    }
}

fn uuid() -> String {
    // Simple unique-ish placeholder. Replace with UUID generator when needed.
    let ts = Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("evt-{}", ts)
}

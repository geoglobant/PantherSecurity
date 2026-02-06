use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::domain::policy::{Decision, PolicyConditions, PolicyRule, PolicySet};
use crate::domain::risk::{Finding, RiskScore, Severity};
use crate::domain::telemetry::{
    ActionContext, AttestationProvider, AttestationResult, AttestationStatus, DeviceInfo,
    IntegritySignals, Platform, SessionInfo, TelemetryEvent,
};

#[derive(Debug)]
pub struct DtoError {
    pub message: String,
}

impl DtoError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PlatformDto {
    Ios,
    Android,
}

impl From<Platform> for PlatformDto {
    fn from(value: Platform) -> Self {
        match value {
            Platform::Ios => PlatformDto::Ios,
            Platform::Android => PlatformDto::Android,
        }
    }
}

impl From<PlatformDto> for Platform {
    fn from(value: PlatformDto) -> Self {
        match value {
            PlatformDto::Ios => Platform::Ios,
            PlatformDto::Android => Platform::Android,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AttestationProviderDto {
    AppAttest,
    PlayIntegrity,
    None,
}

impl From<AttestationProvider> for AttestationProviderDto {
    fn from(value: AttestationProvider) -> Self {
        match value {
            AttestationProvider::AppAttest => AttestationProviderDto::AppAttest,
            AttestationProvider::PlayIntegrity => AttestationProviderDto::PlayIntegrity,
            AttestationProvider::None => AttestationProviderDto::None,
        }
    }
}

impl From<AttestationProviderDto> for AttestationProvider {
    fn from(value: AttestationProviderDto) -> Self {
        match value {
            AttestationProviderDto::AppAttest => AttestationProvider::AppAttest,
            AttestationProviderDto::PlayIntegrity => AttestationProvider::PlayIntegrity,
            AttestationProviderDto::None => AttestationProvider::None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AttestationStatusDto {
    Pass,
    Fail,
    Unknown,
}

impl From<AttestationStatus> for AttestationStatusDto {
    fn from(value: AttestationStatus) -> Self {
        match value {
            AttestationStatus::Pass => AttestationStatusDto::Pass,
            AttestationStatus::Fail => AttestationStatusDto::Fail,
            AttestationStatus::Unknown => AttestationStatusDto::Unknown,
        }
    }
}

impl From<AttestationStatusDto> for AttestationStatus {
    fn from(value: AttestationStatusDto) -> Self {
        match value {
            AttestationStatusDto::Pass => AttestationStatus::Pass,
            AttestationStatusDto::Fail => AttestationStatus::Fail,
            AttestationStatusDto::Unknown => AttestationStatus::Unknown,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DecisionDto {
    Allow,
    StepUp,
    Degrade,
    Deny,
}

impl From<Decision> for DecisionDto {
    fn from(value: Decision) -> Self {
        match value {
            Decision::Allow => DecisionDto::Allow,
            Decision::StepUp => DecisionDto::StepUp,
            Decision::Degrade => DecisionDto::Degrade,
            Decision::Deny => DecisionDto::Deny,
        }
    }
}

impl From<DecisionDto> for Decision {
    fn from(value: DecisionDto) -> Self {
        match value {
            DecisionDto::Allow => Decision::Allow,
            DecisionDto::StepUp => Decision::StepUp,
            DecisionDto::Degrade => Decision::Degrade,
            DecisionDto::Deny => Decision::Deny,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SeverityDto {
    Low,
    Medium,
    High,
    Critical,
}

impl From<Severity> for SeverityDto {
    fn from(value: Severity) -> Self {
        match value {
            Severity::Low => SeverityDto::Low,
            Severity::Medium => SeverityDto::Medium,
            Severity::High => SeverityDto::High,
            Severity::Critical => SeverityDto::Critical,
        }
    }
}

impl From<SeverityDto> for Severity {
    fn from(value: SeverityDto) -> Self {
        match value {
            SeverityDto::Low => Severity::Low,
            SeverityDto::Medium => Severity::Medium,
            SeverityDto::High => Severity::High,
            SeverityDto::Critical => Severity::Critical,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DeviceInfoDto {
    pub platform: PlatformDto,
    pub os_version: String,
    pub model: String,
}

impl From<DeviceInfo> for DeviceInfoDto {
    fn from(value: DeviceInfo) -> Self {
        Self {
            platform: value.platform.into(),
            os_version: value.os_version,
            model: value.model,
        }
    }
}

impl From<DeviceInfoDto> for DeviceInfo {
    fn from(value: DeviceInfoDto) -> Self {
        Self {
            platform: value.platform.into(),
            os_version: value.os_version,
            model: value.model,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SessionInfoDto {
    pub session_id: String,
    pub user_id_hash: Option<String>,
}

impl From<SessionInfo> for SessionInfoDto {
    fn from(value: SessionInfo) -> Self {
        Self {
            session_id: value.session_id,
            user_id_hash: value.user_id_hash,
        }
    }
}

impl From<SessionInfoDto> for SessionInfo {
    fn from(value: SessionInfoDto) -> Self {
        Self {
            session_id: value.session_id,
            user_id_hash: value.user_id_hash,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct IntegritySignalsDto {
    pub jailbreak: bool,
    pub root: bool,
    pub debugger: bool,
    pub hooking: bool,
    pub proxy_detected: bool,
}

impl From<IntegritySignals> for IntegritySignalsDto {
    fn from(value: IntegritySignals) -> Self {
        Self {
            jailbreak: value.jailbreak,
            root: value.root,
            debugger: value.debugger,
            hooking: value.hooking,
            proxy_detected: value.proxy_detected,
        }
    }
}

impl From<IntegritySignalsDto> for IntegritySignals {
    fn from(value: IntegritySignalsDto) -> Self {
        Self {
            jailbreak: value.jailbreak,
            root: value.root,
            debugger: value.debugger,
            hooking: value.hooking,
            proxy_detected: value.proxy_detected,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AttestationResultDto {
    pub provider: AttestationProviderDto,
    pub result: AttestationStatusDto,
    pub timestamp: Option<String>,
}

impl From<AttestationResult> for AttestationResultDto {
    fn from(value: AttestationResult) -> Self {
        Self {
            provider: value.provider.into(),
            result: value.status.into(),
            timestamp: value.timestamp,
        }
    }
}

impl From<AttestationResultDto> for AttestationResult {
    fn from(value: AttestationResultDto) -> Self {
        Self {
            provider: value.provider.into(),
            status: value.result.into(),
            timestamp: value.timestamp,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ActionContextDto {
    pub name: String,
    pub context: Option<String>,
}

impl From<ActionContext> for ActionContextDto {
    fn from(value: ActionContext) -> Self {
        Self {
            name: value.name,
            context: value.context,
        }
    }
}

impl From<ActionContextDto> for ActionContext {
    fn from(value: ActionContextDto) -> Self {
        Self {
            name: value.name,
            context: value.context,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TelemetryEventDto {
    pub event_id: String,
    pub app_id: String,
    pub app_version: String,
    pub env: String,
    pub device: DeviceInfoDto,
    pub session: Option<SessionInfoDto>,
    pub signals: IntegritySignalsDto,
    pub attestation: Option<AttestationResultDto>,
    pub action: ActionContextDto,
    pub timestamp: String,
    pub signature: String,
}

impl TryFrom<TelemetryEvent> for TelemetryEventDto {
    type Error = DtoError;

    fn try_from(value: TelemetryEvent) -> Result<Self, Self::Error> {
        let timestamp = value
            .timestamp
            .ok_or_else(|| DtoError::new("telemetry.timestamp is required"))?;
        let signature = value
            .signature
            .ok_or_else(|| DtoError::new("telemetry.signature is required"))?;

        Ok(Self {
            event_id: value.event_id,
            app_id: value.app_id,
            app_version: value.app_version,
            env: value.env,
            device: value.device.into(),
            session: value.session.map(Into::into),
            signals: value.signals.into(),
            attestation: value.attestation.map(Into::into),
            action: value.action.into(),
            timestamp,
            signature,
        })
    }
}

impl From<TelemetryEventDto> for TelemetryEvent {
    fn from(value: TelemetryEventDto) -> Self {
        Self {
            event_id: value.event_id,
            app_id: value.app_id,
            app_version: value.app_version,
            env: value.env,
            device: value.device.into(),
            session: value.session.map(Into::into),
            signals: value.signals.into(),
            attestation: value.attestation.map(Into::into),
            action: value.action.into(),
            timestamp: Some(value.timestamp),
            signature: Some(value.signature),
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PolicyConditionsDto {
    pub attestation: Option<AttestationStatusDto>,
    pub debugger: Option<bool>,
    pub hooking: Option<bool>,
    pub proxy_detected: Option<bool>,
    pub app_version: Option<String>,
    pub risk_score_gte: Option<u32>,
}

impl From<PolicyConditions> for PolicyConditionsDto {
    fn from(value: PolicyConditions) -> Self {
        Self {
            attestation: value.attestation_status.map(Into::into),
            debugger: value.debugger,
            hooking: value.hooking,
            proxy_detected: value.proxy_detected,
            app_version: value.app_version,
            risk_score_gte: value.risk_score_gte,
        }
    }
}

impl From<PolicyConditionsDto> for PolicyConditions {
    fn from(value: PolicyConditionsDto) -> Self {
        Self {
            attestation_status: value.attestation.map(Into::into),
            debugger: value.debugger,
            hooking: value.hooking,
            proxy_detected: value.proxy_detected,
            app_version: value.app_version,
            risk_score_gte: value.risk_score_gte,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PolicyRuleDto {
    pub action: String,
    pub decision: DecisionDto,
    pub conditions: Option<PolicyConditionsDto>,
}

impl From<PolicyRule> for PolicyRuleDto {
    fn from(value: PolicyRule) -> Self {
        Self {
            action: value.action,
            decision: value.decision.into(),
            conditions: Some(value.conditions.into()),
        }
    }
}

impl From<PolicyRuleDto> for PolicyRule {
    fn from(value: PolicyRuleDto) -> Self {
        Self {
            action: value.action,
            decision: value.decision.into(),
            conditions: value.conditions.map(Into::into).unwrap_or_default(),
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PolicyDto {
    pub policy_id: String,
    pub app_id: String,
    pub app_version: String,
    pub env: String,
    pub rules: Vec<PolicyRuleDto>,
    pub signature: String,
    pub issued_at: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PolicyUpsertDto {
    pub device_platform: String,
    pub policy: PolicyDto,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PolicyUpsertResponse {
    pub status: String,
    pub stored_at: String,
}

impl PolicyDto {
    pub fn new(policy: PolicySet, signature: String, issued_at: String) -> Self {
        Self {
            policy_id: policy.policy_id,
            app_id: policy.app_id,
            app_version: policy.app_version,
            env: policy.env,
            rules: policy.rules.into_iter().map(Into::into).collect(),
            signature,
            issued_at,
        }
    }
}

impl From<PolicyDto> for PolicySet {
    fn from(value: PolicyDto) -> Self {
        Self {
            policy_id: value.policy_id,
            app_id: value.app_id,
            app_version: value.app_version,
            env: value.env,
            rules: value.rules.into_iter().map(Into::into).collect(),
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FindingDto {
    pub category: String,
    pub severity: SeverityDto,
    pub evidence: Option<serde_json::Value>,
}

impl From<Finding> for FindingDto {
    fn from(value: Finding) -> Self {
        Self {
            category: value.category,
            severity: value.severity.into(),
            evidence: None,
        }
    }
}

impl From<FindingDto> for Finding {
    fn from(value: FindingDto) -> Self {
        Self {
            category: value.category,
            severity: value.severity.into(),
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PipelineInfoDto {
    pub provider: String,
    pub run_id: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReportArtifactsDto {
    pub format: String,
    pub payload: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReportUploadDto {
    pub report_id: String,
    pub app_id: String,
    pub env: String,
    pub source: String,
    pub pipeline: Option<PipelineInfoDto>,
    pub artifacts: ReportArtifactsDto,
    pub findings: Option<Vec<FindingDto>>,
    pub timestamp: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StatusOk {
    pub status: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StatusAccepted {
    pub status: String,
}

pub fn compute_risk(findings: &[FindingDto]) -> Result<RiskScore, DtoError> {
    let mut score: u32 = 0;
    for finding in findings {
        score += match finding.severity {
            SeverityDto::Low => 5,
            SeverityDto::Medium => 10,
            SeverityDto::High => 20,
            SeverityDto::Critical => 30,
        };
    }
    Ok(RiskScore::new(score))
}

pub fn validate_telemetry_event(dto: &TelemetryEventDto) -> Result<(), DtoError> {
    validate_non_empty("event_id", &dto.event_id)?;
    validate_non_empty("app_id", &dto.app_id)?;
    validate_non_empty("app_version", &dto.app_version)?;
    validate_non_empty("env", &dto.env)?;
    validate_non_empty("device.os_version", &dto.device.os_version)?;
    validate_non_empty("device.model", &dto.device.model)?;
    validate_non_empty("action.name", &dto.action.name)?;
    validate_non_empty("timestamp", &dto.timestamp)?;
    validate_non_empty("signature", &dto.signature)?;
    Ok(())
}

pub fn validate_policy(dto: &PolicyDto) -> Result<(), DtoError> {
    validate_non_empty("policy_id", &dto.policy_id)?;
    validate_non_empty("app_id", &dto.app_id)?;
    validate_non_empty("app_version", &dto.app_version)?;
    validate_non_empty("env", &dto.env)?;
    validate_non_empty("signature", &dto.signature)?;
    validate_non_empty("issued_at", &dto.issued_at)?;
    if dto.rules.is_empty() {
        return Err(DtoError::new("policy.rules must not be empty"));
    }
    for rule in &dto.rules {
        validate_non_empty("policy.rule.action", &rule.action)?;
    }
    Ok(())
}

pub fn validate_report_upload(dto: &ReportUploadDto) -> Result<(), DtoError> {
    validate_non_empty("report_id", &dto.report_id)?;
    validate_non_empty("app_id", &dto.app_id)?;
    validate_non_empty("env", &dto.env)?;
    validate_non_empty("source", &dto.source)?;
    validate_non_empty("artifacts.format", &dto.artifacts.format)?;
    validate_non_empty("artifacts.payload", &dto.artifacts.payload)?;
    validate_non_empty("timestamp", &dto.timestamp)?;
    Ok(())
}

fn validate_non_empty(field: &str, value: &str) -> Result<(), DtoError> {
    if value.trim().is_empty() {
        return Err(DtoError::new(format!("{} must not be empty", field)));
    }
    Ok(())
}

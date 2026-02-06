#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceInfo {
    pub platform: Platform,
    pub os_version: String,
    pub model: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Platform {
    Ios,
    Android,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionInfo {
    pub session_id: String,
    pub user_id_hash: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegritySignals {
    pub jailbreak: bool,
    pub root: bool,
    pub debugger: bool,
    pub hooking: bool,
    pub proxy_detected: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttestationProvider {
    AppAttest,
    PlayIntegrity,
    None,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttestationStatus {
    Pass,
    Fail,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttestationResult {
    pub provider: AttestationProvider,
    pub status: AttestationStatus,
    pub timestamp: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionContext {
    pub name: String,
    pub context: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TelemetryEvent {
    pub event_id: String,
    pub app_id: String,
    pub app_version: String,
    pub env: String,
    pub device: DeviceInfo,
    pub session: Option<SessionInfo>,
    pub signals: IntegritySignals,
    pub attestation: Option<AttestationResult>,
    pub action: ActionContext,
    pub timestamp: Option<String>,
    pub signature: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TelemetryAuth {
    pub api_token: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TelemetryEnvelope {
    pub event: TelemetryEvent,
    pub auth: TelemetryAuth,
}

impl TelemetryEnvelope {
    pub fn new(event: TelemetryEvent, auth: TelemetryAuth) -> Self {
        Self { event, auth }
    }
}

impl TelemetryEvent {
    pub fn signing_payload(&self) -> String {
        // Placeholder for canonical serialization.
        format!(
            "{}:{}:{}:{}:{}",
            self.event_id, self.app_id, self.app_version, self.env, self.action.name
        )
    }
}

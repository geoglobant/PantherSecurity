use reqwest::blocking::Client;
use serde::Serialize;

use crate::domain::policy::PolicySet;
use crate::domain::telemetry::{Platform, TelemetryAuth, TelemetryEnvelope};
use crate::ports::{PolicyStore, PortError, TelemetrySink};
use crate::adapters::serialization::{
    validate_telemetry_event, PolicyDto, PolicyUpsertDto, PolicyUpsertResponse, TelemetryEventDto,
};

#[derive(Clone, Debug)]
pub struct HttpConfig {
    pub base_url: String,
    pub api_token: Option<String>,
}

#[derive(Clone)]
pub struct HttpTelemetryClient {
    client: Client,
    config: HttpConfig,
}

impl HttpTelemetryClient {
    pub fn new(config: HttpConfig) -> Result<Self, PortError> {
        let client = Client::builder()
            .build()
            .map_err(|err| PortError::new(err.to_string()))?;
        Ok(Self { client, config })
    }

    pub fn fetch_policy_current(
        &self,
        app_id: &str,
        app_version: &str,
        env: &str,
        platform: Platform,
    ) -> Result<PolicySet, PortError> {
        #[derive(Serialize)]
        struct PolicyQuery<'a> {
            app_id: &'a str,
            app_version: &'a str,
            env: &'a str,
            device_platform: &'a str,
        }

        let url = format!("{}/v1/policies/current", self.config.base_url.trim_end_matches('/'));
        let query = PolicyQuery {
            app_id,
            app_version,
            env,
            device_platform: match platform {
                Platform::Ios => "ios",
                Platform::Android => "android",
            },
        };

        let mut request = self.client.get(url).query(&query);
        if let Some(token) = &self.config.api_token {
            request = request.bearer_auth(token);
        }

        let response = request
            .send()
            .map_err(|err| PortError::new(err.to_string()))?;

        if !response.status().is_success() {
            return Err(PortError::new(format!(
                "policy fetch failed: {}",
                response.status()
            )));
        }

        let policy = response
            .json::<PolicyDto>()
            .map_err(|err| PortError::new(err.to_string()))?;

        Ok(policy.into())
    }

    pub fn upsert_policy(
        &self,
        request: &PolicyUpsertDto,
    ) -> Result<PolicyUpsertResponse, PortError> {
        let url = format!("{}/v1/policies", self.config.base_url.trim_end_matches('/'));
        let mut req = self.client.post(url).json(request);
        if let Some(token) = &self.config.api_token {
            req = req.bearer_auth(token);
        }

        let response = req
            .send()
            .map_err(|err| PortError::new(err.to_string()))?;

        if !response.status().is_success() {
            return Err(PortError::new(format!(
                "policy upsert failed: {}",
                response.status()
            )));
        }

        response
            .json::<PolicyUpsertResponse>()
            .map_err(|err| PortError::new(err.to_string()))
    }

    fn auth_token(&self, auth: &TelemetryAuth) -> Option<String> {
        auth.api_token
            .clone()
            .or_else(|| self.config.api_token.clone())
    }
}

impl TelemetrySink for HttpTelemetryClient {
    fn send(&self, envelope: &TelemetryEnvelope) -> Result<(), PortError> {
        let event = TelemetryEventDto::try_from(envelope.event.clone())
            .map_err(|err| PortError::new(err.message))?;
        validate_telemetry_event(&event).map_err(|err| PortError::new(err.message))?;

        let url = format!("{}/v1/telemetry/events", self.config.base_url.trim_end_matches('/'));
        let mut request = self.client.post(url).json(&event);

        if let Some(token) = self.auth_token(&envelope.auth) {
            request = request.bearer_auth(token);
        }

        let response = request
            .send()
            .map_err(|err| PortError::new(err.to_string()))?;

        if !response.status().is_success() {
            return Err(PortError::new(format!(
                "telemetry send failed: {}",
                response.status()
            )));
        }

        Ok(())
    }
}

impl PolicyStore for HttpTelemetryClient {
    fn get_policy(
        &self,
        app_id: &str,
        app_version: &str,
        env: &str,
        platform: Platform,
    ) -> Result<PolicySet, PortError> {
        self.fetch_policy_current(app_id, app_version, env, platform)
    }
}

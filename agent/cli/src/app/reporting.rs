use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::Utc;
use reqwest::blocking::Client;
use serde::Serialize;
use uuid::Uuid;

use crate::domain::report::{Finding, Report, Severity};

#[derive(Debug)]
pub struct ReportError {
    pub message: String,
}

impl ReportError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Serialize)]
pub struct ReportArtifactsDto {
    format: String,
    payload: String,
}

#[derive(Serialize)]
pub struct PipelineInfoDto {
    provider: String,
    run_id: String,
}

#[derive(Serialize)]
pub struct FindingDto {
    category: String,
    severity: Severity,
    details: Option<String>,
}

#[derive(Serialize)]
pub struct ReportUploadDto {
    report_id: String,
    app_id: String,
    env: String,
    source: String,
    pipeline: Option<PipelineInfoDto>,
    artifacts: ReportArtifactsDto,
    findings: Vec<FindingDto>,
    timestamp: String,
}

pub struct ReportOptions {
    pub endpoint: String,
    pub app_id: String,
    pub env: String,
    pub source: String,
    pub pipeline_provider: Option<String>,
    pub pipeline_run_id: Option<String>,
    pub token: Option<String>,
}

pub fn submit_report(report: &Report, options: ReportOptions) -> Result<(), ReportError> {
    let payload = build_payload(report, &options)?;
    let client = Client::new();
    let mut request = client.post(&payload.endpoint).json(&payload.body);
    if let Some(token) = &options.token {
        request = request.bearer_auth(token);
    }
    let response = request
        .send()
        .map_err(|err| ReportError::new(err.to_string()))?;

    if !response.status().is_success() {
        return Err(ReportError::new(format!(
            "upload failed: {}",
            response.status()
        )));
    }

    Ok(())
}

pub fn build_payload(
    report: &Report,
    options: &ReportOptions,
) -> Result<ReportPayload, ReportError> {
    let report_id = Uuid::new_v4().to_string();
    let timestamp = Utc::now().to_rfc3339();

    let report_json = serde_json::to_string(report)
        .map_err(|err| ReportError::new(err.to_string()))?;
    let artifacts = ReportArtifactsDto {
        format: "json".to_string(),
        payload: STANDARD.encode(report_json.as_bytes()),
    };

    let pipeline = match (options.pipeline_provider, options.pipeline_run_id) {
        (Some(provider), Some(run_id)) => Some(PipelineInfoDto { provider, run_id }),
        _ => None,
    };

    let findings = report
        .findings
        .iter()
        .map(|finding| FindingDto {
            category: finding.category.clone(),
            severity: finding.severity.clone(),
            details: finding.details.clone(),
        })
        .collect::<Vec<_>>();

    Ok(ReportPayload {
        endpoint: options.endpoint.clone(),
        body: ReportUploadDto {
            report_id,
            app_id: options.app_id.clone(),
            env: options.env.clone(),
            source: options.source.clone(),
            pipeline,
            artifacts,
            findings,
            timestamp,
        },
    })
}

pub struct ReportPayload {
    pub endpoint: String,
    pub body: ReportUploadDto,
}

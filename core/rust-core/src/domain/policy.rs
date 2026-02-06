use crate::domain::risk::RiskScore;
use crate::domain::telemetry::{ActionContext, AttestationResult, AttestationStatus, IntegritySignals};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Decision {
    Allow,
    StepUp,
    Degrade,
    Deny,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyConditions {
    pub attestation_status: Option<AttestationStatus>,
    pub debugger: Option<bool>,
    pub hooking: Option<bool>,
    pub proxy_detected: Option<bool>,
    pub app_version: Option<String>,
    pub risk_score_gte: Option<u32>,
}

impl Default for PolicyConditions {
    fn default() -> Self {
        Self {
            attestation_status: None,
            debugger: None,
            hooking: None,
            proxy_detected: None,
            app_version: None,
            risk_score_gte: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyRule {
    pub action: String,
    pub decision: Decision,
    pub conditions: PolicyConditions,
}

impl PolicyRule {
    pub fn matches(
        &self,
        ctx: &ActionContext,
        signals: &IntegritySignals,
        attestation: Option<&AttestationResult>,
        risk_score: RiskScore,
        app_version: &str,
    ) -> bool {
        if self.action != ctx.name {
            return false;
        }

        if let Some(required) = self.conditions.attestation_status {
            match attestation {
                Some(att) if att.status == required => {}
                _ => return false,
            }
        }

        if let Some(required) = self.conditions.debugger {
            if signals.debugger != required {
                return false;
            }
        }

        if let Some(required) = self.conditions.hooking {
            if signals.hooking != required {
                return false;
            }
        }

        if let Some(required) = self.conditions.proxy_detected {
            if signals.proxy_detected != required {
                return false;
            }
        }

        if let Some(required) = &self.conditions.app_version {
            if app_version != required {
                return false;
            }
        }

        if let Some(min_score) = self.conditions.risk_score_gte {
            if risk_score.value() < min_score {
                return false;
            }
        }

        true
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicySet {
    pub policy_id: String,
    pub app_id: String,
    pub app_version: String,
    pub env: String,
    pub rules: Vec<PolicyRule>,
}

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn evaluate(
        policy: &PolicySet,
        ctx: &ActionContext,
        signals: &IntegritySignals,
        attestation: Option<&AttestationResult>,
        risk_score: RiskScore,
    ) -> Decision {
        for rule in &policy.rules {
            if rule.matches(ctx, signals, attestation, risk_score, &policy.app_version) {
                return rule.decision.clone();
            }
        }
        Decision::Allow
    }
}

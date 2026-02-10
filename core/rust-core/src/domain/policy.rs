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

        if let Some(required) = &self.conditions.attestation_status {
            match attestation {
                Some(att) if &att.status == required => {}
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::risk::RiskScore;
    use crate::domain::telemetry::{ActionContext, AttestationProvider, AttestationResult, AttestationStatus, IntegritySignals};

    fn base_signals() -> IntegritySignals {
        IntegritySignals {
            jailbreak: false,
            root: false,
            debugger: false,
            hooking: false,
            proxy_detected: false,
        }
    }

    fn action(name: &str) -> ActionContext {
        ActionContext {
            name: name.to_string(),
            context: None,
        }
    }

    fn attestation(status: AttestationStatus) -> AttestationResult {
        AttestationResult {
            provider: AttestationProvider::AppAttest,
            status,
            timestamp: None,
        }
    }

    #[test]
    fn rule_matches_on_action_and_debugger_signal() {
        let rule = PolicyRule {
            action: "login".to_string(),
            decision: Decision::StepUp,
            conditions: PolicyConditions {
                debugger: Some(true),
                ..PolicyConditions::default()
            },
        };

        let ctx = action("login");
        let mut signals = base_signals();
        signals.debugger = true;

        assert!(rule.matches(&ctx, &signals, None, RiskScore::new(5), "1.0.0"));

        signals.debugger = false;
        assert!(!rule.matches(&ctx, &signals, None, RiskScore::new(5), "1.0.0"));
    }

    #[test]
    fn rule_matches_attestation_and_risk_score() {
        let rule = PolicyRule {
            action: "transfer".to_string(),
            decision: Decision::Deny,
            conditions: PolicyConditions {
                attestation_status: Some(AttestationStatus::Fail),
                risk_score_gte: Some(70),
                ..PolicyConditions::default()
            },
        };

        let ctx = action("transfer");
        let signals = base_signals();
        let att = attestation(AttestationStatus::Fail);

        assert!(rule.matches(&ctx, &signals, Some(&att), RiskScore::new(80), "1.0.0"));
        assert!(!rule.matches(&ctx, &signals, Some(&att), RiskScore::new(50), "1.0.0"));
    }

    #[test]
    fn policy_engine_returns_first_matching_rule() {
        let policy = PolicySet {
            policy_id: "policy".to_string(),
            app_id: "fintech.mobile".to_string(),
            app_version: "1.0.0".to_string(),
            env: "local".to_string(),
            rules: vec![
                PolicyRule {
                    action: "view_card".to_string(),
                    decision: Decision::Deny,
                    conditions: PolicyConditions {
                        hooking: Some(true),
                        ..PolicyConditions::default()
                    },
                },
                PolicyRule {
                    action: "view_card".to_string(),
                    decision: Decision::Allow,
                    conditions: PolicyConditions::default(),
                },
            ],
        };

        let ctx = action("view_card");
        let mut signals = base_signals();
        signals.hooking = true;

        let decision = PolicyEngine::evaluate(&policy, &ctx, &signals, None, RiskScore::new(10));
        assert_eq!(decision, Decision::Deny);
    }

    #[test]
    fn policy_engine_defaults_to_allow_when_no_match() {
        let policy = PolicySet {
            policy_id: "policy".to_string(),
            app_id: "fintech.mobile".to_string(),
            app_version: "1.0.0".to_string(),
            env: "local".to_string(),
            rules: vec![PolicyRule {
                action: "transfer".to_string(),
                decision: Decision::Deny,
                conditions: PolicyConditions {
                    proxy_detected: Some(true),
                    ..PolicyConditions::default()
                },
            }],
        };

        let ctx = action("transfer");
        let signals = base_signals();

        let decision = PolicyEngine::evaluate(&policy, &ctx, &signals, None, RiskScore::new(10));
        assert_eq!(decision, Decision::Allow);
    }
}

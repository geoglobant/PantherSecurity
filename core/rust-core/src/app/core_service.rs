use crate::domain::policy::{Decision, PolicyEngine, PolicySet};
use crate::domain::risk::Finding;
use crate::domain::telemetry::{
    ActionContext, AttestationResult, IntegritySignals, Platform, TelemetryAuth,
    TelemetryEnvelope, TelemetryEvent,
};
use crate::ports::{
    Clock, CryptoSigner, PolicyStore, PortError, RiskScorer, TelemetrySink,
};

pub struct CoreService<TS, PS, C, S, RS>
where
    TS: TelemetrySink,
    PS: PolicyStore,
    C: Clock,
    S: CryptoSigner,
    RS: RiskScorer,
{
    telemetry: TS,
    policy_store: PS,
    clock: C,
    signer: S,
    risk_scorer: RS,
}

impl<TS, PS, C, S, RS> CoreService<TS, PS, C, S, RS>
where
    TS: TelemetrySink,
    PS: PolicyStore,
    C: Clock,
    S: CryptoSigner,
    RS: RiskScorer,
{
    pub fn new(telemetry: TS, policy_store: PS, clock: C, signer: S, risk_scorer: RS) -> Self {
        Self {
            telemetry,
            policy_store,
            clock,
            signer,
            risk_scorer,
        }
    }

    pub fn emit_telemetry(
        &self,
        mut event: TelemetryEvent,
        auth: TelemetryAuth,
    ) -> Result<TelemetryEnvelope, PortError> {
        event.timestamp = Some(self.clock.now());
        let payload = event.signing_payload();
        let signature = self.signer.sign(payload.as_bytes())?;
        event.signature = Some(signature);
        let envelope = TelemetryEnvelope::new(event, auth);
        self.telemetry.send(&envelope)?;
        Ok(envelope)
    }

    pub fn fetch_policy(
        &self,
        app_id: &str,
        app_version: &str,
        env: &str,
        platform: Platform,
    ) -> Result<PolicySet, PortError> {
        self.policy_store
            .get_policy(app_id, app_version, env, platform)
    }

    pub fn decide_action(
        &self,
        policy: &PolicySet,
        ctx: &ActionContext,
        signals: &IntegritySignals,
        attestation: Option<&AttestationResult>,
        findings: &[Finding],
    ) -> Decision {
        let risk_score = self.risk_scorer.score(signals, attestation, findings);
        PolicyEngine::evaluate(policy, ctx, signals, attestation, risk_score)
    }
}

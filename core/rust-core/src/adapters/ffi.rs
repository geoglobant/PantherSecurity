use crate::domain::pinning::SpkiPinset;
use crate::domain::policy::{Decision, PolicyConditions, PolicyRule, PolicySet, PolicyEngine};
use crate::domain::risk::RiskScore;
use crate::domain::telemetry::{ActionContext, AttestationResult, AttestationStatus, IntegritySignals};
use chrono::{DateTime, Utc};

const FFI_DECISION_ALLOW: u32 = 0;
const FFI_DECISION_STEP_UP: u32 = 1;
const FFI_DECISION_DEGRADE: u32 = 2;
const FFI_DECISION_DENY: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FfiStr {
    pub ptr: *const u8,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FfiIntegritySignals {
    pub jailbreak: u8,
    pub root: u8,
    pub debugger: u8,
    pub hooking: u8,
    pub proxy_detected: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FfiPolicyConditions {
    pub attestation_status: i32,
    pub debugger: i32,
    pub hooking: i32,
    pub proxy_detected: i32,
    pub app_version: FfiStr,
    pub risk_score_gte: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FfiPolicyRule {
    pub action: FfiStr,
    pub decision: u32,
    pub conditions: FfiPolicyConditions,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FfiPolicySet {
    pub policy_id: FfiStr,
    pub app_id: FfiStr,
    pub app_version: FfiStr,
    pub env: FfiStr,
    pub rules_ptr: *const FfiPolicyRule,
    pub rules_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FfiStrArray {
    pub ptr: *const FfiStr,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FfiPinset {
    pub current: FfiStrArray,
    pub previous: FfiStrArray,
    pub rotated_at: FfiStr,
    pub rotation_window_days: i32,
}

fn parse_decision(value: u32) -> Result<Decision, ()> {
    match value {
        FFI_DECISION_ALLOW => Ok(Decision::Allow),
        FFI_DECISION_STEP_UP => Ok(Decision::StepUp),
        FFI_DECISION_DEGRADE => Ok(Decision::Degrade),
        FFI_DECISION_DENY => Ok(Decision::Deny),
        _ => Err(()),
    }
}

fn parse_optional_bool(value: i32) -> Result<Option<bool>, ()> {
    match value {
        -1 => Ok(None),
        0 => Ok(Some(false)),
        1 => Ok(Some(true)),
        _ => Err(()),
    }
}

fn parse_optional_attestation(value: i32) -> Result<Option<AttestationStatus>, ()> {
    match value {
        -1 => Ok(None),
        0 => Ok(Some(AttestationStatus::Unknown)),
        1 => Ok(Some(AttestationStatus::Pass)),
        2 => Ok(Some(AttestationStatus::Fail)),
        _ => Err(()),
    }
}

fn parse_bool_flag(value: u8) -> Result<bool, ()> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(()),
    }
}

fn str_from_ffi(value: FfiStr) -> Result<Option<String>, ()> {
    if value.ptr.is_null() {
        return Ok(None);
    }

    if value.len == 0 {
        return Ok(Some(String::new()));
    }

    let bytes = unsafe { std::slice::from_raw_parts(value.ptr, value.len) };
    let text = std::str::from_utf8(bytes).map_err(|_| ())?;
    Ok(Some(text.to_string()))
}

fn str_array_from_ffi(array: FfiStrArray) -> Result<Vec<String>, ()> {
    if array.len == 0 {
        return Ok(Vec::new());
    }

    if array.ptr.is_null() {
        return Err(());
    }

    let slice = unsafe { std::slice::from_raw_parts(array.ptr, array.len) };
    let mut values = Vec::with_capacity(slice.len());
    for item in slice {
        match str_from_ffi(*item)? {
            Some(value) => values.push(value),
            None => return Err(()),
        }
    }

    Ok(values)
}

fn parse_policy_conditions(conditions: FfiPolicyConditions) -> Result<PolicyConditions, ()> {
    let required_attestation = parse_optional_attestation(conditions.attestation_status)?;
    let condition_app_version = str_from_ffi(conditions.app_version)?;

    Ok(PolicyConditions {
        attestation_status: required_attestation,
        debugger: parse_optional_bool(conditions.debugger)?,
        hooking: parse_optional_bool(conditions.hooking)?,
        proxy_detected: parse_optional_bool(conditions.proxy_detected)?,
        app_version: match condition_app_version {
            Some(value) if !value.is_empty() => Some(value),
            _ => None,
        },
        risk_score_gte: if conditions.risk_score_gte == u32::MAX {
            None
        } else {
            Some(conditions.risk_score_gte)
        },
    })
}

fn parse_policy_rule(rule: &FfiPolicyRule) -> Result<PolicyRule, ()> {
    let action = str_from_ffi(rule.action).and_then(|value| value.ok_or(()))?;
    let decision = parse_decision(rule.decision)?;
    let conditions = parse_policy_conditions(rule.conditions)?;

    Ok(PolicyRule {
        action,
        decision,
        conditions,
    })
}

#[no_mangle]
pub extern "C" fn ps_evaluate_rule(
    action: FfiStr,
    app_version: FfiStr,
    rule_action: FfiStr,
    decision: u32,
    conditions: FfiPolicyConditions,
    signals: FfiIntegritySignals,
    attestation_status: i32,
    risk_score: u32,
) -> u32 {
    let decision = match parse_decision(decision) {
        Ok(value) => value,
        Err(_) => return FFI_DECISION_DENY,
    };

    let action_name = match str_from_ffi(action).and_then(|value| value.ok_or(())) {
        Ok(value) => value,
        Err(_) => return FFI_DECISION_DENY,
    };

    let rule_action_name = match str_from_ffi(rule_action).and_then(|value| value.ok_or(())) {
        Ok(value) => value,
        Err(_) => return FFI_DECISION_DENY,
    };

    let app_version_value = match str_from_ffi(app_version) {
        Ok(value) => value,
        Err(_) => return FFI_DECISION_DENY,
    };

    let signals = match (
        parse_bool_flag(signals.jailbreak),
        parse_bool_flag(signals.root),
        parse_bool_flag(signals.debugger),
        parse_bool_flag(signals.hooking),
        parse_bool_flag(signals.proxy_detected),
    ) {
        (Ok(jailbreak), Ok(root), Ok(debugger), Ok(hooking), Ok(proxy_detected)) => IntegritySignals {
            jailbreak,
            root,
            debugger,
            hooking,
            proxy_detected,
        },
        _ => return FFI_DECISION_DENY,
    };

    let runtime_attestation = match parse_optional_attestation(attestation_status) {
        Ok(Some(status)) => Some(AttestationResult {
            provider: crate::domain::telemetry::AttestationProvider::None,
            status,
            timestamp: None,
        }),
        Ok(None) => None,
        Err(_) => return FFI_DECISION_DENY,
    };

    let conditions = match parse_policy_conditions(conditions) {
        Ok(value) => value,
        Err(_) => return FFI_DECISION_DENY,
    };

    let rule = PolicyRule {
        action: rule_action_name,
        decision,
        conditions,
    };

    let policy = PolicySet {
        policy_id: "ffi".to_string(),
        app_id: "ffi".to_string(),
        app_version: app_version_value.unwrap_or_default(),
        env: "ffi".to_string(),
        rules: vec![rule],
    };

    let decision = PolicyEngine::evaluate(
        &policy,
        &ActionContext {
            name: action_name,
            context: None,
        },
        &signals,
        runtime_attestation.as_ref(),
        RiskScore::new(risk_score),
    );

    match decision {
        Decision::Allow => FFI_DECISION_ALLOW,
        Decision::StepUp => FFI_DECISION_STEP_UP,
        Decision::Degrade => FFI_DECISION_DEGRADE,
        Decision::Deny => FFI_DECISION_DENY,
    }
}

#[no_mangle]
pub extern "C" fn ps_evaluate_policy(
    policy: *const FfiPolicySet,
    action: FfiStr,
    signals: FfiIntegritySignals,
    attestation_status: i32,
    risk_score: u32,
) -> u32 {
    if policy.is_null() {
        return FFI_DECISION_DENY;
    }

    let action_name = match str_from_ffi(action).and_then(|value| value.ok_or(())) {
        Ok(value) => value,
        Err(_) => return FFI_DECISION_DENY,
    };

    let policy = unsafe { &*policy };
    let policy_id = match str_from_ffi(policy.policy_id) {
        Ok(value) => value.unwrap_or_default(),
        Err(_) => return FFI_DECISION_DENY,
    };
    let app_id = match str_from_ffi(policy.app_id) {
        Ok(value) => value.unwrap_or_default(),
        Err(_) => return FFI_DECISION_DENY,
    };
    let app_version = match str_from_ffi(policy.app_version) {
        Ok(value) => value.unwrap_or_default(),
        Err(_) => return FFI_DECISION_DENY,
    };
    let env = match str_from_ffi(policy.env) {
        Ok(value) => value.unwrap_or_default(),
        Err(_) => return FFI_DECISION_DENY,
    };

    if policy.rules_ptr.is_null() && policy.rules_len > 0 {
        return FFI_DECISION_DENY;
    }

    let rules = if policy.rules_len == 0 {
        Vec::new()
    } else {
        let slice = unsafe { std::slice::from_raw_parts(policy.rules_ptr, policy.rules_len) };
        let mut parsed = Vec::with_capacity(slice.len());
        for rule in slice {
            match parse_policy_rule(rule) {
                Ok(value) => parsed.push(value),
                Err(_) => return FFI_DECISION_DENY,
            }
        }
        parsed
    };

    let signals = match (
        parse_bool_flag(signals.jailbreak),
        parse_bool_flag(signals.root),
        parse_bool_flag(signals.debugger),
        parse_bool_flag(signals.hooking),
        parse_bool_flag(signals.proxy_detected),
    ) {
        (Ok(jailbreak), Ok(root), Ok(debugger), Ok(hooking), Ok(proxy_detected)) => IntegritySignals {
            jailbreak,
            root,
            debugger,
            hooking,
            proxy_detected,
        },
        _ => return FFI_DECISION_DENY,
    };

    let runtime_attestation = match parse_optional_attestation(attestation_status) {
        Ok(Some(status)) => Some(AttestationResult {
            provider: crate::domain::telemetry::AttestationProvider::None,
            status,
            timestamp: None,
        }),
        Ok(None) => None,
        Err(_) => return FFI_DECISION_DENY,
    };

    let policy_set = PolicySet {
        policy_id,
        app_id,
        app_version,
        env,
        rules,
    };

    let decision = PolicyEngine::evaluate(
        &policy_set,
        &ActionContext {
            name: action_name,
            context: None,
        },
        &signals,
        runtime_attestation.as_ref(),
        RiskScore::new(risk_score),
    );

    match decision {
        Decision::Allow => FFI_DECISION_ALLOW,
        Decision::StepUp => FFI_DECISION_STEP_UP,
        Decision::Degrade => FFI_DECISION_DEGRADE,
        Decision::Deny => FFI_DECISION_DENY,
    }
}

#[no_mangle]
pub extern "C" fn ps_pinning_is_allowed(pinset: FfiPinset, presented_hash: FfiStr) -> u8 {
    let current = match str_array_from_ffi(pinset.current) {
        Ok(values) => values,
        Err(_) => return 0,
    };
    let previous = match str_array_from_ffi(pinset.previous) {
        Ok(values) => values,
        Err(_) => return 0,
    };
    let rotated_at = match str_from_ffi(pinset.rotated_at) {
        Ok(Some(value)) if !value.is_empty() => {
            DateTime::parse_from_rfc3339(&value)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
        }
        _ => None,
    };

    let rotation_window_days = if pinset.rotation_window_days < 0 {
        None
    } else {
        Some(pinset.rotation_window_days as u32)
    };

    let presented = match str_from_ffi(presented_hash).and_then(|value| value.ok_or(())) {
        Ok(value) => value,
        Err(_) => return 0,
    };

    let pinset = SpkiPinset {
        current,
        previous,
        rotated_at,
        rotation_window_days,
    };

    if pinset.is_allowed(&presented, Utc::now()) {
        1
    } else {
        0
    }
}

import Foundation

@_silgen_name("ps_evaluate_policy")
func ps_evaluate_policy(
    _ policy: UnsafePointer<FfiPolicySet>,
    _ action: FfiStr,
    _ signals: FfiIntegritySignals,
    _ attestationStatus: Int32,
    _ riskScore: UInt32
) -> UInt32

@_silgen_name("ps_pinning_is_allowed")
func ps_pinning_is_allowed(_ pinset: FfiPinset, _ presentedHash: FfiStr) -> UInt8

struct FfiStr {
    var ptr: UnsafePointer<UInt8>?
    var len: Int
}

struct FfiIntegritySignals {
    var jailbreak: UInt8
    var root: UInt8
    var debugger: UInt8
    var hooking: UInt8
    var proxy_detected: UInt8
}

struct FfiPolicyConditions {
    var attestation_status: Int32
    var debugger: Int32
    var hooking: Int32
    var proxy_detected: Int32
    var app_version: FfiStr
    var risk_score_gte: UInt32
}

struct FfiPolicyRule {
    var action: FfiStr
    var decision: UInt32
    var conditions: FfiPolicyConditions
}

struct FfiPolicySet {
    var policy_id: FfiStr
    var app_id: FfiStr
    var app_version: FfiStr
    var env: FfiStr
    var rules_ptr: UnsafePointer<FfiPolicyRule>?
    var rules_len: Int
}

struct FfiStrArray {
    var ptr: UnsafePointer<FfiStr>?
    var len: Int
}

struct FfiPinset {
    var current: FfiStrArray
    var previous: FfiStrArray
    var rotated_at: FfiStr
    var rotation_window_days: Int32
}

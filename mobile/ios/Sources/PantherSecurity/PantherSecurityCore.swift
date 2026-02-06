import Foundation
import Darwin

final class PantherSecurityCore {
    func evaluate(
        policy: PantherSecurityPolicyResponse,
        action: PantherSecurityActionContext,
        signals: PantherSecurityIntegritySignals,
        attestationStatus: String?,
        riskScore: UInt32
    ) -> PantherSecurityDecision {
        let decision = withFfiPolicySet(policy: policy) { policyPtr in
            withFfiStr(action.name) { actionStr in
                let ffiSignals = FfiIntegritySignals(
                    jailbreak: signals.jailbreak ? 1 : 0,
                    root: signals.root ? 1 : 0,
                    debugger: signals.debugger ? 1 : 0,
                    hooking: signals.hooking ? 1 : 0,
                    proxy_detected: signals.proxyDetected ? 1 : 0
                )
                let attestationCode = mapAttestationStatus(attestationStatus)
                return ps_evaluate_policy(policyPtr, actionStr, ffiSignals, attestationCode, riskScore)
            }
        }

        return mapDecision(decision)
    }

    func validatePinning(pinning: PantherSecurityPinning, presentedSpkiHash: String) -> Bool {
        return withFfiPinset(pinning: pinning) { pinset in
            withFfiStr(presentedSpkiHash) { presented in
                ps_pinning_is_allowed(pinset, presented) == 1
            }
        }
    }

    private func withFfiPolicySet<T>(policy: PantherSecurityPolicyResponse, _ body: (UnsafePointer<FfiPolicySet>) -> T) -> T {
        let rules = policy.rules
        return withFfiStr(policy.policyId) { policyId in
            withFfiStr(policy.appId) { appId in
                withFfiStr(policy.appVersion) { appVersion in
                    withFfiStr(policy.env) { env in
                        withFfiPolicyRules(rules: rules) { rulesPtr, rulesLen in
                            var policySet = FfiPolicySet(
                                policy_id: policyId,
                                app_id: appId,
                                app_version: appVersion,
                                env: env,
                                rules_ptr: rulesPtr,
                                rules_len: rulesLen
                            )
                            return body(&policySet)
                        }
                    }
                }
            }
        }
    }

    private func withFfiPolicyRules<T>(rules: [PantherSecurityPolicyRule], _ body: (UnsafePointer<FfiPolicyRule>?, Int) -> T) -> T {
        guard !rules.isEmpty else {
            return body(nil, 0)
        }

        var cRules: [FfiPolicyRule] = []
        cRules.reserveCapacity(rules.count)

        let appVersionPtrs: [UnsafeMutablePointer<CChar>?] = rules.map { rule in
            guard let value = rule.conditions?.appVersion else { return nil }
            return strdup(value)
        }

        defer {
            for ptr in appVersionPtrs {
                if let ptr {
                    free(UnsafeMutableRawPointer(ptr))
                }
            }
        }

        return withFfiStrArray(rules.map { $0.action }) { actionPtrs, _ in
            guard let actionPtrs else {
                return body(nil, 0)
            }
            var actionIndex = 0

            for rule in rules {
                let actionStr = actionPtrs[actionIndex]
                let appVersionPtr = appVersionPtrs[actionIndex]
                actionIndex += 1

                let appVersionStr: FfiStr = {
                    guard let appVersionPtr else { return FfiStr(ptr: nil, len: 0) }
                    let raw = UnsafeRawPointer(appVersionPtr).assumingMemoryBound(to: UInt8.self)
                    return FfiStr(ptr: raw, len: strlen(appVersionPtr))
                }()

                let conditions = rule.conditions
                let ffiConditions = FfiPolicyConditions(
                    attestation_status: mapAttestationStatus(conditions?.attestation),
                    debugger: mapOptionalBool(conditions?.debugger),
                    hooking: mapOptionalBool(conditions?.hooking),
                    proxy_detected: mapOptionalBool(conditions?.proxyDetected),
                    app_version: appVersionStr,
                    risk_score_gte: mapRiskScoreGte(conditions?.riskScoreGte)
                )

                let ffiRule = FfiPolicyRule(
                    action: actionStr,
                    decision: mapDecisionCode(rule.decision),
                    conditions: ffiConditions
                )
                cRules.append(ffiRule)
            }

            return cRules.withUnsafeBufferPointer { buffer in
                body(buffer.baseAddress, buffer.count)
            }
        }
    }

    private func withFfiPinset<T>(pinning: PantherSecurityPinning, _ body: (FfiPinset) -> T) -> T {
        return withFfiStrArray(pinning.currentSpkiHashes) { currentPtr, currentLen in
            withFfiStrArray(pinning.previousSpkiHashes) { previousPtr, previousLen in
                withFfiStr(pinning.rotatedAt ?? "") { rotatedAt in
                    let pinset = FfiPinset(
                        current: FfiStrArray(ptr: currentPtr, len: currentLen),
                        previous: FfiStrArray(ptr: previousPtr, len: previousLen),
                        rotated_at: rotatedAt,
                        rotation_window_days: Int32(pinning.rotationWindowDays ?? -1)
                    )
                    return body(pinset)
                }
            }
        }
    }
}

private func mapOptionalBool(_ value: Bool?) -> Int32 {
    guard let value else { return -1 }
    return value ? 1 : 0
}

private func mapRiskScoreGte(_ value: Int?) -> UInt32 {
    guard let value else { return UInt32.max }
    return UInt32(max(0, value))
}

private func mapAttestationStatus(_ value: String?) -> Int32 {
    guard let value else { return -1 }
    switch value.lowercased() {
    case "pass": return 1
    case "fail": return 2
    case "unknown": return 0
    default: return -1
    }
}

private func mapDecisionCode(_ value: String) -> UInt32 {
    switch value.uppercased() {
    case "ALLOW": return 0
    case "STEP_UP": return 1
    case "DEGRADE": return 2
    case "DENY": return 3
    default: return 3
    }
}

private func mapDecision(_ value: UInt32) -> PantherSecurityDecision {
    switch value {
    case 0: return .allow
    case 1: return .stepUp
    case 2: return .degrade
    case 3: return .deny
    default: return .deny
    }
}

private func withFfiStr<T>(_ value: String, _ body: (FfiStr) -> T) -> T {
    return value.withCString { cStr in
        let raw = UnsafeRawPointer(cStr).assumingMemoryBound(to: UInt8.self)
        let ffi = FfiStr(ptr: raw, len: strlen(cStr))
        return body(ffi)
    }
}

private func withFfiStrArray<T>(_ values: [String], _ body: (UnsafePointer<FfiStr>?, Int) -> T) -> T {
    guard !values.isEmpty else {
        return body(nil, 0)
    }

    let cStrings: [UnsafeMutablePointer<CChar>?] = values.map { strdup($0) }
    defer {
        for ptr in cStrings {
            if let ptr {
                free(UnsafeMutableRawPointer(ptr))
            }
        }
    }

    let ffiStrings = cStrings.map { ptr -> FfiStr in
        guard let ptr else { return FfiStr(ptr: nil, len: 0) }
        let raw = UnsafeRawPointer(ptr).assumingMemoryBound(to: UInt8.self)
        return FfiStr(ptr: raw, len: strlen(ptr))
    }

    return ffiStrings.withUnsafeBufferPointer { buffer in
        body(buffer.baseAddress, buffer.count)
    }
}

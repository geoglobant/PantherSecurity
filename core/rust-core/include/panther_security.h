#pragma once

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    const uint8_t *ptr;
    size_t len;
} FfiStr;

typedef struct {
    uint8_t jailbreak;
    uint8_t root;
    uint8_t debugger;
    uint8_t hooking;
    uint8_t proxy_detected;
} FfiIntegritySignals;

typedef struct {
    int32_t attestation_status;
    int32_t debugger;
    int32_t hooking;
    int32_t proxy_detected;
    FfiStr app_version;
    uint32_t risk_score_gte;
} FfiPolicyConditions;

typedef struct {
    FfiStr action;
    uint32_t decision;
    FfiPolicyConditions conditions;
} FfiPolicyRule;

typedef struct {
    FfiStr policy_id;
    FfiStr app_id;
    FfiStr app_version;
    FfiStr env;
    const FfiPolicyRule *rules_ptr;
    size_t rules_len;
} FfiPolicySet;

typedef struct {
    const FfiStr *ptr;
    size_t len;
} FfiStrArray;

typedef struct {
    FfiStrArray current;
    FfiStrArray previous;
    FfiStr rotated_at;
    int32_t rotation_window_days;
} FfiPinset;

uint32_t ps_evaluate_rule(
    FfiStr action,
    FfiStr app_version,
    FfiStr rule_action,
    uint32_t decision,
    FfiPolicyConditions conditions,
    FfiIntegritySignals signals,
    int32_t attestation_status,
    uint32_t risk_score
);

uint32_t ps_evaluate_policy(
    const FfiPolicySet *policy,
    FfiStr action,
    FfiIntegritySignals signals,
    int32_t attestation_status,
    uint32_t risk_score
);

uint8_t ps_pinning_is_allowed(FfiPinset pinset, FfiStr presented_hash);

#ifdef __cplusplus
}
#endif

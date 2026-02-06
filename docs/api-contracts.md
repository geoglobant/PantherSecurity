# API Contracts (Draft)

This document provides examples and initial guidance. The formal specification is in:
- `docs/openapi.yaml`

This document describes initial contracts between SDK, Agent/CLI, and Backend.

## Principles
- JSON over HTTPS
- Path-based versioning: `/v1/...`
- Idempotency for events: `event_id`
- Payload signing (SDK) and strong authentication (mTLS or JWT)
- Forward compatibility: unknown fields must be ignored

## Authentication
- **SDK**: token per app + payload signature (HMAC or device asymmetric signature).
- **Agent/CLI**: token per pipeline/service or mTLS.
- **Header**: `Authorization: Bearer <token>` (required when `API_TOKEN` is set on the backend).

## Telemetry Ingestion

### POST /v1/telemetry/events
Receives signed events from the SDK.

**Request (example)**
```json
{
  "event_id": "evt_01HXYZ...",
  "app_id": "fintech.mobile",
  "app_version": "1.2.3",
  "env": "prod",
  "device": {
    "platform": "ios",
    "os_version": "17.3",
    "model": "iPhone15,3"
  },
  "session": {
    "session_id": "sess_01HXYZ...",
    "user_id_hash": "sha256:..."
  },
  "signals": {
    "jailbreak": false,
    "debugger": false,
    "hooking": false,
    "proxy_detected": false
  },
  "attestation": {
    "provider": "app_attest",
    "result": "pass",
    "timestamp": "2026-02-06T18:40:00Z"
  },
  "action": {
    "name": "transfer",
    "context": "pix"
  },
  "timestamp": "2026-02-06T18:40:02Z",
  "signature": "base64..."
}
```

**Response**
```json
{ "status": "ok", "stored_at": "2026-02-06T19:12:00Z" }
```

### GET /v1/policies/current
Distributes the current policy for app/version/environment.

**Request (query)**
- `app_id`
- `app_version`
- `env`
- `device_platform`

**Response (example)**
```json
{
  "policy_id": "pol_01HXYZ...",
  "app_id": "fintech.mobile",
  "app_version": "1.2.3",
  "env": "prod",
  "rules": [
    {
      "action": "login",
      "decision": "STEP_UP",
      "conditions": {
        "attestation": "pass",
        "debugger": false
      }
    },
    {
      "action": "transfer",
      "decision": "DENY",
      "conditions": {
        "app_version": "1.2.3"
      }
    }
  ],
  "signature": "base64...",
  "issued_at": "2026-02-06T18:41:00Z"
}
```

### GET /v1/policies
List stored policies. Optional filters by `app_id`, `app_version`, `env`, `device_platform`.

**Response (example)**
```json
[
  {
    "device_platform": "ios",
    "policy": {
      "policy_id": "pol_01HXYZ...",
      "app_id": "fintech.mobile",
      "app_version": "1.2.3",
      "env": "prod",
      "rules": [
        {
          "action": "login",
          "decision": "STEP_UP",
          "conditions": {
            "attestation": "pass",
            "debugger": false
          }
        }
      ],
      "signature": "base64...",
      "issued_at": "2026-02-06T18:41:00Z"
    }
  }
]
```

### GET /v1/policies/versions
List historical policy versions. Optional filters by `app_id`, `app_version`, `env`, `device_platform`.

**Response (example)**
```json
[
  {
    "device_platform": "ios",
    "stored_at": "2026-02-06T19:10:00Z",
    "policy": {
      "policy_id": "pol_01HXYZ...",
      "app_id": "fintech.mobile",
      "app_version": "1.2.3",
      "env": "prod",
      "rules": [
        {
          "action": "login",
          "decision": "STEP_UP",
          "conditions": {
            "attestation": "pass",
            "debugger": false
          }
        }
      ],
      "signature": "base64...",
      "issued_at": "2026-02-06T18:41:00Z"
    }
  }
]
```

### POST /v1/policies
Create or update a policy for app/version/environment.

**Request (example)**
```json
{
  "device_platform": "ios",
  "policy": {
    "policy_id": "pol_01HXYZ...",
    "app_id": "fintech.mobile",
    "app_version": "1.2.3",
    "env": "prod",
    "rules": [
      {
        "action": "login",
        "decision": "STEP_UP",
        "conditions": {
          "attestation": "pass",
          "debugger": false
        }
      }
    ],
    "signature": "base64...",
    "issued_at": "2026-02-06T18:41:00Z"
  }
}
```

**Response**
```json
{ "status": "ok" }
```

## Agent Report Upload

### POST /v1/reports/upload
Upload reports from Agent/CLI (JSON or SARIF).

**Request (example)**
```json
{
  "report_id": "rep_01HXYZ...",
  "app_id": "fintech.mobile",
  "env": "staging",
  "source": "ci",
  "pipeline": {
    "provider": "github_actions",
    "run_id": "123456"
  },
  "artifacts": {
    "format": "sarif",
    "payload": "base64..."
  },
  "timestamp": "2026-02-06T18:42:00Z"
}
```

**Response**
```json
{ "status": "accepted" }
```

## Data models (minimal)

### Policy Rule
```json
{
  "action": "transfer",
  "decision": "DEGRADE",
  "conditions": {
    "risk_score_gte": 70,
    "attestation": "fail"
  }
}
```

### Findings (aggregated)
```json
{
  "category": "backend_tls",
  "severity": "high",
  "evidence": {
    "endpoint": "https://api.example.com",
    "issue": "hsts_missing"
  }
}
```

## Next steps
- Evolve OpenAPI with stricter validations
- Define versioned and signed policies
- Define risk models and correlation

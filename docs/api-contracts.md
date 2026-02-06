# API Contracts (Draft)

Este documento fornece exemplos e orientacoes iniciais. A especificacao formal esta em:
- `docs/openapi.yaml`

Este documento descreve contratos iniciais entre SDK, Agent/CLI e Backend.

## Principios
- JSON over HTTPS
- Versionamento por caminho: `/v1/...`
- Idempotencia para eventos: `event_id`
- Assinatura de payload (SDK) e autenticacao forte (mTLS ou JWT)
- Compatibilidade forward: campos desconhecidos devem ser ignorados

## Autenticacao
- **SDK**: token por app + assinatura do payload (HMAC ou assinatura assimetrica do device).
- **Agent/CLI**: token por pipeline/servico ou mTLS.
- **Header**: `Authorization: Bearer <token>` (exigido quando `API_TOKEN` esta definido no backend).

## Telemetry Ingestion

### POST /v1/telemetry/events
Recebe eventos assinados do SDK.

**Request (exemplo)**
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
Distribui a politica atual para app/versao/ambiente.

**Request (query)**
- `app_id`
- `app_version`
- `env`
- `device_platform`

**Response (exemplo)**
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
Lista policies armazenadas. Filtros opcionais por `app_id`, `app_version`, `env`, `device_platform`.

**Response (exemplo)**
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
Lista versoes historicas de policies. Filtros opcionais por `app_id`, `app_version`, `env`, `device_platform`.

**Response (exemplo)**
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
Cria ou atualiza uma policy para app/versao/ambiente.

**Request (exemplo)**
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
Upload de relatorios do Agent/CLI (JSON ou SARIF).

**Request (exemplo)**
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

## Modelos de dados (minimos)

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

### Findings (agregado)
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

## Proximos passos
- Evoluir o OpenAPI com validacoes mais estritas
- Definir politicas versionadas e assinadas
- Definir modelos de risco e correlacao

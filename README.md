# Mobile AppSec Platform

**Tagline:** Continuous mobile security from commit to runtime.

Plataforma de seguranca mobile para fintechs que orquestra sinais e politicas desde o commit ate o runtime do app. Nao substitui scanners existentes; coordena, correlaciona e aplica politicas de seguranca continuas.

## Objetivos
- Proteger apps em runtime via SDK mobile (iOS + Android).
- Testar backend e CI/CD via Agent/CLI agnostico de linguagem.
- Correlacionar findings de CI/CD com telemetria runtime via Policy Backend.
- Aplicar politicas dinamicas (ALLOW, STEP_UP, DEGRADE, DENY).

## Componentes
- **Mobile SDK**
  - Nome do SDK: **PantherSecurity**
  - Core em Rust com wrappers Swift e Kotlin.
  - Pinning (SPKI com rotacao), secure storage, signals de integridade, attestation.
  - Engine local de politicas e enforcement de acoes sensiveis.
- **Security Agent / CLI (Rust)**
  - Scans backend e CI/CD, integra SAST/SCA/secret scanning.
  - Checks estaticos de build mobile.
  - Exporta JSON e SARIF e envia relatorios ao backend.
- **Policy + Telemetry Backend**
  - Ingestao de eventos e relatatorios.
  - Correlacao, risk score e distribuicao de politicas.

## Estrutura do monorepo
```
core/
  rust-core/
mobile/
  ios/
  android/
agent/
  cli/
  plugins/
backend/
  policy-service/
  telemetry-ingestion/
docs/
```

## Docs
- `docs/architecture.md`
- `docs/roadmap.md`
- `docs/mvp.md`

## Status
Esqueleto inicial do monorepo. Componentes ainda sem implementacao.

## Contratos de API (draft)
- `docs/api-contracts.md`

## OpenAPI
- `docs/openapi.yaml`

## Scripts
- `scripts/run-telemetry-ingestion.sh`
- `scripts/run-policy-service.sh`
- `scripts/run-backend.sh`

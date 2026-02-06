# Arquitetura de Alto Nivel

## Visao geral
A plataforma combina tres pilares:
1. **SDK Mobile** (iOS + Android) para protecao em runtime.
2. **Security Agent / CLI** para verificacoes de CI/CD e backend.
3. **Policy + Telemetry Backend** para correlacao, risk scoring e distribuicao de politicas.

O foco inicial e fintechs, priorizando acoes sensiveis (login, refresh token, transferencias, Pix, beneficiarios, dados de cartao e alteracao de credenciais).

## Componentes

### 1) Mobile SDK
**Core em Rust** com wrappers:
- Swift (iOS)
- Kotlin (Android)

Responsabilidades:
- Pinning TLS (SPKI com rotacao)
- Secure storage (Keychain / Keystore)
- Signals de integridade (jailbreak/root, debugger/hooking, proxy/MITM)
- Attestation (App Attest e Play Integrity)
- Engine local de politicas (em Rust)
- Enforcement por acao (ALLOW, STEP_UP, DEGRADE, DENY)
- Telemetria assinada

### 2) Security Agent / CLI (Rust)
Executa em CI/CD ou servidores.

Capacidades:
- Scanning de backend (TLS/HSTS/headers, CORS, rate limiting, authZ invariants, refresh token replay)
- OpenAPI/GraphQL checks
- Orquestracao de SAST e SCA/SBOM
- Secret scanning
- Checks estaticos de build mobile (entitlements, ATS, manifest, cleartext, debuggable)

Saidas:
- JSON
- SARIF
- Upload para Policy Backend

### 3) Policy + Telemetry Backend
Servicos principais:
- **Telemetry Ingestion**: recebe eventos do SDK e relatorios do Agent.
- **Policy Service**: calcula risk score, correlaciona sinais, distribui politicas.

Politicas sao segmentadas por:
- app
- versao
- ambiente

Exemplos:
- Bloquear transferencias em uma versao vulneravel
- Exigir STEP_UP para login
- Modo leitura para incident response

## Fluxos principais

### 1) Runtime
1. SDK coleta sinais locais e executa attestation.
2. SDK envia telemetria assinada para ingestao.
3. Policy Service correlaciona e calcula risk score.
4. Politicas retornam ao app e sao avaliadas localmente.
5. Engine de politicas decide ALLOW/STEP_UP/DEGRADE/DENY.

### 2) CI/CD e Backend
1. Agent/CLI executa testes e scans no pipeline.
2. Relatorios gerados (JSON/SARIF).
3. Upload para Policy Backend.
4. Correlacao com runtime para reforco dinamico de politicas.

## Interfaces e contratos
- **Rust Core <-> Mobile**: C ABI para wrappers Swift/Kotlin.
- **SDK <-> Backend**: API de telemetria e distribuicao de politicas.
- **Agent <-> Backend**: API de upload de relatorios.
- **Plugin system**: extensao de checks no Agent/CLI.

## Principios
- Clean Architecture
- Monorepo
- Core Rust com arquitetura hexagonal (ports/adapters)
- Separacao clara entre coleta, correlacao e enforcement
- Politicas assinadas e versionadas

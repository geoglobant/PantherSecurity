# Roadmap

Este roadmap foca em obter um MVP funcional com sinais de runtime, correlacao basica e enforcement de politicas.
Este documento sera atualizado a cada entrega relevante.

## Fase 0 - Fundacao (2 a 4 semanas)
- Estrutura do monorepo e padroes de arquitetura.
- Contratos iniciais entre SDK, Agent e Backend.
- OpenAPI inicial para telemetria, policy e upload de reports.
- Esqueleto do Rust core (C ABI, modelos de telemetria, policy engine base).
- Esqueleto do Agent/CLI com pipeline de plugins.
- Esqueleto do Backend (telemetry ingestion + policy service).

## Fase 1 - MVP (4 a 6 semanas)
- SDK: inicializacao/config, policy fetch, risk signals basicos, pinning.
- SDK: decision engine (ALLOW / STEP_UP / DENY).
- Backend: endpoint de ingestao de eventos e distribuicao de policy.
- Backend: storage simples.
- Agent CLI: `agent scan perimeter`, `agent scan rate-limit`, `agent scan authz`, `agent scan mobile-build`, `agent report`.

## Fase 2 - MVP CI/CD + Correlacao (4 a 6 semanas)
- Agent/CLI com checks de backend (TLS/HSTS/headers, CORS, rate limiting).
- Checks estaticos de build mobile (ATS/entitlements/manifest).
- Export JSON e SARIF.
- Correlacao basica entre findings do Agent e runtime.

## Fase 3 - Endurecimento (6 a 8 semanas)
- Attestation completa (App Attest / Play Integrity).
- Policy engine mais expressivo (regras condicionais).
- Risk score server-side com pesos por sinal.
- Kill switch e modo leitura para incident response.
- Observabilidade e trilhas de auditoria.

## Fora de escopo (por enquanto)
- Detecao avancada de fraude e ML.
- Remediacao automatica no pipeline.
- Marketplace publico de plugins.

## Metricas de sucesso (MVP)
- 90% das acoes sensiveis cobertas por enforcement.
- Tempo de propagacao de politica < 2 minutos.
- Alertas de runtime correlacionados com findings do CI/CD.

## Atualizacoes recentes
- 2026-02-06: DESTAQUE - Script para copiar xcframework no sample iOS em `scripts/install-ios-xcframework.sh`.
- 2026-02-06: DESTAQUE - Script para gerar xcframework iOS do core Rust em `scripts/build-ios-xcframework.sh`.
- 2026-02-06: DESTAQUE - SDK renomeado para PantherSecurity (Swift package e samples atualizados).
- 2026-02-06: DESTAQUE - Wrapper Swift via FFI para core Rust (policy + pinning).
- 2026-02-06: DESTAQUE - Projeto Android iniciado com arquitetura clean equivalente.
- 2026-02-06: DESTAQUE - Sample iOS com Xcode project gerado (Tuist) em `mobile/ios/sampleIOS`.
- 2026-02-06: DESTAQUE - Projeto iOS inicial com Clean Architecture e SDK integrado para testes.
- 2026-02-06: DESTAQUE - Logica de pinning SPKI com rotacao adicionada ao core SDK.
- 2026-02-06: DESTAQUE - SDK core com init/config, policy fetch e sinais baseline em `core/rust-core/src/sdk.rs`.
- 2026-02-06: DESTAQUE - Core HTTP adapter suporta `POST /v1/policies` (admin upsert).
- 2026-02-06: DESTAQUE - Policy upsert agora retorna `stored_at` no backend.
- 2026-02-06: DESTAQUE - Adapter HTTP no core para telemetria e policy fetch.
- 2026-02-06: DESTAQUE - Backend agora registra historico de policies e expõe `/v1/policies/versions`.
- 2026-02-06: DESTAQUE - Testes basicos adicionados ao policy-service.
- 2026-02-06: DESTAQUE - Listagem de policies via `GET /v1/policies`.
- 2026-02-06: DESTAQUE - SDK core agora suporta auth via `TelemetryAuth`/`TelemetryEnvelope`.
- 2026-02-06: DESTAQUE - Endpoint administrativo de policy adicionado em `/v1/policies`.
- 2026-02-06: DESTAQUE - Autenticacao basica via `Authorization: Bearer` nos servicos backend.
- 2026-02-06: DESTAQUE - Agent CLI agora aceita `--token` para `agent report`.
- 2026-02-06: DESTAQUE - Agent CLI agora envia `agent report` para `/v1/reports/upload`.
- 2026-02-06: DESTAQUE - Storage simples com SQLite para eventos, policies e reports.
- 2026-02-06: DESTAQUE - Scripts locais para subir backend em `scripts/run-backend.sh`.
- 2026-02-06: Esqueleto do monorepo criado.
- 2026-02-06: Documentos iniciais (arquitetura, contratos e roadmap) publicados.
- 2026-02-06: OpenAPI draft disponivel em `docs/openapi.yaml`.
- 2026-02-06: Esqueleto do Rust core (hexagonal) criado em `core/rust-core`.
- 2026-02-06: Stubs de C ABI/FFI adicionados ao core em `core/rust-core/src/adapters/ffi.rs`.
- 2026-02-06: Modelos de serializacao (DTOs) alinhados ao OpenAPI adicionados em `core/rust-core/src/adapters/serialization.rs`.
- 2026-02-06: FFI expandido para avaliar PolicySet e regras em lote no core.
- 2026-02-06: Serializacao strict com validacoes (deny unknown fields + validadores) no core.
- 2026-02-06: Esqueleto do Agent/CLI com pipeline de plugins em `agent/cli`.
- 2026-02-06: Documento de MVP publicado em `docs/mvp.md`.
- 2026-02-06: Comandos basicos do Agent CLI definidos (`scan` e `report`).
- 2026-02-06: Stubs de backend criados para ingestao de eventos e distribuicao de policy.

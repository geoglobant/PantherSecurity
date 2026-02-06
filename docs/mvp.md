# MVP Esperado (Fase 1)

Este documento descreve o MVP esperado para a primeira fase do produto.

## SDK
- [x] Inicializacao e configuracao (core)
- [x] Policy fetch (core)
- [x] Risk signals basicos (baseline)
- [x] Pinning (logica de SPKI com rotacao no core)
- [x] Decision engine (ALLOW / STEP_UP / DENY)
- [x] Projeto iOS inicial com Clean Architecture e SDK integrado (HTTP)
- [x] SDK PantherSecurity com wrapper Swift via FFI (policy + pinning)

## Android
- [x] Projeto Android inicial com Clean Architecture equivalente (stub)

## Agent CLI
Comandos esperados:
- [x] `agent scan perimeter` (stub)
- [x] `agent scan rate-limit` (stub)
- [x] `agent scan authz` (stub)
- [x] `agent scan mobile-build` (stub)
- [x] `agent report` (upload para backend)

## Backend
- [x] Endpoint de ingestao de eventos
- [x] Endpoint de distribuicao de policy
- [x] Storage simples (SQLite)

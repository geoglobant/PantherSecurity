# Security Examples (Swift + Backend)

This folder contains practical examples for the security gaps demonstrated in the sample apps.
Each example includes:
- A short explanation of the risk
- Swift-side implementation guidance
- Client-side test ideas
- Backend recommendations

## Topics
- `mitm-proxy-pinning.md` — TLS pinning and MITM detection
- `debugger-hooking.md` — Debugger and runtime hooking detection
- `device-integrity-jailbreak-root.md` — Jailbreak/root signals and heuristics
- `attestation.md` — App Attest (iOS) validation flow
- `bola-idor.md` — BOLA/IDOR prevention on sensitive endpoints
- `risk-score.md` — Server risk scoring and policy thresholds

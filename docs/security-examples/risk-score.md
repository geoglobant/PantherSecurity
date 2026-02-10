# Risk Score (Server-side)

## Risk summary
Risk score aggregates signals and behavior to drive policy decisions.

## Example scoring (simple)
```text
score = 0
if debugger == true -> +30
if hooking == true -> +30
if proxy_detected == true -> +25
if attestation == fail -> +40
if jailbreak/root == true -> +20
score = min(score, 100)
```

## Client-side test ideas
- Toggle signals in the simulator and confirm outcomes change.
- Verify that high risk forces STEP_UP or DENY for sensitive actions.

## Backend recommendations
- Keep scoring in the backend (server is source of truth).
- Return the score with policy decisions for observability.
- Tune thresholds per action (transfer > view card).

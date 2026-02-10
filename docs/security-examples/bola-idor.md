# BOLA / IDOR (Broken Object Level Authorization)

## Risk summary
Attackers access or modify resources they don’t own by guessing IDs.

## Swift example (client-side safety)
Clients should not trust local checks alone, but you can reduce risk by:
- Avoiding sequential IDs in URLs
- Not exposing internal identifiers unless necessary

```swift
struct TransferRequest: Codable {
    let beneficiaryId: String
    let amount: Decimal
    let idempotencyKey: String
}
```

## Client-side test ideas
- Attempt to submit a transfer with a beneficiary ID from another user.
- Verify the server rejects it (403/404).

## Backend recommendations (required)
- Enforce ownership checks on every request.
- Use access control per resource, not only per endpoint.
- Log and alert on repeated authorization failures.

Example pseudocode:
```text
if !beneficiary.belongs_to(user) { return 403 }
```

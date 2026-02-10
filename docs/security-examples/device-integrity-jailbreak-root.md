# Device Integrity (Jailbreak / Root)

## Risk summary
Jailbroken or rooted devices can bypass OS protections and tamper with app behavior.

## Swift example (heuristics)
```swift
import Foundation

func isJailbroken() -> Bool {
    #if targetEnvironment(simulator)
    return false
    #else
    let suspiciousPaths = [
        "/Applications/Cydia.app",
        "/Library/MobileSubstrate/MobileSubstrate.dylib",
        "/bin/bash",
        "/usr/sbin/sshd",
        "/etc/apt"
    ]
    if suspiciousPaths.contains(where: { FileManager.default.fileExists(atPath: $0) }) {
        return true
    }

    // Test writing outside sandbox
    let testPath = "/private/jailbreak_test.txt"
    do {
        try "test".write(toFile: testPath, atomically: true, encoding: .utf8)
        try FileManager.default.removeItem(atPath: testPath)
        return true
    } catch {
        return false
    }
    #endif
}
```

## Recommended Apple frameworks
- `Foundation` (filesystem checks)

## Client-side test ideas
- Run on a jailbroken device and verify the signal flips.
- Validate in release builds (debug checks can be bypassed).

## Backend recommendations
- Use jailbreak/root as a risk signal, not a single point of failure.
- Combine with attestation and behavior signals before hard blocking.

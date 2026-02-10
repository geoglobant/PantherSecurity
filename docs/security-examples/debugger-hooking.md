# Debugger / Runtime Hooking

## Risk summary
Debuggers and runtime hooks can bypass client logic or exfiltrate secrets.

## Swift example (basic debugger detection)
```swift
import Darwin

func isDebuggerAttached() -> Bool {
    var info = kinfo_proc()
    var size = MemoryLayout<kinfo_proc>.size
    var mib = [CTL_KERN, KERN_PROC, KERN_PROC_PID, getpid()]
    let result = sysctl(&mib, u_int(mib.count), &info, &size, nil, 0)
    if result != 0 { return false }
    return (info.kp_proc.p_flag & P_TRACED) != 0
}
```

## Swift example (basic hooking heuristics)
```swift
import Foundation

func hasSuspiciousLibraries() -> Bool {
    let suspicious = ["frida", "substrate", "cycript"]
    for i in 0..<_dyld_image_count() {
        if let name = _dyld_get_image_name(i) {
            let image = String(cString: name).lowercased()
            if suspicious.contains(where: { image.contains($0) }) {
                return true
            }
        }
    }
    return false
}
```

## Recommended Apple frameworks
- `Darwin` (sysctl, process info)
- `mach-o/dyld` (loaded images)

## Client-side test ideas
- Attach LLDB to the app and verify detection.
- Use Frida on a dev device and confirm the signal flips.

## Backend recommendations
- Treat debugger/hooking as high-risk signals.
- Require step-up or deny high-impact actions (transfer, beneficiary add, password change).

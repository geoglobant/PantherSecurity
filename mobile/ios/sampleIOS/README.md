# iOS Sample (Xcode)

This sample project uses the PantherSecurity SDK via Swift Package Manager and provides a simple UI to test:
- Fetch Policy
- Send Login Telemetry

## How to open in Xcode

### Option A (recommended): generate the Xcode Project via Tuist
1. Generate the Xcode project:
   ```bash
   cd mobile/ios/sampleIOS
   tuist generate
   ```
2. Open `mobile/ios/sampleIOS/MobileAppSecSample.xcodeproj`.

### Option B: open Package.swift directly
1. Open `mobile/ios/sampleIOS/Package.swift` in Xcode.

## How the SDK is added to the project
The project uses Swift Package Manager pointing to the local SDK package:
- `mobile/ios/Package.swift`

In `Project.swift` (Tuist), the local package is referenced like this:
```swift
.package(path: "..")
```
And the target depends on the `PantherSecurity` product:
```swift
.package(product: "PantherSecurity")
```

## Xcode flow to load the SDK via Package
1. Open `MobileAppSecSample.xcodeproj`.
2. In Xcode, go to **File > Add Packages...**.
3. Click **Add Local...** and select `mobile/ios`.
4. Select the `PantherSecurity` product and confirm.
5. The `MobileAppSecSample` target should list `PantherSecurity` in **Frameworks, Libraries, and Embedded Content**.

## FFI: how to link the Rust core in Xcode
The Swift wrapper calls C functions from the Rust core (FFI). To work at runtime, you must
build the Rust core and link the library in Xcode.

Suggested flow:
1. Build the Rust core for iOS device and simulator:
   ```bash
   scripts/install-ios-xcframework.sh
   ```
2. The xcframework will be copied to `mobile/ios/sampleIOS/Frameworks/PantherSecurityCore.xcframework`.
3. In Xcode, add the xcframework in **Frameworks, Libraries, and Embedded Content**.
4. Make sure the header `core/rust-core/include/panther_security.h` is available in the project.

## Tips
- After running `scripts/install-ios-xcframework.sh`, run `tuist generate` again.
- If Xcode cannot see the xcframework, clean DerivedData and reopen the project.
- Keep the xcframework inside `mobile/ios/sampleIOS/Frameworks` so Tuist links it automatically.
- If you see `Undefined symbol: _ps_*`, rebuild the xcframework after updating the Rust core.

## Running the sample
1. Start the local backend:
   ```bash
   scripts/run-backend.sh
   ```
2. Run the app in the iOS simulator.
3. Use the buttons to trigger policy and telemetry calls.

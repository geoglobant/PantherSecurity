// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "MobileAppSecIOS",
    platforms: [.iOS(.v16)],
    products: [
        .library(name: "AppDomain", targets: ["AppDomain"]),
        .library(name: "AppData", targets: ["AppData"]),
        .library(name: "AppPresentation", targets: ["AppPresentation"]),
        .library(name: "PantherSecurity", targets: ["PantherSecurity"]),
        .executable(name: "App", targets: ["App"])
    ],
    dependencies: [],
    targets: [
        .target(
            name: "AppDomain",
            path: "Sources/Domain"
        ),
        .target(
            name: "PantherSecurity",
            path: "Sources/PantherSecurity"
        ),
        .target(
            name: "AppData",
            dependencies: ["AppDomain", "PantherSecurity"],
            path: "Sources/Data"
        ),
        .target(
            name: "AppPresentation",
            dependencies: ["AppDomain", "AppData"],
            path: "Sources/Presentation"
        ),
        .executableTarget(
            name: "App",
            dependencies: ["AppPresentation"],
            path: "Sources/App"
        ),
        .testTarget(
            name: "AppTests",
            dependencies: ["AppDomain", "AppData", "AppPresentation", "PantherSecurity"],
            path: "Tests"
        )
    ]
)

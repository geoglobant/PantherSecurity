import ProjectDescription

let project = Project(
    name: "MobileAppSecSample",
    packages: [
        .package(path: "..")
    ],
    targets: [
        .target(
            name: "MobileAppSecSample",
            destinations: .iOS,
            product: .app,
            bundleId: "com.panther.mobileappsecsample",
            deploymentTargets: .iOS("16.0"),
            infoPlist: .default,
            sources: ["Sources/SampleApp/**"],
            resources: [],
            dependencies: [
                .package(product: "PantherSecurity"),
                .xcframework(path: "Frameworks/PantherSecurityCore.xcframework")
            ]
        )
    ]
)

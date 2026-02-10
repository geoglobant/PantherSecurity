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
        ),
        .target(
            name: "MobileAppSecSampleTests",
            destinations: .iOS,
            product: .unitTests,
            bundleId: "com.panther.mobileappsecsample.tests",
            deploymentTargets: .iOS("16.0"),
            infoPlist: .default,
            sources: ["Tests/**"],
            resources: [],
            dependencies: [
                .package(product: "PantherSecurity"),
                .xcframework(path: "Frameworks/PantherSecurityCore.xcframework")
            ]
        )
    ]
)

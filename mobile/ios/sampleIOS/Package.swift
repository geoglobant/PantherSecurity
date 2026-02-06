// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "MobileAppSecSample",
    platforms: [.iOS(.v16)],
    products: [
        .executable(name: "MobileAppSecSample", targets: ["SampleApp"])
    ],
    dependencies: [
        .package(path: "..")
    ],
    targets: [
        .executableTarget(
            name: "SampleApp",
            dependencies: [
                .product(name: "PantherSecurity", package: "MobileAppSecIOS")
            ],
            path: "Sources/SampleApp"
        ),
        .testTarget(
            name: "SampleAppTests",
            dependencies: ["SampleApp"],
            path: "Tests"
        )
    ]
)

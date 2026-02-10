# MITM / Proxy Interception (TLS Pinning)

## Risk summary
Traffic interception can expose credentials and modify requests in transit.

## Swift example (SPKI pinning)
Use `URLSession` + `Security` to validate the server certificate SPKI hash.

```swift
final class PinningDelegate: NSObject, URLSessionDelegate {
    private let allowedSpkiHashes: Set<String>

    init(allowedSpkiHashes: [String]) {
        self.allowedSpkiHashes = Set(allowedSpkiHashes)
    }

    func urlSession(
        _ session: URLSession,
        didReceive challenge: URLAuthenticationChallenge,
        completionHandler: @escaping (URLSession.AuthChallengeDisposition, URLCredential?) -> Void
    ) {
        guard challenge.protectionSpace.authenticationMethod == NSURLAuthenticationMethodServerTrust,
              let trust = challenge.protectionSpace.serverTrust,
              let serverCert = SecTrustGetCertificateAtIndex(trust, 0) else {
            completionHandler(.cancelAuthenticationChallenge, nil)
            return
        }

        guard let spkiHash = serverSpkiSha256(cert: serverCert),
              allowedSpkiHashes.contains(spkiHash) else {
            completionHandler(.cancelAuthenticationChallenge, nil)
            return
        }

        completionHandler(.useCredential, URLCredential(trust: trust))
    }
}

private func serverSpkiSha256(cert: SecCertificate) -> String? {
    guard let key = SecCertificateCopyKey(cert),
          let data = SecKeyCopyExternalRepresentation(key, nil) as Data? else {
        return nil
    }
    // Use CryptoKit or CommonCrypto to hash the public key bytes
    return sha256Base64(data: data)
}

private func sha256Base64(data: Data) -> String {
    // Replace with a real SHA-256 + Base64 implementation
    return data.base64EncodedString()
}
```

## Recommended Apple frameworks
- `Security` (certificate handling)
- `Network` (optional: network path inspection)
- `CryptoKit` (SHA-256)

## Client-side test ideas
- Use a proxy (Charles / Proxyman / mitmproxy) and confirm the request is blocked.
- Install a custom root CA and ensure the app still rejects the connection.

## Backend recommendations
- Enforce TLS 1.2+ and strong cipher suites.
- Use HSTS and rotate certificates with overlap (current + previous pins).
- Treat proxy detection as a signal to require step-up for sensitive actions.

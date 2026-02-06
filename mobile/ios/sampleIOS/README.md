# iOS Sample (Xcode)

Este projeto de exemplo usa o SDK PantherSecurity via Swift Package Manager e oferece uma UI simples para testar:
- Fetch Policy
- Send Login Telemetry

## Como abrir no Xcode

### Opcao A (recomendada): gerar o Xcode Project via Tuist
1. Gere o projeto Xcode:
   ```bash
   cd mobile/ios/sampleIOS
   tuist generate
   ```
2. Abra `mobile/ios/sampleIOS/MobileAppSecSample.xcodeproj`.

### Opcao B: abrir o Package.swift diretamente
1. Abra `mobile/ios/sampleIOS/Package.swift` no Xcode.

## Como o SDK entra no projeto
O projeto usa Swift Package Manager apontando para o package local do SDK:
- `mobile/ios/Package.swift`

No `Project.swift` (Tuist), o package local e referenciado assim:
```swift
.package(path: "../../mobile/ios")
```
E o target depende do produto `PantherSecurity`:
```swift
.package(product: "PantherSecurity")
```

## Fluxo no Xcode para carregar o SDK via Package
1. Abra `MobileAppSecSample.xcodeproj`.
2. No Xcode, va em **File > Add Packages...**.
3. Clique em **Add Local...** e selecione `mobile/ios`.
4. Selecione o produto `PantherSecurity` e confirme.
5. O target `MobileAppSecSample` deve listar `PantherSecurity` em **Frameworks, Libraries, and Embedded Content**.

## FFI: como ligar o core Rust no Xcode
O wrapper Swift chama funcoes C do core Rust (FFI). Para funcionar em runtime, e preciso
compilar o Rust core e linkar a biblioteca no Xcode.

Fluxo sugerido:
1. Compile o core Rust para iOS device e simulator:
   ```bash
   scripts/install-ios-xcframework.sh
   ```
2. A xcframework sera copiada para `mobile/ios/sampleIOS/Frameworks/PantherSecurityCore.xcframework`.
3. No Xcode, adicione a xcframework em **Frameworks, Libraries, and Embedded Content**.
4. Garanta que o header `core/rust-core/include/panther_security.h` esteja disponivel no projeto.

## Dicas
- Depois de rodar `scripts/install-ios-xcframework.sh`, rode `tuist generate` novamente.
- Se o Xcode nao enxergar a xcframework, limpe o DerivedData e reabra o projeto.
- Mantenha a xcframework dentro de `mobile/ios/sampleIOS/Frameworks` para o Tuist linkar automaticamente.
- Se aparecer `Undefined symbol: _ps_*`, recrie a xcframework apos o update do Rust core.

## Rodando o sample
1. Suba o backend local:
   ```bash
   scripts/run-backend.sh
   ```
2. Rode o app no simulador iOS.
3. Use os botoes para disparar chamadas de policy e telemetria.

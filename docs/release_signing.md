# Release signing and artifact verification

This document shows recommended steps and examples to produce signed Windows executables and verifiable release artifacts.

## 1) Authenticode (Windows) — local / CI with SignTool

If you have a PFX code-signing certificate (EV recommended), sign locally with Microsoft SignTool:

```powershell
signtool sign /tr http://timestamp.digicert.com /td SHA256 /fd SHA256 /f cert.pfx /p "PFX_PASSWORD" stamp_album_gen2.exe
```

In CI (GitHub Actions on `windows-latest`) store the PFX as a base64 secret `SIGNING_PFX` and password in `SIGNING_PFX_PASSWORD`. The provided workflow decodes and signs using `signtool`.

Alternative on Linux using `osslsigncode`:

```bash
osslsigncode sign -pkcs12 cert.pfx -h sha256 -t http://timestamp.digicert.com -in stamp_album_gen2.exe -out stamp_album_gen2-signed.exe
```

Yes, you can create a PFX code-signing certificate using a .cer file, but you will also need the corresponding private key. Both the certificate and the private key must be combined into a single PFX file using a tool like OpenSSL.

Creating a PFX Code-Signing Certificate
Yes, you can create a PFX code-signing certificate using a .cer file, but there are specific requirements to consider.

Requirements
To successfully create a PFX file, you need:

.cer File: This is your code-signing certificate.
Private Key: The corresponding private key must be available. The private key is essential as it is used to sign the code.
Steps to Create a PFX File
You can use OpenSSL to combine the .cer file and the private key into a PFX file. Here’s how:

Install OpenSSL: Ensure OpenSSL is installed on your system.
Use the Command: Run the following command in your terminal or command prompt:

```sh
openssl pkcs12 -export -out certificate.pfx -inkey privateKey.key -in certificate.cer
```

openssl pkcs12 -export -out certificate.pfx -inkey privateKey.key -in certificate.cer
-export: Indicates that you want to create a PFX file.
-out certificate.pfx: Specifies the name of the output PFX file.
-inkey privateKey.key: Points to your private key file.
-in certificate.cer: Points to your .cer certificate file.
Set a Password: During the process, you will be prompted to set a password for the PFX file. This password is crucial for protecting the private key.
Important Notes
Ensure that the private key is exportable if it was generated on a secure device.
If your certificate was issued after June 2023, it may be stored in a secure hardware module, making it non-exportable.
By following these steps, you can successfully create a PFX code-signing certificate from a .cer file and its corresponding private key.



## 2) Produce checksum and GPG detached signature

```bash
sha256sum stamp_album_gen2.exe > stamp_album_gen2.exe.sha256
gpg --armor --detach-sign stamp_album_gen2.exe
```

## 3) Sigstore / Cosign (optional, cross-platform)

Generate a cosign key pair locally:

```bash
cosign generate-key-pair
# produces cosign.key and cosign.pub
```

Sign artifact:

```bash
cosign sign-blob --key cosign.key stamp_album_gen2.exe
```

Verify:

```bash
cosign verify-blob --key cosign.pub --signature stamp_album_gen2.exe.sig stamp_album_gen2.exe
```

## 4) Release packaging suggestions

- Attach the signed EXE, the `.sha256`, the `.asc` (GPG signature) and cosign outputs to your GitHub Release.
- Publish instructions for users to verify the signature and checksums before running the binary.

## 5) False-positive handling

- Upload to VirusTotal to see which engines flag the binary.
- Submit the signed artifact and VirusTotal report to vendors (Microsoft Defender portal, etc.) requesting reclassification.

## 6) Short user guidance (safety first)

- Ask users to restore from quarantine only if they downloaded the release from your official release page and checksums/signatures verify.
- Do NOT instruct users to disable AV/SmartScreen globally.

If you want, I can:
- Add a unit test that asserts `STAMP_FLAGS_DIR` lookup behavior.
- Tweak the GitHub Actions workflow to build Linux/macOS artifacts too and sign them.

### CI Secrets required by the workflow

- `SIGNING_PFX` — base64-encoded PKCS#12 (PFX) file for Authenticode signing (optional).
- `SIGNING_PFX_PASSWORD` — password for the PFX file.
- `GPG_PRIVATE_KEY` — base64-encoded export of your GPG private key (for creating detached `.asc` signatures).
- `COSIGN_KEY` — base64-encoded cosign private key (optional) for sigstore signing.
- `GITHUB_TOKEN` — used by the release action (automatically provided in GitHub Actions runners).

Notes about macOS signing/notarization

- Apple code signing and notarization require an Apple Developer ID certificate and additional steps (not covered by this workflow). The workflow will produce macOS binaries and will create checksums/signatures (GPG/cosign), but if you need Gatekeeper-trusted notarized DMG/PKG artifacts we should add an additional macOS signing & notarization job which requires an Apple Developer account and secure handling of the signing identity.

If you want, I can add the macOS notarization steps or integrate an OIDC-based signing provider for tightened secret handling.

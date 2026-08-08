External signing architecture

Overview

This document describes a simple, secure external-signing architecture and provides an example signing agent script you can run on a dedicated signing host (recommended production approach).

Goals

- Remove private signing keys from ephemeral hosted runners
- Provide an auditable, repeatable signing flow
- Integrate with CI without exposing private keys

High-level flow

1. CI builds artifacts and uploads them to an authenticated artifact store (GitHub Artifacts, S3, internal HTTPS server).
2. CI emits a signing request (HTTP webhook to signing agent, or creates a queue message / GitHub repository_dispatch event) containing:
   - artifact location (URL with short-lived token or pre-signed URL)
   - artifact metadata (file name, platform, release tag, checksum)
   - optional callback (where to upload signature or which GitHub release to update)
3. Signing agent (running on a hardened Windows host for Authenticode signing) receives request, fetches artifact using the provided ephemeral credentials, and:
   - verifies checksum
   - signs artifact with private key stored in an HSM or local protected store
   - uploads detached signature and/or signed artifact to the target location
   - writes audit record (timestamp, signer identity, signature fingerprint, request id)
4. CI (or release automation) collects signatures and publishes release assets.

Security considerations

- Keep the signing host isolated, restricted network access, and limited user accounts.
- Store private keys in an HSM or protected directory with strict ACLs. If using a file-based PFX, protect with a strong passphrase and restrict access.
- Use short-lived artifact URLs for download by the signing agent.
- Ensure signing requests are authenticated and authorized (e.g., HMAC-signed webhook, mutually authenticated TLS, or repository_dispatch with a secret token).
- Keep an append-only audit log of signing activity and signature fingerprints.

Example CI: produce artifact + create signing request

- Build artifacts in CI as usual.
- Upload artifacts to a secure bucket or use GitHub Actions `upload-artifact` and then `download-artifact` from the signing host (preferred: upload to S3 or GitHub release and provide a short-lived token).
- POST a JSON payload to the signing agent webhook or use `curl` to call a small service that will enqueue the request.

Payload example (JSON):
{
  "repo": "owner/repo",
  "artifact_url": "<https://my-storage-host/artifacts/..../stamp_album_gen2-windows.exe?X-Amz-Signature=>...",
  "artifact_name": "stamp_album_gen2-windows.exe",
  "artifact_sha256": "...",
  "release_tag": "v1.2.3",
  "callback": "<https://api.github.com/repos/owner/repo/releases/123/assets>",
  "request_id": "uuid"
}

Signing agent (high level)

- Should run on Windows (for Authenticode / SignTool) so `signtool.exe` is available.
- Validate the request origin and payload.
- Download the artifact over HTTPS using ephemeral credentials.
- Verify SHA256 checksum.
- Import the PFX (or use HSM) and call `signtool.exe sign`.
- Upload signature file and optionally the signed artifact to the callback/upload location.
- Post back a status to CI or update a database/issue.

Audit

- Each signing operation should write: timestamp, artifact name, artifact checksum, signer id (system account), signature fingerprint, request id, success/failure, and a link to where signatures were published.

Operational checklist

- Provision a Windows host with required storage and `signtool.exe` (Windows SDK) installed.
- Harden the host and limit SSH/RDP access.
- Store PFX in an HSM or protected path (C:\keys\cert.pfx) with strict ACL.
- Create a service account for signing agent and rotate credentials regularly.
- Configure signed release artifact storage location and retention policy.

Next steps

- Review the example signing agent script in `tools/signing_agent/` and adapt it to your environment.
- Decide on webhook vs. queue for sending signing requests.
- Implement secret management for the signing host (use local vault or cloud KMS/HSM).

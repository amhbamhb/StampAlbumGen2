# Stamp Album Generator 2 Website

This is a separate Rust project for the public website at `stampalbumgenerator2.com`.

Current scope:

- Friendly landing site for stamp collectors
- App release focus on Linux and Windows binaries only
- Source code links intentionally not live yet
- Placeholder section for future GitHub integration

## Run locally

```bash
cargo run -p stampalbumgenerator2-site
```

Open:

- <http://localhost:3000>
- <http://localhost:3000/healthz>
- <http://localhost:3000/release/linux>
- <http://localhost:3000/release/windows>

Optional port override:

```bash
PORT=8080 cargo run -p stampalbumgenerator2-site
```

## Real-time local edits (instant rebuild/restart)

One command from workspace root:

```bash
./stampalbumgenerator2-site/scripts/dev.sh
```

What this does:

- Watches Rust files in `src/` and web files in `static/`.
- Automatically rebuilds/restarts the server on save.
- Automatically refreshes the browser after restart (localhost only).

Notes:

- The script auto-installs `cargo-watch` if it is missing.
- Default local URL is <http://localhost:4205>.
- If an older site process is still using the same port, the script stops it automatically.
- To use a different port: `PORT=4300 ./stampalbumgenerator2-site/scripts/dev.sh`

## Version number in all pages

The public app version is sourced from one file:

- `stampalbumgenerator2-site/APP_VERSION`

Update that file once (for example from `1.6.51` to a new release), and the version is shown automatically in:

- Homepage footer
- Linux build card
- Windows build card
- Server-rendered `/release/linux` and `/release/windows` pages

## Domain setup plan for stampalbumgenerator2.com

1. Deploy this service to a Linux host or container platform.
2. Point DNS `A` record (`@`) to your server IP.
3. Point DNS `CNAME` record (`www`) to `stampalbumgenerator2.com`.
4. Put a reverse proxy in front (Nginx, Caddy, Traefik) for TLS.
5. Obtain TLS cert via Let\'s Encrypt.
6. Route both `stampalbumgenerator2.com` and `www.stampalbumgenerator2.com` to this service.

## nginx reverse proxy (no Docker)

Example nginx site config:

```nginx
server {
  listen 80;
  server_name stampalbumgenerator2.com www.stampalbumgenerator2.com;

  location / {
    proxy_pass http://127.0.0.1:3000;
    proxy_http_version 1.1;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
  }
}
```

After DNS is ready, enable TLS with Certbot and redirect HTTP to HTTPS.

## Security baseline (always top priority)

Application defaults already enabled in [stampalbumgenerator2-site/src/main.rs](stampalbumgenerator2-site/src/main.rs):

- Content Security Policy (CSP)
- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- strict referrer policy
- permissions policy to disable unused browser features
- request body size limit for DoS reduction

Cloudflare settings to keep enabled for production:

1. SSL mode: `Full (strict)`.
2. Always Use HTTPS: enabled.
3. HSTS: enabled (`max-age >= 31536000`, include subdomains, preload only when ready).
4. WAF managed rules: enabled.
5. Bot Fight Mode (or Super Bot Fight): enabled.
6. Rate limiting on dynamic endpoints (at minimum `/release/*`).
7. Security Level: `Medium` or higher.
8. Disable direct origin access by firewalling origin to Cloudflare IP ranges only.

nginx hardening additions for production:

1. Add `server_tokens off;`.
2. Add HTTPS redirect server block (80 -> 443).
3. Add TLS 1.2/1.3 only in the 443 block.
4. Send `Strict-Transport-Security` header at the edge.
5. Keep origin port bound to localhost/private interface only.

## Future GitHub wiring (not active yet)

When your repository is ready, update the placeholder in `static/index.html`.

Suggested future links:

- Release notes page
- Public binary downloads
- Documentation pages
- Source repository (when you decide to publish)

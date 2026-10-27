# Cross-Platform Build Guide

## Status & Recommendations

Your Rust project `stamp_album_gen2` can be built for multiple platforms. Here's what works and what needs additional setup:

### ✅ READY TO BUILD NOW

#### 1. Linux GNU (x86_64-unknown-linux-gnu) - YOUR CURRENT SETUP

```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

**Output:** `target/x86_64-unknown-linux-gnu/release/stamp_album_gen2`
**Use case:** Works on standard Linux distros with compatible glibc
**Size:** ~30-40 MB

#### 2. Linux ARM64 (aarch64-unknown-linux-gnu)

```bash
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

**Output:** `target/aarch64-unknown-linux-gnu/release/stamp_album_gen2`
**Use case:** Raspberry Pi 4/5, ARM64 servers
**Size:** ~30-40 MB

### ⚠️ NEEDS SPECIAL SETUP

#### 3. Linux MUSL (x86_64-unknown-linux-musl) - STATIC BINARY

**Challenge:** Your project uses `reqwest` which depends on `openssl-sys`. For MUSL cross-compilation, you need MUSL-specific OpenSSL development files, which are not packaged in standard Ubuntu.

**Solution Options:**

## **Option A: Use Docker/cargo-cross (RECOMMENDED)**

```bash
# Install cross tool
cargo install cross

# Build with cross (handles all dependencies automatically)
cross build --release --target x86_64-unknown-linux-musl
```

*Note: Failed to install due to CUDA conflicts in your environment. Can use without installing by running Docker directly.*

**Option B: Manual compilation (requires more setup)**
You would need to:

1. Build OpenSSL from source for MUSL
2. Point build variables to the compiled OpenSSL
3. Set numerous environment variables

## **Option C: Use Rust's built-in static linking (simplest)**

```bash
OPENSSL_STATIC=1 cargo build --release --target x86_64-unknown-linux-musl
```

## *This requires openssl-sys to find precompiled static OpenSSL libs*

**Action for MUSL:** We recommend documenting this as a GitHub Actions CI step (see later section).

### ⚠️ Cross-Compiling for Windows MSVC (`x86_64-pc-windows-msvc`)

Building for `x86_64-pc-windows-msvc` on Linux is **not supported directly** because it requires the Microsoft Visual C++ (MSVC) toolchain (e.g., `lib.exe`, `link.exe`), which is not available on Linux.

If you see errors like `failed to find tool "lib.exe": No such file or directory`, follow these approaches:

#### Recommended Approach: Use `cross`
Use the [`cross`](https://github.com/cross-rs/cross) tool, which uses Docker containers pre-configured with the correct cross-compilation environment for MSVC.

1. Install `cross`:
   ```bash
   cargo install cross
   ```
2. Build your target:
   ```bash
   cross build --target x86_64-pc-windows-msvc --release
   ```

#### Alternative Approach: Use `gnu` Target
If you do not strictly require MSVC, you can use the `x86_64-pc-windows-gnu` target, which relies on the MinGW-w64 toolchain.

1. Add the target:
   ```bash
   rustup target add x86_64-pc-windows-gnu
   ```
2. Install MinGW-w64 on your Linux system (e.g., `sudo apt install mingw-w64` on Ubuntu).
3. Build for the target:
   ```bash
   cargo build --target x86_64-pc-windows-gnu --release
   ```


### 📌 REQUIRES ANOTHER MACHINE

#### 4. macOS x86_64 (Intel Mac)

- **Must build ON macOS**
- Command: `cargo build --release --target x86_64-apple-darwin`
- Alternative: Use GitHub Actions with macOS runner

#### 5. macOS ARM64 (Apple Silicon M1/M2/M3)

- **Can build on Intel Mac:** `cargo build --release --target aarch64-apple-darwin`
- **Can build on Apple Silicon (native):** Similar command
- Alternative: Use GitHub Actions with macOS runner

#### 6. macOS Universal (Intel + ARM64 in one binary)

- **Requires macOS machine**
- Build both targets, then use `lipo` to combine

#### 7. Windows MSVC (x86_64-pc-windows-msvc)

- **Should build ON Windows**
- Command: `cargo build --release --target x86_64-pc-windows-msvc`
- Alternative: Use GitHub Actions with Windows runner

#### 8. Windows GNU (x86_64-pc-windows-gnu) - Cross-compile from Linux

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

**Requires:** MinGW toolchain (`mingw-w64` package)
**May need:** Additional configuration

---

## Quick Start: Build What You Can Today

### Step 1: Linux GNU (Native)

```bash
cd /media/ray/program/Rust/StampAlbumGen2
cargo build --release --target x86_64-unknown-linux-gnu
ls -lh target/x86_64-unknown-linux-gnu/release/stamp_album_gen2
```

### Step 2: Linux ARM64 (if you have ARM hardware or want to prepare)

```bash
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

### Step 3: Windows GNU (from Linux)

```bash
rustup target add x86_64-pc-windows-gnu
# May need: sudo apt-get install mingw-w64
cargo build --release --target x86_64-pc-windows-gnu
```

---

## Using GitHub Actions for CI/CD Builds

For MUSL, macOS, and Windows builds, the recommended approach is **GitHub Actions** which provides free runners for all platforms.

Example `.github/workflows/cross-compile.yml`:

```yaml
name: Cross-Platform Release Build

on:
  push:
    tags:
      - 'v*'

jobs:
  build-linux-gnu:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: rustup target add x86_64-unknown-linux-gnu
      - run: cargo build --release --target x86_64-unknown-linux-gnu
      - uses: actions/upload-artifact@v3
        with:
          name: stamp_album_gen2-linux-gnu
          path: target/x86_64-unknown-linux-gnu/release/stamp_album_gen2

  build-linux-musl:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: rustup target add x86_64-unknown-linux-musl
      - run: sudo apt-get update && sudo apt-get install -y musl-tools
      - run: |
          OPENSSL_STATIC=1 cargo build --release --target x86_64-unknown-linux-musl
      - uses: actions/upload-artifact@v3
        with:
          name: stamp_album_gen2-linux-musl
          path: target/x86_64-unknown-linux-musl/release/stamp_album_gen2

  build-macos-intel:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: rustup target add x86_64-apple-darwin
      - run: cargo build --release --target x86_64-apple-darwin
      - uses: actions/upload-artifact@v3
        with:
          name: stamp_album_gen2-macos-intel
          path: target/x86_64-apple-darwin/release/stamp_album_gen2

  build-macos-arm64:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: rustup target add aarch64-apple-darwin
      - run: cargo build --release --target aarch64-apple-darwin
      - uses: actions/upload-artifact@v3
        with:
          name: stamp_album_gen2-macos-arm64
          path: target/aarch64-apple-darwin/release/stamp_album_gen2

  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --release --target x86_64-pc-windows-msvc
      - uses: actions/upload-artifact@v3
        with:
          name: stamp_album_gen2-windows-msvc
          path: target/x86_64-pc-windows-msvc/release/stamp_album_gen2.exe
```

---

## Build Script (Local)

Use this script to build all available targets locally:

```bash
#!/bin/bash
# build-all-targets.sh - Build all available cross-compilation targets

set -e

VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/version = "\([^"]*\)".*/\1/')
RELEASE_DIR="releases/$VERSION"

mkdir -p "$RELEASE_DIR"

echo "🔨 Building stamp_album_gen2 v$VERSION"
echo ""

# Linux GNU (native)
echo "📦 Building Linux GNU (x86_64)..."
cargo build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/stamp_album_gen2 \
   "$RELEASE_DIR/stamp_album_gen2_linux_x86_64"

# Linux ARM64 (if available)
echo "📦 Building Linux ARM64 (aarch64)..."
rustup target add aarch64-unknown-linux-gnu 2>/dev/null || true
cargo build --release --target aarch64-unknown-linux-gnu
cp target/aarch64-unknown-linux-gnu/release/stamp_album_gen2 \
   "$RELEASE_DIR/stamp_album_gen2_linux_aarch64"

# Windows GNU (if mingw64 available)
if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "📦 Building Windows GNU (x86_64)..."
    rustup target add x86_64-pc-windows-gnu 2>/dev/null || true
    cargo build --release --target x86_64-pc-windows-gnu
    cp target/x86_64-pc-windows-gnu/release/stamp_album_gen2.exe \
       "$RELEASE_DIR/stamp_album_gen2_windows_x86_64.exe"
else
    echo "⏭️  Skipping Windows GNU (mingw-w64 not installed)"
fi

echo ""
echo "✅ Build complete! Binaries in: $RELEASE_DIR"
ls -lh "$RELEASE_DIR/"
```

---

## Troubleshooting

### MUSL Build Fails with "Could not find openssl"

**Cause:** MUSL cross-compilation requires special setup for OpenSSL
**Solution:** Use GitHub Actions or docker-based `cargo-cross`

### Windows Build from Linux

**Cause:** MinGW toolchain not installed
**Solution:** `sudo apt-get install mingw-w64`

### macOS Builds on Linux

**Cause:** Can't cross-compile to macOS without specialized tools
**Solution:** Build on actual macOS or use GitHub Actions macOS runners

---

## Next Steps

1. **Immediate:** Build Linux GNU and document it
2. **This week:** Set up GitHub Actions CI for automated cross-compilation
3. **Later:** Build macOS and Windows binaries using CI



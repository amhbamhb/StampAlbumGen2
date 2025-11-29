# Quick Command Reference - Cross-Platform Builds

## What Works Right Now ✅

### Build for Linux (your machine)

```bash
# GNU (dynamic linking - works on most Linux)
cargo build --release --target x86_64-unknown-linux-gnu

# ARM64 (Raspberry Pi, etc)
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

**Binaries:** Check in `target/{target}/release/stamp_album_gen2`

## Verify Your Builds

```bash
# Check binary info
file target/x86_64-unknown-linux-gnu/release/stamp_album_gen2

# Show size
ls -lh target/x86_64-unknown-linux-gnu/release/stamp_album_gen2

# Test it works
./target/x86_64-unknown-linux-gnu/release/stamp_album_gen2 --help

# Check dynamic dependencies
ldd target/x86_64-unknown-linux-gnu/release/stamp_album_gen2
```

## All Possible Builds (Commands Only)

### Linux Targets (from your machine)

```bash
# GNU 64-bit (standard)
cargo build --release --target x86_64-unknown-linux-gnu

# GNU ARM64 (Raspberry Pi)
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu

# MUSL static (fully portable - harder to build locally)
rustup target add x86_64-unknown-linux-musl
OPENSSL_STATIC=1 cargo build --release --target x86_64-unknown-linux-musl

# MUSL ARM64
rustup target add aarch64-unknown-linux-musl
OPENSSL_STATIC=1 cargo build --release --target aarch64-unknown-linux-musl
```

### Windows Targets (from your machine with MinGW)

```bash
# First: sudo apt-get install mingw-w64

# GNU 64-bit (cross-compile from Linux)
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

### macOS Targets (needs macOS machine or GitHub Actions)

```bash
# Intel Mac (run on Intel Mac)
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# Apple Silicon (run on Apple Silicon Mac or Intel Mac with target)
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Universal binary (requires macOS machine)
# Build both above, then:
lipo -create \
  target/x86_64-apple-darwin/release/stamp_album_gen2 \
  target/aarch64-apple-darwin/release/stamp_album_gen2 \
  -output stamp_album_gen2_universal
```

### Windows MSVC (needs Windows machine or GitHub Actions)

```bash
# Must run on Windows
cargo build --release --target x86_64-pc-windows-msvc
```

## Environment Setup (Optional Tweaks)

### For faster builds

```bash
# Use all CPU cores
export CARGO_BUILD_JOBS=$(nproc)
```

### For more static linking

```bash
export RUSTFLAGS="-C target-feature=+crt-static"
```

### For cross-compilation troubleshooting

```bash
export RUST_BACKTRACE=1
cargo build --release --target x86_64-unknown-linux-musl -v
```

## File Locations After Build

```
target/
├── x86_64-unknown-linux-gnu/
│   └── release/
│       └── stamp_album_gen2          ← Linux 64-bit GNU
├── aarch64-unknown-linux-gnu/
│   └── release/
│       └── stamp_album_gen2          ← Linux ARM64
├── x86_64-unknown-linux-musl/
│   └── release/
│       └── stamp_album_gen2          ← Linux 64-bit static
├── x86_64-pc-windows-gnu/
│   └── release/
│       └── stamp_album_gen2.exe      ← Windows 64-bit
├── x86_64-apple-darwin/
│   └── release/
│       └── stamp_album_gen2          ← macOS Intel
├── aarch64-apple-darwin/
│   └── release/
│       └── stamp_album_gen2          ← macOS Apple Silicon
└── x86_64-pc-windows-msvc/
    └── release/
        └── stamp_album_gen2.exe      ← Windows MSVC
```

## Organize Release Binaries

```bash
#!/bin/bash
VERSION=$(grep '^version' Cargo.toml | sed 's/.*"\([^"]*\)".*/\1/')
RELEASE_DIR="releases/v${VERSION}"
mkdir -p "$RELEASE_DIR"

# Copy Linux builds
cp target/x86_64-unknown-linux-gnu/release/stamp_album_gen2 \
   "$RELEASE_DIR/stamp_album_gen2_linux_x86_64"
   
cp target/aarch64-unknown-linux-gnu/release/stamp_album_gen2 \
   "$RELEASE_DIR/stamp_album_gen2_linux_aarch64"

# Copy other platforms as available...

echo "Binaries organized in: $RELEASE_DIR"
ls -lh "$RELEASE_DIR"
```

## Troubleshooting

### "Could not find openssl"

- **MUSL build:** Use GitHub Actions or Docker for reliable builds
- **GNU build:** Should work out of the box

### "mingw-w64 not found"

```bash
sudo apt-get install mingw-w64
```

### Build takes forever

- First build compiles dependencies (slow)
- Subsequent builds use cache (fast)
- MUSL builds are slower due to static linking

### Binary doesn't run on target system

```bash
# Check what it needs:
ldd ./binary  # Linux
otool -L ./binary  # macOS
```

### Want to skip build cache

```bash
cargo clean
cargo build --release --target x86_64-unknown-linux-gnu
```

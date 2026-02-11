# Cross-compilation script for Orange Pi (aarch64) using Docker
# Prerequisites: Docker installed and running

# Define project paths
PROJECT_DIR=$(pwd)/panel
OUTPUT_DIR=$(pwd)/panel/src-tauri/target/aarch64-unknown-linux-gnu/release/bundle

echo "Starting build for aarch64-unknown-linux-gnu..."

# Run the build inside a container with cross-compilation toolchain
# We use a multi-arch rust image or a specific tauri-cross image
# Using 'debian:bookworm' as base and installing tools is flexible but slow.
# Using 'messense/rust-musl-cross:aarch64-musl' is good for static binaries but Tauri needs glibc usually.
# Recommended: Use 'rust' official image and install arm64 cross compiler.

# However, Tauri apps depend on webkit2gtk which is painful to cross-compile.
# The best approach for Tauri cross-compilation is using `cross` with a custom image 
# or a specialized Dockerfile.

# Below is a simplified approach assuming we want to build the Debian package (.deb)
# which handles dependencies well on the target system.

docker run --rm -v "$(pwd):/app" -w /app/panel rust:1.77-bookworm /bin/bash -c "
    # 1. Setup Architecture
    dpkg --add-architecture arm64
    apt-get update
    
    # 2. Install Cross-Compilation Toolchain & Dependencies
    apt-get install -y \
        gcc-aarch64-linux-gnu \
        g++-aarch64-linux-gnu \
        libwebkit2gtk-4.0-dev:arm64 \
        libgtk-3-dev:arm64 \
        libayatana-appindicator3-dev:arm64 \
        librsvg2-dev:arm64 \
        libssl-dev:arm64 \
        libasound2-dev:arm64 \
        curl \
        wget \
        file \
        libdbus-1-dev:arm64

    # 3. Setup Rust target
    rustup target add aarch64-unknown-linux-gnu
    
    # 4. Configure Cargo Linker
    mkdir -p ~/.cargo
    echo '[target.aarch64-unknown-linux-gnu]' > ~/.cargo/config.toml
    echo 'linker = \"aarch64-linux-gnu-gcc\"' >> ~/.cargo/config.toml
    echo 'rustflags = [\"-C\", \"link-arg=-Wl,--allow-multiple-definition\"]' >> ~/.cargo/config.toml
    
    # 5. Install Node.js (for frontend build)
    curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
    apt-get install -y nodejs
    
    # 6. Install Tauri CLI
    cargo install tauri-cli --version 2.0.0-beta.20 --locked
    # Or rely on local npm script if available, but cargo tauri is reliable in container

    # 7. Install Frontend Deps
    npm install

    # 8. Build
    # PKG_CONFIG_SYSROOT_DIR is crucial for finding arm64 libs
    export PKG_CONFIG_PATH=/usr/lib/aarch64-linux-gnu/pkgconfig
    export PKG_CONFIG_ALLOW_CROSS=1
    
    echo 'Building Tauri App...'
    npm run tauri build -- --target aarch64-unknown-linux-gnu
"

echo "Build complete. Check $OUTPUT_DIR for the output files."

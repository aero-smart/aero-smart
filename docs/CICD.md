# CI/CD Pipeline for AeroSmart Panel

## Overview

This document describes the automated CI/CD pipeline for building the AeroSmart Panel Tauri application for OrangePi Zero 3 (aarch64).

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    GitHub Repository                         │
│                  aero-smart/aero-smart                       │
└────────────┬────────────────────────────────────────────────┘
             │
             │ Push to main / PR
             │ (panel/** or shared/** changes)
             │
             ▼
┌─────────────────────────────────────────────────────────────┐
│           GitHub Actions Workflow                            │
│         .github/workflows/build-aarch64.yml                  │
└────────────┬────────────────────────────────────────────────┘
             │
             ├─► Setup Environment
             │   ├── Rust toolchain (stable)
             │   ├── Node.js 24
             │   ├── pnpm 10.20.0
             │   └── Add aarch64-unknown-linux-gnu target
             │
             ├─► Install Cross-Compilation Dependencies
             │   ├── Add arm64 architecture to dpkg
             │   ├── Install aarch64 GCC/G++ toolchain
             │   ├── Install arm64 system libraries:
             │   │   ├── libwebkit2gtk-4.1-dev:arm64
             │   │   ├── libgtk-3-dev:arm64
             │   │   ├── libayatana-appindicator3-dev:arm64
             │   │   ├── librsvg2-dev:arm64
             │   │   ├── libssl-dev:arm64
             │   │   ├── libasound2-dev:arm64
             │   │   └── libdbus-1-dev:arm64
             │   └── Configure Cargo linker
             │
             ├─► Build Application
             │   ├── Install frontend dependencies (pnpm)
             │   ├── Build Tauri app for aarch64-unknown-linux-gnu
             │   └── Generate .deb package
             │
             └─► Upload Artifacts
                 ├── Debian package (.deb)
                 ├── AppImage (if generated)
                 └── BUILD_INFO.txt
                      ├── Branch name
                      ├── Commit SHA
                      ├── Build date
                      └── Target platform info
```

## Target Platform

**Hardware:** OrangePi Zero 3
- **CPU:** Allwinner H618 (4x Cortex-A53, 64-bit)
- **Architecture:** aarch64 (ARM64)
- **Display:** 1024x600 pixels

**Software Stack:**
- **OS:** Linux-based (Debian/Ubuntu compatible)
- **Runtime:** Tauri 2.x (WebView2 + Rust backend)
- **Display Mode:** Fullscreen, no decorations

## Build Triggers

The workflow is triggered automatically by:

1. **Push to main branch**
   - Only when files in `panel/**` or `shared/**` are modified
   - Or when the workflow file itself is modified

2. **Pull Requests**
   - Targeting the main branch
   - Affecting `panel/**` or `shared/**`

3. **Manual Trigger**
   - Via GitHub Actions UI (workflow_dispatch)

## Build Outputs

### Debian Package (.deb)
- **Location:** `panel/src-tauri/target/aarch64-unknown-linux-gnu/release/bundle/deb/`
- **Name Pattern:** `aero-smart-panel_<version>_arm64.deb`
- **Installation:**
  ```bash
  sudo dpkg -i aero-smart-panel_<version>_arm64.deb
  sudo apt-get install -f  # Install dependencies if needed
  ```

### AppImage (Optional)
- **Location:** `panel/src-tauri/target/aarch64-unknown-linux-gnu/release/bundle/appimage/`
- **Name Pattern:** `aero-smart-panel_<version>_aarch64.AppImage`
- **Usage:**
  ```bash
  chmod +x aero-smart-panel_<version>_aarch64.AppImage
  ./aero-smart-panel_<version>_aarch64.AppImage
  ```

## Artifacts Retention

- **Retention Period:** 30 days
- **Download:** Via GitHub Actions UI → workflow run → Artifacts section
- **Naming:** `aero-smart-panel-aarch64-<commit-sha>`

## Configuration Files

### Window Configuration
File: `panel/src-tauri/tauri.conf.json`

```json
{
  "app": {
    "windows": [
      {
        "title": "Aero Smart Panel",
        "width": 1024,
        "height": 600,
        "resizable": false,
        "fullscreen": true,
        "decorations": false,
        "alwaysOnTop": true,
        "skipTaskbar": true
      }
    ]
  }
}
```

### Cargo Configuration
The workflow automatically creates `~/.cargo/config.toml`:

```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
rustflags = ["-C", "link-arg=-Wl,--allow-multiple-definition"]
```

## Local Development

### Build Locally for aarch64

**Prerequisites:**
```bash
# Add arm64 architecture
sudo dpkg --add-architecture arm64

# Install cross-compilation toolchain
sudo apt-get install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu

# Install arm64 dependencies (adjust sources.list first)
sudo apt-get install libwebkit2gtk-4.1-dev:arm64 \
  libgtk-3-dev:arm64 \
  libayatana-appindicator3-dev:arm64 \
  librsvg2-dev:arm64 \
  libssl-dev:arm64 \
  libasound2-dev:arm64 \
  libdbus-1-dev:arm64

# Add Rust target
rustup target add aarch64-unknown-linux-gnu
```

**Build Commands:**
```bash
# Option 1: Using npm script
cd panel
pnpm tauri:build:aarch64

# Option 2: Using just
just build-panel-aarch64

# Option 3: Using the Docker-based script
./build_aarch64.sh
```

## Testing on OrangePi Zero 3

1. Download the .deb artifact from GitHub Actions
2. Transfer to OrangePi:
   ```bash
   scp aero-smart-panel_*.deb pi@orangepi.local:~/
   ```
3. Install on OrangePi:
   ```bash
   sudo dpkg -i ~/aero-smart-panel_*.deb
   sudo apt-get install -f
   ```
4. Run:
   ```bash
   aero-smart-panel
   ```

## Troubleshooting

### Build Failures

**Issue:** Missing arm64 dependencies
- **Solution:** Check that all required arm64 libraries are installed
- The workflow includes `|| true` to continue even if some packages fail

**Issue:** Linker errors
- **Solution:** Verify cargo config has correct linker settings
- Check that gcc-aarch64-linux-gnu is installed

### Runtime Issues on Target

**Issue:** Missing shared libraries
- **Solution:** Install via apt on target device:
  ```bash
  sudo apt-get install libwebkit2gtk-4.1 libgtk-3-0 libayatana-appindicator3-1
  ```

**Issue:** Display not fullscreen
- **Solution:** Check tauri.conf.json has correct window settings
- Verify display resolution is 1024x600

## Related Files

- **Workflow:** `.github/workflows/build-aarch64.yml`
- **Tauri Config:** `panel/src-tauri/tauri.conf.json`
- **Package Scripts:** `panel/package.json`
- **Build Commands:** `justfile`
- **Docker Build:** `build_aarch64.sh`
- **Documentation:** `panel/README.md`, `README.md`

## Future Enhancements

- [ ] Add automatic release creation for tagged commits
- [ ] Support for other ARM platforms (Raspberry Pi, etc.)
- [ ] Code signing for releases
- [ ] Incremental builds with better caching
- [ ] Multi-architecture support (x86_64 + aarch64 in single workflow)

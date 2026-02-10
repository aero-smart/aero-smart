setup:
  cargo install taplo-cli cargo-shear --locked

gen-ts-schema:
  @echo "Generating TypeScript schema..."
  cd shared && cargo run --bin gen-ts-schemas -F ts-rs,sql
  @echo "Moving file from shared/bindings/generated to panel/src/types"
  if [ ! -d panel/src/types ]; then mkdir -p panel/src/types; fi
  mv shared/bindings/generated/*.ts panel/src/types/
  @echo "Removing old generated files..."
  rm -rf shared/bindings/generated/*.ts
  @echo "Format the moved TypeScript files..."
  cd panel && pnpm run format
  @echo "TypeScript schema generation completed."

# ============================================================================
# CI Commands
# ============================================================================

# CI for shared library
ci-shared:
  @echo "Running CI for shared library..."
  cargo check -p aerosmart-shared
  cargo clippy -p aerosmart-shared --all-targets -- -D warnings
  cargo build -p aerosmart-shared

# CI for firmware
ci-firmware:
  @echo "Running CI for firmware..."
  cargo check -p aerosmart-firmware --target thumbv7em-none-eabi
  cargo clippy -p aerosmart-firmware --target thumbv7em-none-eabi -- -D warnings
  cargo build -p aerosmart-firmware --target thumbv7em-none-eabi

# CI for panel
ci-panel:
  @echo "Running CI for panel..."
  cd panel && pnpm install
  cd panel && pnpm run type-check
  cd panel && pnpm run lint
  cd panel && pnpm run build

# ============================================================================
# Build Commands
# ============================================================================

# Build firmware
build-firmware:
  @echo "Building firmware in release mode..."
  cargo build -p aerosmart-firmware --target thumbv7em-none-eabi --release

# Build panel (Tauri application)
build-panel:
  @echo "Building panel application..."
  cd panel && pnpm install
  cd panel && pnpm run build

# Build panel for aarch64 (OrangePi Zero 3)
build-panel-aarch64:
  @echo "Building panel application for aarch64 (OrangePi Zero 3)..."
  cd panel && pnpm install
  cd panel && pnpm run tauri:build:aarch64

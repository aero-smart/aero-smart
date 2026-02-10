# panel

This is the AeroSmart Panel - a Tauri-based desktop application for controlling and monitoring the AeroSmart wind tunnel system.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Vue (Official)](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur).

## Recommended Browser Setup

- Chromium-based browsers (Chrome, Edge, Brave, etc.):
  - [Vue.js devtools](https://chromewebstore.google.com/detail/vuejs-devtools/nhdogjmejiglipccpnnnanhbledajbpd)
  - [Turn on Custom Object Formatter in Chrome DevTools](http://bit.ly/object-formatters)
- Firefox:
  - [Vue.js devtools](https://addons.mozilla.org/en-US/firefox/addon/vue-js-devtools/)
  - [Turn on Custom Object Formatter in Firefox DevTools](https://fxdx.dev/firefox-devtools-custom-object-formatters/)

## Type Support for `.vue` Imports in TS

TypeScript cannot handle type information for `.vue` imports by default, so we replace the `tsc` CLI with `vue-tsc` for type checking. In editors, we need [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) to make the TypeScript language service aware of `.vue` types.

## Customize configuration

See [Vite Configuration Reference](https://vite.dev/config/).

## Project Setup

```sh
pnpm install
```

### Compile and Hot-Reload for Development

```sh
pnpm dev
```

### Type-Check, Compile and Minify for Production

```sh
pnpm build
```

### Run Unit Tests with [Vitest](https://vitest.dev/)

```sh
pnpm test:unit
```

### Run End-to-End Tests with [Playwright](https://playwright.dev)

```sh
# Install browsers for the first run
npx playwright install

# When testing on CI, must build the project first
pnpm build

# Runs the end-to-end tests
pnpm test:e2e
# Runs the tests only on Chromium
pnpm test:e2e --project=chromium
# Runs the tests of a specific file
pnpm test:e2e tests/example.spec.ts
# Runs the tests in debug mode
pnpm test:e2e --debug
```

### Lint with [ESLint](https://eslint.org/)

```sh
pnpm lint
```

## Tauri Desktop Application

This panel is built as a Tauri desktop application, providing a native application experience with web technologies.

### Development Mode

Run the Tauri app in development mode with hot-reload:

```sh
pnpm tauri:dev
```

### Build for Production

Build the Tauri application for your current platform:

```sh
pnpm tauri:build
```

### Build for OrangePi Zero 3 (aarch64)

The application is optimized for OrangePi Zero 3 with a 1024x600 display. To build for this platform:

**Using npm script:**
```sh
pnpm tauri:build:aarch64
```

**Using just:**
```sh
just build-panel-aarch64
```

**Using the build script:**
```sh
../build_aarch64.sh
```

#### Prerequisites for Cross-Compilation

For building aarch64 binaries on x86_64 Linux:

1. Add arm64 architecture:
   ```sh
   sudo dpkg --add-architecture arm64
   ```

2. Install cross-compilation toolchain:
   ```sh
   sudo apt-get install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
   ```

3. Install aarch64 dependencies:
   ```sh
   sudo apt-get install \
     libwebkit2gtk-4.1-dev:arm64 \
     libgtk-3-dev:arm64 \
     libayatana-appindicator3-dev:arm64 \
     librsvg2-dev:arm64 \
     libssl-dev:arm64 \
     libasound2-dev:arm64 \
     libdbus-1-dev:arm64
   ```

4. Add Rust target:
   ```sh
   rustup target add aarch64-unknown-linux-gnu
   ```

5. Configure Cargo linker in `~/.cargo/config.toml`:
   ```toml
   [target.aarch64-unknown-linux-gnu]
   linker = "aarch64-linux-gnu-gcc"
   ```

#### CI/CD Pipeline

The repository includes a GitHub Actions workflow (`.github/workflows/build-aarch64.yml`) that automatically builds the aarch64 version when code is pushed to the main branch. The built artifacts are uploaded and can be downloaded from the Actions tab.

#### Display Configuration

The application is configured for a 1024x600 display with:
- Fullscreen mode enabled
- Window decorations disabled
- Always on top
- Skip taskbar

These settings are defined in `src-tauri/tauri.conf.json` and are optimized for embedded display use.


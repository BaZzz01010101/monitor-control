# Rust + Slint — Windows-only Dell monitor DDC/CI controller

## Workspace

4 crates under `crates/`:
- **core** (`dell-controller-core`) — library: DDC/CI abstraction (`DdcBackend` trait), HDR, capabilities parsing, profiles
- **cli** (`dellctl`) — binary CLI tool via `clap`
- **lab** (`dell-lab`) — binary exploratory tool
- **tray** (`dell-controller-tray`) — binary GUI app: Slint window + Windows system tray via `tray-icon`

Only `tray` has Slint `.slint` files (`crates/tray/ui/`). Compiled at build time via `slint_build` in `build.rs`.

## Build & run

```powershell
# Default build (enables Slint live preview):
cargo build --workspace --bins --features slint/live-preview
# Requires SLINT_LIVE_PREVIEW=1 env var at runtime

# Single crate builds:
cargo build -p dellctl
cargo build -p dell-lab
cargo build -p dell-controller-tray

# Run tray app:
cargo run -p dell-controller-tray

# Run CLI:
cargo run -p dellctl -- monitors
cargo run -p dellctl -- get 0 brightness
```

## Test

```powershell
# All tests across workspace:
cargo test

# Single crate:
cargo test -p dell-controller-core
cargo test -p dell-controller-tray

# Single test:
cargo test -p dell-controller-tray --test app_controller autostart_worker_event_updates_ui_state
cargo test -p dell-controller-tray --lib settings_pane_contains_only_navigation_shell
```

Tests use `tempfile` for filesystem isolation and mock `DdcBackend` implementations (see `crates/core/tests/queue.rs`, `crates/tray/tests/app_controller.rs`). Prefer behavioral/state-based tests over source-inspection tests.

## Key architecture

- **DDC/CI backend**: `DdcBackend` trait in `crates/core/src/ddc.rs`. Windows impl in `windows_backend.rs` via `GetVCPFeatureAndVCPFeatureReply`/`SetVCPFeature`. `CommandQueue<B>` wraps with retry (`RetryPolicy`: 3 attempts, 40ms delay) and serialization via `synchronization_lock()`.
- **Bridge pattern**: `AppController` (state machine) in `crates/tray/src/app_controller.rs` — pure logic, no UI. `UiBridge` in `ui_bridge.rs` maps state to Slint properties.
- **Worker**: background thread (`worker.rs`) handling monitor enumeration + DDC reads/writes with write coalescing.
- **HDR**: via Windows `DisplayConfigGetDeviceInfo`/`DisplayConfigSetDeviceInfo` (advanced color API) in `crates/core/src/hdr.rs`.
- **Hotkeys**: `global-hotkey` crate, **git fork** at `BaZzz01010101/global-hotkey` (not crates.io).

## Persistence & state

- **Settings**: `%APPDATA%/Dell Controller/settings.toml` — autostart, selected monitor, hotkey bindings
- **Window state**: `%APPDATA%/Dell Controller/state.toml` — window position
- **User profiles**: `%APPDATA%/Dell Controller/*.toml` (merged with built-in `crates/core/src/profiles/u4025qw.toml`)
- **Autostart**: Windows registry `HKCU\...\Run\DellController` (`crates/core/src/startup.rs`)

## Conventions

- Use `parking_lot::Mutex` over `std::sync::Mutex` (workspace dep)
- `anyhow::Result` in binaries, `thiserror` for library error types
- `VcpCode(u8)` typed wrapper with hex formatting (`{:02X}`), `VcpValue(u32)`
- No CI workflows, no rustfmt/clippy config — cargo defaults

## Testing rules

- Do **not** add tests that inspect hand-written source files as text (exact contents, formatting, whitespace, line endings).
- Prefer behavioral, state-based, API-level, or integration tests.
- Source-inspection tests allowed only for generated artifacts or machine-readable outputs.
- If a source-inspection test for a hand-written file seems unavoidable, ask first.

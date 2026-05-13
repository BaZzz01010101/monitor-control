# Settings Autostart Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a `Start with Windows` setting to the Slint settings pane using the existing autostart backend path.

**Architecture:** Keep the existing pane-switch architecture and extend only the settings pane UI plus the already-present bridge/controller state path. The Slint view will expose `autostart-enabled` and a toggle callback, while Rust will bind that state to `UiState.autostart_enabled` and dispatch the existing `UiAction::ToggleAutostart`.

**Tech Stack:** Rust, Slint 1.16, existing tray worker/controller architecture

---

### Task 1: Lock Settings Pane Requirements With Failing Tests

**Files:**
- Modify: `crates/tray/src/ui_bridge.rs`
- Test: `crates/tray/src/ui_bridge.rs`

- [ ] **Step 1: Write the failing test**

Add assertions to `settings_pane_contains_only_navigation_shell()` so it requires:
- `Switch`
- `autostart-enabled`
- `toggle-autostart`
- `General`
- `Start with Windows`

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p dell-controller-tray --lib settings_pane_contains_only_navigation_shell`
Expected: FAIL because `SettingsPane.slint` does not yet contain the switch/settings row

- [ ] **Step 3: Write minimal implementation**

Implement only enough Slint UI and callback wiring to satisfy the test:
- add `autostart-enabled` property to `SettingsPane`
- add `toggle-autostart` callback
- add a compact `General` card with one settings row and a `Switch`

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p dell-controller-tray --lib settings_pane_contains_only_navigation_shell`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/tray/src/ui_bridge.rs crates/tray/ui/SettingsPane.slint
git commit -m "feat: add autostart switch to settings pane"
```

### Task 2: Bind Settings UI To Existing Rust State/Actions

**Files:**
- Modify: `crates/tray/ui/MainWindow.slint`
- Modify: `crates/tray/src/ui_bridge.rs`
- Test: `crates/tray/tests/app_controller.rs`

- [ ] **Step 1: Write the failing test**

Add a controller-level test that applies `WorkerEvent::AutostartState { enabled: true, status: String::new() }` and asserts `ui_state().autostart_enabled` becomes `true`.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p dell-controller-tray --test app_controller autostart_worker_event_updates_ui_state`
Expected: FAIL because the test does not exist yet

- [ ] **Step 3: Write minimal implementation**

Wire the Slint properties/callbacks:
- add `in-out property <bool> autostart-enabled` to `MainWindow`
- bind it into `SettingsPane`
- add `callback toggle-autostart()`
- in `UiBridge.apply_state()`, call `set_autostart_enabled(state.autostart_enabled)`
- in `wire_callbacks()`, forward `on_toggle_autostart` to `UiAction::ToggleAutostart`

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p dell-controller-tray --test app_controller autostart_worker_event_updates_ui_state`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/tray/ui/MainWindow.slint crates/tray/src/ui_bridge.rs crates/tray/tests/app_controller.rs
git commit -m "feat: wire settings autostart switch to controller"
```

### Task 3: Full Verification

**Files:**
- Test: `crates/tray/src/ui_bridge.rs`
- Test: `crates/tray/tests/app_controller.rs`

- [ ] **Step 1: Run targeted settings tests**

Run: `cargo test -p dell-controller-tray --lib settings_pane_contains_only_navigation_shell`
Expected: PASS

- [ ] **Step 2: Run targeted controller test**

Run: `cargo test -p dell-controller-tray --test app_controller autostart_worker_event_updates_ui_state`
Expected: PASS

- [ ] **Step 3: Run full tray crate verification**

Run: `cargo test -p dell-controller-tray`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add crates/tray/ui/MainWindow.slint crates/tray/ui/SettingsPane.slint crates/tray/src/ui_bridge.rs crates/tray/tests/app_controller.rs crates/tray/src/ui_bridge.rs docs/superpowers/plans/2026-05-14-settings-autostart.md
git commit -m "feat: add settings autostart toggle"
```

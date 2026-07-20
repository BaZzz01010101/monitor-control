# HDR DDC Picture Cache Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Preserve per-monitor DDC brightness and contrast values during HDR, restore cached controls immediately when HDR is disabled, and reconcile them safely with a background read.

**Architecture:** `AppController` owns an in-memory cache keyed by the selected monitor key and keeps DDC values separate from Windows SDR-content brightness. When HDR is disabled with a complete cache, the worker emits a lightweight confirmed HDR snapshot without DDC picture reads, completes the switch, and then performs a full snapshot in the background. The controller ignores that background snapshot for any feature the user changed after the cached controls became interactive, while the existing throttled DDC write continues normally.

**Tech Stack:** Rust, Slint, Windows DisplayConfig, DDC/CI, `std::sync::mpsc` worker.

## Global Constraints

- Cache values in memory only; do not change persistence or settings formats.
- Key cached values by the selected monitor key; never reuse one monitor's values for another.
- Cached values are displayed but never written automatically to the monitor.
- Do not read or write DDC brightness/contrast while HDR is active.
- While HDR is active, Brightness uses Windows SDR-content brightness and Contrast displays the last cached DDC value while remaining disabled.
- If both cached DDC values exist when HDR is disabled, complete the switch from the cache and read actual DDC values afterward.
- If either cached value is absent, read both DDC values before completing the switch.
- A user edit made before the background reconciliation arrives must update the monitor through the existing throttled/coalesced DDC path and must not be overwritten by the stale read.
- Preserve unrelated uncommitted work and do not add source-text or pixel-snapshot tests.

---

### Task 1: Controller cache and transition policy

**Files:**
- Modify: `crates/tray/src/app_controller.rs`
- Test: `crates/tray/tests/app_controller.rs`

**Interfaces:**
- Produces: `WorkerRequest::SetHdr { enabled, use_cached_ddc_values }`.
- Consumes: `WorkerEvent::HdrUpdateFinished { enabled, error, used_cached_ddc_values }` from Task 2.
- Produces: a per-monitor `DdcPictureCache` and background-reconciliation guards for brightness and contrast.

- [ ] **Step 1: Add failing controller tests**

Add behavioral tests with these exact setups and assertions:

- `hdr_active_snapshot_preserves_cached_ddc_contrast`: apply a DDC snapshot containing Brightness 50 and Contrast 80, then an HDR-active Windows snapshot containing SDR Brightness 35 and unavailable Contrast; assert Brightness is enabled at 35 and Contrast is disabled but still displays 80.
- `disabling_hdr_requests_cached_restore_only_for_a_complete_selected_monitor_cache`: assert a DDC-to-HDR controller sends `SetHdr { enabled: false, use_cached_ddc_values: true }`, while a controller started directly in HDR sends the same request with `false`.
- `cached_hdr_off_completion_enables_ddc_controls_before_background_reconciliation`: apply unavailable deferred DDC picture values and a successful cached completion; assert pending clears and Brightness 50/Contrast 80 are enabled.
- `background_reconciliation_updates_untouched_cached_values`: after cached completion, apply a full DDC snapshot containing 57/77 and assert both displayed values change.
- `user_edit_wins_over_the_in_flight_restore_snapshot`: change Brightness to 63 after cached completion, assert a VCP `0x10` write is emitted, then apply a stale 45/75 snapshot after the normal time guard and assert Brightness remains 63 while untouched Contrast becomes 75.
- `ddc_picture_cache_is_scoped_by_monitor_key`: cache the first monitor, select a second monitor directly in HDR, and assert its Contrast remains `n/a` and its HDR-off request does not claim cached values.

- [ ] **Step 2: Run the controller tests and verify RED**

```powershell
$env:CARGO_TARGET_DIR='D:\TEMP\dell-controller-codex-target'
cargo test -p dell-controller-tray --test app_controller
```

Expected: compilation or assertions fail because the HDR request/completion cache flags and cache behavior do not exist yet.

- [ ] **Step 3: Implement the minimal controller state**

Add an internal cache with separate optional feature values:

```rust
#[derive(Clone, Debug, Default)]
struct DdcPictureCache {
    brightness: Option<FeatureState>,
    contrast: Option<FeatureState>,
}

ddc_picture_cache: HashMap<String, DdcPictureCache>,
restore_cached_ddc_requested: bool,
awaiting_ddc_restore_refresh: bool,
brightness_changed_during_restore_refresh: bool,
contrast_changed_during_restore_refresh: bool,
```

Update a monitor's cache only from available full DDC snapshots. HDR-active snapshots update the Windows brightness state but obtain Contrast's displayed value from that monitor's cache and keep it disabled. When handling `ToggleHdr(false)`, set `use_cached_ddc_values` only when both cached values exist for the selected monitor.

When cached restoration is confirmed, show the cache and enable the DDC controls. Mark user edits made before the first subsequent full DDC snapshot; that snapshot may update availability/maximum but cannot replace an edited value. Clear the reconciliation markers after consuming that snapshot. Normal slider actions continue producing `WriteFeature` requests.

- [ ] **Step 4: Run the controller tests and verify GREEN**

```powershell
$env:CARGO_TARGET_DIR='D:\TEMP\dell-controller-codex-target'
cargo test -p dell-controller-tray --test app_controller
```

Expected: all controller integration tests pass.

### Task 2: Deferred DDC read in the worker

**Files:**
- Modify: `crates/tray/src/worker.rs`

**Interfaces:**
- Consumes: `WorkerRequest::SetHdr { enabled, use_cached_ddc_values }`.
- Produces: `WorkerEvent::HdrUpdateFinished { enabled, error, used_cached_ddc_values }`.
- Produces: internal `PictureReadMode::{Full, DeferDdc}` used by `WorkerDevice::snapshot`.

- [ ] **Step 1: Add failing worker tests**

Add state/API-level tests with these exact assertions:

- `deferred_inactive_hdr_snapshot_skips_ddc_picture_reads`: call `read_picture_features` for inactive HDR with `DeferDdc`; assert neither callback runs, both features are unavailable, and the backend remains `Ddc`.
- `cached_hdr_off_task_completes_before_full_picture_refresh`: execute cached HDR-off and assert the call sequence is HDR write, deferred snapshot, full snapshot while the event sequence is deferred Snapshot, cached `HdrUpdateFinished`, full Snapshot.
- Existing uncached HDR task coverage must assert the full confirmed Snapshot precedes `HdrUpdateFinished` and `used_cached_ddc_values` is false.
- `failed_hdr_change_does_not_claim_cached_restoration`: request cached HDR-off with a failing HDR API and assert one recovery Snapshot, an error completion with `used_cached_ddc_values` false, and no background Snapshot.

- [ ] **Step 2: Run the worker unit tests and verify RED**

```powershell
$env:CARGO_TARGET_DIR='D:\TEMP\dell-controller-codex-target'
cargo test -p dell-controller-tray --lib worker::tests
```

Expected: compilation or assertions fail because snapshot read modes and ordered deferred restoration do not exist.

- [ ] **Step 3: Implement ordered deferred restoration**

Extend the internal snapshot API:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PictureReadMode {
    Full,
    DeferDdc,
}

trait WorkerDevice {
    fn snapshot(
        &mut self,
        selected_monitor_key: Option<&str>,
        picture_read_mode: PictureReadMode,
    ) -> DeviceSnapshot;
}
```

For a successful cached HDR-off request, perform the following exact order:

```text
set HDR off
emit snapshot with DDC brightness/contrast deferred
emit HdrUpdateFinished with used_cached_ddc_values = true
emit a full snapshot in the background
```

All ordinary snapshots use `Full`. HDR-on, HDR-off without cache, and failed HDR changes retain snapshot-before-completion ordering and report `used_cached_ddc_values = false`. The deferred snapshot must still resolve and confirm the selected Windows target and must never read VCP `0x10` or `0x12`.

- [ ] **Step 4: Run the worker tests and verify GREEN**

```powershell
$env:CARGO_TARGET_DIR='D:\TEMP\dell-controller-codex-target'
cargo test -p dell-controller-tray --lib worker::tests
```

Expected: all worker unit tests pass.

### Task 3: Regression verification

**Files:**
- Verify only; do not add generated artifacts to the repository.

**Interfaces:**
- Consumes: Tasks 1 and 2.
- Produces: fresh verification evidence for the complete worktree.

- [ ] **Step 1: Run formatting and focused tests**

```powershell
cargo fmt --all -- --check
git diff --check
$env:CARGO_TARGET_DIR='D:\TEMP\dell-controller-codex-target'
cargo test -p dell-controller-tray --test app_controller
cargo test -p dell-controller-tray --lib worker::tests
```

Expected: every command exits successfully.

- [ ] **Step 2: Run the full required checks**

```powershell
$env:CARGO_TARGET_DIR='D:\TEMP\dell-controller-codex-target'
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --bins --features slint/live-preview
```

Expected: all tests pass, Clippy reports no warnings, and all binaries build.

- [ ] **Step 3: Validate on Windows 11 and clean external artifacts**

Manually verify cached and cold-start HDR-off transitions, edits made before the background refresh, contrast preservation in HDR, external Windows HDR changes, and selected-monitor changes. Then run `git status --short` and remove only the external target directory:

```powershell
cargo clean --target-dir D:\TEMP\dell-controller-codex-target
```

Do not create a commit unless the user explicitly requests one; the modified files belong to the existing uncommitted HDR feature set.

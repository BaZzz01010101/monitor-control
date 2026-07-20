# Passive HDR Information Tooltip Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the focus-stealing HDR information popup with a stable, passive hover-only tooltip.

**Architecture:** Split the existing Slint component into a passive `HdrInfoIcon`, which owns only hover timing, and a non-interactive `HdrInfoTooltip`, which owns only rendering. Forward the icon's absolute window coordinates through `HeaderCard` callbacks and render the tooltip last in `MainWindow`, above normal content without a second window.

**Tech Stack:** Rust workspace, Slint 1.16.1, Windows 11.

## Global Constraints

- The icon has no click, keyboard-focus, Escape, or accessibility behavior.
- Hover delay remains 350 ms; pointer leave hides immediately.
- Preserve the existing tooltip copy and visual styling.
- Do not use `PopupWindow`.
- Do not add source-text or pixel-snapshot tests.
- Preserve all unrelated uncommitted changes.

---

### Task 1: Passive icon and in-window tooltip

**Files:**
- Modify: `crates/tray/ui/HdrInfoTooltip.slint`
- Modify: `crates/tray/ui/HeaderCard.slint`
- Modify: `crates/tray/ui/MainWindow.slint`

**Interfaces:**
- Produces: `HdrInfoIcon.show-tooltip(length, length)` and `HdrInfoIcon.hide-tooltip()` callbacks.
- Produces: matching `HeaderCard.show-hdr-info(length, length)` and `HeaderCard.hide-hdr-info()` callbacks.
- Produces: `HdrInfoTooltip.message: string` as a passive visual component.

- [ ] **Step 1: Confirm the current failing behavior**

Run the app and verify the reported baseline: opening the current popup removes focus from the icon; moving within the icon can close and restart the tooltip; click, Space, and Escape cause focus/show/hide loops.

- [ ] **Step 2: Replace the popup-backed component**

Replace `HdrInfoTooltip.slint` with two components following this structure:

```slint
import { Palette } from "std-widgets.slint";

export component HdrInfoIcon inherits Rectangle {
    callback show-tooltip(length, length);
    callback hide-tooltip();

    width: 16px;
    height: 16px;
    border-radius: self.width / 2;
    border-width: 1px;
    border-color: Palette.control-foreground.with-alpha(0.68);
    background: touch.has-hover ? Palette.control-foreground.with-alpha(0.08) : transparent;

    Text {
        text: "i";
        color: Palette.control-foreground;
        font-size: 11px;
        font-weight: 700;
        horizontal-alignment: center;
        vertical-alignment: center;
    }

    touch := TouchArea {
        width: parent.width;
        height: parent.height;

        changed has-hover => {
            if (self.has-hover) {
                hover-delay.start();
            } else {
                hover-delay.stop();
                root.hide-tooltip();
            }
        }
    }

    hover-delay := Timer {
        interval: 350ms;
        triggered => {
            if (touch.has-hover) {
                root.show-tooltip(root.absolute-position.x, root.absolute-position.y);
            }
        }
    }
}

export component HdrInfoTooltip inherits Rectangle {
    in property <string> message;

    width: 350px;
    height: 92px;
    border-radius: 6px;
    border-width: 1px;
    border-color: Palette.border;
    background: Palette.background;
    drop-shadow-blur: 8px;
    drop-shadow-color: #00000030;
    drop-shadow-offset-y: 3px;

    Text {
        x: 12px;
        y: 10px;
        width: parent.width - 24px;
        height: parent.height - 20px;
        text: root.message;
        color: Palette.foreground;
        font-size: 13px;
        wrap: word-wrap;
        vertical-alignment: center;
    }
}
```

The final implementation must omit `PopupWindow`, `FocusScope`, `clicked`, `key-pressed`, `accessible-*`, pressed styling, and pointer cursor declarations.

- [ ] **Step 3: Forward hover requests through the header**

In `HeaderCard.slint`, import both components, add these callbacks, and forward the icon events:

```slint
callback show-hdr-info(length, length);
callback hide-hdr-info();

HdrInfoIcon {
    show-tooltip(x, y) => { root.show-hdr-info(x, y); }
    hide-tooltip => { root.hide-hdr-info(); }
}
```

- [ ] **Step 4: Render the tooltip as the last main-window layer**

In `MainWindow.slint`, import `HdrInfoTooltip`, add private visibility and anchor properties, update them from the `HeaderCard` callbacks, and place this conditional component after the main/settings content:

```slint
private property <bool> hdr-info-tooltip-visible: false;
private property <length> hdr-info-tooltip-x: 0px;
private property <length> hdr-info-tooltip-y: 0px;

show-hdr-info(x, y) => {
    root.hdr-info-tooltip-x = x;
    root.hdr-info-tooltip-y = y;
    root.hdr-info-tooltip-visible = true;
}
hide-hdr-info => {
    root.hdr-info-tooltip-visible = false;
}

if (!root.settings-open && root.hdr-info-tooltip-visible) : HdrInfoTooltip {
    x: max(8px, min(root.width - self.width - 8px, root.hdr-info-tooltip-x - 12px));
    y: max(8px, min(root.height - self.height - 8px, root.hdr-info-tooltip-y + 23px));
    message: "With Windows HDR on, Brightness controls Windows’ SDR content brightness. It does not change HDR content or the monitor backlight. Contrast is managed by HDR and is unavailable.";
}
```

Clear `hdr-info-tooltip-visible` before opening Settings so a stale tooltip cannot reappear when returning to the main pane.

- [ ] **Step 5: Compile the Slint UI**

Run:

```powershell
$env:CARGO_TARGET_DIR='D:\TEMP\dell-controller-codex-target'
cargo build -p dell-controller-tray
```

Expected: successful compilation with no Slint diagnostics.

- [ ] **Step 6: Manually validate the interaction state machine**

Verify all of the following on Windows:

- Hovering for less than 350 ms and leaving shows nothing.
- Hovering for 350 ms shows the hint once.
- Moving by one pixel while still over the icon keeps the hint visible.
- Clicking before or after the hint appears has no effect.
- Tab skips the icon; Space and Escape have no tooltip effect.
- Leaving hides immediately; re-entering starts a fresh 350 ms delay.
- The hint appears above the normal content and does not shift layout.

Do not create an implementation-only commit because these UI files already contain dependent uncommitted HDR feature work; leave them together for that feature's final commit.

### Task 2: Regression verification

**Files:**
- Verify only; no additional files.

**Interfaces:**
- Consumes: the passive tooltip implementation from Task 1.
- Produces: verification evidence for the complete worktree.

- [ ] **Step 1: Run formatting and whitespace checks**

```powershell
cargo fmt --all -- --check
git diff --check
```

Expected: both commands exit successfully.

- [ ] **Step 2: Run all automated checks outside the repository**

```powershell
$env:CARGO_TARGET_DIR='D:\TEMP\dell-controller-codex-target'
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --bins --features slint/live-preview
```

Expected: all tests pass, Clippy reports no warnings, and all binaries build.

- [ ] **Step 3: Recheck worktree scope and clean external artifacts**

Run `git status --short` and confirm that no repository-local target directory or unrelated file was added. Remove the external Cargo target with:

```powershell
cargo clean --target-dir D:\TEMP\dell-controller-codex-target
```

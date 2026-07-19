# AppSwitch Physical-Pixel Rendering Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Render the shared switch with crisp one-physical-pixel capsule edges and no right-side stroke seam at Windows 150% display scaling.

**Architecture:** Keep the `AppSwitch` public interface and 40×20 logical layout unchanged. Snap a private drawing layer to the nearest physical-pixel origin, construct the rail from nested filled rounded rectangles, and show focus through a centered thumb size/color transition instead of a rail outline.

**Tech Stack:** Slint 1.16, Rust workspace tooling, Windows 11 fractional DPI

## Global Constraints

- Preserve the existing `AppSwitch` public properties, callback, logical geometry, state behavior, accessibility, and pending animation.
- Do not modify controller, worker, bridge, persistence, dependencies, or settings.
- Do not add source-text tests or pixel-snapshot tests.
- Validate the rasterization correction manually on Windows at 150% display scaling.
- Preserve all unrelated uncommitted changes.

---

### Task 1: Pixel-aligned filled capsule rendering

**Files:**
- Modify: `crates/tray/ui/AppSwitch.slint`

**Interfaces:**
- Consumes: `checked: bool`, `enabled: bool`, `pending: bool`, `accessible-name: string`, and `toggled(bool)` from the existing `AppSwitch` interface.
- Produces: The same `AppSwitch` interface and 40×20 logical layout with renderer-independent filled outlines.

- [x] **Step 1: Record the manual failing case**

At Windows 150% display scaling, focus either switch and inspect it with Windows Magnifier. Confirm the current rendering shows both expected defects: blurred straight top/bottom borders and a one-pixel discontinuity at the right side. This is the regression check because project constraints exclude pixel-snapshot tests and a state/API test cannot observe GPU rasterization.

- [x] **Step 2: Add physical-origin snapping**

Add these private properties near the existing internal properties:

```slint
private property <length> pixel-offset-x: round(root.absolute-position.x / 1phx) * 1phx - root.absolute-position.x;
private property <length> pixel-offset-y: round(root.absolute-position.y / 1phx) * 1phx - root.absolute-position.y;
```

Wrap the focus indication, rail, pending path, and thumb in a transparent drawing layer. Keep `TouchArea` and `FocusScope` at the root so hit testing and focus geometry do not move:

```slint
drawing-layer := Rectangle {
    x: root.pixel-offset-x;
    y: root.pixel-offset-y;
    width: root.width;
    height: root.height;
    background: transparent;

    // Visual children move here.
}
```

- [x] **Step 3: Replace closed stroked outlines with filled capsules**

Inside `drawing-layer`, replace the focus and rail rectangles with:

```slint
if focus-scope.has-focus && root.interactive : Rectangle {
    x: -1phx;
    y: -1phx;
    width: parent.width + 2phx;
    height: parent.height + 2phx;
    border-radius: parent.height / 2 + 1phx;
    background: Palette.accent-background;
}

rail := Rectangle {
    width: parent.width;
    height: parent.height;
    border-radius: self.height / 2;
    background: root.checked ? Palette.accent-background : Palette.border;

    if !root.checked : Rectangle {
        x: 1phx;
        y: 1phx;
        width: parent.width - 2phx;
        height: parent.height - 2phx;
        border-radius: parent.border-radius - 1phx;
        background: Palette.control-background;
    }

    Rectangle {
        width: parent.width;
        height: parent.height;
        border-radius: parent.border-radius;
        background: touch.pressed ? #00000020 : touch.has-hover ? #ffffff18 : transparent;
    }
}
```

Move the pending `Path` into `drawing-layer` without changing its geometry or stroke. Replace the thumb's stroked checked outline with an outer fill and checked-only inset fill:

```slint
thumb := Rectangle {
    x: root.pending ? (root.width - self.width) / 2 : root.checked ? root.width - self.width - 4px : 4px;
    y: 4px;
    width: 12px;
    height: 12px;
    border-radius: self.height / 2;
    background: root.checked ? Palette.border : Palette.control-foreground;

    if root.checked : Rectangle {
        x: 1phx;
        y: 1phx;
        width: parent.width - 2phx;
        height: parent.height - 2phx;
        border-radius: parent.border-radius - 1phx;
        background: Palette.accent-foreground;
    }

    animate x {
        duration: 140ms;
        easing: ease-in-out;
    }
}
```

- [x] **Step 4: Compile the Slint component**

Run:

```powershell
cargo check -p dell-controller-tray
```

Expected: exit code 0 with no Slint type, binding-loop, or generated Rust errors. If the running tray process locks the executable during later build steps, stop only the process whose executable path resolves under this workspace, then restart it after verification.

- [x] **Step 5: Run workspace verification**

Run:

```powershell
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --bins --features slint/live-preview
git diff --check
```

Expected: every command exits with code 0; tests pass and Clippy reports no warnings.

- [ ] **Step 6: Validate the corrected rasterization**

Launch the rebuilt tray application at Windows 150% display scaling. Check focused and unfocused HDR and autostart switches with Windows Magnifier. Confirm the straight top/bottom border portions occupy one physical row and the right side is continuous. Also check checked, unchecked, hover, pressed, disabled, pending, and keyboard focus states for regressions.

- [ ] **Step 7: Preserve the working tree for review**

Do not create an implementation commit automatically because `AppSwitch.slint` is part of the broader uncommitted shared-switch feature. Report the exact file changed, verification results, and the remaining required manual 150% visual confirmation.

## Review Revision — 2026-07-19

Manual review superseded the original focus halo and 12×12 bordered-thumb snippets in Step 3:

- Use a 12×12 borderless thumb normally and shrink it to 10×10 on focus or press.
- Preserve the thumb centers at logical x coordinates 10px and 30px while its size changes.
- Use `Palette.accent-foreground` when checked and `Palette.control-foreground.with-alpha(0.6)` when unchecked; strengthen either state to 80% opacity during focus or press.
- Do not draw a focus outline around the rail.
- Move a fixed 16×12 transparent slot between endpoints. Center the thumb inside it, animate only the thumb width and color, and derive its height and local x/y from the animated width.
- Give hover priority over focus so a hovered thumb is 12×12; keep an active press at 10×10.
- During pending, remember the confirmed origin and expand the thumb into a 16×12 pill anchored toward the opposite side instead of centering it. Keep it at the origin through the confirmed snapshot; completion moves and contracts it on success or contracts it in place on failure.

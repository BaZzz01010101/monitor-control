# Passive HDR Information Tooltip Design

## Goal

Replace the HDR information icon's button-and-popup interaction with a passive Windows-style hover tooltip. Showing or hiding the hint must not change keyboard focus, disturb pointer hover, or create delayed reopen loops.

## Cause

The current icon is an accessible button backed by a `FocusScope`, while its hint is a Slint `PopupWindow`. Showing a `PopupWindow` moves focus out of the parent item and closing it restores that focus. The popup also creates a separate pointer surface. Combining those behaviors with hover-delay, click, Escape, and outside-click handling produces conflicting state transitions: hover can be lost and regained, focus moves between the icon and popup, and the hover timer can reopen a tooltip that was just closed.

## Interaction Design

The 16-pixel information icon becomes passive:

- Entering the icon starts one 350 ms timer.
- If the pointer is still over the icon when the timer fires, the hint appears.
- The hint remains visible while the pointer remains over the icon.
- Leaving the icon immediately stops the timer and hides the hint.
- Pointer movement within the icon does not restart the timer or hide the hint.
- Clicks, keyboard focus, Space, Enter, Escape, and accessibility actions have no tooltip behavior.
- The icon uses the normal arrow cursor and has no focus, pressed, button, or accessibility styling.

## Rendering Design

`PopupWindow` is removed. The icon continues to own the hover timer and reports its absolute window position when the hint should appear. `HeaderCard` forwards show and hide requests to `MainWindow`.

`MainWindow` renders the hint as a non-interactive rectangle after the main content, anchored below the reported icon position. Rendering it last keeps it above cards without changing layout. Because it is part of the existing window and contains no `TouchArea` or `FocusScope`, it neither intercepts pointer input nor changes keyboard focus.

The existing tooltip text, size, colors, border, shadow, and approximate placement are preserved. No accessibility description is added to the icon, HDR label, or switch.

## Alternatives Considered

Keeping `PopupWindow` with a different close policy would still move focus and create a separate pointer surface. Rendering an oversized tooltip rectangle inside the small icon component would be simpler, but later sibling cards can paint over it because Slint uses tree order rather than a general z-index. A final overlay in `MainWindow` avoids both problems.

## Validation

No source-text or pixel-snapshot tests will be added. Manual validation covers stable delayed hover, movement within the icon, clicking before and after display, tab navigation skipping the icon, Space and Escape having no effect, and repeated leave/re-enter behavior.

Automated verification remains:

- `cargo fmt --all -- --check`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --bins --features slint/live-preview`

# AppSwitch Physical-Pixel Rendering Design

## Goal

Make the shared switch outline crisp at Windows fractional display scaling, especially 150%, without changing its logical size, state behavior, accessibility, or pending animation.

Success means the straight top and bottom edges occupy one physical pixel, the right edge has no visible seam, and keyboard focus is visible through slider-style thumb feedback. Rounded corners remain anti-aliased.

## Cause

`AppSwitch` currently draws the rail and focus indication with stroked rounded rectangles. A one-physical-pixel stroke is centered on the path. If layout places the switch at a fractional physical coordinate, the straight edges cover adjacent pixel rows. The rounded path can also expose a one-pixel closure seam at the right side.

Changing `1px` to `1phx` corrected the thickness but did not align the path origin or eliminate the stroked-path seam.

## Rendering Design

The component keeps its 40×20 logical geometry and public interface. A private drawing layer offsets its contents so its absolute origin is rounded to the nearest physical pixel. The root item remains unchanged for layout, focus, accessibility, and pointer input.

Stroked capsule outlines are replaced with layered filled shapes:

- The unchecked rail uses a border-colored outer capsule and a control-background inner capsule inset by `1phx`.
- The checked rail remains a solid accent capsule.
- The thumb is a 12×12 borderless circle. It uses the accent foreground when checked and a 60%-opacity control foreground when unchecked.
- Focus and press keep the thumb centered while shrinking it to 10×10 and strengthening its state-appropriate color, matching the interaction pattern of Slint's Fluent slider. The rail receives no focus outline.
- Hover expands a focused thumb back to the normal 12×12 size; an active press remains 10×10.
- A fixed 16×12 positioning slot moves between switch endpoints. The circle is centered inside it and animates only its width; height and local position derive from that width so the size transition cannot drift.
- Pending remembers the last confirmed endpoint and expands the thumb toward the requested side into a 16×12 pill. Success moves and contracts it at the confirmed destination; failure contracts it at the origin.

Filled capsules do not have a stroke closure and their straight boundaries can align with the physical pixel grid. The pending dash remains a 2-pixel stroked `Path`; it is an animated segment rather than a closed border and is not involved in the reported defect.

## Preserved Behavior

- The switch remains 40×20 logical pixels with a 12×12 thumb.
- Checked, disabled, hover, pressed, focus, keyboard, pointer, and accessibility behavior remain unchanged.
- Pending keeps priority over disabled styling, retains the thumb at its confirmed endpoint as a directional pill, and retains the moving perimeter segment.
- HDR remains confirmed-state-only, and autostart remains immediate.
- No controller, worker, persistence, or bridge changes are required.

## Alternatives Considered

Only snapping the existing strokes would improve the horizontal blur but could retain the right-side path seam. Increasing borders to two physical pixels would hide rasterization defects but make the switch visually heavier. Changing the application renderer would expand scope and would not guarantee identical results across supported Slint backends.

## Validation

No source-text or pixel-snapshot tests will be added. The renderer-specific defect is validated manually on Windows at 150% display scaling, checking both switch instances and focus states. Automated verification remains:

- `cargo fmt --all -- --check`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --bins --features slint/live-preview`

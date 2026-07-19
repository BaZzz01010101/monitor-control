# Shared Switch with HDR Pending Animation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the separate HDR and autostart switches with one accessible switch that visualizes asynchronous HDR updates without making autostart asynchronous.

**Architecture:** A reusable Slint `AppSwitch` owns normal and pending visuals. `AppController` exposes the existing private HDR pending state through `UiState`, and `UiBridge` propagates it to `MainWindow`; existing worker event ordering remains authoritative for the confirmed checked state.

**Tech Stack:** Rust, Slint 1.16.1, Windows 11.

## Global Constraints

- Keep HDR state non-optimistic and preserve snapshot-before-completion ordering.
- Keep autostart immediate and never pending.
- Do not change the HDR worker protocol, persistence, settings schema, or dependencies.
- Do not copy Slint widget source or use its private palette APIs.
- Do not add tests that inspect hand-written source files as text or use pixel snapshots.

---

## Implementation Changes

- [x] Create `AppSwitch.slint` with `checked`, `enabled`, `pending`, and `accessible-name` input properties and a `toggled(bool)` callback.
- [x] Independently implement Fluent-like 40x20 geometry with Slint's public `Palette`, a 12x12 thumb, and accessible pointer, focus, Space/Enter, and default-action behavior.
- [x] Expand the thumb toward the requested side into a 16x12 pill at the confirmed endpoint while pending, then settle it on completion; animate an 8x2 rounded accent segment clockwise around the inset pill perimeter at approximately 900 ms per loop.
- [x] Give pending styling priority over disabled styling and reject all input while disabled or pending.
- [x] Replace the HDR and autostart switches with `AppSwitch`; pass controller-backed pending state only to HDR.
- [x] Add `UiState::hdr_pending` and a matching `MainWindow` property; set it when a request is accepted and clear it only on `HdrUpdateFinished`.
- [x] Keep the existing status text, worker protocol, and confirmed-state behavior unchanged.

## Test Plan

- [x] Add controller coverage for request acceptance, duplicate gating, snapshot ordering, success, failure, disappearance, and autostart isolation.
- [x] Add bridge coverage for `hdr_pending`, requested HDR callback values, and existing checked/enabled state.
- [ ] Manually validate identical switch appearance, pointer/keyboard/focus/accessibility behavior, pending animation, success settling, failure rollback, and palette behavior.
- [x] Run `cargo fmt --all -- --check`.
- [x] Run `cargo test --workspace`.
- [x] Run `cargo clippy --workspace --all-targets -- -D warnings`.
- [x] Run `cargo build --workspace --bins --features slint/live-preview`.

## Assumptions

- Windows 11 remains the only target.
- The pending visual does not need a requested-target property because the confirmed snapshot determines the final endpoint.
- No dependency upgrade or migration is required.

#[path = "../src/tray_events.rs"]
mod tray_events;

use tray_events::{TrayAction, TrayClickTracker, DEFAULT_DOUBLE_CLICK_MS};

#[test]
fn first_tray_left_click_does_not_open_window() {
    let mut tracker = TrayClickTracker::default();

    assert_eq!(tracker.left_up_at(1_000), None);
}

#[test]
fn second_tray_left_click_inside_double_click_window_opens_window() {
    let mut tracker = TrayClickTracker::default();

    assert_eq!(tracker.left_up_at(1_000), None);
    assert_eq!(
        tracker.left_up_at(1_000 + DEFAULT_DOUBLE_CLICK_MS - 1),
        Some(TrayAction::OpenWindow)
    );
}

#[test]
fn slow_second_tray_left_click_starts_a_new_double_click_window() {
    let mut tracker = TrayClickTracker::default();

    assert_eq!(tracker.left_up_at(1_000), None);
    assert_eq!(
        tracker.left_up_at(1_000 + DEFAULT_DOUBLE_CLICK_MS + 1),
        None
    );
    assert_eq!(
        tracker.left_up_at(1_000 + DEFAULT_DOUBLE_CLICK_MS + 20),
        Some(TrayAction::OpenWindow)
    );
}

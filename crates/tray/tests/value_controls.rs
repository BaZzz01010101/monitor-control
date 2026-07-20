#[path = "../src/value_controls.rs"]
mod value_controls;

use value_controls::{
    adjusted_feature_value, key_feature_delta, parse_feature_value, wheel_feature_delta,
    WriteThrottle,
};

#[test]
fn wheel_delta_maps_to_single_steps() {
    assert_eq!(wheel_feature_delta(120), Some(1));
    assert_eq!(wheel_feature_delta(-120), Some(-1));
    assert_eq!(wheel_feature_delta(0), None);
}

#[test]
fn up_down_keys_map_to_single_steps() {
    assert_eq!(key_feature_delta(0x26), Some(1));
    assert_eq!(key_feature_delta(0x28), Some(-1));
    assert_eq!(key_feature_delta(0x25), None);
}

#[test]
fn adjusted_values_are_clamped_to_feature_range() {
    assert_eq!(adjusted_feature_value(50, 100, 1), 51);
    assert_eq!(adjusted_feature_value(0, 100, -1), 0);
    assert_eq!(adjusted_feature_value(100, 100, 1), 100);
}

#[test]
fn edited_values_parse_and_clamp() {
    assert_eq!(parse_feature_value("75", 100), Some(75));
    assert_eq!(parse_feature_value(" 125 ", 100), Some(100));
    assert_eq!(parse_feature_value("", 100), None);
    assert_eq!(parse_feature_value("nope", 100), None);
}

#[test]
fn first_live_update_sends_immediately() {
    let mut throttle = WriteThrottle::new(140);

    assert_eq!(throttle.schedule(1_000, 50), Some(50));
    assert!(!throttle.has_pending());
}

#[test]
fn repeated_updates_inside_the_window_coalesce_to_the_last_value() {
    let mut throttle = WriteThrottle::new(140);

    assert_eq!(throttle.schedule(1_000, 50), Some(50));
    assert_eq!(throttle.schedule(1_040, 51), None);
    assert_eq!(throttle.schedule(1_090, 53), None);
    assert!(throttle.has_pending());

    assert_eq!(throttle.tick(1_139), None);
    assert_eq!(throttle.tick(1_140), Some(53));
    assert!(!throttle.has_pending());
}

#[test]
fn force_send_flushes_immediately_and_clears_any_pending_value() {
    let mut throttle = WriteThrottle::new(140);

    assert_eq!(throttle.schedule(1_000, 50), Some(50));
    assert_eq!(throttle.schedule(1_040, 51), None);
    assert!(throttle.has_pending());

    assert_eq!(throttle.force(1_060, 52), 52);
    assert!(!throttle.has_pending());
    assert_eq!(throttle.tick(1_200), None);
}

#[test]
fn trailing_send_opens_a_new_leading_window() {
    let mut throttle = WriteThrottle::new(140);

    assert_eq!(throttle.schedule(1_000, 50), Some(50));
    assert_eq!(throttle.schedule(1_030, 51), None);
    assert_eq!(throttle.tick(1_140), Some(51));
    assert_eq!(throttle.schedule(1_141, 52), None);
    assert_eq!(throttle.tick(1_280), Some(52));
}

#[test]
fn reset_discards_pending_value_and_opens_a_new_leading_window() {
    let mut throttle = WriteThrottle::new(140);

    assert_eq!(throttle.schedule(1_000, 50), Some(50));
    assert_eq!(throttle.schedule(1_040, 51), None);
    assert!(throttle.has_pending());

    throttle.reset();

    assert!(!throttle.has_pending());
    assert_eq!(throttle.tick(1_200), None);
    assert_eq!(throttle.schedule(1_050, 52), Some(52));
}

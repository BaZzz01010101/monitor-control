#[path = "../src/debug_ui.rs"]
mod debug_ui;

use std::path::Path;

use debug_ui::{debug_ui_output_path, launch_mode_from_args, LaunchMode};

#[test]
fn launch_mode_defaults_to_normal() {
    assert_eq!(
        launch_mode_from_args(std::iter::empty::<&str>()),
        LaunchMode::Normal
    );
    assert_eq!(
        launch_mode_from_args(["monitors", "--help"]),
        LaunchMode::Normal
    );
}

#[test]
fn launch_mode_recognizes_debug_ui_token() {
    assert_eq!(launch_mode_from_args(["debug-ui"]), LaunchMode::DebugUi);
    assert_eq!(
        launch_mode_from_args(["something", "debug-ui"]),
        LaunchMode::DebugUi
    );
}

#[test]
fn debug_ui_output_path_is_written_into_dot_tmp() {
    let path = debug_ui_output_path(Path::new("D:/DEV/dell-controller"), 1234);

    assert_eq!(
        path,
        Path::new("D:/DEV/dell-controller/.tmp/dell-controller-tray-ui-1234.bmp")
    );
}

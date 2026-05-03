use dell_controller_tray::app_controller::{
    AppController, ControllerEffect, FeatureId, FeatureSnapshot, InputRoute, MonitorSnapshot,
    UiAction, WorkerEvent, WorkerRequest, BRIGHTNESS_CODE, CONTRAST_CODE, INPUT_DP_VALUE,
    INPUT_HDMI_VALUE, INPUT_USB_C_VALUE,
};

fn sample_snapshot() -> MonitorSnapshot {
    MonitorSnapshot {
        monitor_title: "Dell U4025QW".into(),
        input_summary: "DP".into(),
        hdr_status: "Windows HDR: On".into(),
        brightness: FeatureSnapshot {
            value: 50,
            maximum: 100,
            available: true,
        },
        contrast: FeatureSnapshot {
            value: 80,
            maximum: 100,
            available: true,
        },
        selected_input: InputRoute::DisplayPort,
        input_enabled: true,
        has_monitor: true,
    }
}

#[test]
fn open_window_requests_show_and_initial_refreshes() {
    let mut controller = AppController::default();

    let effects = controller.handle_action(UiAction::OpenWindow, 1_000);

    assert_eq!(
        effects,
        vec![
            ControllerEffect::ShowWindow,
            ControllerEffect::Worker(WorkerRequest::RefreshAll),
            ControllerEffect::Worker(WorkerRequest::RefreshAutostart),
        ]
    );
}

#[test]
fn preview_feature_uses_leading_and_trailing_writes() {
    let mut controller = AppController::default();
    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()));

    let first = controller.handle_action(
        UiAction::PreviewFeature {
            feature: FeatureId::Brightness,
            value: 51,
        },
        1_000,
    );
    assert_eq!(
        first,
        vec![ControllerEffect::Worker(WorkerRequest::WriteFeature {
            code: BRIGHTNESS_CODE,
            value: 51,
        })]
    );

    let second = controller.handle_action(
        UiAction::PreviewFeature {
            feature: FeatureId::Brightness,
            value: 53,
        },
        1_040,
    );
    assert!(second.is_empty());

    let flushed = controller.flush_pending(1_140);
    assert_eq!(
        flushed,
        vec![ControllerEffect::Worker(WorkerRequest::WriteFeature {
            code: BRIGHTNESS_CODE,
            value: 53,
        })]
    );
}

#[test]
fn commit_feature_forces_immediate_write() {
    let mut controller = AppController::default();
    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()));

    controller.handle_action(
        UiAction::PreviewFeature {
            feature: FeatureId::Contrast,
            value: 70,
        },
        1_000,
    );

    let committed = controller.handle_action(
        UiAction::CommitFeature {
            feature: FeatureId::Contrast,
            value: 68,
        },
        1_050,
    );

    assert_eq!(
        committed,
        vec![ControllerEffect::Worker(WorkerRequest::WriteFeature {
            code: CONTRAST_CODE,
            value: 68,
        })]
    );
    assert!(controller.flush_pending(1_200).is_empty());
}

#[test]
fn worker_snapshot_populates_the_ui_state() {
    let mut controller = AppController::default();

    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()));
    let state = controller.ui_state();

    assert_eq!(state.monitor_title, "Dell U4025QW");
    assert_eq!(state.input_summary, "DP");
    assert_eq!(state.hdr_status, "Windows HDR: On");
    assert_eq!(state.brightness.value, 50);
    assert_eq!(state.brightness.text, "50");
    assert!(state.brightness.enabled);
    assert_eq!(state.contrast.value, 80);
    assert_eq!(state.selected_input, InputRoute::DisplayPort);
    assert!(state.input_enabled);
    assert!(!state.no_monitor);
}

#[test]
fn no_monitor_snapshot_disables_controls_and_resets_input() {
    let mut controller = AppController::default();
    let mut snapshot = sample_snapshot();
    snapshot.has_monitor = false;
    snapshot.input_enabled = false;
    snapshot.input_summary = "Input".into();
    snapshot.hdr_status = "Windows HDR: unavailable".into();
    snapshot.selected_input = InputRoute::None;
    snapshot.brightness.available = false;
    snapshot.contrast.available = false;

    controller.apply_worker_event(WorkerEvent::Snapshot(snapshot));
    let state = controller.ui_state();

    assert!(state.no_monitor);
    assert!(!state.brightness.enabled);
    assert!(!state.contrast.enabled);
    assert!(!state.input_enabled);
    assert_eq!(state.selected_input, InputRoute::None);
}

#[test]
fn input_actions_map_to_vcp_values() {
    let mut controller = AppController::default();
    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()));

    let usb_c = controller.handle_action(UiAction::SetInput(InputRoute::UsbC), 1_000);
    let dp = controller.handle_action(UiAction::SetInput(InputRoute::DisplayPort), 1_001);
    let hdmi = controller.handle_action(UiAction::SetInput(InputRoute::Hdmi), 1_002);

    assert_eq!(
        usb_c,
        vec![ControllerEffect::Worker(WorkerRequest::SetInput {
            value: INPUT_USB_C_VALUE,
        })]
    );
    assert_eq!(
        dp,
        vec![ControllerEffect::Worker(WorkerRequest::SetInput {
            value: INPUT_DP_VALUE,
        })]
    );
    assert_eq!(
        hdmi,
        vec![ControllerEffect::Worker(WorkerRequest::SetInput {
            value: INPUT_HDMI_VALUE,
        })]
    );
}

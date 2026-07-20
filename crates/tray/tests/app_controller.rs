use dell_controller_tray::app_controller::{
    AppController, ControllerEffect, FeatureId, FeatureSnapshot, InputRoute, MonitorChoice,
    MonitorSnapshot, PictureBackend, ShortcutTarget, UiAction, UiPane, WorkerEvent, WorkerRequest,
    BRIGHTNESS_CODE, CONTRAST_CODE, INPUT_HDMI_VALUE, INPUT_USB_C_VALUE,
};
use dell_controller_tray::persistence::PersistedSettings;

fn sample_snapshot() -> MonitorSnapshot {
    MonitorSnapshot {
        monitor_title: "Dell U4025QW".into(),
        input_summary: "DP".into(),
        hdr_status: "Windows HDR: On".into(),
        diagnostic_status: String::new(),
        monitor_choices: vec![MonitorChoice {
            key: "dell-u4025qw|u4025qw|0".into(),
            title: "Dell U4025QW".into(),
        }],
        selected_monitor_key: "dell-u4025qw|u4025qw|0".into(),
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
        hdr_enabled: true,
        hdr_available: true,
        hdr_active: false,
        picture_backend: PictureBackend::Ddc,
        advanced_color_monitor: None,
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
        ]
    );
}

#[test]
fn settings_actions_switch_the_active_pane() {
    let mut controller = AppController::default();

    controller.handle_action(UiAction::OpenSettings, 1_000);
    assert_eq!(controller.ui_state().active_pane, UiPane::Settings);

    controller.handle_action(UiAction::CloseSettings, 1_100);
    assert_eq!(controller.ui_state().active_pane, UiPane::Main);
}

#[test]
fn opening_the_window_resets_the_settings_pane_to_main() {
    let mut controller = AppController::default();

    controller.handle_action(UiAction::OpenSettings, 1_000);
    controller.handle_action(UiAction::HideWindow, 1_050);
    controller.handle_action(UiAction::OpenWindow, 1_100);

    assert_eq!(controller.ui_state().active_pane, UiPane::Main);
}

#[test]
fn opening_the_window_while_already_visible_preserves_the_active_pane() {
    let mut controller = AppController::default();

    controller.handle_action(UiAction::OpenWindow, 900);
    controller.handle_action(UiAction::OpenSettings, 1_000);

    let effects = controller.handle_action(UiAction::OpenWindow, 1_100);

    assert_eq!(controller.ui_state().active_pane, UiPane::Settings);
    assert!(effects.is_empty());
}

#[test]
fn autostart_worker_event_updates_ui_state() {
    let mut controller = AppController::default();

    controller.apply_worker_event(
        WorkerEvent::AutostartState {
            enabled: true,
            status: String::new(),
        },
        1_000,
    );

    assert!(controller.ui_state().autostart_enabled);
}

#[test]
fn toggle_autostart_updates_ui_state_immediately() {
    let mut controller = AppController::default();

    controller.apply_worker_event(
        WorkerEvent::AutostartState {
            enabled: false,
            status: String::new(),
        },
        900,
    );

    let effects = controller.handle_action(UiAction::ToggleAutostart, 1_000);

    assert_eq!(
        effects,
        vec![ControllerEffect::Worker(WorkerRequest::SetAutostart {
            enabled: true,
            quiet: false,
        })]
    );
    assert!(controller.ui_state().autostart_enabled);
    assert!(!controller.ui_state().hdr_pending);
}

#[test]
fn hdr_toggle_waits_for_worker_confirmation() {
    let mut controller = AppController::default();
    let mut snapshot = sample_snapshot();
    snapshot.hdr_enabled = false;
    snapshot.hdr_available = true;
    snapshot.hdr_status = "Windows HDR: Off".into();
    controller.apply_worker_event(WorkerEvent::Snapshot(snapshot), 900);

    let effects = controller.handle_action(UiAction::ToggleHdr(true), 1_000);

    assert_eq!(
        effects,
        vec![
            ControllerEffect::Worker(WorkerRequest::CancelPictureWrites),
            ControllerEffect::Worker(WorkerRequest::SetHdr {
                enabled: true,
                use_cached_ddc_values: false,
            }),
        ]
    );
    assert!(!controller.ui_state().hdr_enabled);
    assert!(!controller.ui_state().hdr_toggle_enabled);
    assert!(controller.ui_state().hdr_pending);
    assert!(!controller.ui_state().brightness.enabled);
    assert!(!controller.ui_state().contrast.enabled);
    assert!(controller
        .handle_action(UiAction::ToggleHdr(true), 1_010)
        .is_empty());

    let mut confirmed = sample_snapshot();
    confirmed.hdr_enabled = true;
    confirmed.hdr_available = true;
    controller.apply_worker_event(WorkerEvent::Snapshot(confirmed), 1_020);

    assert!(controller.ui_state().hdr_enabled);
    assert!(!controller.ui_state().hdr_toggle_enabled);
    assert!(controller.ui_state().hdr_pending);

    controller.apply_worker_event(
        WorkerEvent::HdrUpdateFinished {
            enabled: true,
            error: None,
            used_cached_ddc_values: false,
        },
        1_030,
    );

    assert!(controller.ui_state().hdr_enabled);
    assert!(controller.ui_state().hdr_toggle_enabled);
    assert!(!controller.ui_state().hdr_pending);
    assert_eq!(controller.ui_state().status_text, "Windows HDR enabled");
}

#[test]
fn failed_hdr_toggle_restores_confirmed_state_and_reports_error() {
    let mut controller = AppController::default();
    let mut snapshot = sample_snapshot();
    snapshot.hdr_enabled = false;
    snapshot.hdr_available = true;
    controller.apply_worker_event(WorkerEvent::Snapshot(snapshot.clone()), 900);
    controller.handle_action(UiAction::ToggleHdr(true), 1_000);
    assert!(controller.ui_state().hdr_pending);

    controller.apply_worker_event(WorkerEvent::Snapshot(snapshot), 1_010);
    assert!(controller.ui_state().hdr_pending);
    controller.apply_worker_event(
        WorkerEvent::HdrUpdateFinished {
            enabled: true,
            error: Some("HDR update failed: Windows rejected the change".into()),
            used_cached_ddc_values: false,
        },
        1_020,
    );

    assert!(!controller.ui_state().hdr_enabled);
    assert!(controller.ui_state().hdr_toggle_enabled);
    assert!(!controller.ui_state().hdr_pending);
    assert_eq!(
        controller.ui_state().status_text,
        "HDR update failed: Windows rejected the change"
    );
}

#[test]
fn unavailable_hdr_toggle_is_ignored() {
    let mut controller = AppController::default();

    let effects = controller.handle_action(UiAction::ToggleHdr(true), 1_000);

    assert!(effects.is_empty());
    assert!(!controller.ui_state().hdr_enabled);
    assert!(!controller.ui_state().hdr_pending);
}

#[test]
fn disappearing_monitor_keeps_hdr_pending_until_completion_and_unavailable_afterward() {
    let mut controller = AppController::default();
    let mut available = sample_snapshot();
    available.hdr_enabled = false;
    available.hdr_available = true;
    controller.apply_worker_event(WorkerEvent::Snapshot(available), 900);
    controller.handle_action(UiAction::ToggleHdr(true), 1_000);

    let mut disappeared = sample_snapshot();
    disappeared.has_monitor = false;
    disappeared.hdr_enabled = false;
    disappeared.hdr_available = false;
    disappeared.hdr_status = "Windows HDR: unavailable".into();
    controller.apply_worker_event(WorkerEvent::Snapshot(disappeared), 1_010);

    assert!(controller.ui_state().hdr_pending);
    assert!(!controller.ui_state().hdr_toggle_enabled);

    controller.apply_worker_event(
        WorkerEvent::HdrUpdateFinished {
            enabled: true,
            error: Some("HDR update failed: selected monitor disappeared".into()),
            used_cached_ddc_values: false,
        },
        1_020,
    );

    assert!(!controller.ui_state().hdr_pending);
    assert!(!controller.ui_state().hdr_toggle_enabled);
}

#[test]
fn hydrating_persisted_settings_seeds_durable_ui_values() {
    let mut controller = AppController::default();

    controller.hydrate_persisted_settings(&PersistedSettings {
        autostart_enabled: true,
        selected_monitor_key: "dell-u4025qw|u4025qw|0".into(),
        tb_shortcut: "Ctrl+Alt+T".into(),
        dp_shortcut: "Ctrl+Alt+D".into(),
        hdmi_shortcut: "Ctrl+Alt+H".into(),
    });

    let state = controller.ui_state();
    assert!(state.autostart_enabled);
    assert_eq!(state.tb_shortcut.value, "Ctrl+Alt+T");
    assert_eq!(state.dp_shortcut.value, "Ctrl+Alt+D");
    assert_eq!(state.hdmi_shortcut.value, "Ctrl+Alt+H");
    assert_eq!(state.selected_monitor_key, "dell-u4025qw|u4025qw|0");
}

#[test]
fn selecting_a_monitor_updates_durable_state_and_worker_selection() {
    let mut controller = AppController::default();

    let effects = controller.handle_action(
        UiAction::SelectMonitor("dell-u4025qw|u4025qw|1".into()),
        1_000,
    );

    assert_eq!(
        effects,
        vec![
            ControllerEffect::Worker(WorkerRequest::CancelPictureWrites),
            ControllerEffect::Worker(WorkerRequest::SelectMonitor {
                key: "dell-u4025qw|u4025qw|1".into(),
            }),
        ]
    );
    assert_eq!(
        controller.persisted_settings().selected_monitor_key,
        "dell-u4025qw|u4025qw|1"
    );
}

#[test]
fn shortcut_capture_commits_and_focus_loss_clears_preview_state() {
    let mut controller = AppController::default();

    controller.handle_action(
        UiAction::PreviewShortcut {
            target: ShortcutTarget::Hdmi,
            preview: "Ctrl+".into(),
        },
        1_010,
    );

    let previewing = controller.ui_state();
    assert_eq!(previewing.hdmi_shortcut.value, "None");
    assert_eq!(previewing.hdmi_shortcut.preview, "Ctrl+");
    assert!(previewing.hdmi_shortcut.awaiting_final_key);

    controller.apply_shortcut_registration_success(ShortcutTarget::Hdmi, "Ctrl+Q".into(), None);

    let committed = controller.ui_state();
    assert_eq!(committed.hdmi_shortcut.value, "Ctrl+Q");
    assert_eq!(committed.hdmi_shortcut.preview, "");
    assert!(!committed.hdmi_shortcut.awaiting_final_key);

    controller.handle_action(
        UiAction::PreviewShortcut {
            target: ShortcutTarget::Hdmi,
            preview: "Shift+".into(),
        },
        1_040,
    );
    controller.handle_action(
        UiAction::DeactivateShortcutCapture(ShortcutTarget::Hdmi),
        1_050,
    );

    let deactivated = controller.ui_state();
    assert_eq!(deactivated.hdmi_shortcut.value, "Ctrl+Q");
    assert_eq!(deactivated.hdmi_shortcut.preview, "");
    assert!(!deactivated.hdmi_shortcut.awaiting_final_key);
}

#[test]
fn successful_shortcut_registration_resets_the_displaced_shortcut() {
    let mut controller = AppController::default();

    controller.apply_shortcut_registration_success(
        ShortcutTarget::DisplayPort,
        "Ctrl+Alt+D".into(),
        None,
    );
    controller.apply_shortcut_registration_success(ShortcutTarget::Hdmi, "Ctrl+Alt+H".into(), None);

    controller.apply_shortcut_registration_success(
        ShortcutTarget::UsbC,
        "Ctrl+Alt+H".into(),
        Some(ShortcutTarget::Hdmi),
    );

    let state = controller.ui_state();
    assert_eq!(state.tb_shortcut.value, "Ctrl+Alt+H");
    assert_eq!(state.dp_shortcut.value, "Ctrl+Alt+D");
    assert_eq!(state.hdmi_shortcut.value, "None");
    assert_eq!(state.tb_shortcut.error_text, "");
    assert_eq!(state.hdmi_shortcut.error_text, "");
}

#[test]
fn failed_shortcut_registration_restores_the_previous_value_and_sets_error() {
    let mut controller = AppController::default();

    controller.apply_shortcut_registration_success(
        ShortcutTarget::DisplayPort,
        "Ctrl+Alt+D".into(),
        None,
    );
    controller.handle_action(
        UiAction::PreviewShortcut {
            target: ShortcutTarget::DisplayPort,
            preview: "Ctrl+Alt+".into(),
        },
        1_000,
    );

    controller.apply_shortcut_registration_failure(
        ShortcutTarget::DisplayPort,
        "Ctrl+Alt+D".into(),
        "That shortcut is already in use.".into(),
    );

    let state = controller.ui_state();
    assert_eq!(state.dp_shortcut.value, "Ctrl+Alt+D");
    assert_eq!(state.dp_shortcut.preview, "");
    assert!(!state.dp_shortcut.awaiting_final_key);
    assert_eq!(
        state.dp_shortcut.error_text,
        "That shortcut is already in use."
    );
}

#[test]
fn preview_feature_uses_leading_and_trailing_writes() {
    let mut controller = AppController::default();
    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()), 900);

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
fn hdr_active_brightness_uses_windows_sdr_backend_and_contrast_is_disabled() {
    let mut controller = AppController::default();
    let mut snapshot = sample_snapshot();
    snapshot.hdr_active = true;
    snapshot.picture_backend = PictureBackend::WindowsSdr;
    snapshot.brightness.value = 40;
    snapshot.contrast.available = false;
    controller.apply_worker_event(WorkerEvent::Snapshot(snapshot), 900);

    assert!(controller.ui_state().brightness.enabled);
    assert!(!controller.ui_state().contrast.enabled);
    assert_eq!(
        controller.handle_action(
            UiAction::PreviewFeature {
                feature: FeatureId::Brightness,
                value: 55,
            },
            1_000,
        ),
        vec![ControllerEffect::Worker(
            WorkerRequest::SetSdrContentBrightness { percent: 55 }
        )]
    );
    assert!(controller
        .handle_action(
            UiAction::PreviewFeature {
                feature: FeatureId::Contrast,
                value: 70,
            },
            1_010,
        )
        .is_empty());
}

#[test]
fn hdr_active_snapshot_preserves_cached_ddc_contrast() {
    let mut controller = AppController::default();
    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()), 900);

    let mut hdr = sample_snapshot();
    hdr.hdr_active = true;
    hdr.picture_backend = PictureBackend::WindowsSdr;
    hdr.brightness.value = 35;
    hdr.contrast.available = false;
    controller.apply_worker_event(WorkerEvent::Snapshot(hdr), 1_000);

    let state = controller.ui_state();
    assert_eq!(state.brightness.value, 35);
    assert!(state.brightness.enabled);
    assert_eq!(state.contrast.value, 80);
    assert_eq!(state.contrast.text, "80");
    assert!(!state.contrast.enabled);
}

#[test]
fn disabling_hdr_requests_cached_restore_only_for_a_complete_selected_monitor_cache() {
    let mut cached = AppController::default();
    cached.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()), 900);
    let mut hdr = sample_snapshot();
    hdr.hdr_active = true;
    hdr.picture_backend = PictureBackend::WindowsSdr;
    hdr.brightness.value = 35;
    hdr.contrast.available = false;
    cached.apply_worker_event(WorkerEvent::Snapshot(hdr.clone()), 950);

    assert_eq!(
        cached.handle_action(UiAction::ToggleHdr(false), 1_000),
        vec![
            ControllerEffect::Worker(WorkerRequest::CancelPictureWrites),
            ControllerEffect::Worker(WorkerRequest::SetHdr {
                enabled: false,
                use_cached_ddc_values: true,
            }),
        ]
    );

    let mut cold = AppController::default();
    cold.apply_worker_event(WorkerEvent::Snapshot(hdr), 900);

    assert_eq!(
        cold.handle_action(UiAction::ToggleHdr(false), 1_000),
        vec![
            ControllerEffect::Worker(WorkerRequest::CancelPictureWrites),
            ControllerEffect::Worker(WorkerRequest::SetHdr {
                enabled: false,
                use_cached_ddc_values: false,
            }),
        ]
    );
}

fn complete_cached_hdr_off_transition(controller: &mut AppController) {
    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()), 900);

    let mut hdr = sample_snapshot();
    hdr.hdr_active = true;
    hdr.picture_backend = PictureBackend::WindowsSdr;
    hdr.brightness.value = 35;
    hdr.contrast.available = false;
    controller.apply_worker_event(WorkerEvent::Snapshot(hdr), 950);
    controller.handle_action(UiAction::ToggleHdr(false), 1_000);

    let mut deferred = sample_snapshot();
    deferred.hdr_enabled = false;
    deferred.hdr_active = false;
    deferred.picture_backend = PictureBackend::Ddc;
    deferred.brightness.available = false;
    deferred.contrast.available = false;
    controller.apply_worker_event(WorkerEvent::Snapshot(deferred), 1_010);
    controller.apply_worker_event(
        WorkerEvent::HdrUpdateFinished {
            enabled: false,
            error: None,
            used_cached_ddc_values: true,
        },
        1_020,
    );
}

#[test]
fn cached_hdr_off_completion_enables_ddc_controls_before_background_reconciliation() {
    let mut controller = AppController::default();
    complete_cached_hdr_off_transition(&mut controller);

    let state = controller.ui_state();
    assert!(!state.hdr_pending);
    assert_eq!(state.brightness.value, 50);
    assert_eq!(state.brightness.text, "50");
    assert!(state.brightness.enabled);
    assert_eq!(state.contrast.value, 80);
    assert_eq!(state.contrast.text, "80");
    assert!(state.contrast.enabled);
}

#[test]
fn background_reconciliation_updates_untouched_cached_values() {
    let mut controller = AppController::default();
    complete_cached_hdr_off_transition(&mut controller);

    let mut refreshed = sample_snapshot();
    refreshed.hdr_enabled = false;
    refreshed.brightness.value = 57;
    refreshed.contrast.value = 77;
    controller.apply_worker_event(WorkerEvent::Snapshot(refreshed), 2_000);

    let state = controller.ui_state();
    assert_eq!(state.brightness.value, 57);
    assert_eq!(state.contrast.value, 77);
    assert!(state.brightness.enabled);
    assert!(state.contrast.enabled);
}

#[test]
fn user_edit_wins_over_the_in_flight_restore_snapshot() {
    let mut controller = AppController::default();
    complete_cached_hdr_off_transition(&mut controller);

    assert_eq!(
        controller.handle_action(
            UiAction::PreviewFeature {
                feature: FeatureId::Brightness,
                value: 63,
            },
            1_030,
        ),
        vec![ControllerEffect::Worker(WorkerRequest::WriteFeature {
            code: BRIGHTNESS_CODE,
            value: 63,
        })]
    );

    let mut stale = sample_snapshot();
    stale.hdr_enabled = false;
    stale.brightness.value = 45;
    stale.contrast.value = 75;
    controller.apply_worker_event(WorkerEvent::Snapshot(stale), 5_000);

    let state = controller.ui_state();
    assert_eq!(state.brightness.value, 63);
    assert_eq!(state.contrast.value, 75);
}

#[test]
fn ddc_picture_cache_is_scoped_by_monitor_key() {
    let mut controller = AppController::default();
    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()), 900);
    controller.handle_action(UiAction::SelectMonitor("second-monitor".into()), 950);

    let mut second_hdr = sample_snapshot();
    second_hdr.selected_monitor_key = "second-monitor".into();
    second_hdr.monitor_choices.push(MonitorChoice {
        key: "second-monitor".into(),
        title: "Second monitor".into(),
    });
    second_hdr.hdr_active = true;
    second_hdr.picture_backend = PictureBackend::WindowsSdr;
    second_hdr.brightness.value = 42;
    second_hdr.contrast.available = false;
    controller.apply_worker_event(WorkerEvent::Snapshot(second_hdr), 1_000);

    let state = controller.ui_state();
    assert_eq!(state.contrast.text, "n/a");
    assert!(!state.contrast.enabled);
    assert_eq!(
        controller.handle_action(UiAction::ToggleHdr(false), 1_100),
        vec![
            ControllerEffect::Worker(WorkerRequest::CancelPictureWrites),
            ControllerEffect::Worker(WorkerRequest::SetHdr {
                enabled: false,
                use_cached_ddc_values: false,
            }),
        ]
    );
}

#[test]
fn hdr_transition_cancels_a_trailing_picture_write() {
    let mut controller = AppController::default();
    let mut snapshot = sample_snapshot();
    snapshot.hdr_enabled = false;
    snapshot.hdr_available = true;
    controller.apply_worker_event(WorkerEvent::Snapshot(snapshot), 900);

    controller.handle_action(
        UiAction::PreviewFeature {
            feature: FeatureId::Brightness,
            value: 51,
        },
        1_000,
    );
    controller.handle_action(
        UiAction::PreviewFeature {
            feature: FeatureId::Brightness,
            value: 52,
        },
        1_040,
    );
    assert!(controller.ui_state().brightness.enabled);

    let effects = controller.handle_action(UiAction::ToggleHdr(true), 1_050);

    assert_eq!(
        effects,
        vec![
            ControllerEffect::Worker(WorkerRequest::CancelPictureWrites),
            ControllerEffect::Worker(WorkerRequest::SetHdr {
                enabled: true,
                use_cached_ddc_values: false,
            }),
        ]
    );
    assert!(controller.flush_pending(1_140).is_empty());
}

#[test]
fn windows_sdr_failure_restores_the_confirmed_snapshot_value() {
    let mut controller = AppController::default();
    let mut snapshot = sample_snapshot();
    snapshot.hdr_active = true;
    snapshot.picture_backend = PictureBackend::WindowsSdr;
    snapshot.brightness.value = 40;
    snapshot.contrast.available = false;
    controller.apply_worker_event(WorkerEvent::Snapshot(snapshot.clone()), 900);

    controller.handle_action(
        UiAction::CommitFeature {
            feature: FeatureId::Brightness,
            value: 65,
        },
        1_000,
    );
    assert_eq!(controller.ui_state().brightness.value, 65);

    controller.apply_worker_event(WorkerEvent::Snapshot(snapshot), 1_010);
    assert_eq!(controller.ui_state().brightness.value, 65);
    controller.apply_worker_event(
        WorkerEvent::SdrContentBrightnessFinished {
            requested_percent: 65,
            error: Some("SDR brightness update failed".into()),
        },
        1_020,
    );

    assert_eq!(controller.ui_state().brightness.value, 40);
    assert_eq!(controller.ui_state().brightness.text, "40");
    assert_eq!(
        controller.ui_state().status_text,
        "SDR brightness update failed"
    );
}

#[test]
fn older_windows_sdr_completion_does_not_discard_a_newer_throttled_value() {
    let mut controller = AppController::default();
    let mut snapshot = sample_snapshot();
    snapshot.hdr_active = true;
    snapshot.picture_backend = PictureBackend::WindowsSdr;
    snapshot.brightness.value = 40;
    snapshot.contrast.available = false;
    controller.apply_worker_event(WorkerEvent::Snapshot(snapshot.clone()), 900);

    assert_eq!(
        controller.handle_action(
            UiAction::PreviewFeature {
                feature: FeatureId::Brightness,
                value: 50,
            },
            1_000,
        ),
        vec![ControllerEffect::Worker(
            WorkerRequest::SetSdrContentBrightness { percent: 50 }
        )]
    );
    assert!(controller
        .handle_action(
            UiAction::PreviewFeature {
                feature: FeatureId::Brightness,
                value: 60,
            },
            1_040,
        )
        .is_empty());

    snapshot.brightness.value = 50;
    controller.apply_worker_event(WorkerEvent::Snapshot(snapshot), 1_050);
    controller.apply_worker_event(
        WorkerEvent::SdrContentBrightnessFinished {
            requested_percent: 50,
            error: None,
        },
        1_060,
    );

    assert_eq!(controller.ui_state().brightness.value, 60);
    assert_eq!(
        controller.flush_pending(1_140),
        vec![ControllerEffect::Worker(
            WorkerRequest::SetSdrContentBrightness { percent: 60 }
        )]
    );
}

#[test]
fn external_windows_sdr_snapshot_updates_brightness_without_a_pending_write() {
    let mut controller = AppController::default();
    let mut snapshot = sample_snapshot();
    snapshot.hdr_active = true;
    snapshot.picture_backend = PictureBackend::WindowsSdr;
    snapshot.brightness.value = 40;
    snapshot.contrast.available = false;
    controller.apply_worker_event(WorkerEvent::Snapshot(snapshot.clone()), 900);

    snapshot.brightness.value = 72;
    controller.apply_worker_event(WorkerEvent::Snapshot(snapshot), 1_000);

    assert_eq!(controller.ui_state().brightness.value, 72);
    assert_eq!(controller.ui_state().brightness.text, "72");
}

#[test]
fn external_hdr_mode_transition_cancels_picture_writes_and_restores_fresh_ddc_values() {
    let mut controller = AppController::default();
    let mut hdr = sample_snapshot();
    hdr.hdr_active = true;
    hdr.picture_backend = PictureBackend::WindowsSdr;
    hdr.brightness.value = 35;
    hdr.contrast.available = false;
    controller.apply_worker_event(WorkerEvent::Snapshot(hdr), 900);

    controller.handle_action(
        UiAction::PreviewFeature {
            feature: FeatureId::Brightness,
            value: 45,
        },
        1_000,
    );
    controller.handle_action(
        UiAction::PreviewFeature {
            feature: FeatureId::Brightness,
            value: 46,
        },
        1_040,
    );

    let mut sdr = sample_snapshot();
    sdr.hdr_enabled = false;
    sdr.hdr_active = false;
    sdr.picture_backend = PictureBackend::Ddc;
    sdr.brightness.value = 58;
    sdr.contrast.value = 77;
    controller.apply_worker_event(WorkerEvent::Snapshot(sdr), 1_050);

    let state = controller.ui_state();
    assert_eq!(state.brightness.value, 58);
    assert!(state.brightness.enabled);
    assert_eq!(state.contrast.value, 77);
    assert!(state.contrast.enabled);
    assert_eq!(
        controller.flush_pending(1_140),
        vec![ControllerEffect::Worker(WorkerRequest::CancelPictureWrites)]
    );
}

#[test]
fn commit_feature_waits_for_the_trailing_throttle_window() {
    let mut controller = AppController::default();
    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()), 900);

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

    assert!(committed.is_empty());
    assert_eq!(
        controller.flush_pending(1_140),
        vec![ControllerEffect::Worker(WorkerRequest::WriteFeature {
            code: CONTRAST_CODE,
            value: 68,
        })]
    );
}

#[test]
fn worker_snapshot_populates_the_ui_state() {
    let mut controller = AppController::default();

    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()), 1_000);
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
    snapshot.hdr_enabled = false;
    snapshot.hdr_available = false;
    snapshot.selected_input = InputRoute::None;
    snapshot.brightness.available = false;
    snapshot.contrast.available = false;

    controller.apply_worker_event(WorkerEvent::Snapshot(snapshot), 1_000);
    let state = controller.ui_state();

    assert!(state.no_monitor);
    assert!(!state.brightness.enabled);
    assert!(!state.contrast.enabled);
    assert!(!state.input_enabled);
    assert!(!state.hdr_enabled);
    assert!(!state.hdr_toggle_enabled);
    assert_eq!(state.selected_input, InputRoute::None);
}

#[test]
fn no_monitor_snapshot_disables_recently_changed_controls() {
    let mut controller = AppController::default();
    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()), 900);
    controller.handle_action(
        UiAction::PreviewFeature {
            feature: FeatureId::Brightness,
            value: 60,
        },
        1_000,
    );

    let mut snapshot = sample_snapshot();
    snapshot.has_monitor = false;
    snapshot.input_enabled = false;
    snapshot.input_summary = "Input".into();
    snapshot.hdr_status = "Windows HDR: unavailable".into();
    snapshot.hdr_enabled = false;
    snapshot.hdr_available = false;
    snapshot.selected_input = InputRoute::None;
    snapshot.brightness.available = false;
    snapshot.contrast.available = false;

    controller.apply_worker_event(WorkerEvent::Snapshot(snapshot), 1_100);

    let state = controller.ui_state();
    assert!(state.no_monitor);
    assert!(!state.brightness.enabled);
    assert_eq!(state.brightness.text, "n/a");
}

#[test]
fn input_actions_map_to_vcp_values() {
    let mut controller = AppController::default();
    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()), 1_000);

    let usb_c = controller.handle_action(UiAction::SetInput(InputRoute::UsbC), 1_000);
    let dp = controller.handle_action(UiAction::SetInput(InputRoute::DisplayPort), 1_040);
    let hdmi = controller.handle_action(UiAction::SetInput(InputRoute::Hdmi), 1_090);

    assert_eq!(
        usb_c,
        vec![ControllerEffect::Worker(WorkerRequest::SetInput {
            value: INPUT_USB_C_VALUE,
        })]
    );
    assert!(dp.is_empty());
    assert!(hdmi.is_empty());

    let state = controller.ui_state();
    assert_eq!(state.selected_input, InputRoute::Hdmi);
    assert_eq!(
        controller.flush_pending(1_140),
        vec![ControllerEffect::Worker(WorkerRequest::SetInput {
            value: INPUT_HDMI_VALUE,
        })]
    );
}

#[test]
fn recent_input_change_blocks_snapshot_selected_input_updates() {
    let mut controller = AppController::default();
    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()), 900);

    controller.handle_action(UiAction::SetInput(InputRoute::Hdmi), 1_000);

    let mut snapshot = sample_snapshot();
    snapshot.selected_input = InputRoute::DisplayPort;
    snapshot.input_summary = "DP".into();
    controller.apply_worker_event(WorkerEvent::Snapshot(snapshot), 1_200);

    let state = controller.ui_state();
    assert_eq!(state.selected_input, InputRoute::Hdmi);
}

#[test]
fn stale_snapshot_does_not_override_a_newer_preview_value() {
    let mut controller = AppController::default();
    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()), 900);

    controller.handle_action(
        UiAction::PreviewFeature {
            feature: FeatureId::Brightness,
            value: 51,
        },
        1_000,
    );
    controller.handle_action(
        UiAction::PreviewFeature {
            feature: FeatureId::Brightness,
            value: 60,
        },
        1_040,
    );

    let mut stale = sample_snapshot();
    stale.brightness.value = 51;
    controller.apply_worker_event(WorkerEvent::Snapshot(stale), 1_100);

    let state = controller.ui_state();
    assert_eq!(state.brightness.value, 60);
    assert_eq!(state.brightness.text, "60");
}

#[test]
fn snapshot_after_guard_can_override_a_committed_value() {
    let mut controller = AppController::default();
    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()), 900);

    controller.handle_action(
        UiAction::CommitFeature {
            feature: FeatureId::Contrast,
            value: 68,
        },
        1_000,
    );

    let mut stale = sample_snapshot();
    stale.contrast.value = 80;
    controller.apply_worker_event(WorkerEvent::Snapshot(stale), 2_501);

    let state = controller.ui_state();
    assert_eq!(state.contrast.value, 80);
    assert_eq!(state.contrast.text, "80");
}

#[test]
fn error_clears_optimistic_value_and_allows_following_snapshot_to_restore_backend_state() {
    let mut controller = AppController::default();
    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()), 900);

    controller.handle_action(
        UiAction::CommitFeature {
            feature: FeatureId::Brightness,
            value: 75,
        },
        1_000,
    );
    controller.apply_worker_event(WorkerEvent::Error("Set failed".into()), 1_050);

    let state_after_error = controller.ui_state();
    assert_eq!(state_after_error.status_text, "Set failed");

    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()), 1_100);

    let restored = controller.ui_state();
    assert_eq!(restored.brightness.value, 50);
    assert_eq!(restored.brightness.text, "50");
}

#[test]
fn commit_feature_uses_the_same_trailing_throttle_as_preview() {
    let mut controller = AppController::default();
    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()), 900);

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

    let committed = controller.handle_action(
        UiAction::CommitFeature {
            feature: FeatureId::Brightness,
            value: 55,
        },
        1_040,
    );
    assert!(committed.is_empty());

    assert_eq!(
        controller.flush_pending(1_140),
        vec![ControllerEffect::Worker(WorkerRequest::WriteFeature {
            code: BRIGHTNESS_CODE,
            value: 55,
        })]
    );
}

#[test]
fn recent_user_change_blocks_only_that_feature_from_snapshot_updates() {
    let mut controller = AppController::default();
    controller.apply_worker_event(WorkerEvent::Snapshot(sample_snapshot()), 900);

    controller.handle_action(
        UiAction::PreviewFeature {
            feature: FeatureId::Brightness,
            value: 60,
        },
        1_000,
    );

    let mut snapshot = sample_snapshot();
    snapshot.brightness.value = 51;
    snapshot.contrast.value = 70;
    controller.apply_worker_event(WorkerEvent::Snapshot(snapshot), 1_200);

    let state = controller.ui_state();
    assert_eq!(state.brightness.value, 60);
    assert_eq!(state.brightness.text, "60");
    assert_eq!(state.contrast.value, 70);
    assert_eq!(state.contrast.text, "70");
}

use std::{
    collections::VecDeque,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use dell_controller_core::{
    ddc::VcpCode, enumerate_monitors, hdr, profiles::profile_for_model, startup, CommandQueue,
    RetryPolicy, WindowsDdcBackend, WindowsMonitor,
};

use crate::{
    app_controller::{
        FeatureSnapshot, InputRoute, MonitorChoice, MonitorSnapshot, WorkerEvent, WorkerRequest,
        BRIGHTNESS_CODE, CONTRAST_CODE, INPUT_DP_VALUE, INPUT_HDMI_VALUE, INPUT_USB_C_VALUE,
    },
    monitor_text::{compact_input_label, monitor_heading},
};

const INPUT_CODE: u8 = 0x60;
const DDC_FAILURE_REFRESH_THRESHOLD: u8 = 2;

#[derive(Clone)]
pub struct WorkerHandle {
    request_tx: Sender<WorkerRequest>,
}

impl WorkerHandle {
    pub fn send(&self, request: WorkerRequest) -> Result<(), mpsc::SendError<WorkerRequest>> {
        self.request_tx.send(request)
    }
}

pub fn spawn_worker(event_tx: Sender<WorkerEvent>) -> WorkerHandle {
    let (request_tx, request_rx) = mpsc::channel();
    thread::spawn(move || run_worker(request_rx, event_tx));
    WorkerHandle { request_tx }
}

fn run_worker(request_rx: Receiver<WorkerRequest>, event_tx: Sender<WorkerEvent>) {
    let mut device = WindowsWorkerDevice::default();
    run_worker_loop(request_rx, event_tx, &mut device);
}

fn run_worker_loop<D: WorkerDevice>(
    request_rx: Receiver<WorkerRequest>,
    event_tx: Sender<WorkerEvent>,
    device: &mut D,
) {
    let mut pending = PendingWorkerRequests::default();
    let mut runtime = WorkerRuntimeState::default();

    loop {
        if pending.is_empty() {
            let Ok(request) = request_rx.recv() else {
                break;
            };
            pending.push(request);
        }

        pending.drain_available(&request_rx);
        if let Some(task) = pending.pop_next_task() {
            execute_worker_task(task, device, &event_tx, &mut runtime);
        }
    }
}

#[derive(Default)]
struct WorkerRuntimeState {
    selected_monitor_key: Option<String>,
    consecutive_ddc_failures: u8,
}

struct DeviceSnapshot {
    snapshot: MonitorSnapshot,
    selected_monitor_missing: bool,
    ddc_failure: bool,
}

trait WorkerDevice {
    fn refresh_monitors(&mut self) -> Result<(), String>;
    fn snapshot(&mut self, selected_monitor_key: Option<&str>) -> DeviceSnapshot;
    fn write_vcp(
        &mut self,
        selected_monitor_key: Option<&str>,
        code: u8,
        value: u32,
    ) -> Result<(), String>;
    fn autostart_enabled(&self) -> bool;
    fn set_autostart_enabled(&mut self, enabled: bool) -> Result<(), String>;
}

#[derive(Default)]
struct WindowsWorkerDevice {
    monitors: Vec<WindowsMonitor>,
}

impl WorkerDevice for WindowsWorkerDevice {
    fn refresh_monitors(&mut self) -> Result<(), String> {
        match enumerate_monitors() {
            Ok(found) => {
                self.monitors = found;
                Ok(())
            }
            Err(error) => {
                self.monitors.clear();
                Err(error.to_string())
            }
        }
    }

    fn snapshot(&mut self, selected_monitor_key: Option<&str>) -> DeviceSnapshot {
        build_snapshot(&self.monitors, selected_monitor_key)
    }

    fn write_vcp(
        &mut self,
        selected_monitor_key: Option<&str>,
        code: u8,
        value: u32,
    ) -> Result<(), String> {
        write_feature(&self.monitors, selected_monitor_key, code, value)
    }

    fn autostart_enabled(&self) -> bool {
        startup::is_autostart_enabled().unwrap_or(false)
    }

    fn set_autostart_enabled(&mut self, enabled: bool) -> Result<(), String> {
        startup::set_autostart_enabled(enabled).map_err(|error| error.to_string())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorkerTask {
    RefreshAll,
    ReadSnapshot,
    RefreshAutostart,
    SelectMonitor {
        key: String,
    },
    SetAutostart {
        enabled: bool,
        quiet: bool,
    },
    WriteVcp {
        code: u8,
        value: u32,
        input_status: bool,
    },
}

#[derive(Default)]
struct PendingWorkerRequests {
    requests: VecDeque<WorkerRequest>,
}

impl From<Vec<WorkerRequest>> for PendingWorkerRequests {
    fn from(requests: Vec<WorkerRequest>) -> Self {
        Self {
            requests: requests.into(),
        }
    }
}

impl PendingWorkerRequests {
    fn is_empty(&self) -> bool {
        self.requests.is_empty()
    }

    fn push(&mut self, request: WorkerRequest) {
        self.requests.push_back(request);
    }

    fn drain_available(&mut self, request_rx: &Receiver<WorkerRequest>) {
        while let Ok(request) = request_rx.try_recv() {
            self.push(request);
        }
    }

    fn pop_next_task(&mut self) -> Option<WorkerTask> {
        let request = self.requests.pop_front()?;
        match request {
            WorkerRequest::RefreshAll => Some(WorkerTask::RefreshAll),
            WorkerRequest::ReadSnapshot => {
                self.requests
                    .retain(|request| !matches!(request, WorkerRequest::ReadSnapshot));
                Some(WorkerTask::ReadSnapshot)
            }
            WorkerRequest::RefreshAutostart => Some(WorkerTask::RefreshAutostart),
            WorkerRequest::SelectMonitor { key } => Some(WorkerTask::SelectMonitor { key }),
            WorkerRequest::SetAutostart { enabled, quiet } => {
                Some(WorkerTask::SetAutostart { enabled, quiet })
            }
            WorkerRequest::WriteFeature { code, value } => {
                Some(self.coalesce_write(code, value, false))
            }
            WorkerRequest::SetInput { value } => Some(self.coalesce_write(INPUT_CODE, value, true)),
        }
    }

    fn coalesce_write(&mut self, code: u8, value: u32, input_status: bool) -> WorkerTask {
        let mut latest_value = value;
        let mut latest_input_status = input_status;
        let mut retained = VecDeque::new();

        while let Some(request) = self.requests.pop_front() {
            match write_request(&request) {
                Some(write) if write.code == code => {
                    latest_value = write.value;
                    latest_input_status = write.input_status;
                }
                _ => retained.push_back(request),
            }
        }

        self.requests = retained;
        WorkerTask::WriteVcp {
            code,
            value: latest_value,
            input_status: latest_input_status,
        }
    }
}

#[derive(Clone, Copy)]
struct VcpWrite {
    code: u8,
    value: u32,
    input_status: bool,
}

fn write_request(request: &WorkerRequest) -> Option<VcpWrite> {
    match request {
        WorkerRequest::WriteFeature { code, value } => Some(VcpWrite {
            code: *code,
            value: *value,
            input_status: false,
        }),
        WorkerRequest::SetInput { value } => Some(VcpWrite {
            code: INPUT_CODE,
            value: *value,
            input_status: true,
        }),
        _ => None,
    }
}

fn execute_worker_task<D: WorkerDevice>(
    task: WorkerTask,
    device: &mut D,
    event_tx: &Sender<WorkerEvent>,
    runtime: &mut WorkerRuntimeState,
) {
    match task {
        WorkerTask::RefreshAll => match device.refresh_monitors() {
            Ok(()) => {
                emit_device_snapshot(device, event_tx, runtime);
                let _ = event_tx.send(WorkerEvent::Status("Monitors refreshed".into()));
            }
            Err(error) => {
                emit_device_snapshot(device, event_tx, runtime);
                let _ = event_tx.send(WorkerEvent::Error(format!(
                    "Monitor refresh failed: {error}"
                )));
            }
        },
        WorkerTask::ReadSnapshot => {
            emit_device_snapshot(device, event_tx, runtime);
        }
        WorkerTask::RefreshAutostart => {
            let _ = event_tx.send(WorkerEvent::AutostartState {
                enabled: device.autostart_enabled(),
                status: String::new(),
            });
        }
        WorkerTask::SelectMonitor { key } => {
            runtime.selected_monitor_key = Some(key);
            runtime.consecutive_ddc_failures = 0;
            emit_device_snapshot(device, event_tx, runtime);
        }
        WorkerTask::SetAutostart { enabled, quiet } => {
            match device.set_autostart_enabled(enabled) {
                Ok(()) => {
                    let _ = event_tx.send(WorkerEvent::AutostartState {
                        enabled,
                        status: if quiet {
                            String::new()
                        } else if enabled {
                            "Autostart enabled".into()
                        } else {
                            "Autostart disabled".into()
                        },
                    });
                }
                Err(error) => {
                    let _ = event_tx.send(WorkerEvent::Error(format!(
                        "Autostart update failed: {error}"
                    )));
                    let _ = event_tx.send(WorkerEvent::AutostartState {
                        enabled: device.autostart_enabled(),
                        status: String::new(),
                    });
                }
            }
        }
        WorkerTask::WriteVcp {
            code,
            value,
            input_status,
        } => match device.write_vcp(runtime.selected_monitor_key.as_deref(), code, value) {
            Ok(()) if input_status => {
                let _ = event_tx.send(WorkerEvent::Status(format!("Input set to {value:#X}")));
            }
            Ok(()) => {}
            Err(message) => {
                let _ = event_tx.send(WorkerEvent::Error(message));
            }
        },
    }
}

fn emit_device_snapshot<D: WorkerDevice>(
    device: &mut D,
    event_tx: &Sender<WorkerEvent>,
    runtime: &mut WorkerRuntimeState,
) {
    let result = device.snapshot(runtime.selected_monitor_key.as_deref());
    let selected_monitor_missing = result.selected_monitor_missing;
    let ddc_failure = result.ddc_failure;
    let _ = event_tx.send(WorkerEvent::Snapshot(result.snapshot));

    if selected_monitor_missing {
        let _ = event_tx.send(WorkerEvent::Status(
            "Selected monitor is unavailable; using the first detected monitor".into(),
        ));
    }

    if ddc_failure {
        runtime.consecutive_ddc_failures = runtime.consecutive_ddc_failures.saturating_add(1);
        if runtime.consecutive_ddc_failures >= DDC_FAILURE_REFRESH_THRESHOLD {
            runtime.consecutive_ddc_failures = 0;
            match device.refresh_monitors() {
                Ok(()) => {
                    let _ = event_tx.send(WorkerEvent::Status(
                        "Monitor topology refreshed after repeated DDC failures".into(),
                    ));
                    let refreshed = device.snapshot(runtime.selected_monitor_key.as_deref());
                    let _ = event_tx.send(WorkerEvent::Snapshot(refreshed.snapshot));
                }
                Err(error) => {
                    let _ = event_tx.send(WorkerEvent::Error(format!(
                        "Monitor refresh after DDC failures failed: {error}"
                    )));
                }
            }
        }
    } else {
        runtime.consecutive_ddc_failures = 0;
    }
}

fn write_feature(
    monitors: &[WindowsMonitor],
    selected_monitor_key: Option<&str>,
    code: u8,
    value: u32,
) -> Result<(), String> {
    let choices = monitor_choices(monitors);
    let Some((index, _choice, _missing)) =
        selected_monitor(monitors, &choices, selected_monitor_key)
    else {
        return Err("No monitor available".into());
    };
    let Some(monitor) = monitors.get(index) else {
        return Err("Selected monitor is unavailable".into());
    };

    let queue: CommandQueue<_> = CommandQueue::new(monitor.backend.clone(), RetryPolicy::default());
    queue
        .set(VcpCode::new(code), value)
        .map_err(|error| format!("Set failed: {error}"))
}

fn build_snapshot(
    monitors: &[WindowsMonitor],
    selected_monitor_key: Option<&str>,
) -> DeviceSnapshot {
    let choices = monitor_choices(monitors);
    let Some((index, choice, selected_monitor_missing)) =
        selected_monitor(monitors, &choices, selected_monitor_key)
    else {
        return DeviceSnapshot {
            snapshot: MonitorSnapshot {
                monitor_title: "No DDC/CI monitor detected".into(),
                input_summary: "Input".into(),
                hdr_status: "Windows HDR: unavailable".into(),
                diagnostic_status: String::new(),
                monitor_choices: choices,
                selected_monitor_key: String::new(),
                brightness: FeatureSnapshot {
                    value: 0,
                    maximum: 100,
                    available: false,
                },
                contrast: FeatureSnapshot {
                    value: 0,
                    maximum: 100,
                    available: false,
                },
                selected_input: InputRoute::None,
                input_enabled: false,
                has_monitor: false,
            },
            selected_monitor_missing: false,
            ddc_failure: false,
        };
    };
    let monitor = &monitors[index];

    let queue: CommandQueue<_> = CommandQueue::new(monitor.backend.clone(), RetryPolicy::default());
    let brightness = read_feature(&queue, BRIGHTNESS_CODE);
    let contrast = read_feature(&queue, CONTRAST_CODE);
    let input = read_input(monitor, &queue);
    let capability_supports_input = monitor
        .capabilities
        .as_ref()
        .is_some_and(|capabilities| capabilities.supports_vcp(INPUT_CODE));
    let input_enabled = input.available || capability_supports_input;
    let ddc_failure = !brightness.available && !contrast.available && !input.available;
    let selected_monitor_key = choice.key.clone();

    DeviceSnapshot {
        snapshot: MonitorSnapshot {
            monitor_title: monitor_heading(
                &monitor.info.description,
                monitor.info.model.as_deref(),
            ),
            input_summary: input.summary,
            hdr_status: hdr_status_text(),
            diagnostic_status: diagnostic_status(monitors),
            monitor_choices: choices,
            selected_monitor_key,
            brightness,
            contrast,
            selected_input: input.route,
            input_enabled,
            has_monitor: true,
        },
        selected_monitor_missing,
        ddc_failure,
    }
}

fn read_feature(
    queue: &CommandQueue<std::sync::Arc<WindowsDdcBackend>>,
    code: u8,
) -> FeatureSnapshot {
    match queue.get(VcpCode::new(code)) {
        Ok(feature) => FeatureSnapshot {
            value: feature.current.min(feature.maximum),
            maximum: feature.maximum.max(1),
            available: true,
        },
        Err(_) => FeatureSnapshot {
            value: 0,
            maximum: 100,
            available: false,
        },
    }
}

struct InputSnapshot {
    summary: String,
    route: InputRoute,
    available: bool,
}

fn read_input(
    monitor: &WindowsMonitor,
    queue: &CommandQueue<std::sync::Arc<WindowsDdcBackend>>,
) -> InputSnapshot {
    match queue.get(VcpCode::new(INPUT_CODE)) {
        Ok(feature) => {
            let profile = profile_for_model(monitor.info.model.as_deref());
            let label = profile
                .as_ref()
                .and_then(|profile| profile.control("input"))
                .and_then(|control| control.value_label(feature.current))
                .unwrap_or("unknown");
            InputSnapshot {
                summary: compact_input_label(label).into(),
                route: input_route_from_value(feature.current),
                available: true,
            }
        }
        Err(_) => InputSnapshot {
            summary: "Input".into(),
            route: InputRoute::None,
            available: false,
        },
    }
}

fn monitor_choices(monitors: &[WindowsMonitor]) -> Vec<MonitorChoice> {
    let mut choices = Vec::with_capacity(monitors.len());
    for (index, monitor) in monitors.iter().enumerate() {
        let duplicate_ordinal = monitors[..index]
            .iter()
            .filter(|other| {
                other.info.description == monitor.info.description
                    && other.info.model == monitor.info.model
            })
            .count();
        choices.push(MonitorChoice {
            key: monitor_key(monitor, duplicate_ordinal),
            title: monitor_heading(&monitor.info.description, monitor.info.model.as_deref()),
        });
    }
    choices
}

fn selected_monitor<'a>(
    monitors: &[WindowsMonitor],
    choices: &'a [MonitorChoice],
    selected_monitor_key: Option<&str>,
) -> Option<(usize, &'a MonitorChoice, bool)> {
    if monitors.is_empty() || choices.is_empty() {
        return None;
    }

    if let Some(key) = selected_monitor_key.filter(|key| !key.is_empty()) {
        if let Some(index) = choices.iter().position(|choice| choice.key == key) {
            return Some((index, &choices[index], false));
        }
        return Some((0, &choices[0], true));
    }

    Some((0, &choices[0], false))
}

fn monitor_key(monitor: &WindowsMonitor, duplicate_ordinal: usize) -> String {
    format!(
        "{}|{}|{}",
        normalize_key_part(&monitor.info.description),
        normalize_key_part(monitor.info.model.as_deref().unwrap_or("")),
        duplicate_ordinal
    )
}

fn normalize_key_part(value: &str) -> String {
    let mut normalized = String::new();
    let mut last_was_separator = false;
    for ch in value.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            normalized.push(ch);
            last_was_separator = false;
        } else if !last_was_separator {
            normalized.push('-');
            last_was_separator = true;
        }
    }
    normalized.trim_matches('-').to_string()
}

fn diagnostic_status(monitors: &[WindowsMonitor]) -> String {
    let messages = monitors
        .iter()
        .flat_map(|monitor| {
            let title = monitor_heading(&monitor.info.description, monitor.info.model.as_deref());
            [
                monitor
                    .diagnostics
                    .capability_error
                    .as_ref()
                    .map(|error| format!("{title}: capabilities unavailable ({error})")),
                monitor
                    .diagnostics
                    .parse_error
                    .as_ref()
                    .map(|error| format!("{title}: capabilities parse failed ({error})")),
            ]
        })
        .flatten()
        .collect::<Vec<_>>();

    if messages.is_empty() {
        String::new()
    } else {
        format!("Monitor diagnostics: {}", messages.join("; "))
    }
}

fn input_route_from_value(value: u32) -> InputRoute {
    match value & 0xFF {
        INPUT_DP_VALUE => InputRoute::DisplayPort,
        INPUT_USB_C_VALUE => InputRoute::UsbC,
        INPUT_HDMI_VALUE => InputRoute::Hdmi,
        _ => InputRoute::None,
    }
}

fn hdr_status_text() -> String {
    match hdr::hdr_states() {
        Ok(states) => hdr_status_text_from_states(&states),
        Err(_) => "Windows HDR: unavailable".into(),
    }
}

fn hdr_status_text_from_states(states: &[hdr::HdrState]) -> String {
    match states {
        [state] => format!(
            "Windows HDR: {}",
            if state.enabled {
                "On"
            } else if state.supported {
                "Available, off"
            } else {
                "Not supported"
            }
        ),
        states if !states.is_empty() => {
            let enabled = states.iter().filter(|state| state.enabled).count();
            if enabled == 0 {
                "Windows HDR: multiple displays, off".into()
            } else if enabled == states.len() {
                "Windows HDR: multiple displays, on".into()
            } else {
                "Windows HDR: multiple displays, mixed".into()
            }
        }
        _ => "Windows HDR: unavailable".into(),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use super::*;

    #[derive(Default)]
    struct RecordingWorkerDevice {
        calls: Vec<String>,
        ddc_failure_snapshots: bool,
        missing_selected_monitor: bool,
    }

    impl WorkerDevice for RecordingWorkerDevice {
        fn refresh_monitors(&mut self) -> Result<(), String> {
            self.calls.push("refresh".into());
            Ok(())
        }

        fn snapshot(&mut self, selected_monitor_key: Option<&str>) -> DeviceSnapshot {
            self.calls.push(format!(
                "snapshot:{}",
                selected_monitor_key.unwrap_or("<none>")
            ));
            DeviceSnapshot {
                snapshot: test_snapshot(
                    selected_monitor_key
                        .filter(|key| !key.is_empty())
                        .unwrap_or("dell-u4025qw|u4025qw|0"),
                ),
                selected_monitor_missing: self.missing_selected_monitor,
                ddc_failure: self.ddc_failure_snapshots,
            }
        }

        fn write_vcp(
            &mut self,
            selected_monitor_key: Option<&str>,
            code: u8,
            value: u32,
        ) -> Result<(), String> {
            self.calls.push(format!(
                "write:{}:{code:02X}:{value}",
                selected_monitor_key.unwrap_or("<none>")
            ));
            Ok(())
        }

        fn autostart_enabled(&self) -> bool {
            false
        }

        fn set_autostart_enabled(&mut self, enabled: bool) -> Result<(), String> {
            self.calls.push(format!("autostart:{enabled}"));
            Ok(())
        }
    }

    fn test_snapshot(selected_monitor_key: &str) -> MonitorSnapshot {
        MonitorSnapshot {
            monitor_title: "Dell U4025QW".into(),
            input_summary: "DP".into(),
            hdr_status: "Windows HDR: On".into(),
            diagnostic_status: String::new(),
            monitor_choices: vec![MonitorChoice {
                key: "dell-u4025qw|u4025qw|0".into(),
                title: "Dell U4025QW".into(),
            }],
            selected_monitor_key: selected_monitor_key.into(),
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

    fn hdr_state(display_index: usize, supported: bool, enabled: bool) -> hdr::HdrState {
        hdr::HdrState {
            display_index,
            supported,
            enabled,
            wide_color_enforced: false,
            force_disabled: false,
            bits_per_color_channel: 10,
            color_encoding: "RGB".into(),
        }
    }

    #[test]
    fn queued_writes_for_the_same_vcp_execute_only_the_newest_value() {
        let mut pending = PendingWorkerRequests::from(vec![
            WorkerRequest::WriteFeature {
                code: BRIGHTNESS_CODE,
                value: 51,
            },
            WorkerRequest::WriteFeature {
                code: CONTRAST_CODE,
                value: 72,
            },
            WorkerRequest::WriteFeature {
                code: BRIGHTNESS_CODE,
                value: 64,
            },
        ]);

        assert_eq!(
            pending.pop_next_task(),
            Some(WorkerTask::WriteVcp {
                code: BRIGHTNESS_CODE,
                value: 64,
                input_status: false,
            })
        );
        assert_eq!(
            pending.pop_next_task(),
            Some(WorkerTask::WriteVcp {
                code: CONTRAST_CODE,
                value: 72,
                input_status: false,
            })
        );
        assert_eq!(pending.pop_next_task(), None);
    }

    #[test]
    fn input_writes_coalesce_with_raw_writes_for_the_input_vcp_code() {
        let mut pending = PendingWorkerRequests::from(vec![
            WorkerRequest::WriteFeature {
                code: INPUT_CODE,
                value: INPUT_DP_VALUE,
            },
            WorkerRequest::SetInput {
                value: INPUT_HDMI_VALUE,
            },
        ]);

        assert_eq!(
            pending.pop_next_task(),
            Some(WorkerTask::WriteVcp {
                code: INPUT_CODE,
                value: INPUT_HDMI_VALUE,
                input_status: true,
            })
        );
        assert_eq!(pending.pop_next_task(), None);
    }

    #[test]
    fn queued_snapshot_requests_coalesce_to_one_snapshot_task() {
        let mut pending = PendingWorkerRequests::from(vec![
            WorkerRequest::ReadSnapshot,
            WorkerRequest::ReadSnapshot,
            WorkerRequest::WriteFeature {
                code: BRIGHTNESS_CODE,
                value: 62,
            },
        ]);

        assert_eq!(pending.pop_next_task(), Some(WorkerTask::ReadSnapshot));
        assert_eq!(
            pending.pop_next_task(),
            Some(WorkerTask::WriteVcp {
                code: BRIGHTNESS_CODE,
                value: 62,
                input_status: false,
            })
        );
        assert_eq!(pending.pop_next_task(), None);
    }

    #[test]
    fn write_tasks_do_not_emit_snapshots() {
        let (event_tx, event_rx) = mpsc::channel();
        let mut device = RecordingWorkerDevice::default();

        execute_worker_task(
            WorkerTask::WriteVcp {
                code: BRIGHTNESS_CODE,
                value: 64,
                input_status: false,
            },
            &mut device,
            &event_tx,
            &mut WorkerRuntimeState::default(),
        );

        assert_eq!(device.calls, vec!["write:<none>:10:64"]);
        assert!(event_rx.try_recv().is_err());
    }

    #[test]
    fn select_monitor_updates_the_runtime_selection_used_by_writes() {
        let (event_tx, _event_rx) = mpsc::channel();
        let mut runtime = WorkerRuntimeState::default();
        let mut device = RecordingWorkerDevice::default();

        execute_worker_task(
            WorkerTask::SelectMonitor {
                key: "dell-u4025qw|u4025qw|1".into(),
            },
            &mut device,
            &event_tx,
            &mut runtime,
        );
        execute_worker_task(
            WorkerTask::WriteVcp {
                code: BRIGHTNESS_CODE,
                value: 64,
                input_status: false,
            },
            &mut device,
            &event_tx,
            &mut runtime,
        );

        assert_eq!(
            device.calls,
            vec![
                "snapshot:dell-u4025qw|u4025qw|1",
                "write:dell-u4025qw|u4025qw|1:10:64"
            ]
        );
    }

    #[test]
    fn missing_selected_monitor_emits_a_fallback_status() {
        let (event_tx, event_rx) = mpsc::channel();
        let mut runtime = WorkerRuntimeState {
            selected_monitor_key: Some("missing".into()),
            consecutive_ddc_failures: 0,
        };
        let mut device = RecordingWorkerDevice {
            missing_selected_monitor: true,
            ..Default::default()
        };

        execute_worker_task(
            WorkerTask::ReadSnapshot,
            &mut device,
            &event_tx,
            &mut runtime,
        );

        assert!(matches!(event_rx.recv().unwrap(), WorkerEvent::Snapshot(_)));
        assert_eq!(
            event_rx.recv().unwrap(),
            WorkerEvent::Status(
                "Selected monitor is unavailable; using the first detected monitor".into()
            )
        );
    }

    #[test]
    fn repeated_ddc_failures_refresh_monitor_topology() {
        let (event_tx, _event_rx) = mpsc::channel();
        let mut runtime = WorkerRuntimeState::default();
        let mut device = RecordingWorkerDevice {
            ddc_failure_snapshots: true,
            ..Default::default()
        };

        execute_worker_task(
            WorkerTask::ReadSnapshot,
            &mut device,
            &event_tx,
            &mut runtime,
        );
        execute_worker_task(
            WorkerTask::ReadSnapshot,
            &mut device,
            &event_tx,
            &mut runtime,
        );

        assert_eq!(
            device.calls,
            vec![
                "snapshot:<none>",
                "snapshot:<none>",
                "refresh",
                "snapshot:<none>"
            ]
        );
    }

    #[test]
    fn hdr_status_uses_conservative_text_for_multiple_displays() {
        assert_eq!(
            hdr_status_text_from_states(&[hdr_state(0, true, true)]),
            "Windows HDR: On"
        );
        assert_eq!(
            hdr_status_text_from_states(&[hdr_state(0, true, true), hdr_state(1, true, false),]),
            "Windows HDR: multiple displays, mixed"
        );
    }

    struct FailingAutostartDevice {
        enabled: bool,
    }

    impl WorkerDevice for FailingAutostartDevice {
        fn refresh_monitors(&mut self) -> Result<(), String> {
            Ok(())
        }

        fn snapshot(&mut self, _selected_monitor_key: Option<&str>) -> DeviceSnapshot {
            unreachable!("snapshot is not part of this test")
        }

        fn write_vcp(
            &mut self,
            _selected_monitor_key: Option<&str>,
            _code: u8,
            _value: u32,
        ) -> Result<(), String> {
            unreachable!("write_vcp is not part of this test")
        }

        fn autostart_enabled(&self) -> bool {
            self.enabled
        }

        fn set_autostart_enabled(&mut self, _enabled: bool) -> Result<(), String> {
            Err("permission denied".into())
        }
    }

    #[test]
    fn failed_autostart_set_re_emits_actual_state() {
        let (event_tx, event_rx) = mpsc::channel();
        let mut device = FailingAutostartDevice { enabled: false };

        execute_worker_task(
            WorkerTask::SetAutostart {
                enabled: true,
                quiet: false,
            },
            &mut device,
            &event_tx,
            &mut WorkerRuntimeState::default(),
        );

        assert_eq!(
            event_rx.recv().unwrap(),
            WorkerEvent::Error("Autostart update failed: permission denied".into())
        );
        assert_eq!(
            event_rx.recv().unwrap(),
            WorkerEvent::AutostartState {
                enabled: false,
                status: String::new(),
            }
        );
    }
}

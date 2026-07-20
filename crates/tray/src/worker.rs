use log::{debug, error, info};
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
        FeatureSnapshot, InputRoute, MonitorChoice, MonitorSnapshot, PictureBackend, WorkerEvent,
        WorkerRequest, BRIGHTNESS_CODE, CONTRAST_CODE, INPUT_DP_VALUE, INPUT_HDMI_VALUE,
        INPUT_USB_C_VALUE,
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
    debug!("worker thread spawned");
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
    picture_values_deferred: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PictureReadMode {
    Full,
    DeferDdc,
}

trait WorkerDevice {
    fn refresh_monitors(&mut self) -> Result<(), String>;
    fn snapshot(
        &mut self,
        selected_monitor_key: Option<&str>,
        picture_read_mode: PictureReadMode,
    ) -> DeviceSnapshot;
    fn write_vcp(
        &mut self,
        selected_monitor_key: Option<&str>,
        code: u8,
        value: u32,
    ) -> Result<(), String>;
    fn set_hdr_enabled(
        &mut self,
        selected_monitor_key: Option<&str>,
        enabled: bool,
    ) -> Result<(), String>;
    fn set_sdr_content_brightness(
        &mut self,
        selected_monitor_key: Option<&str>,
        percent: u32,
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

    fn snapshot(
        &mut self,
        selected_monitor_key: Option<&str>,
        picture_read_mode: PictureReadMode,
    ) -> DeviceSnapshot {
        build_snapshot(&self.monitors, selected_monitor_key, picture_read_mode)
    }

    fn write_vcp(
        &mut self,
        selected_monitor_key: Option<&str>,
        code: u8,
        value: u32,
    ) -> Result<(), String> {
        write_feature(&self.monitors, selected_monitor_key, code, value)
    }

    fn set_hdr_enabled(
        &mut self,
        selected_monitor_key: Option<&str>,
        enabled: bool,
    ) -> Result<(), String> {
        set_hdr_for_monitor(&self.monitors, selected_monitor_key, enabled)
    }

    fn set_sdr_content_brightness(
        &mut self,
        selected_monitor_key: Option<&str>,
        percent: u32,
    ) -> Result<(), String> {
        set_sdr_content_brightness_for_monitor(&self.monitors, selected_monitor_key, percent)
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
    SetHdr {
        enabled: bool,
        use_cached_ddc_values: bool,
    },
    SetSdrContentBrightness {
        percent: u32,
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
        let mut pending = Self::default();
        for request in requests {
            pending.push(request);
        }
        pending
    }
}

impl PendingWorkerRequests {
    fn is_empty(&self) -> bool {
        self.requests.is_empty()
    }

    fn push(&mut self, request: WorkerRequest) {
        if matches!(request, WorkerRequest::CancelPictureWrites) {
            self.requests.retain(|queued| !is_picture_write(queued));
            return;
        }
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
            WorkerRequest::SetHdr {
                enabled,
                use_cached_ddc_values,
            } => Some(WorkerTask::SetHdr {
                enabled,
                use_cached_ddc_values,
            }),
            WorkerRequest::SetSdrContentBrightness { percent } => {
                Some(self.coalesce_sdr_brightness(percent))
            }
            WorkerRequest::CancelPictureWrites => self.pop_next_task(),
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

    fn coalesce_sdr_brightness(&mut self, percent: u32) -> WorkerTask {
        let mut latest_percent = percent;
        self.requests.retain(|request| {
            if let WorkerRequest::SetSdrContentBrightness { percent } = request {
                latest_percent = *percent;
                false
            } else {
                true
            }
        });
        WorkerTask::SetSdrContentBrightness {
            percent: latest_percent,
        }
    }
}

fn is_picture_write(request: &WorkerRequest) -> bool {
    matches!(
        request,
        WorkerRequest::WriteFeature {
            code: BRIGHTNESS_CODE | CONTRAST_CODE,
            ..
        } | WorkerRequest::SetSdrContentBrightness { .. }
    )
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
                info!("monitors refreshed");
                emit_device_snapshot(device, event_tx, runtime);
                let _ = event_tx.send(WorkerEvent::Status("Monitors refreshed".into()));
            }
            Err(error) => {
                error!("monitor refresh failed: {error}");
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
        WorkerTask::SetHdr {
            enabled,
            use_cached_ddc_values,
        } => {
            let error = device
                .set_hdr_enabled(runtime.selected_monitor_key.as_deref(), enabled)
                .err()
                .map(|error| format!("HDR update failed: {error}"));
            let may_defer_ddc = error.is_none() && !enabled && use_cached_ddc_values;
            let used_cached_ddc_values = if may_defer_ddc {
                emit_device_snapshot_with_mode(device, event_tx, runtime, PictureReadMode::DeferDdc)
            } else {
                emit_device_snapshot(device, event_tx, runtime);
                false
            };
            let _ = event_tx.send(WorkerEvent::HdrUpdateFinished {
                enabled,
                error,
                used_cached_ddc_values,
            });
            if used_cached_ddc_values {
                emit_device_snapshot(device, event_tx, runtime);
            }
        }
        WorkerTask::SetSdrContentBrightness { percent } => {
            let error = device
                .set_sdr_content_brightness(runtime.selected_monitor_key.as_deref(), percent)
                .err()
                .map(|error| format!("SDR content brightness update failed: {error}"));
            emit_device_snapshot(device, event_tx, runtime);
            let _ = event_tx.send(WorkerEvent::SdrContentBrightnessFinished {
                requested_percent: percent,
                error,
            });
        }
        WorkerTask::WriteVcp {
            code,
            value,
            input_status,
        } => {
            debug!("write vcp code={code:02X} value={value}");
            match device.write_vcp(runtime.selected_monitor_key.as_deref(), code, value) {
                Ok(()) if input_status => {
                    let _ = event_tx.send(WorkerEvent::Status(format!("Input set to {value:#X}")));
                }
                Ok(()) => {}
                Err(message) => {
                    let _ = event_tx.send(WorkerEvent::Error(message));
                }
            }
        }
    }
}

fn emit_device_snapshot<D: WorkerDevice>(
    device: &mut D,
    event_tx: &Sender<WorkerEvent>,
    runtime: &mut WorkerRuntimeState,
) {
    emit_device_snapshot_with_mode(device, event_tx, runtime, PictureReadMode::Full);
}

fn emit_device_snapshot_with_mode<D: WorkerDevice>(
    device: &mut D,
    event_tx: &Sender<WorkerEvent>,
    runtime: &mut WorkerRuntimeState,
    picture_read_mode: PictureReadMode,
) -> bool {
    let result = device.snapshot(runtime.selected_monitor_key.as_deref(), picture_read_mode);
    let selected_monitor_missing = result.selected_monitor_missing;
    let ddc_failure = result.ddc_failure;
    let picture_values_deferred = result.picture_values_deferred;
    let _ = event_tx.send(WorkerEvent::Snapshot(result.snapshot));

    if selected_monitor_missing {
        let _ = event_tx.send(WorkerEvent::Status(
            "Selected monitor is unavailable".into(),
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
                    let refreshed = device.snapshot(
                        runtime.selected_monitor_key.as_deref(),
                        PictureReadMode::Full,
                    );
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

    picture_values_deferred
}

fn write_feature(
    monitors: &[WindowsMonitor],
    selected_monitor_key: Option<&str>,
    code: u8,
    value: u32,
) -> Result<(), String> {
    let choices = monitor_choices(monitors);
    let index = strict_selected_monitor_index(&choices, selected_monitor_key)?;
    let Some(monitor) = monitors.get(index) else {
        return Err("Selected monitor is unavailable".into());
    };

    if matches!(code, BRIGHTNESS_CODE | CONTRAST_CODE) {
        let target = monitor.display_target.as_ref().ok_or_else(|| {
            monitor
                .diagnostics
                .display_mapping_error
                .as_ref()
                .map(|error| format!("Windows display mapping is unavailable: {error}"))
                .unwrap_or_else(|| "Windows display mapping is unavailable".into())
        })?;
        let state = hdr::hdr_state(target).map_err(|error| {
            format!("Windows HDR state is unavailable; DDC picture write canceled: {error}")
        })?;
        validate_ddc_picture_write(code, Some(&state))?;
    }

    let queue: CommandQueue<_> = CommandQueue::new(monitor.backend.clone(), RetryPolicy::default());
    queue
        .set(VcpCode::new(code), value)
        .map_err(|error| format!("Set failed: {error}"))
}

fn validate_ddc_picture_write(code: u8, hdr_state: Option<&hdr::HdrState>) -> Result<(), String> {
    if !matches!(code, BRIGHTNESS_CODE | CONTRAST_CODE) {
        return Ok(());
    }

    let state = hdr_state.ok_or_else(|| {
        "Windows HDR state is unavailable; DDC picture write canceled".to_string()
    })?;
    if state.active {
        Err("Windows HDR is active; DDC picture write canceled".into())
    } else {
        Ok(())
    }
}

fn set_hdr_for_monitor(
    monitors: &[WindowsMonitor],
    selected_monitor_key: Option<&str>,
    enabled: bool,
) -> Result<(), String> {
    let choices = monitor_choices(monitors);
    let index = strict_selected_monitor_index(&choices, selected_monitor_key)?;
    let monitor = monitors
        .get(index)
        .ok_or_else(|| "Selected monitor is unavailable".to_string())?;
    let target = monitor.display_target.as_ref().ok_or_else(|| {
        monitor
            .diagnostics
            .display_mapping_error
            .as_ref()
            .map(|error| format!("HDR display mapping is unavailable: {error}"))
            .unwrap_or_else(|| "HDR display mapping is unavailable".into())
    })?;

    hdr::set_hdr_enabled(target, enabled)
        .map(|_| ())
        .map_err(|error| error.to_string())
}

fn set_sdr_content_brightness_for_monitor(
    monitors: &[WindowsMonitor],
    selected_monitor_key: Option<&str>,
    percent: u32,
) -> Result<(), String> {
    let choices = monitor_choices(monitors);
    let index = strict_selected_monitor_index(&choices, selected_monitor_key)?;
    let monitor = monitors
        .get(index)
        .ok_or_else(|| "Selected monitor is unavailable".to_string())?;
    let target = monitor.display_target.as_ref().ok_or_else(|| {
        monitor
            .diagnostics
            .display_mapping_error
            .as_ref()
            .map(|error| format!("Windows display mapping is unavailable: {error}"))
            .unwrap_or_else(|| "Windows display mapping is unavailable".into())
    })?;
    let state = hdr::hdr_state(target).map_err(|error| error.to_string())?;
    if !state.active {
        return Err("Windows HDR is no longer active".into());
    }

    hdr::set_sdr_content_brightness(target, percent)
        .map(|_| ())
        .map_err(|error| error.to_string())
}

fn strict_selected_monitor_index(
    choices: &[MonitorChoice],
    selected_monitor_key: Option<&str>,
) -> Result<usize, String> {
    match selected_monitor_key.filter(|key| !key.is_empty()) {
        Some(key) => choices
            .iter()
            .position(|choice| choice.key == key)
            .ok_or_else(|| "Selected monitor is unavailable".to_string()),
        None if choices.is_empty() => Err("No monitor is selected".into()),
        None => Ok(0),
    }
}

fn build_snapshot(
    monitors: &[WindowsMonitor],
    selected_monitor_key: Option<&str>,
    picture_read_mode: PictureReadMode,
) -> DeviceSnapshot {
    let choices = monitor_choices(monitors);
    let requested_key = selected_monitor_key
        .filter(|key| !key.is_empty())
        .map(str::to_owned);
    let index = match strict_selected_monitor_index(&choices, selected_monitor_key) {
        Ok(index) => index,
        Err(_) if requested_key.is_some() => {
            return unavailable_device_snapshot(
                choices,
                requested_key.unwrap_or_default(),
                "Selected monitor is unavailable",
                diagnostic_status(monitors),
                true,
            );
        }
        Err(_) => {
            return unavailable_device_snapshot(
                choices,
                String::new(),
                "No DDC/CI monitor detected",
                String::new(),
                false,
            );
        }
    };
    let choice = &choices[index];
    let monitor = &monitors[index];

    let hdr = hdr_info(monitor);
    let queue: CommandQueue<_> = CommandQueue::new(monitor.backend.clone(), RetryPolicy::default());
    let (brightness, contrast, picture_backend) = read_picture_features(
        hdr.state.as_ref(),
        picture_read_mode,
        || read_sdr_content_brightness(monitor),
        |code| read_feature(&queue, code),
    );
    let picture_values_deferred = picture_read_mode == PictureReadMode::DeferDdc
        && hdr.state.as_ref().is_some_and(|state| !state.active);
    let input = read_input(monitor, &queue);
    let capability_supports_input = monitor
        .capabilities
        .as_ref()
        .is_some_and(|capabilities| capabilities.supports_vcp(INPUT_CODE));
    let input_enabled = input.available || capability_supports_input;
    let ddc_failure = match picture_backend {
        PictureBackend::Ddc if picture_values_deferred => {
            capability_supports_input && !input.available
        }
        PictureBackend::Ddc => !brightness.available && !contrast.available && !input.available,
        PictureBackend::WindowsSdr | PictureBackend::Unavailable => {
            capability_supports_input && !input.available
        }
    };
    let selected_monitor_key = choice.key.clone();

    DeviceSnapshot {
        snapshot: MonitorSnapshot {
            monitor_title: monitor_heading(
                &monitor.info.description,
                monitor.info.model.as_deref(),
            ),
            input_summary: input.summary,
            hdr_status: hdr.status,
            hdr_enabled: hdr.enabled,
            hdr_available: hdr.available,
            hdr_active: hdr.state.as_ref().is_some_and(|state| state.active),
            picture_backend,
            advanced_color_monitor: monitor.display_monitor,
            diagnostic_status: diagnostic_status(monitors),
            monitor_choices: choices,
            selected_monitor_key,
            brightness,
            contrast,
            selected_input: input.route,
            input_enabled,
            has_monitor: true,
        },
        selected_monitor_missing: false,
        ddc_failure,
        picture_values_deferred,
    }
}

fn unavailable_device_snapshot(
    choices: Vec<MonitorChoice>,
    selected_monitor_key: String,
    monitor_title: &str,
    diagnostic_status: String,
    selected_monitor_missing: bool,
) -> DeviceSnapshot {
    DeviceSnapshot {
        snapshot: MonitorSnapshot {
            monitor_title: monitor_title.into(),
            input_summary: "Input".into(),
            hdr_status: "Windows HDR: unavailable".into(),
            hdr_enabled: false,
            hdr_available: false,
            hdr_active: false,
            picture_backend: PictureBackend::Unavailable,
            advanced_color_monitor: None,
            diagnostic_status,
            monitor_choices: choices,
            selected_monitor_key,
            brightness: unavailable_feature(),
            contrast: unavailable_feature(),
            selected_input: InputRoute::None,
            input_enabled: false,
            has_monitor: false,
        },
        selected_monitor_missing,
        ddc_failure: false,
        picture_values_deferred: false,
    }
}

fn unavailable_feature() -> FeatureSnapshot {
    FeatureSnapshot {
        value: 0,
        maximum: 100,
        available: false,
    }
}

fn read_picture_features(
    hdr_state: Option<&hdr::HdrState>,
    picture_read_mode: PictureReadMode,
    read_sdr: impl FnOnce() -> FeatureSnapshot,
    mut read_ddc: impl FnMut(u8) -> FeatureSnapshot,
) -> (FeatureSnapshot, FeatureSnapshot, PictureBackend) {
    let Some(hdr_state) = hdr_state else {
        return (
            unavailable_feature(),
            unavailable_feature(),
            PictureBackend::Unavailable,
        );
    };

    if hdr_state.active {
        let brightness = read_sdr();
        let backend = if brightness.available {
            PictureBackend::WindowsSdr
        } else {
            PictureBackend::Unavailable
        };
        return (brightness, unavailable_feature(), backend);
    }

    if picture_read_mode == PictureReadMode::DeferDdc {
        return (
            unavailable_feature(),
            unavailable_feature(),
            PictureBackend::Ddc,
        );
    }

    (
        read_ddc(BRIGHTNESS_CODE),
        read_ddc(CONTRAST_CODE),
        PictureBackend::Ddc,
    )
}

fn read_sdr_content_brightness(monitor: &WindowsMonitor) -> FeatureSnapshot {
    let Some(target) = monitor.display_target.as_ref() else {
        return unavailable_feature();
    };
    match hdr::sdr_content_brightness(target) {
        Ok(brightness) => FeatureSnapshot {
            value: brightness.percent,
            maximum: 100,
            available: true,
        },
        Err(error) => {
            debug!(
                "SDR content brightness unavailable for {}: {error}",
                monitor.info.description
            );
            unavailable_feature()
        }
    }
}

fn read_feature(
    queue: &CommandQueue<std::sync::Arc<WindowsDdcBackend>>,
    code: u8,
) -> FeatureSnapshot {
    match queue.get(VcpCode::new(code)) {
        Ok(feature) => {
            let available = true;
            debug!("read vcp code={code:02X}: available={available}");
            FeatureSnapshot {
                value: feature.current.min(feature.maximum),
                maximum: feature.maximum.max(1),
                available,
            }
        }
        Err(_) => {
            let available = false;
            debug!("read vcp code={code:02X}: available={available}");
            FeatureSnapshot {
                value: 0,
                maximum: 100,
                available,
            }
        }
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
                monitor
                    .diagnostics
                    .display_mapping_error
                    .as_ref()
                    .map(|error| format!("{title}: display mapping unavailable ({error})")),
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

struct HdrInfo {
    status: String,
    enabled: bool,
    available: bool,
    state: Option<hdr::HdrState>,
}

fn hdr_info(monitor: &WindowsMonitor) -> HdrInfo {
    let Some(target) = monitor.display_target.as_ref() else {
        return HdrInfo {
            status: "Windows HDR: unavailable".into(),
            enabled: false,
            available: false,
            state: None,
        };
    };
    match hdr::hdr_state(target) {
        Ok(state) => {
            let enabled = state.user_enabled;
            let available = state.supported && !state.limited_by_policy;
            HdrInfo {
                status: hdr_status_text(&state),
                enabled,
                available,
                state: Some(state),
            }
        }
        Err(error) => {
            debug!(
                "HDR state unavailable for {}: {error}",
                monitor.info.description
            );
            HdrInfo {
                status: "Windows HDR: unavailable".into(),
                enabled: false,
                available: false,
                state: None,
            }
        }
    }
}

fn hdr_status_text(state: &hdr::HdrState) -> String {
    let status = if state.limited_by_policy {
        "blocked by policy"
    } else if !state.supported {
        "Not supported"
    } else if state.user_enabled && state.active {
        "On"
    } else if state.user_enabled {
        "On (not active)"
    } else {
        "Off"
    };
    format!("Windows HDR: {status}")
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, sync::mpsc};

    use super::*;

    #[derive(Default)]
    struct RecordingWorkerDevice {
        calls: Vec<String>,
        ddc_failure_snapshots: bool,
        missing_selected_monitor: bool,
        hdr_error: Option<String>,
        sdr_brightness_error: Option<String>,
    }

    impl WorkerDevice for RecordingWorkerDevice {
        fn refresh_monitors(&mut self) -> Result<(), String> {
            self.calls.push("refresh".into());
            Ok(())
        }

        fn snapshot(
            &mut self,
            selected_monitor_key: Option<&str>,
            picture_read_mode: PictureReadMode,
        ) -> DeviceSnapshot {
            let suffix = match picture_read_mode {
                PictureReadMode::Full => "",
                PictureReadMode::DeferDdc => ":defer-ddc",
            };
            self.calls.push(format!(
                "snapshot:{}{suffix}",
                selected_monitor_key.unwrap_or("<none>"),
            ));
            DeviceSnapshot {
                snapshot: test_snapshot(
                    selected_monitor_key
                        .filter(|key| !key.is_empty())
                        .unwrap_or("dell-u4025qw|u4025qw|0"),
                ),
                selected_monitor_missing: self.missing_selected_monitor,
                ddc_failure: self.ddc_failure_snapshots,
                picture_values_deferred: picture_read_mode == PictureReadMode::DeferDdc,
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

        fn set_hdr_enabled(
            &mut self,
            selected_monitor_key: Option<&str>,
            enabled: bool,
        ) -> Result<(), String> {
            self.calls.push(format!(
                "hdr:{}:{enabled}",
                selected_monitor_key.unwrap_or("<none>")
            ));
            match self.hdr_error.take() {
                Some(error) => Err(error),
                None => Ok(()),
            }
        }

        fn set_sdr_content_brightness(
            &mut self,
            selected_monitor_key: Option<&str>,
            percent: u32,
        ) -> Result<(), String> {
            self.calls.push(format!(
                "sdr-brightness:{}:{percent}",
                selected_monitor_key.unwrap_or("<none>")
            ));
            match self.sdr_brightness_error.take() {
                Some(error) => Err(error),
                None => Ok(()),
            }
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
            hdr_enabled: true,
            hdr_available: true,
            hdr_active: true,
            picture_backend: PictureBackend::WindowsSdr,
            advanced_color_monitor: None,
        }
    }

    fn hdr_state(
        supported: bool,
        user_enabled: bool,
        active: bool,
        limited_by_policy: bool,
    ) -> hdr::HdrState {
        hdr::HdrState {
            supported,
            user_enabled,
            active,
            limited_by_policy,
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
    fn queued_windows_brightness_writes_coalesce_to_the_latest_percentage() {
        let mut pending = PendingWorkerRequests::from(vec![
            WorkerRequest::SetSdrContentBrightness { percent: 42 },
            WorkerRequest::SetSdrContentBrightness { percent: 73 },
        ]);

        assert_eq!(
            pending.pop_next_task(),
            Some(WorkerTask::SetSdrContentBrightness { percent: 73 })
        );
        assert_eq!(pending.pop_next_task(), None);
    }

    #[test]
    fn cancelling_picture_writes_preserves_input_writes() {
        let mut pending = PendingWorkerRequests::from(vec![
            WorkerRequest::WriteFeature {
                code: BRIGHTNESS_CODE,
                value: 63,
            },
            WorkerRequest::SetSdrContentBrightness { percent: 48 },
            WorkerRequest::SetInput {
                value: INPUT_HDMI_VALUE,
            },
            WorkerRequest::CancelPictureWrites,
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
    fn active_hdr_picture_reads_only_windows_sdr_brightness() {
        let state = hdr_state(true, true, true, false);
        let calls = RefCell::new(Vec::new());

        let (brightness, contrast, backend) = read_picture_features(
            Some(&state),
            PictureReadMode::Full,
            || {
                calls.borrow_mut().push("windows-sdr");
                FeatureSnapshot {
                    value: 58,
                    maximum: 100,
                    available: true,
                }
            },
            |code| {
                calls.borrow_mut().push(if code == BRIGHTNESS_CODE {
                    "ddc-brightness"
                } else {
                    "ddc-contrast"
                });
                FeatureSnapshot {
                    value: 0,
                    maximum: 100,
                    available: true,
                }
            },
        );

        assert_eq!(*calls.borrow(), vec!["windows-sdr"]);
        assert_eq!(brightness.value, 58);
        assert_eq!(brightness.maximum, 100);
        assert!(!contrast.available);
        assert_eq!(backend, PictureBackend::WindowsSdr);
    }

    #[test]
    fn deferred_inactive_hdr_snapshot_skips_ddc_picture_reads() {
        let state = hdr_state(true, false, false, false);
        let calls = RefCell::new(Vec::new());

        let (brightness, contrast, backend) = read_picture_features(
            Some(&state),
            PictureReadMode::DeferDdc,
            || {
                calls.borrow_mut().push("windows-sdr");
                FeatureSnapshot {
                    value: 0,
                    maximum: 100,
                    available: true,
                }
            },
            |code| {
                calls.borrow_mut().push(if code == BRIGHTNESS_CODE {
                    "ddc-brightness"
                } else {
                    "ddc-contrast"
                });
                FeatureSnapshot {
                    value: 50,
                    maximum: 100,
                    available: true,
                }
            },
        );

        assert!(calls.borrow().is_empty());
        assert!(!brightness.available);
        assert!(!contrast.available);
        assert_eq!(backend, PictureBackend::Ddc);
    }

    #[test]
    fn enabled_but_inactive_hdr_reads_ddc_picture_controls() {
        let state = hdr_state(true, true, false, false);
        let calls = RefCell::new(Vec::new());

        let (_, _, backend) = read_picture_features(
            Some(&state),
            PictureReadMode::Full,
            || {
                calls.borrow_mut().push("windows-sdr");
                FeatureSnapshot {
                    value: 0,
                    maximum: 100,
                    available: true,
                }
            },
            |code| {
                calls.borrow_mut().push(if code == BRIGHTNESS_CODE {
                    "ddc-brightness"
                } else {
                    "ddc-contrast"
                });
                FeatureSnapshot {
                    value: 50,
                    maximum: 100,
                    available: true,
                }
            },
        );

        assert_eq!(*calls.borrow(), vec!["ddc-brightness", "ddc-contrast"]);
        assert_eq!(backend, PictureBackend::Ddc);
    }

    #[test]
    fn unknown_hdr_state_skips_all_picture_reads() {
        let calls = RefCell::new(Vec::new());

        let (brightness, contrast, backend) = read_picture_features(
            None,
            PictureReadMode::Full,
            || {
                calls.borrow_mut().push("windows-sdr");
                FeatureSnapshot {
                    value: 0,
                    maximum: 100,
                    available: true,
                }
            },
            |code| {
                calls.borrow_mut().push(if code == BRIGHTNESS_CODE {
                    "ddc-brightness"
                } else {
                    "ddc-contrast"
                });
                FeatureSnapshot {
                    value: 0,
                    maximum: 100,
                    available: true,
                }
            },
        );

        assert!(calls.borrow().is_empty());
        assert!(!brightness.available);
        assert!(!contrast.available);
        assert_eq!(backend, PictureBackend::Unavailable);
    }

    #[test]
    fn ddc_picture_writes_fail_closed_when_hdr_state_is_active_or_unknown() {
        let active = hdr_state(true, true, true, false);
        let inactive = hdr_state(true, false, false, false);

        assert!(validate_ddc_picture_write(BRIGHTNESS_CODE, Some(&active)).is_err());
        assert!(validate_ddc_picture_write(CONTRAST_CODE, Some(&active)).is_err());
        assert!(validate_ddc_picture_write(BRIGHTNESS_CODE, None).is_err());
        assert!(validate_ddc_picture_write(BRIGHTNESS_CODE, Some(&inactive)).is_ok());
        assert!(validate_ddc_picture_write(INPUT_CODE, None).is_ok());
    }

    #[test]
    fn hdr_requests_are_dispatched_as_worker_tasks() {
        let mut pending = PendingWorkerRequests::from(vec![WorkerRequest::SetHdr {
            enabled: true,
            use_cached_ddc_values: false,
        }]);

        assert_eq!(
            pending.pop_next_task(),
            Some(WorkerTask::SetHdr {
                enabled: true,
                use_cached_ddc_values: false,
            })
        );
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
    fn missing_selected_monitor_reports_unavailability_without_fallback_claim() {
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
            WorkerEvent::Status("Selected monitor is unavailable".into())
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
    fn hdr_status_describes_the_selected_monitor_state() {
        assert_eq!(
            hdr_status_text(&hdr_state(true, true, true, false)),
            "Windows HDR: On"
        );
        assert_eq!(
            hdr_status_text(&hdr_state(true, false, false, false)),
            "Windows HDR: Off"
        );
        assert_eq!(
            hdr_status_text(&hdr_state(true, true, false, false)),
            "Windows HDR: On (not active)"
        );
        assert_eq!(
            hdr_status_text(&hdr_state(true, false, false, true)),
            "Windows HDR: blocked by policy"
        );
    }

    #[test]
    fn hdr_selection_rejects_a_missing_explicit_monitor_without_fallback() {
        let choices = vec![MonitorChoice {
            key: "first-monitor".into(),
            title: "First monitor".into(),
        }];

        let error = strict_selected_monitor_index(&choices, Some("missing-monitor")).unwrap_err();

        assert_eq!(error, "Selected monitor is unavailable");
        assert_eq!(strict_selected_monitor_index(&choices, None).unwrap(), 0);
    }

    #[test]
    fn hdr_task_emits_confirmed_snapshot_before_completion() {
        let (event_tx, event_rx) = mpsc::channel();
        let mut runtime = WorkerRuntimeState {
            selected_monitor_key: Some("selected-monitor".into()),
            consecutive_ddc_failures: 0,
        };
        let mut device = RecordingWorkerDevice::default();

        execute_worker_task(
            WorkerTask::SetHdr {
                enabled: true,
                use_cached_ddc_values: false,
            },
            &mut device,
            &event_tx,
            &mut runtime,
        );

        assert_eq!(
            device.calls,
            vec!["hdr:selected-monitor:true", "snapshot:selected-monitor"]
        );
        assert!(matches!(event_rx.recv().unwrap(), WorkerEvent::Snapshot(_)));
        assert_eq!(
            event_rx.recv().unwrap(),
            WorkerEvent::HdrUpdateFinished {
                enabled: true,
                error: None,
                used_cached_ddc_values: false,
            }
        );
    }

    #[test]
    fn cached_hdr_off_task_completes_before_full_picture_refresh() {
        let (event_tx, event_rx) = mpsc::channel();
        let mut runtime = WorkerRuntimeState {
            selected_monitor_key: Some("selected-monitor".into()),
            consecutive_ddc_failures: 0,
        };
        let mut device = RecordingWorkerDevice::default();

        execute_worker_task(
            WorkerTask::SetHdr {
                enabled: false,
                use_cached_ddc_values: true,
            },
            &mut device,
            &event_tx,
            &mut runtime,
        );

        assert_eq!(
            device.calls,
            vec![
                "hdr:selected-monitor:false",
                "snapshot:selected-monitor:defer-ddc",
                "snapshot:selected-monitor",
            ]
        );
        assert!(matches!(event_rx.recv().unwrap(), WorkerEvent::Snapshot(_)));
        assert_eq!(
            event_rx.recv().unwrap(),
            WorkerEvent::HdrUpdateFinished {
                enabled: false,
                error: None,
                used_cached_ddc_values: true,
            }
        );
        assert!(matches!(event_rx.recv().unwrap(), WorkerEvent::Snapshot(_)));
    }

    #[test]
    fn failed_hdr_task_still_refreshes_confirmed_state_before_error_completion() {
        let (event_tx, event_rx) = mpsc::channel();
        let mut runtime = WorkerRuntimeState {
            selected_monitor_key: Some("missing-monitor".into()),
            consecutive_ddc_failures: 0,
        };
        let mut device = RecordingWorkerDevice {
            hdr_error: Some("Selected monitor is unavailable".into()),
            ..Default::default()
        };

        execute_worker_task(
            WorkerTask::SetHdr {
                enabled: true,
                use_cached_ddc_values: false,
            },
            &mut device,
            &event_tx,
            &mut runtime,
        );

        assert!(matches!(event_rx.recv().unwrap(), WorkerEvent::Snapshot(_)));
        assert_eq!(
            event_rx.recv().unwrap(),
            WorkerEvent::HdrUpdateFinished {
                enabled: true,
                error: Some("HDR update failed: Selected monitor is unavailable".into()),
                used_cached_ddc_values: false,
            }
        );
    }

    #[test]
    fn failed_hdr_change_does_not_claim_cached_restoration() {
        let (event_tx, event_rx) = mpsc::channel();
        let mut runtime = WorkerRuntimeState {
            selected_monitor_key: Some("selected-monitor".into()),
            consecutive_ddc_failures: 0,
        };
        let mut device = RecordingWorkerDevice {
            hdr_error: Some("Windows rejected the change".into()),
            ..Default::default()
        };

        execute_worker_task(
            WorkerTask::SetHdr {
                enabled: false,
                use_cached_ddc_values: true,
            },
            &mut device,
            &event_tx,
            &mut runtime,
        );

        assert!(matches!(event_rx.recv().unwrap(), WorkerEvent::Snapshot(_)));
        assert_eq!(
            event_rx.recv().unwrap(),
            WorkerEvent::HdrUpdateFinished {
                enabled: false,
                error: Some("HDR update failed: Windows rejected the change".into()),
                used_cached_ddc_values: false,
            }
        );
        assert!(event_rx.try_recv().is_err());
    }

    #[test]
    fn sdr_brightness_task_emits_confirmed_snapshot_before_completion() {
        let (event_tx, event_rx) = mpsc::channel();
        let mut runtime = WorkerRuntimeState {
            selected_monitor_key: Some("selected-monitor".into()),
            consecutive_ddc_failures: 0,
        };
        let mut device = RecordingWorkerDevice::default();

        execute_worker_task(
            WorkerTask::SetSdrContentBrightness { percent: 61 },
            &mut device,
            &event_tx,
            &mut runtime,
        );

        assert_eq!(
            device.calls,
            vec![
                "sdr-brightness:selected-monitor:61",
                "snapshot:selected-monitor"
            ]
        );
        assert!(matches!(event_rx.recv().unwrap(), WorkerEvent::Snapshot(_)));
        assert_eq!(
            event_rx.recv().unwrap(),
            WorkerEvent::SdrContentBrightnessFinished {
                requested_percent: 61,
                error: None,
            }
        );
    }

    #[test]
    fn failed_sdr_brightness_task_recovers_snapshot_before_error_completion() {
        let (event_tx, event_rx) = mpsc::channel();
        let mut runtime = WorkerRuntimeState {
            selected_monitor_key: Some("selected-monitor".into()),
            consecutive_ddc_failures: 0,
        };
        let mut device = RecordingWorkerDevice {
            sdr_brightness_error: Some("HDR is no longer active".into()),
            ..Default::default()
        };

        execute_worker_task(
            WorkerTask::SetSdrContentBrightness { percent: 61 },
            &mut device,
            &event_tx,
            &mut runtime,
        );

        assert!(matches!(event_rx.recv().unwrap(), WorkerEvent::Snapshot(_)));
        assert_eq!(
            event_rx.recv().unwrap(),
            WorkerEvent::SdrContentBrightnessFinished {
                requested_percent: 61,
                error: Some("SDR content brightness update failed: HDR is no longer active".into()),
            }
        );
    }

    struct FailingAutostartDevice {
        enabled: bool,
    }

    impl WorkerDevice for FailingAutostartDevice {
        fn refresh_monitors(&mut self) -> Result<(), String> {
            Ok(())
        }

        fn snapshot(
            &mut self,
            _selected_monitor_key: Option<&str>,
            _picture_read_mode: PictureReadMode,
        ) -> DeviceSnapshot {
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

        fn set_hdr_enabled(
            &mut self,
            _selected_monitor_key: Option<&str>,
            _enabled: bool,
        ) -> Result<(), String> {
            unreachable!("set_hdr_enabled is not part of this test")
        }

        fn set_sdr_content_brightness(
            &mut self,
            _selected_monitor_key: Option<&str>,
            _percent: u32,
        ) -> Result<(), String> {
            unreachable!("set_sdr_content_brightness is not part of this test")
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

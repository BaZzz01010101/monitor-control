use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use dell_controller_core::{
    ddc::VcpCode, enumerate_monitors, hdr, profiles::profile_for_model, startup, CommandQueue,
    RetryPolicy, WindowsDdcBackend, WindowsMonitor,
};

use crate::{
    app_controller::{
        FeatureSnapshot, InputRoute, MonitorSnapshot, WorkerEvent, WorkerRequest, BRIGHTNESS_CODE,
        CONTRAST_CODE, INPUT_DP_VALUE, INPUT_HDMI_VALUE, INPUT_USB_C_VALUE,
    },
    monitor_text::{compact_input_label, monitor_heading},
};

const INPUT_CODE: u8 = 0x60;

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
    let mut monitors = Vec::new();

    while let Ok(request) = request_rx.recv() {
        match request {
            WorkerRequest::RefreshAll => match enumerate_monitors() {
                Ok(found) => {
                    monitors = found;
                    let _ = event_tx.send(WorkerEvent::Snapshot(build_snapshot(&monitors)));
                    let _ = event_tx.send(WorkerEvent::Status("Monitors refreshed".into()));
                }
                Err(error) => {
                    monitors.clear();
                    let _ = event_tx.send(WorkerEvent::Snapshot(build_snapshot(&monitors)));
                    let _ = event_tx.send(WorkerEvent::Error(format!(
                        "Monitor refresh failed: {error}"
                    )));
                }
            },
            WorkerRequest::RefreshAutostart => {
                let enabled = startup::is_autostart_enabled().unwrap_or(false);
                let _ = event_tx.send(WorkerEvent::AutostartState {
                    enabled,
                    status: String::new(),
                });
            }
            WorkerRequest::ToggleAutostart => {
                let next = !startup::is_autostart_enabled().unwrap_or(false);
                match startup::set_autostart_enabled(next) {
                    Ok(()) => {
                        let _ = event_tx.send(WorkerEvent::AutostartState {
                            enabled: next,
                            status: if next {
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
                    }
                }
            }
            WorkerRequest::WriteFeature { code, value } => {
                if let Err(message) = write_feature(&monitors, code, value) {
                    let _ = event_tx.send(WorkerEvent::Error(message));
                }
                let _ = event_tx.send(WorkerEvent::Snapshot(build_snapshot(&monitors)));
            }
            WorkerRequest::SetInput { value } => {
                if let Err(message) = write_feature(&monitors, INPUT_CODE, value) {
                    let _ = event_tx.send(WorkerEvent::Error(message));
                } else {
                    let _ = event_tx.send(WorkerEvent::Status(format!("Input set to {value:#X}")));
                }
                let _ = event_tx.send(WorkerEvent::Snapshot(build_snapshot(&monitors)));
            }
        }
    }
}

fn write_feature(monitors: &[WindowsMonitor], code: u8, value: u32) -> Result<(), String> {
    let Some(monitor) = monitors.first() else {
        return Err("No monitor available".into());
    };

    let queue: CommandQueue<_> = CommandQueue::new(monitor.backend.clone(), RetryPolicy::default());
    queue
        .set(VcpCode::new(code), value)
        .map_err(|error| format!("Set failed: {error}"))
}

fn build_snapshot(monitors: &[WindowsMonitor]) -> MonitorSnapshot {
    let Some(monitor) = monitors.first() else {
        return MonitorSnapshot {
            monitor_title: "No DDC/CI monitor detected".into(),
            input_summary: "Input".into(),
            hdr_status: "Windows HDR: unavailable".into(),
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
        };
    };

    let queue: CommandQueue<_> = CommandQueue::new(monitor.backend.clone(), RetryPolicy::default());
    let brightness = read_feature(&queue, BRIGHTNESS_CODE);
    let contrast = read_feature(&queue, CONTRAST_CODE);
    let (input_summary, selected_input) = read_input(monitor, &queue);

    MonitorSnapshot {
        monitor_title: monitor_heading(&monitor.info.description, monitor.info.model.as_deref()),
        input_summary,
        hdr_status: hdr_status_text(),
        brightness,
        contrast,
        selected_input,
        input_enabled: true,
        has_monitor: true,
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

fn read_input(
    monitor: &WindowsMonitor,
    queue: &CommandQueue<std::sync::Arc<WindowsDdcBackend>>,
) -> (String, InputRoute) {
    match queue.get(VcpCode::new(INPUT_CODE)) {
        Ok(feature) => {
            let profile = profile_for_model(monitor.info.model.as_deref());
            let label = profile
                .as_ref()
                .and_then(|profile| profile.control("input"))
                .and_then(|control| control.value_label(feature.current))
                .unwrap_or("unknown");
            (
                compact_input_label(label).into(),
                input_route_from_value(feature.current),
            )
        }
        Err(_) => ("Input".into(), InputRoute::None),
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
        Ok(states) if !states.is_empty() => {
            let state = &states[0];
            format!(
                "Windows HDR: {}",
                if state.enabled {
                    "On"
                } else if state.supported {
                    "Available, off"
                } else {
                    "Not supported"
                }
            )
        }
        _ => "Windows HDR: unavailable".into(),
    }
}

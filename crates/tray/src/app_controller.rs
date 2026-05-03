use crate::value_controls::WriteThrottle;

pub const BRIGHTNESS_CODE: u8 = 0x10;
pub const CONTRAST_CODE: u8 = 0x12;
pub const INPUT_DP_VALUE: u32 = 0x0F;
pub const INPUT_USB_C_VALUE: u32 = 0x19;
pub const INPUT_HDMI_VALUE: u32 = 0x11;
const DEFAULT_LIVE_WRITE_INTERVAL_MS: u64 = 140;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum InputRoute {
    #[default]
    None,
    UsbC,
    DisplayPort,
    Hdmi,
}

impl InputRoute {
    pub const fn to_vcp_value(self) -> Option<u32> {
        match self {
            Self::None => None,
            Self::UsbC => Some(INPUT_USB_C_VALUE),
            Self::DisplayPort => Some(INPUT_DP_VALUE),
            Self::Hdmi => Some(INPUT_HDMI_VALUE),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FeatureId {
    Brightness,
    Contrast,
}

impl FeatureId {
    pub const fn to_vcp_code(self) -> u8 {
        match self {
            Self::Brightness => BRIGHTNESS_CODE,
            Self::Contrast => CONTRAST_CODE,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiAction {
    OpenWindow,
    HideWindow,
    Exit,
    Refresh,
    ToggleAutostart,
    PreviewFeature { feature: FeatureId, value: u32 },
    CommitFeature { feature: FeatureId, value: u32 },
    SetInput(InputRoute),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkerRequest {
    RefreshAll,
    RefreshAutostart,
    ToggleAutostart,
    WriteFeature { code: u8, value: u32 },
    SetInput { value: u32 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeatureSnapshot {
    pub value: u32,
    pub maximum: u32,
    pub available: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MonitorSnapshot {
    pub monitor_title: String,
    pub input_summary: String,
    pub hdr_status: String,
    pub brightness: FeatureSnapshot,
    pub contrast: FeatureSnapshot,
    pub selected_input: InputRoute,
    pub input_enabled: bool,
    pub has_monitor: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkerEvent {
    Snapshot(MonitorSnapshot),
    AutostartState { enabled: bool, status: String },
    Status(String),
    Error(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeatureState {
    pub value: u32,
    pub maximum: u32,
    pub text: String,
    pub enabled: bool,
}

impl Default for FeatureState {
    fn default() -> Self {
        Self {
            value: 0,
            maximum: 100,
            text: "n/a".into(),
            enabled: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiState {
    pub monitor_title: String,
    pub input_summary: String,
    pub hdr_status: String,
    pub brightness: FeatureState,
    pub contrast: FeatureState,
    pub selected_input: InputRoute,
    pub input_enabled: bool,
    pub no_monitor: bool,
    pub status_text: String,
    pub autostart_enabled: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            monitor_title: "No DDC/CI monitor detected".into(),
            input_summary: "Input".into(),
            hdr_status: "Windows HDR: unavailable".into(),
            brightness: FeatureState::default(),
            contrast: FeatureState::default(),
            selected_input: InputRoute::None,
            input_enabled: false,
            no_monitor: true,
            status_text: "Ready".into(),
            autostart_enabled: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ControllerEffect {
    ShowWindow,
    HideWindow,
    Quit,
    Worker(WorkerRequest),
}

#[derive(Debug)]
pub struct AppController {
    state: UiState,
    brightness_throttle: WriteThrottle,
    contrast_throttle: WriteThrottle,
}

impl Default for AppController {
    fn default() -> Self {
        Self {
            state: UiState::default(),
            brightness_throttle: WriteThrottle::new(DEFAULT_LIVE_WRITE_INTERVAL_MS),
            contrast_throttle: WriteThrottle::new(DEFAULT_LIVE_WRITE_INTERVAL_MS),
        }
    }
}

impl AppController {
    pub fn ui_state(&self) -> UiState {
        self.state.clone()
    }

    pub fn handle_action(&mut self, action: UiAction, now_ms: u64) -> Vec<ControllerEffect> {
        match action {
            UiAction::OpenWindow => vec![
                ControllerEffect::ShowWindow,
                ControllerEffect::Worker(WorkerRequest::RefreshAll),
                ControllerEffect::Worker(WorkerRequest::RefreshAutostart),
            ],
            UiAction::HideWindow => vec![ControllerEffect::HideWindow],
            UiAction::Exit => vec![ControllerEffect::Quit],
            UiAction::Refresh => vec![ControllerEffect::Worker(WorkerRequest::RefreshAll)],
            UiAction::ToggleAutostart => {
                vec![ControllerEffect::Worker(WorkerRequest::ToggleAutostart)]
            }
            UiAction::SetInput(route) => route
                .to_vcp_value()
                .filter(|_| self.state.input_enabled)
                .map(|value| {
                    self.state.selected_input = route;
                    ControllerEffect::Worker(WorkerRequest::SetInput { value })
                })
                .into_iter()
                .collect(),
            UiAction::PreviewFeature { feature, value } => {
                self.preview_feature(feature, value, now_ms)
            }
            UiAction::CommitFeature { feature, value } => self.commit_feature(feature, value, now_ms),
        }
    }

    pub fn flush_pending(&mut self, now_ms: u64) -> Vec<ControllerEffect> {
        let mut effects = Vec::new();
        for feature in [FeatureId::Brightness, FeatureId::Contrast] {
            let send = match feature {
                FeatureId::Brightness => self.brightness_throttle.tick(now_ms),
                FeatureId::Contrast => self.contrast_throttle.tick(now_ms),
            };
            if let Some(value) = send {
                effects.push(ControllerEffect::Worker(WorkerRequest::WriteFeature {
                    code: feature.to_vcp_code(),
                    value,
                }));
            }
        }
        effects
    }

    pub fn apply_worker_event(&mut self, event: WorkerEvent) {
        match event {
            WorkerEvent::Snapshot(snapshot) => self.apply_snapshot(snapshot),
            WorkerEvent::AutostartState { enabled, status } => {
                self.state.autostart_enabled = enabled;
                if !status.is_empty() {
                    self.state.status_text = status;
                }
            }
            WorkerEvent::Status(message) | WorkerEvent::Error(message) => {
                self.state.status_text = message;
            }
        }
    }

    fn preview_feature(
        &mut self,
        feature: FeatureId,
        value: u32,
        now_ms: u64,
    ) -> Vec<ControllerEffect> {
        let state = self.feature_state_mut(feature);
        if !state.enabled {
            return Vec::new();
        }

        let clamped = value.min(state.maximum);
        state.value = clamped;
        state.text = clamped.to_string();

        let send_now = match feature {
            FeatureId::Brightness => self.brightness_throttle.schedule(now_ms, clamped),
            FeatureId::Contrast => self.contrast_throttle.schedule(now_ms, clamped),
        };

        send_now
            .map(|value| {
                ControllerEffect::Worker(WorkerRequest::WriteFeature {
                    code: feature.to_vcp_code(),
                    value,
                })
            })
            .into_iter()
            .collect()
    }

    fn commit_feature(
        &mut self,
        feature: FeatureId,
        value: u32,
        now_ms: u64,
    ) -> Vec<ControllerEffect> {
        let state = self.feature_state_mut(feature);
        if !state.enabled {
            return Vec::new();
        }

        let clamped = value.min(state.maximum);
        state.value = clamped;
        state.text = clamped.to_string();

        let value = match feature {
            FeatureId::Brightness => self.brightness_throttle.force(now_ms, clamped),
            FeatureId::Contrast => self.contrast_throttle.force(now_ms, clamped),
        };

        vec![ControllerEffect::Worker(WorkerRequest::WriteFeature {
            code: feature.to_vcp_code(),
            value,
        })]
    }

    fn feature_state_mut(&mut self, feature: FeatureId) -> &mut FeatureState {
        match feature {
            FeatureId::Brightness => &mut self.state.brightness,
            FeatureId::Contrast => &mut self.state.contrast,
        }
    }

    fn apply_snapshot(&mut self, snapshot: MonitorSnapshot) {
        self.state.monitor_title = snapshot.monitor_title;
        self.state.input_summary = snapshot.input_summary;
        self.state.hdr_status = snapshot.hdr_status;
        self.state.brightness = Self::map_feature(snapshot.brightness);
        self.state.contrast = Self::map_feature(snapshot.contrast);
        self.state.selected_input = snapshot.selected_input;
        self.state.input_enabled = snapshot.input_enabled;
        self.state.no_monitor = !snapshot.has_monitor;
    }

    fn map_feature(snapshot: FeatureSnapshot) -> FeatureState {
        if snapshot.available {
            FeatureState {
                value: snapshot.value.min(snapshot.maximum),
                maximum: snapshot.maximum.max(1),
                text: snapshot.value.min(snapshot.maximum).to_string(),
                enabled: true,
            }
        } else {
            FeatureState::default()
        }
    }
}

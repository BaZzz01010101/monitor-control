use crate::value_controls::WriteThrottle;

pub const BRIGHTNESS_CODE: u8 = 0x10;
pub const CONTRAST_CODE: u8 = 0x12;
pub const INPUT_DP_VALUE: u32 = 0x0F;
pub const INPUT_USB_C_VALUE: u32 = 0x19;
pub const INPUT_HDMI_VALUE: u32 = 0x11;
const DEFAULT_LIVE_WRITE_INTERVAL_MS: u64 = 140;
const SNAPSHOT_USER_GUARD_MS: u64 = 1_500;

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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum UiPane {
    #[default]
    Main,
    Settings,
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
    OpenSettings,
    CloseSettings,
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
    ReadSnapshot,
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
    pub active_pane: UiPane,
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
            active_pane: UiPane::Main,
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
    input_throttle: WriteThrottle,
    brightness_optimistic_value: Option<u32>,
    contrast_optimistic_value: Option<u32>,
    brightness_last_user_change_ms: Option<u64>,
    contrast_last_user_change_ms: Option<u64>,
    input_last_user_change_ms: Option<u64>,
}

impl Default for AppController {
    fn default() -> Self {
        Self {
            state: UiState::default(),
            brightness_throttle: WriteThrottle::new(DEFAULT_LIVE_WRITE_INTERVAL_MS),
            contrast_throttle: WriteThrottle::new(DEFAULT_LIVE_WRITE_INTERVAL_MS),
            input_throttle: WriteThrottle::new(DEFAULT_LIVE_WRITE_INTERVAL_MS),
            brightness_optimistic_value: None,
            contrast_optimistic_value: None,
            brightness_last_user_change_ms: None,
            contrast_last_user_change_ms: None,
            input_last_user_change_ms: None,
        }
    }
}

impl AppController {
    pub fn ui_state(&self) -> UiState {
        self.state.clone()
    }

    pub fn handle_action(&mut self, action: UiAction, now_ms: u64) -> Vec<ControllerEffect> {
        match action {
            UiAction::OpenWindow => {
                self.state.active_pane = UiPane::Main;
                vec![
                    ControllerEffect::ShowWindow,
                    ControllerEffect::Worker(WorkerRequest::RefreshAll),
                    ControllerEffect::Worker(WorkerRequest::RefreshAutostart),
                ]
            }
            UiAction::OpenSettings => {
                self.state.active_pane = UiPane::Settings;
                Vec::new()
            }
            UiAction::CloseSettings => {
                self.state.active_pane = UiPane::Main;
                Vec::new()
            }
            UiAction::HideWindow => vec![ControllerEffect::HideWindow],
            UiAction::Exit => vec![ControllerEffect::Quit],
            UiAction::Refresh => vec![ControllerEffect::Worker(WorkerRequest::RefreshAll)],
            UiAction::ToggleAutostart => {
                self.state.autostart_enabled = !self.state.autostart_enabled;
                vec![ControllerEffect::Worker(WorkerRequest::ToggleAutostart)]
            }
            UiAction::SetInput(route) => self.set_input(route, now_ms),
            UiAction::PreviewFeature { feature, value } => {
                self.preview_feature(feature, value, now_ms)
            }
            UiAction::CommitFeature { feature, value } => {
                self.commit_feature(feature, value, now_ms)
            }
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
        if let Some(value) = self.input_throttle.tick(now_ms) {
            effects.push(ControllerEffect::Worker(WorkerRequest::SetInput { value }));
        }
        effects
    }

    pub fn apply_worker_event(&mut self, event: WorkerEvent, now_ms: u64) {
        match event {
            WorkerEvent::Snapshot(snapshot) => self.apply_snapshot(snapshot, now_ms),
            WorkerEvent::AutostartState { enabled, status } => {
                self.state.autostart_enabled = enabled;
                if !status.is_empty() {
                    self.state.status_text = status;
                }
            }
            WorkerEvent::Status(message) => {
                self.state.status_text = message;
            }
            WorkerEvent::Error(message) => {
                self.clear_optimistic_values();
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
        let clamped = {
            let state = self.feature_state_mut(feature);
            if !state.enabled {
                return Vec::new();
            }

            let clamped = value.min(state.maximum);
            state.value = clamped;
            state.text = clamped.to_string();
            clamped
        };
        *self.feature_optimistic_value_mut(feature) = Some(clamped);
        *self.feature_last_user_change_mut(feature) = Some(now_ms);

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
        let clamped = {
            let state = self.feature_state_mut(feature);
            if !state.enabled {
                return Vec::new();
            }

            let clamped = value.min(state.maximum);
            state.value = clamped;
            state.text = clamped.to_string();
            clamped
        };
        *self.feature_optimistic_value_mut(feature) = Some(clamped);
        *self.feature_last_user_change_mut(feature) = Some(now_ms);

        let value = match feature {
            FeatureId::Brightness => self.brightness_throttle.schedule(now_ms, clamped),
            FeatureId::Contrast => self.contrast_throttle.schedule(now_ms, clamped),
        };

        value
            .map(|value| {
                ControllerEffect::Worker(WorkerRequest::WriteFeature {
                    code: feature.to_vcp_code(),
                    value,
                })
            })
            .into_iter()
            .collect()
    }

    fn set_input(&mut self, route: InputRoute, now_ms: u64) -> Vec<ControllerEffect> {
        let Some(value) = route.to_vcp_value() else {
            return Vec::new();
        };
        if !self.state.input_enabled {
            return Vec::new();
        }

        self.state.selected_input = route;
        self.state.input_summary = input_route_summary(route).into();
        self.input_last_user_change_ms = Some(now_ms);

        self.input_throttle
            .schedule(now_ms, value)
            .map(|value| ControllerEffect::Worker(WorkerRequest::SetInput { value }))
            .into_iter()
            .collect()
    }

    fn feature_state_mut(&mut self, feature: FeatureId) -> &mut FeatureState {
        match feature {
            FeatureId::Brightness => &mut self.state.brightness,
            FeatureId::Contrast => &mut self.state.contrast,
        }
    }

    fn apply_snapshot(&mut self, snapshot: MonitorSnapshot, now_ms: u64) {
        let has_monitor = snapshot.has_monitor;
        if !has_monitor {
            self.clear_optimistic_values();
        }

        self.state.monitor_title = snapshot.monitor_title;
        self.state.hdr_status = snapshot.hdr_status;
        self.apply_feature_snapshot(FeatureId::Brightness, snapshot.brightness, now_ms);
        self.apply_feature_snapshot(FeatureId::Contrast, snapshot.contrast, now_ms);
        if self.input_recently_changed(now_ms) {
            self.state.input_enabled = snapshot.input_enabled;
        } else {
            self.state.input_summary = snapshot.input_summary;
            self.state.selected_input = snapshot.selected_input;
            self.state.input_enabled = snapshot.input_enabled;
            self.input_last_user_change_ms = None;
        }
        self.state.no_monitor = !has_monitor;
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

    fn apply_feature_snapshot(
        &mut self,
        feature: FeatureId,
        snapshot: FeatureSnapshot,
        now_ms: u64,
    ) {
        if self.feature_recently_changed(feature, now_ms) {
            if snapshot.available {
                let mapped = Self::map_feature(snapshot);
                let state = self.feature_state_mut(feature);
                state.maximum = mapped.maximum;
                state.enabled = mapped.enabled;
            }
            return;
        }

        if !snapshot.available {
            *self.feature_optimistic_value_mut(feature) = None;
            *self.feature_last_user_change_mut(feature) = None;
            *self.feature_state_mut(feature) = FeatureState::default();
            return;
        }

        let mapped = Self::map_feature(snapshot);
        *self.feature_optimistic_value_mut(feature) = None;
        *self.feature_last_user_change_mut(feature) = None;
        *self.feature_state_mut(feature) = mapped;
    }

    fn feature_optimistic_value_mut(&mut self, feature: FeatureId) -> &mut Option<u32> {
        match feature {
            FeatureId::Brightness => &mut self.brightness_optimistic_value,
            FeatureId::Contrast => &mut self.contrast_optimistic_value,
        }
    }

    fn clear_optimistic_values(&mut self) {
        self.brightness_optimistic_value = None;
        self.contrast_optimistic_value = None;
        self.brightness_last_user_change_ms = None;
        self.contrast_last_user_change_ms = None;
        self.input_last_user_change_ms = None;
    }

    fn feature_last_user_change(&self, feature: FeatureId) -> Option<u64> {
        match feature {
            FeatureId::Brightness => self.brightness_last_user_change_ms,
            FeatureId::Contrast => self.contrast_last_user_change_ms,
        }
    }

    fn feature_last_user_change_mut(&mut self, feature: FeatureId) -> &mut Option<u64> {
        match feature {
            FeatureId::Brightness => &mut self.brightness_last_user_change_ms,
            FeatureId::Contrast => &mut self.contrast_last_user_change_ms,
        }
    }

    fn feature_recently_changed(&self, feature: FeatureId, now_ms: u64) -> bool {
        self.feature_last_user_change(feature)
            .is_some_and(|changed_at| now_ms.saturating_sub(changed_at) < SNAPSHOT_USER_GUARD_MS)
    }

    fn input_recently_changed(&self, now_ms: u64) -> bool {
        self.input_last_user_change_ms
            .is_some_and(|changed_at| now_ms.saturating_sub(changed_at) < SNAPSHOT_USER_GUARD_MS)
    }
}

fn input_route_summary(route: InputRoute) -> &'static str {
    match route {
        InputRoute::None => "Input",
        InputRoute::UsbC => "USB-C",
        InputRoute::DisplayPort => "DP",
        InputRoute::Hdmi => "HDMI",
    }
}

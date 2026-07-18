use crate::persistence::PersistedSettings;
use crate::value_controls::WriteThrottle;
use log::{debug, info};

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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ShortcutTarget {
    UsbC,
    DisplayPort,
    Hdmi,
}

impl ShortcutTarget {
    pub const ALL: [Self; 3] = [Self::UsbC, Self::DisplayPort, Self::Hdmi];

    pub const fn input_route(self) -> InputRoute {
        match self {
            Self::UsbC => InputRoute::UsbC,
            Self::DisplayPort => InputRoute::DisplayPort,
            Self::Hdmi => InputRoute::Hdmi,
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
    ToggleHdr(bool),
    DeactivateShortcutCapture(ShortcutTarget),
    PreviewShortcut {
        target: ShortcutTarget,
        preview: String,
    },
    CommitShortcut {
        target: ShortcutTarget,
        shortcut: String,
    },
    ClearShortcut(ShortcutTarget),
    PersistWindowState,
    SelectMonitor(String),
    PreviewFeature {
        feature: FeatureId,
        value: u32,
    },
    CommitFeature {
        feature: FeatureId,
        value: u32,
    },
    SetInput(InputRoute),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkerRequest {
    RefreshAll,
    ReadSnapshot,
    RefreshAutostart,
    SelectMonitor { key: String },
    SetAutostart { enabled: bool, quiet: bool },
    SetHdr { enabled: bool },
    WriteFeature { code: u8, value: u32 },
    SetInput { value: u32 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MonitorChoice {
    pub key: String,
    pub title: String,
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
    pub hdr_enabled: bool,
    pub hdr_available: bool,
    pub diagnostic_status: String,
    pub monitor_choices: Vec<MonitorChoice>,
    pub selected_monitor_key: String,
    pub brightness: FeatureSnapshot,
    pub contrast: FeatureSnapshot,
    pub selected_input: InputRoute,
    pub input_enabled: bool,
    pub has_monitor: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkerEvent {
    Snapshot(MonitorSnapshot),
    AutostartState {
        enabled: bool,
        status: String,
    },
    HdrUpdateFinished {
        enabled: bool,
        error: Option<String>,
    },
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
pub struct ShortcutFieldState {
    pub value: String,
    pub preview: String,
    pub awaiting_final_key: bool,
    pub error_text: String,
}

impl Default for ShortcutFieldState {
    fn default() -> Self {
        Self {
            value: "None".into(),
            preview: String::new(),
            awaiting_final_key: false,
            error_text: String::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiState {
    pub active_pane: UiPane,
    pub monitor_title: String,
    pub input_summary: String,
    pub hdr_status: String,
    pub hdr_enabled: bool,
    pub hdr_toggle_enabled: bool,
    pub brightness: FeatureState,
    pub contrast: FeatureState,
    pub selected_input: InputRoute,
    pub input_enabled: bool,
    pub no_monitor: bool,
    pub monitor_choices: Vec<MonitorChoice>,
    pub selected_monitor_key: String,
    pub status_text: String,
    pub autostart_enabled: bool,
    pub tb_shortcut: ShortcutFieldState,
    pub dp_shortcut: ShortcutFieldState,
    pub hdmi_shortcut: ShortcutFieldState,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            active_pane: UiPane::Main,
            monitor_title: "No DDC/CI monitor detected".into(),
            input_summary: "Input".into(),
            hdr_status: "Windows HDR: unavailable".into(),
            hdr_enabled: false,
            hdr_toggle_enabled: false,
            brightness: FeatureState::default(),
            contrast: FeatureState::default(),
            selected_input: InputRoute::None,
            input_enabled: false,
            no_monitor: true,
            monitor_choices: Vec::new(),
            selected_monitor_key: String::new(),
            status_text: "Ready".into(),
            autostart_enabled: false,
            tb_shortcut: ShortcutFieldState::default(),
            dp_shortcut: ShortcutFieldState::default(),
            hdmi_shortcut: ShortcutFieldState::default(),
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
    window_hidden: bool,
    brightness_throttle: WriteThrottle,
    contrast_throttle: WriteThrottle,
    input_throttle: WriteThrottle,
    brightness_optimistic_value: Option<u32>,
    contrast_optimistic_value: Option<u32>,
    brightness_last_user_change_ms: Option<u64>,
    contrast_last_user_change_ms: Option<u64>,
    input_last_user_change_ms: Option<u64>,
    durable_selected_monitor_key: String,
    hdr_available: bool,
    hdr_update_pending: bool,
}

impl Default for AppController {
    fn default() -> Self {
        Self {
            state: UiState::default(),
            window_hidden: true,
            brightness_throttle: WriteThrottle::new(DEFAULT_LIVE_WRITE_INTERVAL_MS),
            contrast_throttle: WriteThrottle::new(DEFAULT_LIVE_WRITE_INTERVAL_MS),
            input_throttle: WriteThrottle::new(DEFAULT_LIVE_WRITE_INTERVAL_MS),
            brightness_optimistic_value: None,
            contrast_optimistic_value: None,
            brightness_last_user_change_ms: None,
            contrast_last_user_change_ms: None,
            input_last_user_change_ms: None,
            durable_selected_monitor_key: String::new(),
            hdr_available: false,
            hdr_update_pending: false,
        }
    }
}

impl AppController {
    pub fn ui_state(&self) -> UiState {
        self.state.clone()
    }

    pub fn hydrate_persisted_settings(&mut self, settings: &PersistedSettings) {
        self.state.autostart_enabled = settings.autostart_enabled;
        self.state.selected_monitor_key = settings.selected_monitor_key.clone();
        self.durable_selected_monitor_key = settings.selected_monitor_key.clone();
        self.state.tb_shortcut.value = normalize_shortcut_value(&settings.tb_shortcut);
        self.state.tb_shortcut.preview.clear();
        self.state.tb_shortcut.awaiting_final_key = false;
        self.state.tb_shortcut.error_text.clear();
        self.state.dp_shortcut.value = normalize_shortcut_value(&settings.dp_shortcut);
        self.state.dp_shortcut.preview.clear();
        self.state.dp_shortcut.awaiting_final_key = false;
        self.state.dp_shortcut.error_text.clear();
        self.state.hdmi_shortcut.value = normalize_shortcut_value(&settings.hdmi_shortcut);
        self.state.hdmi_shortcut.preview.clear();
        self.state.hdmi_shortcut.awaiting_final_key = false;
        self.state.hdmi_shortcut.error_text.clear();
    }

    pub fn persisted_settings(&self) -> PersistedSettings {
        PersistedSettings {
            autostart_enabled: self.state.autostart_enabled,
            selected_monitor_key: self.durable_selected_monitor_key.clone(),
            tb_shortcut: self.state.tb_shortcut.value.clone(),
            dp_shortcut: self.state.dp_shortcut.value.clone(),
            hdmi_shortcut: self.state.hdmi_shortcut.value.clone(),
        }
    }

    pub fn shortcut_value(&self, target: ShortcutTarget) -> &str {
        &self.shortcut_state(target).value
    }

    pub fn apply_shortcut_registration_success(
        &mut self,
        target: ShortcutTarget,
        shortcut: String,
        displaced_target: Option<ShortcutTarget>,
    ) {
        let state = self.shortcut_state_mut(target);
        state.value = normalize_shortcut_value(&shortcut);
        state.preview.clear();
        state.awaiting_final_key = false;
        state.error_text.clear();

        if let Some(displaced_target) = displaced_target {
            if displaced_target != target {
                *self.shortcut_state_mut(displaced_target) = ShortcutFieldState::default();
            }
        }
    }

    pub fn apply_shortcut_registration_failure(
        &mut self,
        target: ShortcutTarget,
        previous_value: String,
        error_text: String,
    ) {
        let state = self.shortcut_state_mut(target);
        state.value = normalize_shortcut_value(&previous_value);
        state.preview.clear();
        state.awaiting_final_key = false;
        state.error_text = error_text;
    }

    pub fn apply_shortcut_clear_success(&mut self, target: ShortcutTarget) {
        *self.shortcut_state_mut(target) = ShortcutFieldState::default();
    }

    pub fn handle_action(&mut self, action: UiAction, now_ms: u64) -> Vec<ControllerEffect> {
        debug!("action: {action:?}");
        match action {
            UiAction::OpenWindow => {
                if self.window_hidden {
                    self.window_hidden = false;
                    self.state.active_pane = UiPane::Main;
                    vec![
                        ControllerEffect::ShowWindow,
                        ControllerEffect::Worker(WorkerRequest::RefreshAll),
                    ]
                } else {
                    Vec::new()
                }
            }
            UiAction::OpenSettings => {
                self.state.active_pane = UiPane::Settings;
                Vec::new()
            }
            UiAction::CloseSettings => {
                self.state.active_pane = UiPane::Main;
                Vec::new()
            }
            UiAction::HideWindow => {
                self.window_hidden = true;
                vec![ControllerEffect::HideWindow]
            }
            UiAction::Exit => vec![ControllerEffect::Quit],
            UiAction::Refresh => vec![ControllerEffect::Worker(WorkerRequest::RefreshAll)],
            UiAction::ToggleAutostart => {
                self.state.autostart_enabled = !self.state.autostart_enabled;
                vec![ControllerEffect::Worker(WorkerRequest::SetAutostart {
                    enabled: self.state.autostart_enabled,
                    quiet: false,
                })]
            }
            UiAction::ToggleHdr(enabled) => {
                if !self.state.hdr_toggle_enabled || enabled == self.state.hdr_enabled {
                    return Vec::new();
                }
                self.hdr_update_pending = true;
                self.state.hdr_toggle_enabled = false;
                vec![ControllerEffect::Worker(WorkerRequest::SetHdr { enabled })]
            }
            UiAction::DeactivateShortcutCapture(target) => self.deactivate_shortcut_capture(target),
            UiAction::PreviewShortcut { target, preview } => self.preview_shortcut(target, preview),
            UiAction::CommitShortcut { target, shortcut } => self.commit_shortcut(target, shortcut),
            UiAction::ClearShortcut(target) => self.clear_shortcut(target),
            UiAction::PersistWindowState => Vec::new(),
            UiAction::SelectMonitor(key) => self.select_monitor(key),
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
        debug!("worker event: {event:?}");
        match event {
            WorkerEvent::Snapshot(snapshot) => self.apply_snapshot(snapshot, now_ms),
            WorkerEvent::AutostartState { enabled, status } => {
                self.state.autostart_enabled = enabled;
                if !status.is_empty() {
                    self.state.status_text = status;
                }
            }
            WorkerEvent::HdrUpdateFinished { enabled, error } => {
                self.hdr_update_pending = false;
                self.state.hdr_toggle_enabled = self.hdr_available;
                self.state.status_text = match error {
                    Some(error) => error,
                    None if enabled => "Windows HDR enabled".into(),
                    None => "Windows HDR disabled".into(),
                };
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
        debug!("feature {feature:?}: set to {clamped}");
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
        debug!("feature {feature:?}: set to {clamped}");
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
        debug!("input: {route:?}");
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

    fn deactivate_shortcut_capture(&mut self, target: ShortcutTarget) -> Vec<ControllerEffect> {
        info!("shortcut {target:?}: deactivate capture");
        let state = self.shortcut_state_mut(target);
        state.preview.clear();
        state.awaiting_final_key = false;
        Vec::new()
    }

    fn preview_shortcut(
        &mut self,
        target: ShortcutTarget,
        preview: String,
    ) -> Vec<ControllerEffect> {
        info!("shortcut {target:?}: preview");
        let state = self.shortcut_state_mut(target);
        state.preview = preview;
        state.awaiting_final_key = true;
        state.error_text.clear();
        Vec::new()
    }

    fn commit_shortcut(
        &mut self,
        target: ShortcutTarget,
        shortcut: String,
    ) -> Vec<ControllerEffect> {
        info!("shortcut {target:?}: commit");
        let state = self.shortcut_state_mut(target);
        state.value = normalize_shortcut_value(&shortcut);
        state.preview.clear();
        state.awaiting_final_key = false;
        state.error_text.clear();
        Vec::new()
    }

    fn clear_shortcut(&mut self, target: ShortcutTarget) -> Vec<ControllerEffect> {
        info!("shortcut {target:?}: clear");
        *self.shortcut_state_mut(target) = ShortcutFieldState::default();
        Vec::new()
    }

    fn select_monitor(&mut self, key: String) -> Vec<ControllerEffect> {
        self.state.selected_monitor_key = key.clone();
        self.durable_selected_monitor_key = key.clone();
        self.clear_optimistic_values();
        vec![ControllerEffect::Worker(WorkerRequest::SelectMonitor {
            key,
        })]
    }

    fn shortcut_state(&self, target: ShortcutTarget) -> &ShortcutFieldState {
        match target {
            ShortcutTarget::UsbC => &self.state.tb_shortcut,
            ShortcutTarget::DisplayPort => &self.state.dp_shortcut,
            ShortcutTarget::Hdmi => &self.state.hdmi_shortcut,
        }
    }

    fn shortcut_state_mut(&mut self, target: ShortcutTarget) -> &mut ShortcutFieldState {
        match target {
            ShortcutTarget::UsbC => &mut self.state.tb_shortcut,
            ShortcutTarget::DisplayPort => &mut self.state.dp_shortcut,
            ShortcutTarget::Hdmi => &mut self.state.hdmi_shortcut,
        }
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
        self.state.hdr_enabled = snapshot.hdr_enabled;
        self.hdr_available = snapshot.hdr_available;
        self.state.hdr_toggle_enabled = self.hdr_available && !self.hdr_update_pending;
        self.state.monitor_choices = snapshot.monitor_choices;
        self.state.selected_monitor_key = snapshot.selected_monitor_key;
        if !snapshot.diagnostic_status.is_empty() {
            self.state.status_text = snapshot.diagnostic_status;
        }
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

fn normalize_shortcut_value(value: &str) -> String {
    if value.is_empty() {
        "None".into()
    } else {
        value.into()
    }
}

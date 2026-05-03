use std::path::PathBuf;

use anyhow::Context;
use image::ImageReader;
use tray_icon::{
    menu::{CheckMenuItem, Menu, MenuEvent, MenuItem},
    Icon, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent,
};

use crate::{
    app_controller::UiAction,
    tray_events::{TrayAction, TrayClickTracker},
};

pub struct TrayShell {
    _tray_icon: TrayIcon,
    _menu: Menu,
    open_item: MenuItem,
    refresh_item: MenuItem,
    autostart_item: CheckMenuItem,
    exit_item: MenuItem,
    tray_clicks: TrayClickTracker,
}

impl TrayShell {
    pub fn new() -> anyhow::Result<Self> {
        let open_item = MenuItem::new("Open", true, None);
        let refresh_item = MenuItem::new("Refresh", true, None);
        let autostart_item = CheckMenuItem::new("Autostart", true, false, None);
        let exit_item = MenuItem::new("Exit", true, None);

        let menu = Menu::new();
        menu.append_items(&[&open_item, &refresh_item, &autostart_item, &exit_item])
            .context("failed to build tray menu")?;

        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu.clone()))
            .with_tooltip("Dell Controller")
            .with_menu_on_left_click(false)
            .with_icon(load_tray_icon()?)
            .build()
            .context("failed to create tray icon")?;

        Ok(Self {
            _tray_icon: tray_icon,
            _menu: menu,
            open_item,
            refresh_item,
            autostart_item,
            exit_item,
            tray_clicks: TrayClickTracker::default(),
        })
    }

    pub fn set_autostart_checked(&self, enabled: bool) {
        self.autostart_item.set_checked(enabled);
    }

    pub fn drain_actions(&mut self, now_ms: u64) -> Vec<UiAction> {
        let mut actions = Vec::new();

        while let Ok(event) = MenuEvent::receiver().try_recv() {
            if event.id() == self.open_item.id() {
                actions.push(UiAction::OpenWindow);
            } else if event.id() == self.refresh_item.id() {
                actions.push(UiAction::Refresh);
            } else if event.id() == self.autostart_item.id() {
                actions.push(UiAction::ToggleAutostart);
            } else if event.id() == self.exit_item.id() {
                actions.push(UiAction::Exit);
            }
        }

        while let Ok(event) = TrayIconEvent::receiver().try_recv() {
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } if self.tray_clicks.left_up_at(now_ms) == Some(TrayAction::OpenWindow) => {
                    actions.push(UiAction::OpenWindow);
                }
                TrayIconEvent::DoubleClick {
                    button: MouseButton::Left,
                    ..
                } => actions.push(UiAction::OpenWindow),
                _ => {}
            }
        }

        actions
    }
}

fn load_tray_icon() -> anyhow::Result<Icon> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/icons/monitor_ui.png");
    let image = ImageReader::open(&path)
        .with_context(|| format!("failed to open tray icon {}", path.display()))?
        .decode()
        .with_context(|| format!("failed to decode tray icon {}", path.display()))?
        .into_rgba8();

    let (width, height) = image.dimensions();
    Icon::from_rgba(image.into_raw(), width, height)
        .with_context(|| format!("failed to build tray icon from {}", path.display()))
}

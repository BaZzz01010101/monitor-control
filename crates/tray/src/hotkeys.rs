use log::{info, warn};
use std::collections::HashMap;

use anyhow::Context;
use global_hotkey::{hotkey::HotKey, Error as GlobalHotKeyError, GlobalHotKeyManager};

use crate::app_controller::ShortcutTarget;

#[derive(Clone, Debug)]
struct RegisteredShortcut {
    display_value: String,
    hotkey: HotKey,
}

#[derive(Debug)]
pub struct AssignmentOutcome {
    pub displaced_target: Option<ShortcutTarget>,
}

pub trait HotKeyRegistrar {
    fn register(&self, hotkey: HotKey) -> Result<(), GlobalHotKeyError>;
    fn unregister(&self, hotkey: HotKey) -> Result<(), GlobalHotKeyError>;
}

pub struct GlobalHotKeyRegistrar {
    manager: GlobalHotKeyManager,
}

impl GlobalHotKeyRegistrar {
    pub fn new() -> anyhow::Result<Self> {
        let manager =
            GlobalHotKeyManager::new().context("failed to create global hotkey manager")?;
        Ok(Self { manager })
    }
}

impl HotKeyRegistrar for GlobalHotKeyRegistrar {
    fn register(&self, hotkey: HotKey) -> Result<(), GlobalHotKeyError> {
        self.manager.register(hotkey)
    }

    fn unregister(&self, hotkey: HotKey) -> Result<(), GlobalHotKeyError> {
        self.manager.unregister(hotkey)
    }
}

pub struct ShortcutHotkeys<R = GlobalHotKeyRegistrar> {
    registrar: R,
    registrations: HashMap<ShortcutTarget, RegisteredShortcut>,
    event_targets: HashMap<u32, ShortcutTarget>,
}

impl ShortcutHotkeys<GlobalHotKeyRegistrar> {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self::with_registrar(GlobalHotKeyRegistrar::new()?))
    }
}

impl<R: HotKeyRegistrar> ShortcutHotkeys<R> {
    fn with_registrar(registrar: R) -> Self {
        Self {
            registrar,
            registrations: HashMap::new(),
            event_targets: HashMap::new(),
        }
    }

    pub fn register_shortcut(
        &mut self,
        target: ShortcutTarget,
        shortcut: &str,
    ) -> Result<AssignmentOutcome, String> {
        let normalized_value = normalize_shortcut_value(shortcut);
        let new_hotkey = parse_display_shortcut(&normalized_value)?;
        let current_entry = self.registrations.get(&target).cloned();

        if current_entry
            .as_ref()
            .is_some_and(|entry| entry.hotkey.id() == new_hotkey.id())
        {
            info!("shortcut {target:?} registered as {shortcut}");
            return Ok(AssignmentOutcome {
                displaced_target: None,
            });
        }

        let displaced_target = self
            .event_targets
            .get(&new_hotkey.id())
            .copied()
            .filter(|other| *other != target);
        let displaced_entry =
            displaced_target.and_then(|other| self.registrations.get(&other).cloned());

        if let Some(displaced_entry) = displaced_entry.as_ref() {
            self.registrar
                .unregister(displaced_entry.hotkey)
                .map_err(|error| {
                    let msg = format_registration_error(error);
                    warn!("shortcut {target:?} registration failed: {msg}");
                    msg
                })?;
        }

        if let Err(error) = self.registrar.register(new_hotkey) {
            self.restore_displaced_entry(displaced_entry.as_ref());
            let msg = format_registration_error(error);
            warn!("shortcut {target:?} registration failed: {msg}");
            return Err(msg);
        }

        if let Some(current_entry) = current_entry.as_ref() {
            if let Err(error) = self.registrar.unregister(current_entry.hotkey) {
                let _ = self.registrar.unregister(new_hotkey);
                self.restore_displaced_entry(displaced_entry.as_ref());
                let msg = format_registration_error(error);
                warn!("shortcut {target:?} registration failed: {msg}");
                return Err(msg);
            }
        }

        if let Some(displaced_target) = displaced_target {
            self.remove_registration(displaced_target);
        }
        self.remove_registration(target);
        self.insert_registration(
            target,
            RegisteredShortcut {
                display_value: normalized_value,
                hotkey: new_hotkey,
            },
        );

        info!("shortcut {target:?} registered as {shortcut}");
        Ok(AssignmentOutcome { displaced_target })
    }

    pub fn clear_shortcut(&mut self, target: ShortcutTarget) -> Result<(), String> {
        info!("shortcut {target:?} cleared");
        let Some(entry) = self.registrations.get(&target).cloned() else {
            return Ok(());
        };

        self.registrar
            .unregister(entry.hotkey)
            .map_err(format_registration_error)?;
        self.remove_registration(target);
        Ok(())
    }

    pub fn target_for_event_id(&self, id: u32) -> Option<ShortcutTarget> {
        self.event_targets.get(&id).copied()
    }

    pub fn registered_shortcut_value(&self, target: ShortcutTarget) -> Option<&str> {
        self.registrations
            .get(&target)
            .map(|entry| entry.display_value.as_str())
    }

    fn insert_registration(&mut self, target: ShortcutTarget, entry: RegisteredShortcut) {
        self.event_targets.insert(entry.hotkey.id(), target);
        self.registrations.insert(target, entry);
    }

    fn remove_registration(&mut self, target: ShortcutTarget) {
        if let Some(entry) = self.registrations.remove(&target) {
            self.event_targets.remove(&entry.hotkey.id());
        }
    }

    fn restore_displaced_entry(&self, displaced_entry: Option<&RegisteredShortcut>) {
        if let Some(displaced_entry) = displaced_entry {
            let _ = self.registrar.register(displaced_entry.hotkey);
        }
    }
}

fn parse_display_shortcut(shortcut: &str) -> Result<HotKey, String> {
    if shortcut == "None" {
        return Err("Shortcut must include at least one modifier.".into());
    }

    let parts: Vec<_> = shortcut.split('+').map(str::trim).collect();
    if parts.len() < 2 {
        return Err("Shortcut must include at least one modifier.".into());
    }

    let mut translated_parts = Vec::with_capacity(parts.len());
    for token in &parts[..parts.len() - 1] {
        let translated = match *token {
            "Ctrl" => "ctrl",
            "Alt" => "alt",
            "Shift" => "shift",
            "Win" => "super",
            _ => {
                return Err(format!("Unsupported modifier \"{token}\"."));
            }
        };
        translated_parts.push(translated.to_string());
    }

    translated_parts.push(parts[parts.len() - 1].to_string());
    let parser_string = translated_parts.join("+");
    parser_string
        .parse::<HotKey>()
        .map_err(map_hotkey_parse_error)
}

fn map_hotkey_parse_error(error: global_hotkey::hotkey::HotKeyParseError) -> String {
    use global_hotkey::hotkey::HotKeyParseError;

    match error {
        HotKeyParseError::UnsupportedKey(key) => {
            format!("\"{key}\" is not supported for global shortcuts.")
        }
        HotKeyParseError::EmptyToken(_) | HotKeyParseError::InvalidFormat(_) => {
            "Shortcut must include at least one modifier.".into()
        }
    }
}

fn format_registration_error(error: GlobalHotKeyError) -> String {
    match error {
        GlobalHotKeyError::AlreadyRegistered(_) => {
            "That shortcut is already in use by another application.".into()
        }
        GlobalHotKeyError::FailedToRegister(message) => {
            format!("Failed to register shortcut: {message}")
        }
        other => other.to_string(),
    }
}

fn normalize_shortcut_value(value: &str) -> String {
    if value.is_empty() {
        "None".into()
    } else {
        value.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use global_hotkey::hotkey::{Code, Modifiers};
    use std::cell::RefCell;
    use std::collections::HashSet;

    #[derive(Default)]
    struct FakeRegistrar {
        active: RefCell<HashSet<u32>>,
        fail_on_register: RefCell<HashSet<u32>>,
        fail_on_unregister: RefCell<HashSet<u32>>,
    }

    impl FakeRegistrar {
        fn with_register_failure(hotkey: HotKey) -> Self {
            let registrar = Self::default();
            registrar.fail_on_register.borrow_mut().insert(hotkey.id());
            registrar
        }
    }

    impl HotKeyRegistrar for FakeRegistrar {
        fn register(&self, hotkey: HotKey) -> Result<(), GlobalHotKeyError> {
            if self.fail_on_register.borrow().contains(&hotkey.id()) {
                return Err(GlobalHotKeyError::AlreadyRegistered(hotkey));
            }
            self.active.borrow_mut().insert(hotkey.id());
            Ok(())
        }

        fn unregister(&self, hotkey: HotKey) -> Result<(), GlobalHotKeyError> {
            if self.fail_on_unregister.borrow().contains(&hotkey.id()) {
                return Err(GlobalHotKeyError::FailedToUnRegister(hotkey));
            }
            self.active.borrow_mut().remove(&hotkey.id());
            Ok(())
        }
    }

    #[test]
    fn parsing_display_shortcuts_maps_windows_labels_to_hotkeys() {
        let hotkey = parse_display_shortcut("Ctrl+Alt+Win+Q").expect("shortcut should parse");

        assert_eq!(
            hotkey.mods,
            Modifiers::CONTROL | Modifiers::ALT | Modifiers::SUPER
        );
        assert_eq!(hotkey.key, Code::KeyQ);
    }

    #[test]
    fn parsing_ctrl_alt_pause_maps_to_the_pause_key() {
        let hotkey = parse_display_shortcut("Ctrl+Alt+Pause").expect("shortcut should parse");

        assert_eq!(hotkey.mods, Modifiers::CONTROL | Modifiers::ALT);
        assert_eq!(hotkey.key, Code::Pause);
    }

    #[test]
    fn registering_a_conflicting_shortcut_moves_it_to_the_new_target() {
        let registrar = FakeRegistrar::default();
        let mut hotkeys = ShortcutHotkeys::with_registrar(registrar);

        hotkeys
            .register_shortcut(ShortcutTarget::DisplayPort, "Ctrl+Alt+D")
            .expect("first registration should work");
        hotkeys
            .register_shortcut(ShortcutTarget::Hdmi, "Ctrl+Alt+H")
            .expect("second registration should work");

        let outcome = hotkeys
            .register_shortcut(ShortcutTarget::UsbC, "Ctrl+Alt+H")
            .expect("conflicting registration should move the binding");

        assert_eq!(outcome.displaced_target, Some(ShortcutTarget::Hdmi));
        assert_eq!(
            hotkeys.registered_shortcut_value(ShortcutTarget::UsbC),
            Some("Ctrl+Alt+H")
        );
        assert_eq!(
            hotkeys.registered_shortcut_value(ShortcutTarget::Hdmi),
            None
        );
    }

    #[test]
    fn failed_registration_keeps_the_previous_binding() {
        let failing_hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyQ);
        let registrar = FakeRegistrar::with_register_failure(failing_hotkey);
        let mut hotkeys = ShortcutHotkeys::with_registrar(registrar);

        hotkeys
            .register_shortcut(ShortcutTarget::DisplayPort, "Ctrl+Alt+D")
            .expect("initial registration should work");
        let error = hotkeys
            .register_shortcut(ShortcutTarget::Hdmi, "Ctrl+Alt+Q")
            .expect_err("registration should fail");

        assert_eq!(
            error,
            "That shortcut is already in use by another application."
        );
        assert_eq!(
            hotkeys.registered_shortcut_value(ShortcutTarget::DisplayPort),
            Some("Ctrl+Alt+D")
        );
        assert_eq!(
            hotkeys.registered_shortcut_value(ShortcutTarget::Hdmi),
            None
        );
    }
}

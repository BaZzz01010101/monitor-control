use dell_controller_tray::shortcut_capture::normalize_shortcut_key_with_layout;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyboardLayoutList, HKL};

const LANG_US_ENGLISH: usize = 0x0409;
const LANG_RUSSIAN: usize = 0x0419;

#[test]
fn russian_layout_letters_map_to_us_physical_labels() {
    let Some(layout) = find_loaded_layout(LANG_RUSSIAN) else {
        return;
    };

    assert_eq!(normalize_shortcut_key_with_layout("\u{0439}", layout), "Q");
    assert_eq!(normalize_shortcut_key_with_layout("\u{0446}", layout), "W");
    assert_eq!(normalize_shortcut_key_with_layout("\u{0431}", layout), ",");
}

#[test]
fn shifted_us_symbols_normalize_to_their_base_key_labels() {
    let Some(layout) = find_loaded_layout(LANG_US_ENGLISH) else {
        return;
    };

    assert_eq!(normalize_shortcut_key_with_layout("!", layout), "1");
    assert_eq!(normalize_shortcut_key_with_layout("+", layout), "=");
    assert_eq!(normalize_shortcut_key_with_layout(" ", layout), "Space");
}

fn find_loaded_layout(language_id: usize) -> Option<HKL> {
    let count = unsafe { GetKeyboardLayoutList(None) };
    if count <= 0 {
        return None;
    }

    let mut layouts = vec![HKL::default(); count as usize];
    let loaded = unsafe { GetKeyboardLayoutList(Some(layouts.as_mut_slice())) };
    layouts
        .into_iter()
        .take(loaded.max(0) as usize)
        .find(|layout| ((layout.0 as usize) & 0xffff) == language_id)
}

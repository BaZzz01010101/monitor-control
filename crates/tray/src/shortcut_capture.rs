use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyboardLayout, MapVirtualKeyExW, VkKeyScanExW, HKL, MAPVK_VK_TO_VSC_EX,
};

pub fn normalize_shortcut_key(text: &str) -> String {
    let layout = unsafe { GetKeyboardLayout(0) };
    normalize_shortcut_key_with_layout(text, layout)
}

pub fn normalize_shortcut_key_with_layout(text: &str, layout: HKL) -> String {
    let Some(character) = single_character(text) else {
        return text.to_uppercase();
    };

    normalize_character_with_layout(character, layout).unwrap_or_else(|| text.to_uppercase())
}

fn single_character(text: &str) -> Option<char> {
    let mut characters = text.chars();
    let character = characters.next()?;
    if characters.next().is_some() {
        return None;
    }
    Some(character)
}

fn normalize_character_with_layout(character: char, layout: HKL) -> Option<String> {
    let vk = virtual_key_for_character(character, layout)?;
    let scan_code = unsafe { MapVirtualKeyExW(u32::from(vk), MAPVK_VK_TO_VSC_EX, Some(layout)) };

    us_label_from_scan_code(scan_code)
        .map(str::to_string)
        .or_else(|| us_label_from_virtual_key(vk))
}

fn virtual_key_for_character(character: char, layout: HKL) -> Option<u16> {
    let code_point = u32::from(character);
    if code_point > u32::from(u16::MAX) {
        return None;
    }

    let vk = unsafe { VkKeyScanExW(code_point as u16, layout) };
    if vk == -1 {
        return None;
    }

    Some((vk as u16) & 0x00ff)
}

fn us_label_from_scan_code(scan_code: u32) -> Option<&'static str> {
    match scan_code & 0x00ff {
        0x02 => Some("1"),
        0x03 => Some("2"),
        0x04 => Some("3"),
        0x05 => Some("4"),
        0x06 => Some("5"),
        0x07 => Some("6"),
        0x08 => Some("7"),
        0x09 => Some("8"),
        0x0A => Some("9"),
        0x0B => Some("0"),
        0x0C => Some("-"),
        0x0D => Some("="),
        0x10 => Some("Q"),
        0x11 => Some("W"),
        0x12 => Some("E"),
        0x13 => Some("R"),
        0x14 => Some("T"),
        0x15 => Some("Y"),
        0x16 => Some("U"),
        0x17 => Some("I"),
        0x18 => Some("O"),
        0x19 => Some("P"),
        0x1A => Some("["),
        0x1B => Some("]"),
        0x1E => Some("A"),
        0x1F => Some("S"),
        0x20 => Some("D"),
        0x21 => Some("F"),
        0x22 => Some("G"),
        0x23 => Some("H"),
        0x24 => Some("J"),
        0x25 => Some("K"),
        0x26 => Some("L"),
        0x27 => Some(";"),
        0x28 => Some("'"),
        0x29 => Some("`"),
        0x2B => Some("\\"),
        0x2C => Some("Z"),
        0x2D => Some("X"),
        0x2E => Some("C"),
        0x2F => Some("V"),
        0x30 => Some("B"),
        0x31 => Some("N"),
        0x32 => Some("M"),
        0x33 => Some(","),
        0x34 => Some("."),
        0x35 => Some("/"),
        0x39 => Some("Space"),
        _ => None,
    }
}

fn us_label_from_virtual_key(vk: u16) -> Option<String> {
    let Ok(vk) = u8::try_from(vk) else {
        return None;
    };
    if vk.is_ascii_uppercase() || vk.is_ascii_digit() {
        return Some(char::from(vk).to_string());
    }

    match vk {
        0x20 => Some("Space".into()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_code_mapping_uses_us_physical_labels() {
        assert_eq!(us_label_from_scan_code(0x10), Some("Q"));
        assert_eq!(us_label_from_scan_code(0x27), Some(";"));
        assert_eq!(us_label_from_scan_code(0x39), Some("Space"));
        assert_eq!(us_label_from_scan_code(0x00), None);
    }
}

pub fn monitor_title(description: &str, model: Option<&str>) -> String {
    let description = description.trim();
    let Some(model) = model.map(str::trim).filter(|model| !model.is_empty()) else {
        return description.to_string();
    };

    if contains_ignore_ascii_case(description, model) {
        description.to_string()
    } else {
        format!("{description} ({model})")
    }
}

pub fn monitor_heading(description: &str, model: Option<&str>) -> String {
    let description = description.trim();
    let title = monitor_title(description, model);
    if let Some(model) = model.map(str::trim).filter(|model| !model.is_empty()) {
        return if contains_ignore_ascii_case(description, "dell")
            && !contains_ignore_ascii_case(model, "dell")
        {
            format!("Dell {model}")
        } else {
            model.to_string()
        };
    }

    title
        .split_once(" (")
        .map(|(name, _)| name.trim())
        .unwrap_or(&title)
        .to_string()
}

pub fn compact_input_label(label: &str) -> &'static str {
    let normalized = label.trim().to_ascii_lowercase();
    if normalized.contains("displayport") || normalized == "dp" {
        "DP"
    } else if normalized.contains("usb") || normalized.contains("thunderbolt") {
        "TB"
    } else if normalized.contains("hdmi") {
        "HDMI"
    } else {
        "Input"
    }
}

fn contains_ignore_ascii_case(haystack: &str, needle: &str) -> bool {
    haystack
        .to_ascii_lowercase()
        .contains(&needle.to_ascii_lowercase())
}

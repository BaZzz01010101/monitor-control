use std::path::Path;

fn icon_dimensions(name: &str) -> (u32, u32) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("icons")
        .join(name);
    let image = image::open(&path).unwrap_or_else(|error| {
        panic!("failed to open icon {}: {error}", path.display());
    });
    (image.width(), image.height())
}

#[test]
fn ui_icons_have_enough_resolution_for_their_rendered_sizes() {
    let expectations = [
        ("monitor_ui.png", (224, 144)),
        ("refresh_ui.png", (32, 32)),
        ("brightness_ui.png", (36, 36)),
        ("contrast_ui.png", (36, 36)),
        ("displayport_ui.png", (72, 72)),
        ("hdmi_ui.png", (72, 72)),
        ("thunderbolt_ui.png", (48, 48)),
        ("indicator_selected_ui.png", (48, 48)),
        ("indicator_unselected_ui.png", (48, 48)),
        ("arrow_up_ui.png", (24, 24)),
        ("arrow_down_ui.png", (24, 24)),
    ];

    for (name, (minimum_width, minimum_height)) in expectations {
        let (width, height) = icon_dimensions(name);
        assert!(
            width >= minimum_width,
            "{name} width {width} is below expected minimum {minimum_width}"
        );
        assert!(
            height >= minimum_height,
            "{name} height {height} is below expected minimum {minimum_height}"
        );
    }
}

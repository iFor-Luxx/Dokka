use dokka_scene::Color;

pub(crate) fn windows_color(color: Color) -> windows::UI::Color {
    windows::UI::Color {
        A: color.alpha,
        R: color.red,
        G: color.green,
        B: color.blue,
    }
}

use gpui::*;
pub fn settings() -> impl IntoElement {
    div()
        .size_full()
        .p_10()
        .child("this is settings screen")
}
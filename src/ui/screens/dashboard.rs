use gpui::*;
pub fn dashboard() -> impl IntoElement {
    div()
        .size_full()
        .p_10()
        .child("this is dashboard screen")
}
use gpui::*;
pub fn experiments() -> impl IntoElement {
    div()
        .size_full()
        .p_10()
        .child("this is EExperiments screen")
}
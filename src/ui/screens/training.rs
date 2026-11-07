use gpui::*;
pub fn training() -> impl IntoElement {
    div()
        .size_full()
        .p_10()
        .child("this is training screen , u can train ur model here")
}
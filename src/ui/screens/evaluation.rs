use gpui::*;
pub fn evaluation() -> impl IntoElement {
    div()
        .size_full()
        .p_10()
        .child("this is Evaluation screen")
}
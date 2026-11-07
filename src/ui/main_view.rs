use ely_gpui_component::primitives::IconName;
use ely_gpui_component::shell::{ActivityBar, ActivityItem};
use gpui::*;
use crate::ui::screens::dashboard::dashboard;
use crate::ui::screens::datasets::datasets;
use crate::ui::screens::training::training;
use crate::ui::screens::evaluation::evaluation;
use crate::ui::screens::experiments::experiments;
use crate::ui::screens::settings::settings;

#[derive(Clone, Copy, PartialOrd, PartialEq)]
enum Screen {
    Dashboard,
    Datasets,
    Training,
    Evaluation,
    Experiments,
    Settings,
}
impl Screen {
    fn id(self) -> &'static str {
        match self {
            Screen::Dashboard => "dashboard",
            Screen::Datasets => "datasets",
            Screen::Training => "training",
            Screen::Evaluation => "evaluation",
            Screen::Experiments => "experiment",
            Screen::Settings => "settings",
        }
    }
}
pub struct MainView {
    selected: Screen,
}
impl MainView {
    pub fn new() -> Self {
        Self {
            selected: Screen::Dashboard,
        }
    }
}

impl Render for MainView {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let content = match self.selected {
            Screen::Dashboard => dashboard().into_any_element(),
            Screen::Datasets => datasets().into_any_element(),
            Screen::Training => training().into_any_element(),
            Screen::Evaluation =>evaluation().into_any_element(),
            Screen::Experiments =>experiments().into_any_element(),
            Screen::Settings => settings().into_any_element(),
        };
        let selected = self.selected.id();

        div()
            .flex()
            .size_full()
            .child(
                ActivityBar::new("activity-bar")
                    .item(ActivityItem::new(
                        "dashboard",
                        IconName::LayoutDashboard,
                        "Dashboard",
                    ))
                    .item(ActivityItem::new(
                        "datasets",
                        IconName::Database,
                        "Datasets",
                    ))
                    .item(ActivityItem::new(
                        "training",
                        IconName::Activity,
                        "Training",
                    ))
                    .item(ActivityItem::new(
                        "evaluation",
                        IconName::ChartArea,
                        "Evaluation",
                    ))
                    .item(ActivityItem::new(
                        "experiments",
                        IconName::Network,
                        "Experiments",
                    ))
                    .item(ActivityItem::new(
                        "settings",
                        IconName::Settings,
                        "Settings",
                    ))
                    .selected(selected)
                    .on_select(cx.listener(|this, id: &SharedString, _window, cx| {
                        this.selected = match id.as_ref() {
                            "dashboard" => Screen::Dashboard,
                            "datasets" => Screen::Datasets,
                            "training" => Screen::Training,
                            "evaluation" => Screen::Evaluation,
                            "experiments" => Screen::Experiments,
                            "settings" => Screen::Settings,
                            _ => return,
                        };
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .flex_1()
                    .child(content),
            )
    }
}

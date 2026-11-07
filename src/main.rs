/*
use ely_gpui_component::{
    Assets,
    theme::{Mode, Theme},
};

use gpui::{
    App, Context, IntoElement, Render, Window, WindowOptions,
    div, prelude::*,
};

struct Counter {
    count: i32,
}

impl Render for Counter {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(format!("Count: {}", self.count))
            .child(
                div()
                    .id("increment")
                    .px_4()
                    .py_2()
                    .bg(gpui::rgb(0x2563eb))
                    .text_color(gpui::rgb(0xffffff))
                    .rounded_md()
                    .cursor_pointer()
                    .child("+")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.count += 1;
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .id("decrement")
                    .px_4()
                    .py_2()
                    .bg(gpui::rgb(0xdc2626))
                    .text_color(gpui::rgb(0xffffff))
                    .rounded_md()
                    .cursor_pointer()
                    .child("-")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.count -= 1;
                        cx.notify();
                    })),
            )
    }
}

fn main() {
    gpui_platform::application()
        .with_assets(Assets)
        .run(|cx: &mut App| {
            ely_gpui_component::init(cx)
                .expect("Ely initialization failed");

            Theme::set_mode(Mode::Dark, cx);

            cx.open_window(
                WindowOptions::default(),
                |_, cx| cx.new(|_| Counter { count: 0 }),
            )
                .expect("Failed to open window");
        });
}
*/
/*
use ely_gpui_component::{
    Assets,
    buttons::Button,
    primitives::FocusScope,
    theme::{ActiveTheme, Mode, Theme},
    typography::{Heading, Paragraph},
};
use gpui::{
    App, AppContext, Bounds, Context, FocusHandle, IntoElement, ParentElement, Render, Styled,
    TitlebarOptions, Window, WindowBounds, WindowOptions, px, size,
};

struct Start {
    focus: FocusHandle,
}

impl Render for Start {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let next = if theme.is_dark() {
            Mode::Light
        } else {
            Mode::Dark
        };
        FocusScope::new(&self.focus)
            .root()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_8()
            .bg(theme.colors.bg)
            .child(Heading::h1("Start"))
            .child(Paragraph::new(
                "Every Ely component reads the theme, so one call turns them all.",
            ))
            .child(
                Button::new("mode", format!("{next:?} mode"))
                    .on_click(move |_, _, cx| Theme::set_mode(next, cx)),
            )
    }
}

fn main() {
    gpui_platform::application()
        .with_assets(Assets)
        .run(|cx: &mut App| {
            ely_gpui_component::init(cx).expect("Ely failed to start");
            Theme::set_mode_now(Mode::Dark, cx);
            let bounds = Bounds::centered(None, size(px(640.0), px(400.0)), cx);
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Start".into()),
                    ..Default::default()
                }),
                ..Default::default()
            };
            cx.open_window(options, |window, cx| {
                cx.new(|cx| {
                    let focus = cx.focus_handle();
                    window.focus(&focus, cx);
                    Start { focus }
                })
            })
                .expect("window failed to open");
            cx.activate(true);
        });
}*/
mod ui;

use ely_gpui_component::{
    Assets,
    theme::{Mode, Theme},
};
use gpui::*;
use ui::main_view::MainView;

fn main() {
    gpui_platform::application()
        .with_assets(Assets)
        .run(|cx: &mut App| {
            ely_gpui_component::init(cx).expect("Ely initialization failed");

            Theme::set_mode(Mode::Dark, cx);

            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: point(px(100.0), px(100.0)),
                        size: size(px(1200.0), px(800.0)),
                    })),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Pattre".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| MainView::new()),
            )
            .expect("Failed to open window");
        });
}
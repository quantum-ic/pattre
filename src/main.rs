mod ui;
mod core;

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
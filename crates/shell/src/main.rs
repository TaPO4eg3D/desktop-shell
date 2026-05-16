use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bar::ShellBar;
use compositor::hypr::Hyprland;
use gpui::{
    App, Bounds, Context, FontWeight, Size, Window, WindowBackgroundAppearance, WindowBounds,
    WindowKind, WindowOptions, div, layer_shell::*, point, prelude::*, px, rems, rgba, white,
};
use gpui_platform::application;

fn main() {
    application().run(|cx: &mut App| {
        cx.spawn(async |cx| {
            // Unfortunate workaround for https://github.com/zed-industries/zed/issues/46378
            cx.background_executor()
                .timer(std::time::Duration::from_millis(100))
                .await;

            let mut hypr = Hyprland::new().await;
            cx.spawn(async move |_cx| {
                hypr.active_workspace().await;

                loop {
                    let events = hypr.recv_events().await;
                    println!("Events: {events:?}");

                }
            }).detach();

            cx.update(|cx| {
                let bounds = cx.displays()
                    .first()
                    .map(|display| display.bounds())
                    .expect("Could not find a display, probably because of: https://github.com/zed-industries/zed/issues/46378");

                let height = px(24.);
                let gap = px(36.);

                cx.open_window(
                    WindowOptions {
                        titlebar: None,
                        window_bounds: Some(WindowBounds::Windowed(Bounds {
                            origin: point(gap, px(0.)),
                            size: Size { width: bounds.size.width - gap, height }
                        })),
                        app_id: Some("gpui-layer-shell-example".to_string()),
                        window_background: WindowBackgroundAppearance::Transparent,
                        kind: WindowKind::LayerShell(LayerShellOptions {
                            namespace: "gpui".to_string(),
                            layer: Layer::Top,
                            anchor: Anchor::LEFT | Anchor::RIGHT | Anchor::TOP,
                            margin: None,
                            exclusive_zone: Some(height),
                            keyboard_interactivity: KeyboardInteractivity::None,
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    |_, cx| cx.new(ShellBar::new),
                )
                .unwrap();
            });
        })
        .detach();
    });
}

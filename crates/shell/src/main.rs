use std::{
    path::PathBuf,
    rc::Rc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use bar::ShellBar;
use compositor::hypr;
use gpui::{
    App, Bounds, Context, FontWeight, SharedString, Size, Window, WindowBackgroundAppearance,
    WindowBounds, WindowKind, WindowOptions, div, layer_shell::*, point, prelude::*, px, rems,
    rgba, white,
};
use gpui_component::{Theme, ThemeRegistry};
use gpui_platform::application;

fn init_theme(cx: &mut App, theme_name: &str) {
    assets::Assets::load_fonts(cx).expect("Font load should not fail");
    let theme_name: SharedString = theme_name.into();

    if let Err(err) = ThemeRegistry::watch_dir(PathBuf::from("./themes"), cx, move |cx| {
        if let Some(theme) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
            let mut theme = (*theme).clone();
            theme.font_family = Some("Geist".into());

            Theme::global_mut(cx).apply_config(&Rc::new(theme));
        } else {
            panic!("Unable to find the theme");
        }
    }) {
        panic!("Failed to watch themes directory: {}", err);
    }
}

fn main() {
    application().with_assets(assets::Assets).run(|cx: &mut App| {
        gpui_component::init(cx);
        init_theme(cx, "Tokyo Night");

        cx.spawn(async |cx| {
            // Unfortunate workaround for https://github.com/zed-industries/zed/issues/46378
            cx.background_executor()
                .timer(std::time::Duration::from_millis(100))
                .await;

            let (hypr_controller, hypr_observer) = hypr::init().await;

            cx.update(|cx| {
                let bounds = cx.displays()
                    .first()
                    .map(|display| display.bounds())
                    .expect("Could not find a display, probably because of: https://github.com/zed-industries/zed/issues/46378");

                let height = px(28.);
                let gap = px(36.);

                cx.open_window(
                    WindowOptions {
                        titlebar: None,
                        window_bounds: Some(WindowBounds::Windowed(Bounds {
                            origin: point(gap, px(0.)),
                            size: Size { width: bounds.size.width - gap, height }
                        })),
                        app_id: Some("desktop-shell".to_string()),
                        window_background: WindowBackgroundAppearance::Transparent,
                        kind: WindowKind::LayerShell(LayerShellOptions {
                            namespace: "desktop-shell".to_string(),
                            layer: Layer::Top,
                            anchor: Anchor::LEFT | Anchor::RIGHT | Anchor::TOP,
                            margin: None,
                            exclusive_zone: Some(height),
                            keyboard_interactivity: KeyboardInteractivity::None,
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    |_, cx| cx.new(move |cx| {
                        ShellBar::new(hypr_controller, hypr_observer, cx)
                    }),
                )
                .unwrap();
            });
        })
        .detach();
    });
}

use gpui::{RenderOnce, div};

struct Workspace {
    is_active: bool,
    windows_number: u8,
}

struct Workspaces {
    workspaces: Vec<Workspace>,
}

impl RenderOnce for Workspaces {
    fn render(
        self,
        window: &mut gpui::Window,
        cx: &mut gpui::App,
    ) -> impl gpui::prelude::IntoElement {
        div()
    }
}

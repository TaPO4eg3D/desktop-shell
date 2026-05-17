use compositor::hypr;
use gpui::{
    InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString, Styled, div,
    prelude::FluentBuilder, px, red,
};
use gpui_component::{ActiveTheme, Colorize, StyledExt, black, label::Label, red_100, white};
use smallvec::SmallVec;

#[derive(Clone, Debug)]
pub struct Workspace {
    pub id: i32,
    pub name: SharedString,
    pub windows: u8,

    pub is_active: bool,
}

impl From<hypr::Workspace> for Workspace {
    fn from(value: hypr::Workspace) -> Self {
        Self {
            id: value.id,
            name: value.name.into(),
            windows: value.windows,
            is_active: false,
        }
    }
}

#[derive(IntoElement)]
pub struct WorkspacesComponent {
    workspaces: SmallVec<[Workspace; 10]>,
}

impl WorkspacesComponent {
    pub fn new(workspaces: SmallVec<[Workspace; 10]>) -> Self {
        Self { workspaces }
    }
}

impl RenderOnce for WorkspacesComponent {
    fn render(
        self,
        window: &mut gpui::Window,
        cx: &mut gpui::App,
    ) -> impl gpui::prelude::IntoElement {
        let children = self
            .workspaces
            .iter()
            .filter(|workspace| workspace.id > 0)
            .map(|workspace| {
                let indicators =
                    (0..workspace.windows).map(|_| div().bg(white()).w_full().h(px(1.)));

                div()
                    .id(workspace.id)
                    .relative()
                    .px_2()
                    .font_semibold()
                    .child(workspace.name.clone())
                    .hover(|this| {
                        this.bg(cx.theme().background.darken(0.5))
                            .text_color(cx.theme().foreground)
                    })
                    .text_color(cx.theme().muted_foreground)
                    .when(workspace.is_active, |this| {
                        this.text_color(cx.theme().foreground)
                    })
                    .child(
                        div()
                            .absolute()
                            .bottom(px(1.))
                            .left_0()
                            .flex()
                            .w_full()
                            .gap(px(1.))
                            .children(indicators),
                    )
            });

        div()
            .bg(cx.theme().background)
            .px_1()
            .flex()
            .gap_1()
            .children(children)
    }
}

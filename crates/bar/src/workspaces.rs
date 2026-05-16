use compositor::hypr;
use gpui::{
    IntoElement, ParentElement, RenderOnce, SharedString, Styled, div, prelude::FluentBuilder, red,
};
use gpui_component::{label::Label, red_100, white};
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
        let children = (0..11).map(|i| {
            let workspace = self.workspaces.iter().find(|workspace| workspace.id == i);

            div()
                .when_some(workspace, |this, workspace| {
                    this.child(workspace.name.clone())
                        .when(workspace.is_active, |this| this.text_color(red()))
                })
                .when_none(&workspace, |this| this.child(i.to_string()))
        });

        div().text_color(red()).flex().children(children)
    }
}

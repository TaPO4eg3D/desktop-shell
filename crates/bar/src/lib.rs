use compositor::hypr::{self, HyprlandController, HyprlandObserver};
use gpui::{App, Context, Entity, ParentElement, Render, Styled, div};
use gpui_component::red_100;
use smallvec::SmallVec;

use crate::workspaces::{Workspace, WorkspacesComponent};

mod workspaces;

pub struct ShellBar {
    hypr: HyprlandController,
    workspaces: SmallVec<[Workspace; 10]>,
}

impl ShellBar {
    pub fn new(
        hypr_controller: HyprlandController,
        hypr_observer: HyprlandObserver,
        cx: &mut Context<Self>,
    ) -> Self {
        let value = Self {
            hypr: hypr_controller,
            workspaces: SmallVec::new(),
        };

        value.fetch_workspaces(cx);

        value
    }

    fn fetch_workspaces(&self, cx: &mut Context<Self>) {
        cx.spawn({
            let hypr = self.hypr.clone();

            async move |this, cx| {
                let workspaces = hypr
                    .workspaces()
                    .await
                    .into_iter()
                    .map(Workspace::from)
                    .collect::<SmallVec<_>>();

                this.update(cx, move |this, cx| {
                    this.workspaces = workspaces;

                    cx.notify();
                })
            }
        })
        .detach();
    }
}

impl Render for ShellBar {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::prelude::Context<Self>,
    ) -> impl gpui::prelude::IntoElement {
        div()
            .size_full()
            .flex()
            .bg(red_100())
            .child(WorkspacesComponent::new(self.workspaces.clone()))
    }
}

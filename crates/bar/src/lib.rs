use compositor::hypr::{self, CompositorEvent, HyprlandController, HyprlandObserver};
use gpui::{App, Context, Entity, ParentElement, Render, Styled, div};
use gpui_component::{ActiveTheme, red_100};
use smallvec::SmallVec;

use crate::workspaces::{Workspace, WorkspacesComponent};

mod workspaces;

pub struct ShellBar {
    compositor: HyprlandController,
    workspaces: SmallVec<[Workspace; 10]>,
}

impl ShellBar {
    pub fn new(
        compositor_controller: HyprlandController,
        mut compositor_observer: HyprlandObserver,
        cx: &mut Context<Self>,
    ) -> Self {
        let value = Self {
            compositor: compositor_controller,
            workspaces: SmallVec::new(),
        };
        value.fetch_workspaces(cx);

        cx.spawn(async move |this, cx| {
            loop {
                let events = compositor_observer.recv_events().await;
                for event in events {
                    this.update(cx, |this, cx| this.process_compositor_event(event, cx))
                        .ok();
                }
            }
        })
        .detach();

        value
    }

    fn process_compositor_event(&mut self, event: CompositorEvent, cx: &mut Context<Self>) {
        match event {
            CompositorEvent::ActiveWorkspaceChanged(id) => self.mark_workspace_active(id, cx),
            CompositorEvent::WorkspaceCreated
            | CompositorEvent::WorkspaceRemoved
            | CompositorEvent::WindowClosed
            | CompositorEvent::WindowOpened
            | CompositorEvent::WindowMoved => {
                self.fetch_workspaces(cx);
            }
            CompositorEvent::Unknown => {}
        }
    }

    fn mark_workspace_active(&mut self, id: i32, cx: &mut Context<Self>) {
        self.workspaces
            .iter_mut()
            .for_each(|workspace| workspace.is_active = workspace.id == id);

        cx.notify();
    }

    // fn set_workspace_active(&mut self, id: i32) {}

    fn fetch_workspaces(&self, cx: &mut Context<Self>) {
        cx.spawn({
            let compositor = self.compositor.clone();

            async move |this, cx| {
                let mut workspaces = compositor
                    .workspaces()
                    .await
                    .into_iter()
                    .map(Workspace::from)
                    .collect::<SmallVec<[Workspace; 10]>>();
                workspaces.sort_by_key(|workspace| workspace.id);

                let active_id = compositor.active_workspace_id().await;

                this.update(cx, move |this, cx| {
                    this.workspaces = workspaces;
                    this.mark_workspace_active(active_id, cx);

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
            .font_family(cx.theme().font_family.clone())
            .text_color(cx.theme().foreground)
            .size_full()
            .flex()
            .child(WorkspacesComponent::new(self.workspaces.clone()))
    }
}

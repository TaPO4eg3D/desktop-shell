use gpui::{App, Context, Render, Styled, div};
use gpui_component::red_100;

mod workspaces;

pub struct ShellBar {}

impl ShellBar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for ShellBar {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::prelude::Context<Self>,
    ) -> impl gpui::prelude::IntoElement {
        div().size_full().bg(red_100())
    }
}

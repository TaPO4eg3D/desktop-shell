pub mod hypr;

#[derive(Debug)]
pub enum CompositorEvent {
    WorkspaceChanged(i32),
    WindowOpened,
    WindowClosed,
    Unknown,
}

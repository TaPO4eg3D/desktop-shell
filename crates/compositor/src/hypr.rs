use std::{path::PathBuf, rc::Rc, str::FromStr};

use async_net::unix::UnixStream;
use futures_lite::prelude::*;
use serde::{Deserialize, de::DeserializeOwned};

#[derive(Debug)]
pub enum CompositorEvent {
    ActiveWorkspaceChanged(i32),
    WorkspaceRemoved,
    WorkspaceCreated,
    WindowOpened,
    WindowClosed,
    WindowMoved,
    Unknown,
}

#[derive(Deserialize, Debug)]
pub struct Workspace {
    pub id: i32,
    pub name: String,
    pub monitor: String,
    #[serde(rename = "monitorID")]
    pub monitor_id: i32,
    pub windows: u8,
    #[serde(rename = "hasfullscreen")]
    pub has_fullscreen: bool,
}

/// To save some allocations
#[derive(Deserialize)]
struct WorkspaceIdOnly {
    id: i32,
}

#[derive(Clone)]
struct HyprIPCControl {
    socket_path: PathBuf,
}

impl HyprIPCControl {
    fn new(socket_path: PathBuf) -> Self {
        Self { socket_path }
    }

    async fn json_command<T: DeserializeOwned>(&self, command: &str) -> T {
        let mut buf = Vec::new();
        let mut control_socket = UnixStream::connect(&self.socket_path)
            .await
            .expect("Failed to connect to Hyprland");

        let command = format!("j/{command}");
        control_socket.write_all(command.as_bytes()).await.unwrap();

        let n = control_socket.read_to_end(&mut buf).await.unwrap();
        let msg = str::from_utf8(&buf[..n]).unwrap();

        serde_json::from_str(msg).unwrap()
    }
}

#[derive(Clone)]
pub struct HyprlandController {
    ipc_control: Rc<HyprIPCControl>,
}

impl HyprlandController {
    fn new(ipc_control: HyprIPCControl) -> Self {
        Self {
            ipc_control: Rc::new(ipc_control),
        }
    }

    pub async fn workspaces(&self) -> Vec<Workspace> {
        self.ipc_control.json_command("workspaces").await
    }

    pub async fn active_workspace_id(&self) -> i32 {
        let WorkspaceIdOnly { id } = self
            .ipc_control
            .json_command::<WorkspaceIdOnly>("activeworkspace")
            .await;

        id
    }
}

#[derive(Clone)]
pub struct HyprlandObserver {
    event_socket: UnixStream,
}

impl HyprlandObserver {
    fn new(event_socket: UnixStream) -> Self {
        Self { event_socket }
    }

    pub async fn recv_events(&mut self) -> Vec<CompositorEvent> {
        let mut buf = [0_u8; 1024];

        let result = self
            .event_socket
            .read(&mut buf)
            .await
            .expect("Lost connection with Hyprland");

        let msg = str::from_utf8(&buf[..result]).unwrap();
        let events = msg
            .split('\n')
            .map(|event| {
                let mut iter = event.split(">>");

                let event_name = iter.next().unwrap();
                let event_args = iter.next();

                match event_name {
                    "workspacev2" => {
                        let mut iter = event_args.unwrap().split(",");
                        let workspace_id = iter.next().unwrap();

                        CompositorEvent::ActiveWorkspaceChanged(
                            i32::from_str(workspace_id).unwrap(),
                        )
                    }
                    "createworkspacev2" => CompositorEvent::WorkspaceCreated,
                    "destroyworkspacev2" => CompositorEvent::WorkspaceRemoved,
                    "openwindow" => CompositorEvent::WindowOpened,
                    "closewindow" => CompositorEvent::WindowClosed,
                    "movewindow" => CompositorEvent::WindowMoved,
                    _ => CompositorEvent::Unknown,
                }
            })
            .collect::<Vec<_>>();

        events
    }
}

pub async fn init() -> (HyprlandController, HyprlandObserver) {
    let instance_id = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .expect("It looks like you're not running Hyprland");

    let uid = unsafe { libc::getuid() };
    let socket_dir = PathBuf::from(format!("/run/user/{uid}/hypr/{instance_id}/"));

    let event_socket = UnixStream::connect(socket_dir.join(".socket2.sock"))
        .await
        .expect("Failed to connect to Hyprland");

    let ipc_control = HyprIPCControl::new(socket_dir.join(".socket.sock"));

    let controller = HyprlandController::new(ipc_control);
    let observer = HyprlandObserver::new(event_socket);

    (controller, observer)
}

use std::{path::PathBuf, str::FromStr};

use async_net::unix::UnixStream;
use bytes::BytesMut;
use futures_lite::prelude::*;

use crate::CompositorEvent;

struct HyprIPCControl {}

pub struct Hyprland {
    socket_dir: PathBuf,
    event_socket: UnixStream,
}

impl Hyprland {
    pub async fn new() -> Self {
        let instance_id = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
            .expect("It looks like you're not running Hyprland");

        let uid = unsafe { libc::getuid() };
        let socket_dir = PathBuf::from(format!("/run/user/{uid}/hypr/{instance_id}/"));

        let event_socket = UnixStream::connect(socket_dir.join(".socket2.sock"))
            .await
            .expect("Failed to connect to Hyprland");

        Self {
            socket_dir,
            event_socket,
        }
    }

    pub async fn active_workspace(&mut self) -> u8 {
        let mut buf = Vec::new();
        let mut control_socket = UnixStream::connect(self.socket_dir.join(".socket.sock"))
            .await
            .expect("Failed to connect to Hyprland");

        let command = "j/workspaces";
        control_socket.write_all(command.as_bytes()).await.unwrap();

        let n = control_socket.read_to_end(&mut buf).await.unwrap();
        let msg = str::from_utf8(&buf[..n]).unwrap();
        println!("{msg}");

        0
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

                        CompositorEvent::WorkspaceChanged(i32::from_str(workspace_id).unwrap())
                    }
                    _ => CompositorEvent::Unknown,
                }
            })
            .collect::<Vec<_>>();

        events
    }
}

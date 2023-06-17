use std::{
    fmt::Display,
    io::{BufWriter, Write},
    os::unix::net::UnixStream,
    str::FromStr,
    time::Duration,
};

use crate::{
    add_match,
    connman::{self, NetConnmanManager},
};

use super::time::TimeBlock;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum ConnmanState {
    #[default]
    Offline,
    Idle,
    Ready,
    Online,
}

impl FromStr for ConnmanState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "idle" => Ok(Self::Idle),
            "ready" => Ok(Self::Ready),
            "online" => Ok(Self::Online),
            _ => Ok(Self::Offline),
        }
    }
}
impl Display for ConnmanState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Offline => write!(f, "󰖪 "),
            Self::Idle => write!(f, "󰅤 "),
            Self::Ready => write!(f, "󰖩 "),
            Self::Online => write!(f, "󰅟 "),
        }
    }
}

pub struct ConnmanBlock {
    connected_service: String,
    online: ConnmanState,
    match_token: dbus::channel::Token,
}

impl ConnmanBlock {
    pub fn new(
        system_connection: &calloop_dbus::SyncDBusSource<()>,
        time_block: Option<&mut TimeBlock>,
    ) -> Self {
        let match_token = add_match!(
            system_connection,
            "net.connman",
            "net.connman.Manager",
            "PropertyChanged"
        );

        let connman_proxy =
            system_connection.with_proxy("net.connman", "/", Duration::from_secs(5));

        let online = ConnmanState::from_str(
            connman_proxy
                .get_properties()
                .unwrap()
                .get("State")
                .unwrap()
                .0
                .as_str()
                .unwrap_or_default(),
        )
        .unwrap();

        if matches!(online, ConnmanState::Ready | ConnmanState::Online) {
            if let Some(block) = time_block {
                if block.update_time_ntp {
                    block.update_time();
                }
            }
        }

        let connected_service = connman_proxy
            .get_services()
            .unwrap()
            .into_iter()
            .find(|f| {
                matches!(
                    f.1.get("State").unwrap().0.as_str().unwrap(),
                    "ready" | "online"
                )
            })
            .map(|f| f.1.get("Name").unwrap().0.as_str().unwrap().to_owned())
            .unwrap_or_default();

        Self {
            connected_service,
            online,
            match_token,
        }
    }

    pub fn unregister(&self, system_connection: &calloop_dbus::SyncDBusSource<()>) {
        system_connection.remove_match(self.match_token).unwrap();
    }

    pub fn fmt(&self, f: &mut String) {
        std::fmt::Write::write_fmt(
            f,
            format_args!(
                "{}{}  ",
                self.online,
                match self.online {
                    ConnmanState::Ready | ConnmanState::Online => &self.connected_service,
                    _ => "",
                },
            ),
        )
        .unwrap();
    }

    pub fn fmt_table(&self, f: &mut BufWriter<UnixStream>) -> std::io::Result<()> {
        write!(
            f,
            concat!(include_str!("../table.txt"), "{}\n"),
            self.online, self.connected_service,
        )
    }

    pub fn query_connman(
        &mut self,
        event: dbus::Message,
        dbus: &mut calloop_dbus::SyncDBusSource<()>,
        time_block: Option<&mut TimeBlock>,
    ) -> bool {
        let property: connman::NetConnmanManagerPropertyChanged = event.read_all().unwrap();
        if property.name == "State" {
            let val = property.value.0.as_str().unwrap();
            self.online = ConnmanState::from_str(val).unwrap();

            if matches!(self.online, ConnmanState::Ready | ConnmanState::Online) {
                self.connected_service = dbus
                    .with_proxy("net.connman", "/", Duration::from_secs(5))
                    .get_services()
                    .unwrap()
                    .into_iter()
                    .find(|f| {
                        matches!(
                            f.1.get("State").unwrap().0.as_str().unwrap(),
                            "ready" | "online"
                        )
                    })
                    .map(|f| f.1.get("Name").unwrap().0.as_str().unwrap().to_owned())
                    .unwrap_or_default();
                if let Some(time) = time_block {
                    if !time.is_time_updated && time.update_time_ntp {
                        time.update_time();
                    }
                }
            } else {
                if let Some(time) = time_block {
                    time.is_time_updated = false;
                }
            }

            true
        } else {
            false
        }
    }
}

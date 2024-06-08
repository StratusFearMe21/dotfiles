use std::{
    io::{BufWriter, Write},
    os::unix::net::UnixStream,
    time::Duration,
};

use calloop_dbus::SyncDBusSource;
use dbus::arg::RefArg;

use crate::add_match;
use crate::wireplumber::OrgWireplumberDefaultNode;

pub struct WirePlumberBlock {
    match_token: dbus::channel::Token,
    proxy: dbus::blocking::Proxy<'static, &'static SyncDBusSource<()>>,
    pub max_volume: f64,
    pub volume: f64,
    pub base: f64,
    pub muted: bool,
    pub x_at: f32,
    pub width: f32,
}

impl WirePlumberBlock {
    pub fn new(user_connection: *mut SyncDBusSource<()>, max_volume: f64) -> Self {
        unsafe {
            let match_token = add_match!(&*user_connection, "org.wireplumber.DefaultNode");

            let wireplumber_proxy = (&*user_connection).with_proxy(
                "org.wireplumber.DefaultNode",
                "/",
                Duration::from_secs(5),
            );

            let volume = wireplumber_proxy.volume().unwrap();
            let base = volume.get("base").map_or(1.0, |f| f.as_f64().unwrap());

            Self {
                match_token,
                volume: volume.get("volume").map_or(1.0, |f| f.as_f64().unwrap()) / base,
                base,
                muted: volume.get("mute").map_or(0, |f| f.as_i64().unwrap()) != 0,
                proxy: wireplumber_proxy,
                max_volume,
                x_at: 0.0,
                width: 0.0,
            }
        }
    }

    pub fn unregister(&self, user_connection: &SyncDBusSource<()>) {
        user_connection.remove_match(self.match_token).unwrap();
    }

    pub fn fmt(&self, f: &mut String) {
        f.push(' ');
        f.push_str(self.volume_level());
        std::fmt::Write::write_fmt(f, format_args!("{:.0}% ", self.volume * 100.0)).unwrap();
    }

    pub fn fmt_table(&self, f: &mut BufWriter<UnixStream>) -> std::io::Result<()> {
        write!(f, include_str!("../table.txt"), self.volume_level())?;
        f.write_fmt(format_args!("{:.0}%\n", self.volume * 100.0))
    }

    pub fn adjust_volume(&self, change: f64) {
        self.proxy
            .set_volume((self.volume + change).clamp(0.0, self.max_volume))
            .unwrap();
    }

    pub fn query_default_node(
        &mut self,
        property: crate::mpris::OrgFreedesktopDBusPropertiesPropertiesChanged,
    ) -> bool {
        let mut changed = false;
        if let Some(metadata) = property.changed_properties.get("Volume") {
            changed = true;
            let mut metadata = metadata.0.as_iter().unwrap();
            while let Some(data) = metadata.next() {
                match data.as_str() {
                    Some("base") => {
                        self.base = metadata.next().unwrap().as_f64().unwrap();
                    }
                    Some("volume") => {
                        self.volume = metadata.next().unwrap().as_f64().unwrap() / self.base;
                    }
                    Some("mute") => {
                        self.muted = metadata.next().unwrap().as_i64().unwrap() != 0;
                    }
                    _ => {}
                }
            }
        }
        changed
    }

    #[inline(always)]
    pub fn volume_level(&self) -> &str {
        if self.muted {
            "󰝟 "
        } else if (0.0..0.3).contains(&self.volume) {
            "󰕿 "
        } else if (0.3..0.6).contains(&self.volume) {
            "󰖀 "
        } else {
            "󰕾 "
        }
    }
}

use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    io::{BufWriter, Write},
    os::unix::net::UnixStream,
    time::Duration,
};

use dbus::arg::RefArg;

use crate::{
    add_match,
    upower::{self, BatteryState, BatteryType, OrgFreedesktopUPower, OrgFreedesktopUPowerDevice},
};

macro_rules! match_bat_type {
    ($shared_data:expr) => {
        match $shared_data.bat_type {
            BatteryType::Unknown | BatteryType::Battery => "",
            BatteryType::LinePower | BatteryType::Ups => "󰚥 ",
            BatteryType::Monitor => "󰍹 ",
            BatteryType::Mouse => "󰍽 ",
            BatteryType::Keyboard => "󰌌 ",
            BatteryType::Pda => "󰙈 ",
            BatteryType::Phone => "󰄜 ",
            BatteryType::MediaPlayer => "󰤽 ",
            BatteryType::Tablet => "󰓶 ",
            BatteryType::Computer => "󰇅 ",
            BatteryType::GamingInput => "󰊖 ",
            BatteryType::Pen => "󰏪 ",
            BatteryType::Touchpad => "󰟸 ",
            BatteryType::Modem => "󱂇 ",
            BatteryType::Network => "󰀂 ",
            BatteryType::Headset => "󰋎 ",
            BatteryType::Speakers => "󰓃 ",
            BatteryType::Headphones => "󰋋 ",
            BatteryType::Video => "󰕧 ",
            BatteryType::OtherAudio => "󱡬 ",
            BatteryType::RemoteControl => "󰑔 ",
            BatteryType::Printer => "󰐪 ",
            BatteryType::Scanner => "󰚫 ",
            BatteryType::Camera => "󰄀 ",
            BatteryType::Wearable => "󰖉 ",
            BatteryType::Toy => "󱊈 ",
            BatteryType::BluetoothGeneric => "󰂯 ",
            BatteryType::Last => "󰘁 ",
        }
    };
}
macro_rules! match_battery {
    ($shared_data:expr) => {
        match $shared_data.state {
            BatteryState::Discharging | BatteryState::Unknown => match $shared_data.percentage {
                0..=9 => "󰂎 ",
                10..=19 => "󰁺 ",
                20..=29 => "󰁻 ",
                30..=39 => "󰁼 ",
                40..=49 => "󰁽 ",
                50..=59 => "󰁾 ",
                60..=69 => "󰁿 ",
                70..=79 => "󰂀 ",
                80..=89 => "󰂁 ",
                90..=99 => "󰂂 ",
                _ => "󰁹 ",
            },
            BatteryState::Charging => match $shared_data.percentage {
                0..=9 => "󰢟 ",
                10..=19 => "󰢜 ",
                20..=29 => "󰂆 ",
                30..=39 => "󰂇 ",
                40..=49 => "󰂈 ",
                50..=59 => "󰢝 ",
                60..=69 => "󰂉 ",
                70..=79 => "󰢞 ",
                80..=89 => "󰂊 ",
                90..=99 => "󰂋 ",
                _ => "󰂅 ",
            },
            BatteryState::FullyCharged => "󰂄 ",
            BatteryState::PendingCharge => "󱠴 ",
            BatteryState::PendingDischarge => "󱠵 ",
            BatteryState::Empty => "󰂃 ",
        }
    };
}

pub struct BatteryBlock {
    bat_devices: HashMap<dbus::Path<'static>, BatteryDevice>,
    match_handles: [dbus::channel::Token; 3],
    pub x_at: f32,
    pub width: f32,
}

impl BatteryBlock {
    pub fn new(system_connection: &calloop_dbus::SyncDBusSource<()>) -> Self {
        let match_handles = [
            add_match!(system_connection, "org.freedesktop.UPower"),
            add_match!(system_connection, "org.freedesktop.UPower", "DeviceAdded"),
            add_match!(system_connection, "org.freedesktop.UPower", "DeviceRemoved"),
        ];

        let upower_proxy = system_connection.with_proxy(
            "org.freedesktop.UPower",
            "/org/freedesktop/UPower",
            Duration::from_secs(5),
        );

        let mut shared_data = HashMap::default();

        for i in upower_proxy.enumerate_devices().unwrap() {
            let proxy =
                system_connection.with_proxy("org.freedesktop.UPower", i, Duration::from_secs(5));

            BatteryDevice::insert(proxy, &mut shared_data);
        }

        Self {
            bat_devices: shared_data,
            match_handles,
            x_at: 0.0,
            width: 0.0,
        }
    }

    pub fn unregister(&self, system_connection: &calloop_dbus::SyncDBusSource<()>) {
        for t in self.match_handles {
            system_connection.remove_match(t).unwrap();
        }
    }

    pub fn fmt(&self, f: &mut String) {
        self.bat_devices.values().for_each(|i| {
            std::fmt::Write::write_fmt(
                f,
                format_args!(
                    " {}{}{}% ",
                    match_bat_type!(i),
                    match_battery!(i),
                    i.percentage
                ),
            )
            .unwrap();
            if i.state != BatteryState::Unknown {
                std::fmt::Write::write_fmt(f, format_args!("{:?}{}", i.time, i.time)).unwrap();
            }
            f.push_str(" ");
        });
        f.pop();
    }

    pub fn fmt_table(&self, f: &mut BufWriter<UnixStream>) -> std::io::Result<()> {
        self.bat_devices.values().try_for_each(|i| {
            write!(
                f,
                concat!(include_str!("../table.txt"), "{}%\n"),
                match match_bat_type!(i) {
                    "" => match_battery!(i),
                    t => t,
                },
                i.percentage,
            )?;
            if i.state != BatteryState::Unknown {
                write!(
                    f,
                    concat!(include_str!("../table.txt"), "{:?}\n"),
                    i.time, i.time
                )
            } else {
                Ok(())
            }
        })
    }

    pub fn query_battery(
        &mut self,
        path: dbus::Path<'static>,
        property: upower::OrgFreedesktopDBusPropertiesPropertiesChanged,
    ) {
        if let Some(device) = self.bat_devices.get_mut(&path) {
            if let Some(percentage) = property.changed_properties.get("Percentage") {
                device.percentage = percentage.as_f64().unwrap().floor() as u32;
            }
            if let Some(state) = property.changed_properties.get("State") {
                device.state = BatteryState::from(state.as_u64().unwrap() as u32);
                device.time = TimeTo::Unknown;
            }
            if let Some(time_to_empty) = property.changed_properties.get("TimeToEmpty") {
                let time_to_empty = time_to_empty.as_i64().unwrap();

                if time_to_empty > 0 {
                    device.time = TimeTo::Empty(time_to_empty as f32);
                }
            }
            if let Some(time_to_full) = property.changed_properties.get("TimeToFull") {
                let time_to_full = time_to_full.as_i64().unwrap();

                if time_to_full > 0 {
                    device.time = TimeTo::Full(time_to_full as f32);
                }
            }
        }
    }

    pub fn device_added(
        &mut self,
        event: dbus::Message,
        dbus: &mut calloop_dbus::SyncDBusSource<()>,
    ) {
        let battery: upower::OrgFreedesktopUPowerDeviceAdded = event.read_all().unwrap();
        let proxy = dbus.with_proxy(
            "org.freedesktop.UPower",
            battery.device,
            Duration::from_secs(5),
        );
        BatteryDevice::insert(proxy, &mut self.bat_devices);
    }

    pub fn device_removed(&mut self, event: dbus::Message) {
        let battery: upower::OrgFreedesktopUPowerDeviceRemoved = event.read_all().unwrap();
        self.bat_devices.remove(&battery.device);
    }
}

pub struct BatteryDevice {
    state: BatteryState,
    time: TimeTo,
    bat_type: BatteryType,
    percentage: u32,
}

impl BatteryDevice {
    pub fn insert(
        proxy: dbus::blocking::Proxy<&calloop_dbus::SyncDBusSource<()>>,
        shared_data: &mut HashMap<dbus::Path<'static>, BatteryDevice>,
    ) {
        let bat_type = proxy.type_().unwrap();
        let bat_type = BatteryType::from(bat_type);
        if bat_type != BatteryType::LinePower && bat_type != BatteryType::Unknown {
            let percentage = proxy.percentage().unwrap().floor() as u32;
            let state = BatteryState::from(proxy.state().unwrap());
            let mut time = if matches!(
                state,
                BatteryState::Charging | BatteryState::FullyCharged | BatteryState::PendingCharge
            ) {
                TimeTo::Full(proxy.time_to_full().unwrap_or_default() as f32)
            } else {
                TimeTo::Empty(proxy.time_to_empty().unwrap_or_default() as f32)
            };
            match time {
                TimeTo::Full(c) | TimeTo::Empty(c) => {
                    if c == 0.0 {
                        time = TimeTo::Unknown;
                    }
                }
                _ => unsafe { core::hint::unreachable_unchecked() },
            }
            shared_data.insert(
                proxy.path.into_static(),
                BatteryDevice {
                    percentage,
                    state,
                    time,
                    bat_type,
                },
            );
        }
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
pub enum TimeTo {
    Empty(f32),
    Full(f32),
    #[default]
    Unknown,
}

impl Display for TimeTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeTo::Empty(_) => write!(f, "󰁆"),
            TimeTo::Full(_) => write!(f, "󰁞"),
            TimeTo::Unknown => write!(f, "󰑓"),
        }
    }
}

impl Debug for TimeTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // I could not use a match statement here (sorry)
        match self {
            TimeTo::Empty(c) | TimeTo::Full(c) => {
                if (..0.0).contains(c) {
                    Ok(())
                } else if (0.0..60.0).contains(c) {
                    write!(f, "{:.0} seconds ", c)
                } else if (60.0..3600.0).contains(c) {
                    write!(f, "{:.1} minutes ", c / 60.0)
                } else if (3600.0..86400.0).contains(c) {
                    write!(f, "{:.1} hours ", c / 3600.0)
                } else {
                    write!(f, "{:.1} days ", c / 86400.0)
                }
            }
            TimeTo::Unknown => Ok(()),
        }
    }
}

use std::collections::HashMap;
#[cfg(not(feature = "i3bar"))]
use std::ffi::c_char;
#[cfg(not(feature = "i3bar"))]
use std::ffi::c_int;
use std::ffi::c_void;
#[cfg(not(feature = "i3bar"))]
use std::ffi::CString;
use std::fmt::Debug;
use std::fmt::Display;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::mem::MaybeUninit;
use std::os::unix::net::UnixListener;
use std::os::unix::net::UnixStream;
use std::process::Command;
use std::str::FromStr;
use std::time::Duration;
use std::time::Instant;

use calloop::generic::Generic;
use calloop::ping::Ping;
use calloop::signals::Signal;
use calloop::signals::Signals;
use calloop::timer::Timer;
use calloop::EventLoop;
use calloop::Interest;
use calloop::LoopSignal;
use dbus::arg::RefArg;
use dbus::message::MatchRule;
use nix::sys::inotify::AddWatchFlags;
use nix::sys::inotify::InitFlags;
use nix::sys::inotify::Inotify;
use rand::rngs::OsRng;
use rand::seq::SliceRandom;
use time::format_description;
use time::format_description::modifier::Day;
use time::format_description::modifier::Hour;
use time::format_description::modifier::Minute;
use time::format_description::modifier::Month;
use time::format_description::modifier::Year;
use time::OffsetDateTime;
use time::UtcOffset;
use upower::BatteryState;

use crate::connman::NetConnmanManager;
use crate::upower::BatteryType;
use crate::upower::OrgFreedesktopUPower;
use crate::upower::OrgFreedesktopUPowerDevice;

mod connman;
mod mpris;
mod upower;

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

macro_rules! match_clock {
    ($hour:expr) => {
        unsafe {
            [
                "󱑊 ", // 00:00
                "󱐿 ", // 01:00
                "󱑀 ", // 02:00
                "󱑁 ", // 03:00
                "󱑂 ", // 04:00
                "󱑃 ", // 05:00
                "󱑄 ", // 06:00
                "󱑅 ", // 07:00
                "󱑆 ", // 08:00
                "󱑇 ", // 09:00
                "󱑈 ", // 10:00
                "󱑉 ", // 11:00
                "󱑊 ", // 12:00
                "󱐿 ", // 13:00
                "󱑀 ", // 14:00
                "󱑁 ", // 15:00
                "󱑂 ", // 16:00
                "󱑃 ", // 17:00
                "󱑄 ", // 18:00
                "󱑅 ", // 19:00
                "󱑆 ", // 20:00
                "󱑇 ", // 21:00
                "󱑈 ", // 22:00
                "󱑉 ", // 23:00
            ]
            .get_unchecked($hour as usize)
        }
    };
}

macro_rules! match_volume {
    ($b:expr) => {
        match $b {
            0..=24 => "󰕿 ",
            25..=74 => "󰖀 ",
            _ => "󰕾 ",
        }
    };
}

macro_rules! match_brightness {
    ($b:expr) => {
        match $b {
            0..=15 => "󰃚 ",
            16..=31 => "󰃛 ",
            32..=47 => "󰃜 ",
            48..=63 => "󰃝 ",
            64..=79 => "󰃞 ",
            80..=95 => "󰃟 ",
            _ => "󰃠 ",
        }
    };
}

macro_rules! update_time {
    ($shared_data:expr) => {
        $shared_data.get_shared_data().is_time_updated = true;
        let mut servers = NTP_SERVERS.to_vec();
        servers.shuffle(&mut OsRng);
        let mut args = String::new();
        args.push_str("doas ntpdate ");
        for server in servers {
            args.push_str(server);
            args.push(' ');
        }
        args.pop();
        args.push_str("; doas hwclock -w");
        Command::new("sh").arg("-c").arg(args).spawn().unwrap();
    };
}

pub trait GetSharedData<F: FnMut(Vec<u8>)> {
    fn get_shared_data(&mut self) -> &mut SharedData<F>;
}

pub struct SharedData<F: FnMut(Vec<u8>)> {
    playing: PlaybackStatus,
    song_metadata: (String, String),
    now: OffsetDateTime,
    is_time_updated: bool,
    connected_service: String,
    online: ConnmanState,
    brightness: usize,
    max_brightness: f32,
    bat_devices: HashMap<dbus::Path<'static>, BatteryDevice>,
    signal: LoopSignal,
    callback: F,
}

const TIME_FMT: [format_description::FormatItem; 3] = [
    format_description::FormatItem::Component(format_description::Component::Hour({
        let mut hour = Hour::default();
        hour.is_12_hour_clock = true;
        hour
    })),
    format_description::FormatItem::Literal(b":"),
    format_description::FormatItem::Component(format_description::Component::Minute(
        Minute::default(),
    )),
];

const DATE_FMT: [format_description::FormatItem; 5] = [
    format_description::FormatItem::Component(format_description::Component::Month(
        Month::default(),
    )),
    format_description::FormatItem::Literal(b"/"),
    format_description::FormatItem::Component(format_description::Component::Day(Day::default())),
    format_description::FormatItem::Literal(b"/"),
    format_description::FormatItem::Component(format_description::Component::Year(Year::default())),
];

const NTP_SERVERS: [&str; 18] = [
    "time-a-g.nist.gov",
    "time-b-g.nist.gov",
    "time-c-g.nist.gov",
    "time-d-g.nist.gov",
    "time-e-g.nist.gov",
    "time-a-wwv.nist.gov",
    "time-b-wwv.nist.gov",
    "time-c-wwv.nist.gov",
    "time-d-wwv.nist.gov",
    "time-e-wwv.nist.gov",
    "time-a-b.nist.gov",
    "time-b-b.nist.gov",
    "time-c-b.nist.gov",
    "time-d-b.nist.gov",
    "time-e-b.nist.gov",
    "utcnist.colorado.edu",
    "utcnist2.colorado.edu",
    "utcnist3.colorado.edu",
];

impl<F: FnMut(Vec<u8>)> SharedData<F> {
    pub fn new(signal: LoopSignal, callback: F) -> Self {
        let now = time::OffsetDateTime::now_utc();
        let timezone = tz::TimeZone::local().unwrap();
        let time_offset = timezone.find_current_local_time_type().unwrap().ut_offset();
        let now = now.to_offset(UtcOffset::from_whole_seconds(time_offset).unwrap());
        Self {
            now,
            is_time_updated: false,
            playing: Default::default(),
            song_metadata: Default::default(),
            connected_service: Default::default(),
            online: Default::default(),
            brightness: 0,
            max_brightness: 0.0,
            bat_devices: Default::default(),
            signal,
            callback,
        }
    }
}

impl<F: FnMut(Vec<u8>)> SharedData<F> {
    fn fmt(&self, f: &mut Vec<u8>) -> std::io::Result<()> {
        write!(f, "{}", match_clock!(self.now.hour()))?;
        self.now.format_into(f, TIME_FMT.as_ref()).unwrap();
        f.write_all(b" \xEE\x82\xB1 \xF3\xB0\x83\xB6 ")?;
        self.now.format_into(f, DATE_FMT.as_ref()).unwrap();
        f.write_all(b" \xEE\x82\xB1 ")?;

        write!(
            f,
            "{}{}%  ",
            match_brightness!(self.brightness),
            self.brightness
        )?;

        for i in self.bat_devices.values() {
            write!(
                f,
                "{}{}{}% ",
                match_bat_type!(i),
                match_battery!(i),
                i.percentage
            )?;
            if i.state != BatteryState::Unknown {
                write!(f, "{:?}{}", i.time, i.time)?;
            }
            f.write_all(b"\xEE\x82\xB1 ")?;
        }

        write!(
            f,
            "{}{}  {}",
            self.online,
            match self.online {
                ConnmanState::Ready | ConnmanState::Online => &self.connected_service,
                _ => "",
            },
            self.playing,
        )?;

        if self.playing != PlaybackStatus::Stopped {
            f.write_all(self.song_metadata.0.as_bytes())?;
            f.write_all(b" - ")?;
            f.write_all(self.song_metadata.1.as_bytes())
        } else {
            Ok(())
        }
    }
}

impl<F: FnMut(Vec<u8>)> SharedData<F> {
    fn fmt_table(&self, f: &mut BufWriter<UnixStream>) -> std::io::Result<()> {
        write!(
            f,
            concat!("\n", include_str!("table.txt")),
            match_clock!(self.now.hour()),
        )?;
        self.now.format_into(f, TIME_FMT.as_ref()).unwrap();
        f.write_all(b"\n")?;
        write!(f, include_str!("table.txt"), "󰃶 ")?;
        self.now.format_into(f, DATE_FMT.as_ref()).unwrap();
        f.write_all(b"\n")?;
        write!(
            f,
            concat!(include_str!("table.txt"), "{}%\n"),
            match_brightness!(self.brightness),
            self.brightness
        )?;
        for i in self.bat_devices.values() {
            write!(
                f,
                concat!(include_str!("table.txt"), "{}%\n"),
                match match_bat_type!(i) {
                    "" => match_battery!(i),
                    t => t,
                },
                i.percentage,
            )?;
            if i.state != BatteryState::Unknown {
                write!(
                    f,
                    concat!(include_str!("table.txt"), "{:?}\n"),
                    i.time, i.time
                )?;
            }
        }
        write!(
            f,
            concat!(include_str!("table.txt"), "{}\n"),
            self.online, self.connected_service,
        )?;
        write!(f, include_str!("table.txt"), self.playing,)?;
        f.write_all(self.song_metadata.0.as_bytes())?;
        f.write_all(b" - ")?;
        f.write_all(self.song_metadata.1.as_bytes())
    }
}

struct BatteryDevice {
    state: BatteryState,
    time: TimeTo,
    bat_type: BatteryType,
    percentage: u32,
}

impl BatteryDevice {
    fn insert<F: FnMut(Vec<u8>)>(
        proxy: dbus::blocking::Proxy<&calloop_dbus::DBusSource<()>>,
        shared_data: &mut SharedData<F>,
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
            shared_data.bat_devices.insert(
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum PlaybackStatus {
    /// A track is currently playing.
    Playing,
    /// A track is currently paused.
    Paused,
    /// There is no track currently playing.
    #[default]
    Stopped,
}

impl FromStr for PlaybackStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_lowercase().trim() {
            "playing" => Ok(Self::Playing),
            "paused" => Ok(Self::Paused),
            _ => Ok(Self::Stopped),
        }
    }
}

impl Display for PlaybackStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PlaybackStatus::Playing => write!(f, "󰐊 "),
            PlaybackStatus::Paused => write!(f, "󰏤 "),
            PlaybackStatus::Stopped => write!(f, "󰓛 "),
        }
    }
}

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

#[cfg(not(feature = "i3bar"))]
extern "C" {
    fn onStatus(status: *const c_char);
    fn wl_display_dispatch_pending(display: *mut c_void) -> i32;
    fn wl_display_dispatch(display: *mut c_void) -> i32;
    fn wl_display_flush(display: *mut c_void) -> i32;
    static mut displayFd: c_int;
    static mut display: *mut c_void;
}

#[derive(Copy, Clone, PartialEq, Default)]
enum TimeTo {
    Empty(f32),
    Full(f32),
    #[default]
    Unknown,
}

impl Display for TimeTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeTo::Empty(_) => write!(f, "󰁆 "),
            TimeTo::Full(_) => write!(f, "󰁞 "),
            TimeTo::Unknown => write!(f, "󰑓 "),
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

macro_rules! add_match {
    ($bus:expr,$sender:expr) => {
        $bus.add_match::<upower::OrgFreedesktopDBusPropertiesPropertiesChanged, _>(
            MatchRule::new_signal("org.freedesktop.DBus.Properties", "PropertiesChanged")
                .with_sender($sender),
            |_, _, _| true,
        )
        .unwrap()
    };
    ($bus:expr,$sender:expr,$signal:expr) => {
        $bus.add_match::<upower::OrgFreedesktopDBusPropertiesPropertiesChanged, _>(
            MatchRule::new_signal($sender, $signal).with_sender($sender),
            |_, _, _| true,
        )
        .unwrap()
    };
    ($bus:expr,$sender:expr,$interface:expr,$signal:expr) => {
        $bus.add_match::<upower::OrgFreedesktopDBusPropertiesPropertiesChanged, _>(
            MatchRule::new_signal($interface, $signal).with_sender($sender),
            |_, _, _| true,
        )
        .unwrap()
    };
}

static mut PLAYPAUSE_PING: MaybeUninit<Ping> = MaybeUninit::uninit();

#[no_mangle]
pub unsafe extern "C" fn mpris_play_pause(_: *const c_void, _: *const c_void) {
    PLAYPAUSE_PING.assume_init_ref().ping();
}

macro_rules! write_bar {
    ($self:expr) => {
        let mut string = Vec::new();
        $self.get_shared_data().fmt(&mut string).unwrap();
        ($self.get_shared_data().callback)(string);
    };
}

pub fn insert_into_loop<F: FnMut(Vec<u8>), G: GetSharedData<F>>(
    event_loop: &mut EventLoop<G>,
    shared_data: &mut G,
) {
    let now = Instant::now();
    let timer_start =
        now + Duration::from_secs(60 - shared_data.get_shared_data().now.second() as u64);
    let (user_connection, _): (calloop_dbus::DBusSource<()>, _) =
        calloop_dbus::DBusSource::new_session().unwrap();
    let (system_connection, _): (calloop_dbus::DBusSource<()>, _) =
        calloop_dbus::DBusSource::new_system().unwrap();
    add_match!(user_connection, "org.mpris.MediaPlayer2.playerctld");
    add_match!(system_connection, "org.freedesktop.UPower");
    add_match!(system_connection, "org.freedesktop.UPower", "DeviceAdded");
    add_match!(system_connection, "org.freedesktop.UPower", "DeviceRemoved");
    add_match!(
        system_connection,
        "net.connman",
        "net.connman.Manager",
        "PropertyChanged"
    );

    let brightness_path = {
        std::fs::read_dir("/sys/class/backlight")
            .unwrap()
            .next()
            .map(|f| f.unwrap().path())
    };

    {
        use crate::mpris::OrgMprisMediaPlayer2Player;

        let upower_proxy = system_connection.with_proxy(
            "org.freedesktop.UPower",
            "/org/freedesktop/UPower",
            Duration::from_secs(5),
        );
        let player_proxy = user_connection.with_proxy(
            "org.mpris.MediaPlayer2.playerctld",
            "/org/mpris/MediaPlayer2",
            Duration::from_secs(5),
        );
        let connman_proxy =
            system_connection.with_proxy("net.connman", "/", Duration::from_secs(5));

        shared_data.get_shared_data().playing =
            PlaybackStatus::from_str(&player_proxy.playback_status().unwrap_or_default()).unwrap();

        if let Ok(metadata) = player_proxy.metadata() {
            if let Some(title) = metadata.get("xesam:title") {
                shared_data.get_shared_data().song_metadata.0 = title.as_str().unwrap().to_owned();
            }
            if let Some(artist) = metadata.get("xesam:artist") {
                shared_data.get_shared_data().song_metadata.1 = artist
                    .0
                    .as_iter()
                    .unwrap()
                    .take(1)
                    .map(|f| f.as_str().unwrap_or_default())
                    .next()
                    .unwrap()
                    .to_owned();
            }
        }

        shared_data.get_shared_data().online = ConnmanState::from_str(
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

        if matches!(
            shared_data.get_shared_data().online,
            ConnmanState::Ready | ConnmanState::Online
        ) {
            update_time!(shared_data);
        }

        shared_data.get_shared_data().connected_service = connman_proxy
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

        for i in upower_proxy.enumerate_devices().unwrap() {
            let proxy =
                system_connection.with_proxy("org.freedesktop.UPower", i, Duration::from_secs(5));

            BatteryDevice::insert(proxy, shared_data.get_shared_data());
        }

        if let Some(ref brightness) = brightness_path {
            shared_data.get_shared_data().max_brightness =
                std::fs::read_to_string(brightness.join("max_brightness"))
                    .unwrap()
                    .trim()
                    .parse::<usize>()
                    .unwrap() as f32;

            let mut brightness = std::fs::File::open(brightness.join("brightness")).unwrap();

            let mut br_string = String::new();
            brightness.read_to_string(&mut br_string).unwrap();
            shared_data.get_shared_data().brightness = ((br_string.trim().parse::<f32>().unwrap()
                / shared_data.get_shared_data().max_brightness)
                * 100.0) as _;
        } else {
            shared_data.get_shared_data().brightness = 0;
        }

        write_bar!(shared_data);
    }

    {
        let handle = event_loop.handle();

        let socket_file = dirs::runtime_dir().unwrap().join("rustbar-0");
        let _ = std::fs::remove_file(&socket_file);
        let socket = UnixListener::bind(&socket_file).unwrap();

        handle
            .insert_source(
                Generic::new(socket, Interest::READ, calloop::Mode::Level),
                move |_event, socket, shared_data| {
                    let (file, _) = socket.accept().unwrap();
                    let mut file = BufWriter::new(file);
                    shared_data.get_shared_data().fmt_table(&mut file).unwrap();

                    Ok(calloop::PostAction::Continue)
                },
            )
            .unwrap();

        let notify_instance = Inotify::init(InitFlags::empty()).unwrap();

        let socket_watch = notify_instance
            .add_watch(&socket_file, AddWatchFlags::IN_ALL_EVENTS)
            .unwrap();

        if let Some(ref brightness) = brightness_path {
            let _brightness_watch = notify_instance
                .add_watch(brightness.as_path(), AddWatchFlags::IN_CLOSE_WRITE)
                .unwrap();
        }

        let brightness_path = unsafe { brightness_path.unwrap_unchecked() };
        handle
            .insert_source(
                Generic::new(notify_instance, Interest::BOTH, calloop::Mode::Level),
                move |_, notify, data| {
                    for e in notify.read_events().unwrap() {
                        if e.wd == socket_watch {
                            data.get_shared_data().signal.stop();
                        } else {
                            let br_string =
                                std::fs::read_to_string(brightness_path.join("brightness"))
                                    .unwrap();
                            data.get_shared_data().brightness =
                                ((br_string.trim().parse::<f32>().unwrap()
                                    / data.get_shared_data().max_brightness)
                                    * 100.0) as _;

                            write_bar!(data);
                        }
                    }
                    Ok(calloop::PostAction::Continue)
                },
            )
            .unwrap();

        handle
            .insert_source(
                Signals::new(&[Signal::SIGINT, Signal::SIGTERM]).unwrap(),
                move |_, _, data| {
                    std::fs::remove_file(&socket_file).unwrap();
                    data.get_shared_data().signal.stop();
                },
            )
            .unwrap();
        // The only child process we spawn is ntpdate ever
        handle
            .insert_source(
                Signals::new(&[Signal::SIGCHLD]).unwrap(),
                move |_, _, shared_data| {
                    let now = time::OffsetDateTime::now_utc();
                    let timezone = tz::TimeZone::local().unwrap();
                    let time_offset = timezone.find_current_local_time_type().unwrap().ut_offset();
                    shared_data.get_shared_data().now =
                        now.to_offset(UtcOffset::from_whole_seconds(time_offset).unwrap());
                    write_bar!(shared_data);
                },
            )
            .unwrap();
        handle
            .insert_source(
                Timer::from_deadline(timer_start),
                |_event, _metadata, shared_data| {
                    shared_data.get_shared_data().now += time::Duration::minutes(1);
                    write_bar!(shared_data);
                    calloop::timer::TimeoutAction::ToDuration(Duration::from_secs(60))
                },
            )
            .unwrap();
        #[cfg(not(feature = "i3bar"))]
        unsafe {
            let (ping, ping_source) = calloop::ping::make_ping().unwrap();
            PLAYPAUSE_PING.write(ping);

            let user_conn: *const calloop_dbus::DBusSource<()> = &user_connection as *const _;

            let proxy = (*user_conn).with_proxy(
                "org.mpris.MediaPlayer2.playerctld",
                "/org/mpris/MediaPlayer2",
                Duration::from_secs(5),
            );

            handle
                .insert_source(ping_source, move |_, _, _| {
                    let _ = crate::mpris::OrgMprisMediaPlayer2Player::play_pause(&proxy);
                })
                .unwrap();
        }
        handle
            .insert_source(user_connection, |event, _metadata, shared_data| {
                let Some(member) = event.member() else {
                    return None;
                };
                if &*member == "PropertiesChanged" {
                    let property: mpris::OrgFreedesktopDBusPropertiesPropertiesChanged =
                        event.read_all().unwrap();
                    if let Some(metadata) = property.changed_properties.get("Metadata") {
                        let mut metadata = metadata.0.as_iter().unwrap();
                        while let Some(data) = metadata.next() {
                            match data.as_str() {
                                Some("xesam:title") => {
                                    shared_data.get_shared_data().song_metadata.0 =
                                        metadata.next().unwrap().as_str().unwrap().to_owned();
                                }
                                Some("xesam:artist") => {
                                    shared_data.get_shared_data().song_metadata.1 = metadata
                                        .next()
                                        .unwrap()
                                        .as_iter()
                                        .unwrap()
                                        .next()
                                        .unwrap()
                                        .as_iter()
                                        .unwrap()
                                        .take(1)
                                        .map(|f| f.as_str().unwrap_or_default())
                                        .next()
                                        .unwrap()
                                        .to_owned();
                                }
                                _ => {}
                            }
                        }
                    }
                    if let Some(playback) = property.changed_properties.get("PlaybackStatus") {
                        shared_data.get_shared_data().playing =
                            PlaybackStatus::from_str(playback.as_str().unwrap()).unwrap();

                        if shared_data.get_shared_data().playing == PlaybackStatus::Stopped {
                            shared_data.get_shared_data().song_metadata = Default::default();
                        }
                    }
                    write_bar!(shared_data);
                }
                None
            })
            .unwrap();
        handle
            .insert_source(system_connection, |event, dbus, shared_data| {
                let Some(member) = event.member() else {
                    return None;
                };
                if &*member == "PropertiesChanged" {
                    let property: mpris::OrgFreedesktopDBusPropertiesPropertiesChanged =
                        event.read_all().unwrap();
                    if let Some(device) = shared_data
                        .get_shared_data()
                        .bat_devices
                        .get_mut(&event.path().unwrap().into_static())
                    {
                        if let Some(percentage) = property.changed_properties.get("Percentage") {
                            device.percentage = percentage.as_f64().unwrap().floor() as u32;
                        }
                        if let Some(state) = property.changed_properties.get("State") {
                            device.state = BatteryState::from(state.as_u64().unwrap() as u32);
                            device.time = TimeTo::Unknown;
                        }
                        if let Some(time_to_empty) = property.changed_properties.get("TimeToEmpty")
                        {
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
                        write_bar!(shared_data);
                    }
                } else if &*member == "PropertyChanged" {
                    let property: connman::NetConnmanManagerPropertyChanged =
                        event.read_all().unwrap();
                    if property.name == "State" {
                        let val = property.value.0.as_str().unwrap();
                        shared_data.get_shared_data().online = ConnmanState::from_str(val).unwrap();

                        if matches!(
                            shared_data.get_shared_data().online,
                            ConnmanState::Ready | ConnmanState::Online
                        ) {
                            shared_data.get_shared_data().connected_service = dbus
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
                            if !shared_data.get_shared_data().is_time_updated {
                                update_time!(shared_data);
                            }
                        } else {
                            shared_data.get_shared_data().is_time_updated = false;
                        }

                        write_bar!(shared_data);
                    }
                } else if &*member == "DeviceAdded" {
                    let battery: upower::OrgFreedesktopUPowerDeviceAdded =
                        event.read_all().unwrap();
                    let proxy = dbus.with_proxy(
                        "org.freedesktop.UPower",
                        battery.device,
                        Duration::from_secs(5),
                    );
                    BatteryDevice::insert(proxy, shared_data.get_shared_data());

                    write_bar!(shared_data);
                } else if &*member == "DeviceRemoved" {
                    let battery: upower::OrgFreedesktopUPowerDeviceRemoved =
                        event.read_all().unwrap();
                    shared_data
                        .get_shared_data()
                        .bat_devices
                        .remove(&battery.device);

                    write_bar!(shared_data);
                }
                None
            })
            .unwrap();
    }
}

pub struct SharedDataTransparent<F: FnMut(Vec<u8>)>(pub SharedData<F>);

impl<F: FnMut(Vec<u8>)> GetSharedData<F> for SharedDataTransparent<F> {
    fn get_shared_data(&mut self) -> &mut SharedData<F> {
        &mut self.0
    }
}

#[no_mangle]
#[cfg(not(feature = "i3bar"))]
pub extern "C" fn init() {
    let mut event_loop: EventLoop<_> = EventLoop::try_new().unwrap();
    let mut shared_data =
        SharedDataTransparent(SharedData::new(event_loop.get_signal(), |string| unsafe {
            let string = CString::from_vec_unchecked(string);
            onStatus(string.as_ptr());
        }));
    let handle = event_loop.handle();
    insert_into_loop(&mut event_loop, &mut shared_data);

    handle
        .insert_source(
            Generic::new(unsafe { displayFd }, Interest::READ, calloop::Mode::Level),
            |_, _, _| {
                if unsafe { wl_display_dispatch(display) < 0 } {
                    panic!("display_dispatch");
                }
                Ok(calloop::PostAction::Continue)
            },
        )
        .unwrap();

    event_loop
        .run(None, &mut shared_data, |_| unsafe {
            wl_display_dispatch_pending(display);
            wl_display_flush(display);
        })
        .unwrap();
}

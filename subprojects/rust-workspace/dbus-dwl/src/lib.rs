//
use std::{
    ffi::{c_char, c_int, CStr},
    time::Duration,
};

const FST: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fst.fst"));

use color::DefaultColorParser;
use cssparser::{Parser, ParserInput};
use dbus::{
    blocking::SyncConnection,
    channel::Channel,
    message::MatchRule,
    strings::{Interface, Member},
    MessageType, Path,
};
use dconf::CaDesrtDconfWriterNotify;
use fst::Map;
use logind::{OrgFreedesktopLogin1Manager, OrgFreedesktopLogin1ManagerPrepareForShutdown};
use palette::IntoColor;

mod dconf;
mod logind;

#[repr(C)]
pub struct WatchFFI {
    pub fd_session: c_int,
    pub fd_system: c_int,
    pub logind_lock: c_int,
    pub read_session: c_int,
    pub write_session: c_int,
    pub read_system: c_int,
    pub write_system: c_int,
    pub data_session: *mut SyncConnection,
    pub data_system: *mut SyncConnection,
}

#[no_mangle]
pub unsafe extern "C" fn get_fd() -> WatchFFI {
    let mut dbus_session = Channel::get_private(dbus::channel::BusType::Session).unwrap();
    let mut dbus_system = Channel::get_private(dbus::channel::BusType::System).unwrap();

    dbus_session.set_watch_enabled(true);
    dbus_system.set_watch_enabled(true);

    let watch_fd_session = dbus_session.watch();
    let watch_fd_system = dbus_system.watch();

    let conn_session: SyncConnection = dbus_session.into();
    let conn_system: SyncConnection = dbus_system.into();

    let mut match_rule_nameacquired = MatchRule::default();
    match_rule_nameacquired.msg_type = Some(MessageType::Signal);
    match_rule_nameacquired.path = Some(Path::new("/org/freedesktop/DBus").unwrap());
    match_rule_nameacquired.interface = Some(Interface::new("org.freedesktop.DBus").unwrap());
    match_rule_nameacquired.member = Some(Member::new("NameAcquired").unwrap());

    conn_session
        .add_match(match_rule_nameacquired, |_: (), _, _| true)
        .unwrap();

    conn_session
        .add_match(
            MatchRule::new_signal("ca.desrt.dconf.Writer", "Notify"),
            |_: (), _, _| true,
        )
        .unwrap();

    let system_proxy = conn_system.with_proxy(
        "org.freedesktop.login1",
        "/org/freedesktop/login1",
        Duration::from_secs(5),
    );

    let logind_lock = system_proxy
        .inhibit("shutdown", "DWL", "Gracefully shutdown", "delay")
        .unwrap()
        .into_fd();

    let session_path = system_proxy
        .get_session_by_pid(std::process::id())
        .unwrap()
        .into_static();

    conn_system
        .add_match(
            MatchRule::new_signal("org.freedesktop.login1.Manager", "PrepareForShutdown"),
            |_: (), _, _| true,
        )
        .unwrap();

    conn_system
        .add_match(
            MatchRule::new_signal("org.freedesktop.login1.Session", "Lock").with_path(session_path),
            |_: (), _, _| true,
        )
        .unwrap();

    WatchFFI {
        fd_session: watch_fd_session.fd,
        fd_system: watch_fd_system.fd,
        logind_lock,
        read_session: if watch_fd_session.read { 1 } else { 0 },
        write_session: if watch_fd_session.write { 1 } else { 0 },
        read_system: if watch_fd_system.read { 1 } else { 0 },
        write_system: if watch_fd_system.write { 1 } else { 0 },
        data_session: Box::into_raw(Box::new(conn_session)),
        data_system: Box::into_raw(Box::new(conn_system)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn process_dbus_session(
    dbus_session: *mut SyncConnection,
    function: extern "C" fn(c_int),
) {
    let dbus_session = Box::from_raw(dbus_session);

    if dbus_session
        .channel()
        .read_write(Some(Duration::from_millis(0)))
        .is_ok()
    {
        while let Some(message) = dbus_session.channel().pop_message() {
            if let Ok(msg) = message.read_all::<CaDesrtDconfWriterNotify>() {
                let map = Map::new(FST).unwrap();
                for c in msg.changes {
                    let mut prefix = msg.prefix.clone();
                    prefix.push_str(&c);
                    if let Some(num) = map.get(prefix.as_bytes()) {
                        function(num as c_int);
                    }
                }
            }
        }

        dbus_session.channel().flush();
    }

    Box::into_raw(dbus_session);
}

#[no_mangle]
pub unsafe extern "C" fn process_dbus_system(
    dbus_system: *mut SyncConnection,
    prepare_for_shutdown: extern "C" fn(),
    lock: extern "C" fn(),
) {
    let dbus_system = Box::from_raw(dbus_system);

    if dbus_system
        .channel()
        .read_write(Some(Duration::from_millis(0)))
        .is_ok()
    {
        while let Some(message) = dbus_system.channel().pop_message() {
            if message.interface() == Some("org.freedesktop.login1.Session".into()) {
                lock();
            } else {
                if let Ok(_) = message.read_all::<OrgFreedesktopLogin1ManagerPrepareForShutdown>() {
                    prepare_for_shutdown();
                }
            }
        }

        dbus_system.channel().flush();
    }

    Box::into_raw(dbus_system);
}

#[no_mangle]
pub unsafe extern "C" fn parse_color(
    color: *const c_char,
    c0: &mut f32,
    c1: &mut f32,
    c2: &mut f32,
    c3: &mut f32,
) {
    let mut reference =
        color::Color::LinSrgb(palette::Srgba::from_components((*c0, *c1, *c2, *c3)).into_linear());
    if let Ok(color) = color::parse_color_with::<color::Color>(
        &mut DefaultColorParser::new(Some(&mut reference)),
        &mut Parser::new(&mut ParserInput::new(
            CStr::from_ptr(color).to_str().unwrap_or_default(),
        )),
    ) {
        let new_color: palette::LinSrgba<f32> = color.1.into_color();
        let new_color: palette::Srgba<f32> = palette::Srgba::from_linear(new_color);

        *c0 = new_color.red;
        *c1 = new_color.green;
        *c2 = new_color.blue;
        *c3 = new_color.alpha;
    }
}

//
use std::{ffi::c_int, time::Duration};

const FST: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fst.fst"));

use dbus::{
    blocking::SyncConnection,
    channel::Channel,
    message::MatchRule,
    strings::{Interface, Member},
    MessageType, Path,
};
use dconf::CaDesrtDconfWriterNotify;
use fst::Map;

mod dconf;

#[repr(C)]
pub struct WatchFFI {
    pub fd: c_int,
    pub read: c_int,
    pub write: c_int,
    pub data: *mut SyncConnection,
}

#[no_mangle]
pub unsafe extern "C" fn get_fd() -> WatchFFI {
    let mut dbus_session = Channel::get_private(dbus::channel::BusType::Session).unwrap();

    dbus_session.set_watch_enabled(true);

    let watch_fd = dbus_session.watch();

    let conn: SyncConnection = dbus_session.into();

    let mut match_rule_nameacquired = MatchRule::default();
    match_rule_nameacquired.msg_type = Some(MessageType::Signal);
    match_rule_nameacquired.path = Some(Path::new("/org/freedesktop/DBus").unwrap());
    match_rule_nameacquired.interface = Some(Interface::new("org.freedesktop.DBus").unwrap());
    match_rule_nameacquired.member = Some(Member::new("NameAcquired").unwrap());

    conn.add_match(match_rule_nameacquired, |_: (), _, _| true)
        .unwrap();

    conn.add_match(
        MatchRule::new_signal("ca.desrt.dconf.Writer", "Notify"),
        |_: (), _, _| true,
    )
    .unwrap();

    WatchFFI {
        fd: watch_fd.fd,
        read: if watch_fd.read { 1 } else { 0 },
        write: if watch_fd.write { 1 } else { 0 },
        data: Box::into_raw(Box::new(conn)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn process_dbus(
    dbus_session: *mut SyncConnection,
    function: extern "C" fn(c_int),
) {
    let dbus_session = Box::from_raw(dbus_session);

    dbus_session
        .channel()
        .read_write(Some(Duration::from_millis(0)))
        .unwrap();

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

    Box::into_raw(dbus_session);
}

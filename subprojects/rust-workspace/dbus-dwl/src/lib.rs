use std::{
    ffi::{c_int, CString},
    os::raw::c_char,
    time::Duration,
};

use dbus::{
    blocking::SyncConnection,
    channel::Channel,
    message::MatchRule,
    strings::{Interface, Member},
    MessageType, Path,
};
use dconf::CaDesrtDconfWriterNotify;

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
    let mut dbus_session = Channel::get_private(dbus::channel::BusType::Session);

    while dbus_session.is_err() {
        dbus_session = Channel::get_private(dbus::channel::BusType::Session);
    }

    let mut dbus_session = dbus_session.unwrap();

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
    function: extern "C" fn(*const c_char),
) {
    /*
    let mut match_rule_nameacquired = MatchRule::default();
    match_rule_nameacquired.msg_type = Some(MessageType::Signal);
    match_rule_nameacquired.path = Some(Path::new("/org/freedesktop/DBus").unwrap());
    match_rule_nameacquired.interface = Some(Interface::new("org.freedesktop.DBus").unwrap());
    match_rule_nameacquired.member = Some(Member::new("NameAcquired").unwrap());
    */

    let dbus_session = Box::from_raw(dbus_session);

    dbus_session
        .channel()
        .read_write(Some(Duration::from_millis(0)))
        .unwrap();

    while let Some(message) = dbus_session.channel().pop_message() {
        if let Ok(msg) = message.read_all::<CaDesrtDconfWriterNotify>() {
            let field = CString::new(msg.prefix).unwrap();

            function(field.as_ptr());
        }
    }

    dbus_session.channel().flush();

    Box::into_raw(dbus_session);
}

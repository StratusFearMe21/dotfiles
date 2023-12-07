use std::{
    os::fd::{AsRawFd, FromRawFd, OwnedFd},
    time::Duration,
};

use dbus::{blocking::SyncConnection, channel::Channel, message::MatchRule};
use logind::{OrgFreedesktopLogin1Manager, OrgFreedesktopLogin1ManagerPrepareForShutdown};
use nix::{
    libc::signalfd_siginfo,
    sys::{
        signal::Signal,
        signalfd::{SfdFlags, SigSet, SignalFd},
    },
    unistd::Pid,
};
use polling::{Event, Events, Poller};

mod logind;

fn main() {
    let mut dbus_system = Channel::get_private(dbus::channel::BusType::System).unwrap();
    dbus_system.set_watch_enabled(true);
    let watch_fd_system = dbus_system.watch();
    let conn_system: SyncConnection = dbus_system.into();

    let system_proxy = conn_system.with_proxy(
        "org.freedesktop.login1",
        "/org/freedesktop/login1",
        Duration::from_secs(5),
    );

    let logind_lock = unsafe {
        OwnedFd::from_raw_fd(
            system_proxy
                .inhibit(
                    "shutdown",
                    "S6 user services",
                    "To gracefully shutdown all user services",
                    "delay",
                )
                .unwrap()
                .into_fd(),
        )
    };

    conn_system
        .add_match(
            MatchRule::new_signal("org.freedesktop.login1.Manager", "PrepareForShutdown"),
            |_: (), _, _| true,
        )
        .unwrap();

    let mut args = std::env::args().skip(1);
    let service_start = args.next().unwrap();
    let mut program = std::process::Command::new("s6-svscan")
        .args(args)
        .env("!", std::process::id().to_string())
        .spawn()
        .unwrap();

    while std::process::Command::new(&service_start).status().is_err() {}
    drop(service_start);

    let mask = SigSet::all();
    // mask.add(Signal::SIGINT);
    // mask.add(Signal::SIGTERM);
    mask.thread_block().unwrap();
    let mut signals = SignalFd::with_flags(&mask, SfdFlags::empty()).unwrap();
    let poller = Poller::new().unwrap();

    unsafe {
        poller
            .add_with_mode(
                signals.as_raw_fd(),
                Event::readable(2),
                polling::PollMode::Level,
            )
            .unwrap();
        poller
            .add_with_mode(
                watch_fd_system.fd,
                match (watch_fd_system.read, watch_fd_system.write) {
                    (true, false) => Event::readable(3),
                    (false, true) => Event::writable(3),
                    (true, true) => Event::all(3),
                    (false, false) => Event::none(3),
                },
                polling::PollMode::Level,
            )
            .unwrap();
    }

    let mut events = Events::new();
    'mainloop: loop {
        poller.wait(&mut events, None).unwrap();

        for e in events.iter() {
            match e.key {
                2 => {
                    if matches!(
                        signals.read_signal(),
                        Ok(Some(signalfd_siginfo {
                            ssi_signo: 15 | 2,
                            ..
                        }))
                    ) {
                        nix::sys::signal::kill(Pid::from_raw(program.id() as _), Signal::SIGTERM)
                            .unwrap();
                        program.wait().unwrap();
                        break 'mainloop;
                    }
                }
                3 => {
                    if conn_system
                        .channel()
                        .read_write(Some(Duration::from_millis(0)))
                        .is_ok()
                    {
                        while let Some(message) = conn_system.channel().pop_message() {
                            if message.interface() == Some("org.freedesktop.login1.Manager".into())
                            {
                                if let Ok(_) = message
                                    .read_all::<OrgFreedesktopLogin1ManagerPrepareForShutdown>()
                                {
                                    nix::sys::signal::kill(
                                        Pid::from_raw(program.id() as _),
                                        Signal::SIGTERM,
                                    )
                                    .unwrap();
                                    program.wait().unwrap();
                                    break 'mainloop;
                                }
                            }
                        }

                        conn_system.channel().flush();
                    }
                }
                _ => {}
            }
        }
    }
    drop(logind_lock);
}

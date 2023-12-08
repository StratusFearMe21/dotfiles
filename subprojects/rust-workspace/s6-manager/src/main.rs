use std::{
    ffi::CString,
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
        wait::{waitpid, WaitStatus},
    },
    unistd::{execvp, fork, pipe, read, Pid},
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

    let fds = pipe().unwrap();

    let mut args = std::env::args().skip(1);
    let service_start = args.next().unwrap();
    std::env::set_var("!", std::process::id().to_string());
    let program = {
        let args: Vec<CString> = [
            CString::new("s6-svscan").unwrap(),
            CString::new("-d").unwrap(),
            CString::new(fds.1.to_string()).unwrap(),
        ]
        .into_iter()
        .chain(args.map(|s| CString::new(s).unwrap()))
        .collect();
        let filename = CString::new("s6-svscan").unwrap();
        match unsafe { fork().unwrap() } {
            nix::unistd::ForkResult::Child => {
                execvp(filename.as_c_str(), &args).unwrap();
                return;
            }
            nix::unistd::ForkResult::Parent { child } => child,
        }
    };

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
        poller
            .add_with_mode(fds.0, Event::readable(4), polling::PollMode::Level)
            .unwrap();
    }

    let mut pipe = Some(unsafe { OwnedFd::from_raw_fd(fds.0) });

    let mut events = Events::new();
    'mainloop: loop {
        poller.wait(&mut events, None).unwrap();

        for e in events.iter() {
            match e.key {
                2 => match signals.read_signal() {
                    Ok(Some(signalfd_siginfo {
                        ssi_signo: 15 | 2, ..
                    })) => {
                        nix::sys::signal::kill(program, Signal::SIGTERM).unwrap();
                        while !matches!(waitpid(program, None).unwrap(), WaitStatus::Exited(_, _)) {
                        }
                        break 'mainloop;
                    }
                    Ok(Some(signalfd_siginfo {
                        ssi_signo: 17,
                        ssi_pid,
                        ..
                    })) => {
                        if Pid::from_raw(ssi_pid as i32) == program {
                            break 'mainloop;
                        }
                    }
                    _ => {}
                },
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
                                    nix::sys::signal::kill(program, Signal::SIGTERM).unwrap();
                                    while !matches!(
                                        waitpid(program, None).unwrap(),
                                        WaitStatus::Exited(_, _)
                                    ) {}
                                    break 'mainloop;
                                }
                            }
                        }

                        conn_system.channel().flush();
                    }
                }
                4 => {
                    if let Some(p) = pipe.take() {
                        let mut buf = [0; 512];
                        read(p.as_raw_fd(), &mut buf).unwrap();
                        if buf.contains(&b'\n') {
                            std::process::Command::new(&service_start).spawn().unwrap();
                        }
                    }
                }
                _ => {}
            }
        }
    }
    println!("------ Exited Gracefully ------");
    drop(logind_lock);
}

use std::{ffi::CString, os::fd::AsRawFd, time::Duration};

use clap::Parser;
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

#[derive(Parser)]
struct Command {
    starting_service: String,
    service_dir: String,
    #[clap(short, long)]
    wayland_session: bool,
}

fn main() {
    let mut args = Command::parse();
    let mut dbus_system = Channel::get_private(dbus::channel::BusType::System).unwrap();
    dbus_system.set_watch_enabled(true);
    let watch_fd_system = dbus_system.watch();
    let conn_system: SyncConnection = dbus_system.into();

    let system_proxy = conn_system.with_proxy(
        "org.freedesktop.login1",
        "/org/freedesktop/login1",
        Duration::from_secs(5),
    );

    let logind_lock = system_proxy
        .inhibit(
            "shutdown",
            "S6 user services",
            "To gracefully shutdown all user services",
            "delay",
        )
        .unwrap();

    conn_system
        .add_match(
            MatchRule::new_signal("org.freedesktop.login1.Manager", "PrepareForShutdown"),
            |_: (), _, _| true,
        )
        .unwrap();

    let fds = pipe().unwrap();

    std::env::set_var("!", std::process::id().to_string());
    if args.wayland_session {
        std::env::set_var("WAYLAND_DISPLAY", "wayland-0");
        std::env::set_var("DISPLAY", ":0");
        std::env::set_var("XDG_SESSION_TYPE", "wayland");
        std::env::set_var("XDG_CURRENT_DESKTOP", "dwl");
    }
    let program = unsafe {
        args.service_dir.as_mut_vec().push(0);
        let args: Vec<CString> = vec![
            CString::new("s6-svscan").unwrap(),
            CString::new("-d").unwrap(),
            CString::new(fds.1.as_raw_fd().to_string()).unwrap(),
            CString::from_vec_with_nul_unchecked(args.service_dir.into_bytes()),
        ];
        let filename = CString::new("s6-svscan").unwrap();
        match fork().unwrap() {
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
            .add_with_mode(&fds.0, Event::readable(4), polling::PollMode::Level)
            .unwrap();
    }

    let mut pipe = Some(fds.0);

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
                            std::process::Command::new(&args.starting_service)
                                .spawn()
                                .unwrap();
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

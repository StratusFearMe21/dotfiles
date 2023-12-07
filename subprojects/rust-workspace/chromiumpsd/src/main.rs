use std::{os::fd::AsRawFd, time::Duration};

use nix::{
    libc::signalfd_siginfo,
    sys::signalfd::{SfdFlags, SigSet, SignalFd},
};
use polling::{Event, Events, Poller};

fn main() {
    std::process::Command::new("psd")
        .arg("resync")
        .status()
        .unwrap();

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
    }

    let mut events = Events::new();
    'mainloop: loop {
        poller
            .wait(&mut events, Some(Duration::from_secs(3600)))
            .unwrap();

        if events.is_empty() {
            std::process::Command::new("psd")
                .arg("resync")
                .status()
                .unwrap();
        } else {
            for e in events.iter() {
                if e.key == 2 {
                    if matches!(
                        signals.read_signal(),
                        Ok(Some(signalfd_siginfo {
                            ssi_signo: 15 | 2,
                            ..
                        }))
                    ) {
                        std::process::Command::new("psd")
                            .arg("unsync")
                            .status()
                            .unwrap();
                        break 'mainloop;
                    }
                }
            }
        }
    }
}

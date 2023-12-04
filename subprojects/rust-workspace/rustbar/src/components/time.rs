use std::{
    io::{BufWriter, Write},
    os::unix::net::UnixStream,
    process::Command,
    rc::Rc,
    time::{Duration, Instant},
};

use chrono::{DateTime, Local, Timelike};
use rand::{rngs::OsRng, seq::SliceRandom};
use smithay_client_toolkit::reexports::{
    calloop::{
        // signals::{Signal, Signals},
        timer::{TimeoutAction, Timer},
        LoopHandle,
        RegistrationToken,
    },
    client::QueueHandle,
};

use crate::SimpleLayer;

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

pub const NTP_SERVERS: [&str; 18] = [
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

pub struct TimeBlock {
    pub is_time_updated: bool,
    pub update_time_ntp: bool,
    pub time_servers: Vec<String>,
    pub now: DateTime<Local>,
    pub time_fmt: String,
    pub date_fmt: String,
    pub xs_at: [f32; 2],
    pub widths: [f32; 2],
    handle: RegistrationToken,
}

impl TimeBlock {
    pub fn new(
        handle: &LoopHandle<SimpleLayer>,
        update_time_ntp: bool,
        time_servers: Vec<String>,
        time_fmt: String,
        date_fmt: String,
        qh: Rc<QueueHandle<SimpleLayer>>,
    ) -> Self {
        let now_instant = Instant::now();
        let now = chrono::Local::now();
        let timer_start = now_instant + Duration::from_secs(60 - now.time().second() as u64);
        /*
        // The only child process we spawn is ntpdate ever
        let chld_qh = Rc::clone(&qh);
        let chld_handle = handle
            .insert_source(
                Signals::new(&[Signal::SIGCHLD]).unwrap(),
                move |_, _, shared_data| unsafe {
                    let now = time::OffsetDateTime::now_utc();
                    let timezone = tz::TimeZone::local().unwrap();
                    let time_offset = timezone.find_current_local_time_type().unwrap().ut_offset();
                    shared_data.shared_data.time.as_mut().unwrap_unchecked().now =
                        now.to_offset(UtcOffset::from_whole_seconds(time_offset).unwrap());
                    shared_data.write_bar(Rc::clone(&chld_qh));
                },
            )
            .unwrap();
        */
        let handle = handle
            .insert_source(
                Timer::from_deadline(timer_start),
                move |_event, _metadata, shared_data| unsafe {
                    shared_data.shared_data.time.as_mut().unwrap_unchecked().now +=
                        chrono::Duration::minutes(1);
                    shared_data.write_bar(Rc::clone(&qh).as_ref());
                    TimeoutAction::ToDuration(Duration::from_secs(60))
                },
            )
            .unwrap();

        Self {
            now,
            update_time_ntp,
            time_servers,
            time_fmt,
            date_fmt,
            is_time_updated: false,
            handle,
            xs_at: [0.0; 2],
            widths: [0.0; 2],
        }
    }

    pub fn unregister(&self, handle: &LoopHandle<SimpleLayer>) {
        handle.remove(self.handle);
    }

    pub fn fmt_time(&self, f: &mut String) {
        f.push_str(match_clock!(self.now.hour()));
        use std::fmt::Write;
        write!(f, "{} ", self.now.format(&self.time_fmt),).unwrap();
    }

    pub fn fmt_date(&self, f: &mut String) {
        use std::fmt::Write;
        write!(f, " 󰃶 {} ", self.now.format(&self.date_fmt)).unwrap();
    }

    pub fn fmt_time_table(&self, f: &mut BufWriter<UnixStream>) -> std::io::Result<()> {
        write!(
            f,
            include_str!("../table.txt"),
            match_clock!(self.now.hour()),
        )?;
        write!(f, "{}\n", self.now.format(&self.time_fmt))
    }

    pub fn fmt_date_table(&self, f: &mut BufWriter<UnixStream>) -> std::io::Result<()> {
        write!(f, include_str!("../table.txt"), "󰃶 ")?;
        write!(f, "{}\n", self.now.format(&self.date_fmt))
    }

    pub fn update_time(&mut self) {
        self.is_time_updated = true;
        self.time_servers.shuffle(&mut OsRng);
        let mut args = String::new();
        args.push_str("doas ntpdate ");
        for server in &self.time_servers {
            args.push_str(&server);
            args.push(' ');
        }
        args.pop();
        args.push_str("; doas hwclock -w");
        Command::new("sh").arg("-c").arg(args).spawn().unwrap();
    }
}

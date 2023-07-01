use std::{
    io::{BufWriter, Write},
    os::unix::net::UnixStream,
    process::Command,
    rc::Rc,
    time::{Duration, Instant},
};

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
use time::{
    format_description::{
        self,
        modifier::{Day, Hour, Minute, Month, Weekday, Year},
    },
    OffsetDateTime, UtcOffset,
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

pub const TIME_FMT: [format_description::FormatItem; 3] = [
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

pub const DATE_FMT: [format_description::FormatItem; 5] = [
    format_description::FormatItem::Component(format_description::Component::Month(
        Month::default(),
    )),
    format_description::FormatItem::Literal(b"/"),
    format_description::FormatItem::Component(format_description::Component::Day(Day::default())),
    format_description::FormatItem::Literal(b"/"),
    format_description::FormatItem::Component(format_description::Component::Year(Year::default())),
];

pub const DATE_FMT_W_DAY: [format_description::FormatItem; 7] = [
    format_description::FormatItem::Component(format_description::Component::Month(
        Month::default(),
    )),
    format_description::FormatItem::Literal(b"/"),
    format_description::FormatItem::Component(format_description::Component::Day(Day::default())),
    format_description::FormatItem::Literal(b"/"),
    format_description::FormatItem::Component(format_description::Component::Year(Year::default())),
    format_description::FormatItem::Literal(b" "),
    format_description::FormatItem::Component(format_description::Component::Weekday(
        Weekday::default(),
    )),
];

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
    pub now: OffsetDateTime,
    pub show_day: bool,
    pub x_at: f32,
    pub width: f32,
    handle: RegistrationToken,
}

impl TimeBlock {
    pub fn new(
        handle: &LoopHandle<SimpleLayer>,
        show_day: bool,
        update_time_ntp: bool,
        time_servers: Vec<String>,
        qh: Rc<QueueHandle<SimpleLayer>>,
    ) -> Self {
        let now_instant = Instant::now();
        let now = time::OffsetDateTime::now_utc();
        let timezone = tz::TimeZone::local().unwrap();
        let time_offset = timezone.find_current_local_time_type().unwrap().ut_offset();
        let now = now.to_offset(UtcOffset::from_whole_seconds(time_offset).unwrap());
        let timer_start = now_instant + Duration::from_secs(60 - now.second() as u64);
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
                        time::Duration::minutes(1);
                    shared_data.write_bar(Rc::clone(&qh).as_ref());
                    TimeoutAction::ToDuration(Duration::from_secs(60))
                },
            )
            .unwrap();

        Self {
            now,
            show_day,
            update_time_ntp,
            time_servers,
            is_time_updated: false,
            handle,
            x_at: 0.0,
            width: 0.0,
        }
    }

    pub fn unregister(&self, handle: &LoopHandle<SimpleLayer>) {
        handle.remove(self.handle);
    }

    pub fn fmt(&self, f: &mut String) {
        f.push_str(match_clock!(self.now.hour()));
        self.now
            .format_into(unsafe { f.as_mut_vec() }, TIME_FMT.as_ref())
            .unwrap();
        f.push_str("  󰃶 ");
        if self.show_day {
            self.now
                .format_into(unsafe { f.as_mut_vec() }, DATE_FMT_W_DAY.as_ref())
                .unwrap();
        } else {
            self.now
                .format_into(unsafe { f.as_mut_vec() }, DATE_FMT.as_ref())
                .unwrap();
        }
        f.push(' ');
    }

    pub fn fmt_table(&self, f: &mut BufWriter<UnixStream>) -> std::io::Result<()> {
        write!(
            f,
            include_str!("../table.txt"),
            match_clock!(self.now.hour()),
        )?;
        self.now.format_into(f, TIME_FMT.as_ref()).unwrap();
        f.write_all(b"\n")?;
        write!(f, include_str!("../table.txt"), "󰃶 ")?;
        if self.show_day {
            self.now.format_into(f, DATE_FMT_W_DAY.as_ref()).unwrap();
        } else {
            self.now.format_into(f, DATE_FMT.as_ref()).unwrap();
        }
        f.write_all(b"\n")
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

use std::{
    fmt::Display,
    io::{BufWriter, Write},
    os::unix::net::UnixStream,
    str::FromStr,
    time::Duration,
};

use dbus::arg::RefArg;

use crate::{add_match, mpris};

pub struct PlaybackBlock {
    playing: PlaybackStatus,
    song_metadata: (String, String),
    match_token: dbus::channel::Token,
}

impl PlaybackBlock {
    pub fn new(user_connection: &calloop_dbus::SyncDBusSource<()>) -> Self {
        use crate::mpris::OrgMprisMediaPlayer2Player;

        let match_token = add_match!(user_connection, "org.mpris.MediaPlayer2.playerctld");

        let player_proxy = user_connection.with_proxy(
            "org.mpris.MediaPlayer2.playerctld",
            "/org/mpris/MediaPlayer2",
            Duration::from_secs(5),
        );

        let playing =
            PlaybackStatus::from_str(&player_proxy.playback_status().unwrap_or_default()).unwrap();

        let mut song_metadata = (String::new(), String::new());
        if let Ok(metadata) = player_proxy.metadata() {
            if let Some(title) = metadata.get("xesam:title") {
                song_metadata.0 = title.as_str().unwrap().to_owned();
            }
            if let Some(artist) = metadata.get("xesam:artist") {
                song_metadata.1 = artist
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

        Self {
            playing,
            song_metadata,
            match_token,
        }
    }

    pub fn unregister(&self, user_connection: &calloop_dbus::SyncDBusSource<()>) {
        user_connection.remove_match(self.match_token).unwrap();
    }

    pub fn fmt(&self, f: &mut String) {
        std::fmt::Write::write_fmt(f, format_args!("{}", self.playing)).unwrap();
        if self.playing != PlaybackStatus::Stopped {
            f.push_str(&self.song_metadata.0);
            f.push_str(" - ");
            f.push_str(&self.song_metadata.1);
        }
    }

    pub fn fmt_table(&self, f: &mut BufWriter<UnixStream>) -> std::io::Result<()> {
        write!(f, include_str!("../table.txt"), self.playing)?;
        f.write_all(self.song_metadata.0.as_bytes())?;
        f.write_all(b" - ")?;
        f.write_all(self.song_metadata.1.as_bytes())
    }

    pub fn query_media(&mut self, event: dbus::Message) -> bool {
        let property: mpris::OrgFreedesktopDBusPropertiesPropertiesChanged =
            event.read_all().unwrap();
        let mut changed = false;
        if let Some(metadata) = property.changed_properties.get("Metadata") {
            changed = true;
            let mut metadata = metadata.0.as_iter().unwrap();
            while let Some(data) = metadata.next() {
                match data.as_str() {
                    Some("xesam:title") => {
                        self.song_metadata.0 =
                            metadata.next().unwrap().as_str().unwrap().to_owned();
                    }
                    Some("xesam:artist") => {
                        self.song_metadata.1 = metadata
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
            changed = true;
            self.playing = PlaybackStatus::from_str(playback.as_str().unwrap()).unwrap();

            if self.playing == PlaybackStatus::Stopped {
                self.song_metadata = Default::default();
            }
        }
        changed
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

// static mut PLAYPAUSE_PING: MaybeUninit<Ping> = MaybeUninit::uninit();

// #[no_mangle]
// pub unsafe extern "C" fn mpris_play_pause(_: *const c_void, _: *const c_void) {
//     PLAYPAUSE_PING.assume_init_ref().ping();
// }

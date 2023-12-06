use std::{
    fmt::Display,
    io::{BufWriter, Write},
    os::unix::net::UnixStream,
    str::FromStr,
    time::Duration,
};

use dbus::arg::RefArg;
use iced_tiny_skia::core::image::Handle;
use pct_str::PctStr;
use url::Url;

use crate::{add_match, mpris};

#[derive(Default)]
pub struct Metadata {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
}

pub struct PlaybackBlock {
    pub playing: PlaybackStatus,
    pub song_metadata: Metadata,
    match_token: dbus::channel::Token,
    pub album_art: Option<Handle>,
    pub x_at: f32,
    pub width: f32,
}

pub fn url_to_handle(art_url: Url) -> Option<Handle> {
    match art_url.scheme() {
        "file" => Some(Handle::from_path(
            PctStr::new(art_url.path()).unwrap().decode(),
        )),
        _ => None,
    }
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

        let mut song_metadata = Metadata::default();
        let mut album_art = None;
        if let Ok(metadata) = player_proxy.metadata() {
            if let Some(title) = metadata.get("xesam:title") {
                song_metadata.title = title.as_str().unwrap().to_owned();
            }
            if let Some(album) = metadata.get("xesam:album") {
                song_metadata.album = album.as_str().unwrap().to_owned();
            }
            if let Some(album_artist) = metadata.get("xesam:albumArtist") {
                song_metadata.album_artist = album_artist
                    .0
                    .as_iter()
                    .unwrap()
                    .take(1)
                    .map(|f| f.as_str().unwrap_or_default())
                    .next()
                    .unwrap()
                    .to_owned();
            }
            if let Some(artist) = metadata.get("xesam:artist") {
                song_metadata.artist = artist
                    .0
                    .as_iter()
                    .unwrap()
                    .take(1)
                    .map(|f| f.as_str().unwrap_or_default())
                    .next()
                    .unwrap()
                    .to_owned();
            }
            let art_url = metadata.get("mpris:artUrl");
            if let Some(art) = art_url
                .and_then(|a| a.as_str())
                .and_then(|a| Url::parse(a).ok())
            {
                album_art = url_to_handle(art);
            }
        }

        Self {
            playing,
            song_metadata,
            match_token,
            x_at: 0.0,
            width: 0.0,
            album_art,
        }
    }

    pub fn unregister(&self, user_connection: &calloop_dbus::SyncDBusSource<()>) {
        user_connection.remove_match(self.match_token).unwrap();
    }

    pub fn fmt(&self, f: &mut String) {
        std::fmt::Write::write_fmt(f, format_args!(" {}", self.playing)).unwrap();
        if self.playing != PlaybackStatus::Stopped {
            f.push_str(&self.song_metadata.title);
            // f.push_str(" - ");
            // f.push_str(&self.song_metadata.1);
        }
    }

    pub fn fmt_table(&self, f: &mut BufWriter<UnixStream>) -> std::io::Result<()> {
        write!(f, include_str!("../table.txt"), self.playing)?;
        if self.playing != PlaybackStatus::Stopped {
            f.write_all(self.song_metadata.title.as_bytes())?
            // f.write_all(b" - ")?;
            // f.write_all(self.song_metadata.1.as_bytes())
        }
        Ok(())
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
                        self.song_metadata.title =
                            metadata.next().unwrap().as_str().unwrap().to_owned();
                    }
                    Some("xesam:album") => {
                        self.song_metadata.album =
                            metadata.next().unwrap().as_str().unwrap().to_owned();
                    }
                    Some("xesam:albumArtist") => {
                        self.song_metadata.album_artist = metadata
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
                    Some("xesam:artist") => {
                        self.song_metadata.artist = metadata
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
                    Some("mpris:artUrl") => {
                        let art_url = metadata.next().unwrap().as_str().unwrap();
                        if let Some(art) = Url::parse(art_url).ok() {
                            self.album_art = url_to_handle(art);
                        }
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
                self.album_art = None;
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

use std::{
    fmt::Debug,
    io::BufWriter,
    mem::ManuallyDrop,
    num::NonZeroU64,
    ops::Deref,
    path::{Path, PathBuf},
    time::{Duration, Instant, SystemTime},
};

use id3::{Content, Tag};
use libmpv::{
    events::{Event, PropertyData},
    Mpv, MpvNodeMap, MpvNodeMapIter, MpvNodeString, MpvStr,
};
use libmpv_sys::mpv_handle;
use regex::Regex;
use serde::{Serialize, Serializer};
use smart_default::SmartDefault;
use yoke::{Yoke, Yokeable};

#[derive(Debug)]
struct ListenbrainzData {
    payload: Payload,
    scrobble: bool,
    token: String,
    cache_path: PathBuf,
    timeout: bool,
    scrobble_deadline: Instant,
    pause_instant: Instant,
}

impl Default for ListenbrainzData {
    fn default() -> Self {
        Self {
            payload: Payload::default(),
            scrobble: false,
            token: String::new(),
            cache_path: {
                #[cfg(target_os = "linux")]
                {
                    dirs::cache_dir().unwrap().join("listenbrainz")
                }
                #[cfg(target_os = "android")]
                {
                    Path::new("/storage/emulated/0").join("listenbrainz")
                }
            },
            timeout: true,
            scrobble_deadline: Instant::now(),
            pause_instant: Instant::now(),
        }
    }
}

#[derive(Serialize, Debug)]
struct ListenbrainzSingleListen<'a> {
    listen_type: &'static str,
    payload: [&'a Payload; 1],
}

fn serialize_track_meta<S>(
    y: &Option<Yoke<TrackMetadata<'static>, MpvNodeMap<'static>>>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match *y {
        Some(ref value) => value.get().serialize(s),
        None => s.serialize_none(),
    }
}

#[derive(Serialize, Debug, Default)]
struct Payload {
    #[serde(skip_serializing_if = "Option::is_none")]
    listened_at: Option<NonZeroU64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_track_meta"
    )]
    track_metadata: Option<Yoke<TrackMetadata<'static>, MpvNodeMap<'static>>>,
}

#[derive(Serialize, Default, Debug, Yokeable)]
struct TrackMetadata<'a> {
    additional_info: AdditionalInfo<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artist_name: Option<MpvNodeString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    track_name: Option<MpvNodeString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    release_name: Option<MpvNodeString<'a>>,
}

#[derive(Debug, Yokeable)]
struct StringList<'a>(Vec<&'a str>);

#[derive(Debug)]
struct YokedStringList<'a>(Yoke<StringList<'static>, Option<MpvNodeString<'a>>>);

impl Default for YokedStringList<'_> {
    fn default() -> Self {
        Self(Yoke::new_owned(StringList(Vec::new())))
    }
}

impl YokedStringList<'_> {
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.0.get().0.is_empty()
    }
}

impl Serialize for YokedStringList<'_> {
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.get().0.serialize(serializer)
    }
}

enum MpvOrRustStr<'a> {
    MpvStr(MpvNodeString<'a>),
    RustStr(Yoke<&'static str, Vec<u8>>),
}

impl Default for MpvOrRustStr<'_> {
    fn default() -> Self {
        Self::RustStr(Yoke::attach_to_cart(Vec::new(), |s| unsafe {
            std::str::from_utf8_unchecked(s)
        }))
    }
}

impl Deref for MpvOrRustStr<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::MpvStr(s) => s.deref(),
            Self::RustStr(s) => s.get(),
        }
    }
}

impl Debug for MpvOrRustStr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MpvOrRustStr").field(&self.deref()).finish()
    }
}

impl Serialize for MpvOrRustStr<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.deref().serialize(serializer)
    }
}

#[derive(Serialize, Debug, SmartDefault)]
struct AdditionalInfo<'a> {
    #[default = "mpv"]
    media_player: &'static str,
    #[default = "mpv ListenBrainz Rust"]
    submission_client: &'static str,
    #[default(env!("CARGO_PKG_VERSION"))]
    submission_client_version: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    release_mbid: Option<MpvNodeString<'a>>,
    #[serde(skip_serializing_if = "YokedStringList::is_empty")]
    artist_mbids: YokedStringList<'a>,
    #[serde(skip_serializing_if = "str::is_empty")]
    recording_mbid: MpvOrRustStr<'a>,
    duration_ms: u64,
}

#[derive(Serialize, Default, Debug)]
struct LoveHate<'a> {
    recording_mbid: &'a str,
    score: i32,
}

fn scrobble(listen_type: &'static str, payload: &Payload, token: &str, cache_path: &Path) {
    let send = ListenbrainzSingleListen {
        listen_type,
        payload: [payload],
    };
    #[cfg(debug_assertions)]
    eprintln!("{}", serde_json::to_string_pretty(&send).unwrap());
    let status = ureq::post("https://api.listenbrainz.org/1/submit-listens")
        .set("Authorization", token)
        .send_json(send);
    if status.is_ok() {
        import_cache(token, cache_path);
        return;
    }
    if let Some(listened_at) = payload.listened_at {
        serde_json::to_writer(
            BufWriter::new(
                std::fs::File::create(cache_path.join(format!("{}.json", listened_at))).unwrap(),
            ),
            &payload,
        )
        .unwrap();
    }
}

fn import_cache(token: &str, cache_path: &Path) {
    let mut read_dir = cache_path.read_dir().unwrap();
    let is_occupied = read_dir.next().is_some();
    let is_one_file = read_dir.next().is_none();
    if cache_path.exists() && is_occupied {
        let mut request = if is_one_file {
            br#"{"listen_type":"single","payload":["#.to_vec()
        } else {
            br#"{"listen_type":"import","payload":["#.to_vec()
        };
        for i in std::fs::read_dir(&cache_path).unwrap() {
            let path = i.unwrap().path();
            std::io::copy(
                &mut std::fs::File::open(path.as_path()).unwrap(),
                &mut request,
            )
            .unwrap();
            request.push(b',');
        }
        request.pop();
        request.extend_from_slice(b"]}");
        #[cfg(debug_assertions)]
        eprintln!("{}", unsafe { std::str::from_utf8_unchecked(&request) });
        let status = ureq::post("https://api.listenbrainz.org/1/submit-listens")
            .set("Authorization", token)
            .set("Content-Type", "json")
            .send_bytes(&request);
        if status.is_err() {
            eprintln!("Error importing {:?}", status);
            return;
        }
        std::fs::read_dir(cache_path)
            .unwrap()
            .try_for_each(|i| std::fs::remove_file(i?.path()))
            .unwrap();
    }
}

fn read_recording_id(filename: &str, data: &mut ListenbrainzData) -> Result<(), ()> {
    let Ok(tag) = Tag::read_from_path(filename) else {
        return Err(());
    };

    for f in tag.frames.into_iter() {
        if f.id() == "UFID" {
            let Content::Unknown(u) = f.content else {
                continue;
            };

            let Some(delimeter_pos) = memchr::memchr(0, &u.data) else {
                continue;
            };

            if &u.data[..delimeter_pos] != b"http://musicbrainz.org" {
                continue;
            }

            if let Ok(s) = Yoke::try_attach_to_cart(u.data, |data| {
                std::str::from_utf8(&data[delimeter_pos + 1..])
            }) {
                data.payload
                    .track_metadata
                    .as_mut()
                    .unwrap()
                    .with_mut(move |tm| {
                        tm.additional_info.recording_mbid = MpvOrRustStr::RustStr(s)
                    });
            } else {
                continue;
            }

            return Ok(());
        }
    }

    Err(())
}

macro_rules! scrobble_duration {
    ($duration:expr,$speed:expr) => {
        if $duration <= 40.0 {
            $duration - 1.0
        } else {
            f64::min(240.0, $duration / 2.0)
        } / $speed
    };
}

#[no_mangle]
pub extern "C" fn mpv_open_cplugin(ctx: *mut mpv_handle) -> i8 {
    let uuid_regex =
        Regex::new("[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}").unwrap();
    let mut mpv = ManuallyDrop::new(Mpv::new_with_context(ctx).unwrap());
    mpv.event_context()
        .observe_property("pause", libmpv::Format::Flag, 0)
        .unwrap();
    mpv.event_context()
        .observe_property("speed", libmpv::Format::Double, 0)
        .unwrap();
    let (tx, rx) = flume::unbounded();
    mpv.event_context_mut()
        .set_wakeup_callback(move || tx.send(()).unwrap());

    let mut data = ListenbrainzData::default();

    for i in mpv
        .get_property::<libmpv::MpvNode>("script-opts")
        .unwrap()
        .into_map()
        .unwrap()
        .iter()
    {
        match i.0 {
            "listenbrainz-user-token" => {
                data.token = format!("Token {}", i.1.as_str().unwrap().deref())
            }
            "listenbrainz-cache-path" => {
                #[cfg(target_os = "linux")]
                {
                    data.cache_path = dirs::cache_dir()
                        .unwrap()
                        .join(i.1.as_str().unwrap().deref())
                        .join("listenbrainz");
                }
                #[cfg(target_os = "android")]
                {
                    data.cache_path = Path::new(i.1.as_str().unwrap().deref()).join("listenbrainz");
                }
            }
            _ => {}
        }
    }

    if let Ok(config_dir) = mpv.get_property::<MpvStr>("config-dir") {
        if !config_dir.is_empty() {
            data.cache_path = Path::new(&*config_dir).join("listenbrainz");
        }
    }

    if !data.cache_path.exists() {
        std::fs::create_dir(&data.cache_path).unwrap();
    }

    import_cache(&data.token, &data.cache_path);

    'mainloop: loop {
        let result: Result<(), flume::RecvTimeoutError> =
            if data.timeout && Instant::now() < data.scrobble_deadline {
                rx.recv_deadline(data.scrobble_deadline)
            } else {
                rx.recv().map_err(|e| e.into())
            };

        match result {
            Ok(()) => loop {
                match mpv.event_context_mut().wait_event(0.0) {
                    Some(Ok(Event::Shutdown)) => break 'mainloop,
                    Some(Ok(Event::ClientMessage(m))) => {
                        if m[0] == "key-binding" {
                            let score = match m[1] {
                                "listenbrainz-love" => 1,
                                "listenbrainz-hate" => -1,
                                "listenbrainz-unrate" => 0,
                                _ => continue,
                            };

                            let recording_mbid = data
                                .payload
                                .track_metadata
                                .as_ref()
                                .map(|d| d.get().additional_info.recording_mbid.deref())
                                .unwrap_or("");

                            if recording_mbid.is_empty() {
                                eprintln!(
                                    "This song is unknown to ListenBrainz, and \
                                 cannot be rated"
                                );
                                continue;
                            }

                            let feedback = LoveHate {
                                recording_mbid,
                                score,
                            };

                            let status = ureq::post(
                                "https://api.listenbrainz.org/1/feedback/recording-feedback",
                            )
                            .set("Authorization", &data.token)
                            .send_json(feedback);

                            if status.is_err() {
                                eprintln!("Error submitting feedback: {:?}", status);
                            } else {
                                eprintln!("Feedback submitted successfully");
                            }
                        }
                    }
                    Some(Ok(Event::PropertyChange { name, change, .. })) => {
                        if name == "pause" && data.scrobble {
                            if let Some(PropertyData::Flag(paused)) = change {
                                if paused {
                                    data.pause_instant = Instant::now();
                                    data.timeout = false;
                                } else {
                                    data.scrobble_deadline =
                                        data.scrobble_deadline + data.pause_instant.elapsed();
                                    data.timeout = true;
                                }
                            }
                        } else if name == "speed" && data.scrobble {
                            if let Some(PropertyData::Double(speed)) = change {
                                let duration = mpv.get_property::<f64>("duration").unwrap();
                                let pos = mpv.get_property::<f64>("time-pos").unwrap();
                                data.scrobble_deadline = Instant::now()
                                    + Duration::from_secs_f64(
                                        scrobble_duration!(duration, speed) - pos,
                                    );
                                data.payload.track_metadata.as_mut().map(move |m| {
                                    m.with_mut(move |tm| {
                                        tm.additional_info.duration_ms = (duration * 1000.0) as u64
                                    })
                                });
                                data.timeout = true;
                            }
                        }
                    }
                    Some(Ok(Event::Seek)) => {
                        if mpv.get_property::<i64>("time-pos").unwrap() == 0 {
                            let duration = mpv.get_property::<f64>("duration").unwrap();
                            let speed = mpv.get_property::<f64>("speed").unwrap();

                            data.scrobble_deadline = Instant::now()
                                + Duration::from_secs_f64(scrobble_duration!(duration, speed));
                            data.payload.track_metadata.as_mut().map(move |m| {
                                m.with_mut(move |tm| {
                                    tm.additional_info.duration_ms = (duration * 1000.0) as u64
                                })
                            });
                            data.timeout = true;
                        }
                    }
                    Some(Ok(Event::FileLoaded)) => {
                        let audio_pts: Result<i64, libmpv::Error> = mpv.get_property("audio-pts");
                        if audio_pts.is_err() || audio_pts.unwrap() < 1 {
                            data.timeout = false;

                            data.payload.track_metadata = Some(Yoke::attach_to_cart(
                                mpv.get_property::<libmpv::MpvNode>("metadata")
                                    .unwrap()
                                    .into_map()
                                    .unwrap(),
                                |metadata| {
                                    let mut track_metadata = TrackMetadata::default();
                                    for i in MpvNodeMapIter::new(metadata)
                                        .filter_map(|i| i.1.as_str().map(|s| (i.0, s)))
                                    {
                                        #[cfg(debug_assertions)]
                                        dbg!(i.0);
                                        match i.0 {
                                            "MUSICBRAINZ_ALBUMID" | "MusicBrainz Album Id" => {
                                                track_metadata.additional_info.release_mbid =
                                                    Some(i.1);
                                            }
                                            "MUSICBRAINZ_ARTISTID" | "MusicBrainz Artist Id" => {
                                                let artists = i.1;

                                                #[cfg(debug_assertions)]
                                                dbg!(&artists);

                                                track_metadata.additional_info.artist_mbids =
                                                    YokedStringList(Yoke::wrap_cart_in_option(
                                                        Yoke::attach_to_cart(artists, |artists| {
                                                            StringList(
                                                                uuid_regex
                                                                    .find_iter(artists)
                                                                    .map(|m| m.as_str())
                                                                    .collect(),
                                                            )
                                                        }),
                                                    ));
                                            }
                                            "MUSICBRAINZ_TRACKID" | "http://musicbrainz.org" => {
                                                track_metadata.additional_info.recording_mbid =
                                                    MpvOrRustStr::MpvStr(i.1);
                                            }
                                            "ARTIST" | "artist" => {
                                                track_metadata.artist_name = Some(i.1);
                                            }
                                            "TITLE" | "title" => {
                                                track_metadata.track_name = Some(i.1);
                                            }
                                            "ALBUM" | "album" => {
                                                track_metadata.release_name = Some(i.1);
                                            }
                                            _ => {}
                                        }
                                    }
                                    track_metadata
                                },
                            ));

                            {
                                let track_metadata =
                                    data.payload.track_metadata.as_ref().unwrap().get();

                                #[cfg(debug_assertions)]
                                {
                                    dbg!(
                                        mpv.get_property::<MpvStr>("filename").unwrap().deref()
                                            != track_metadata
                                                .track_name
                                                .as_ref()
                                                .map(|n| n.deref())
                                                .unwrap_or("")
                                    );
                                    dbg!(track_metadata.artist_name.is_some());
                                    dbg!(track_metadata.track_name.is_some());
                                    dbg!(track_metadata.release_name.is_some());
                                    #[cfg(feature = "only-scrobble-if-mbid")]
                                    dbg!(track_metadata.additional_info.release_mbid.is_some());
                                }

                                data.scrobble =
                                    (mpv.get_property::<MpvStr>("filename").unwrap().deref()
                                        != track_metadata
                                            .track_name
                                            .as_ref()
                                            .map(|n| n.deref())
                                            .unwrap_or(""))
                                        && track_metadata.artist_name.is_some()
                                        && track_metadata.track_name.is_some()
                                        && track_metadata.release_name.is_some()
                                        && track_metadata.additional_info.release_mbid.is_some();

                                #[cfg(feature = "only-scrobble-if-mbid")]
                                {
                                    data.scrobble = data.scrobble
                                        && track_metadata.additional_info.release_mbid.is_some();
                                }

                                if track_metadata.additional_info.recording_mbid.is_empty() {
                                    let filename: MpvStr = mpv.get_property("path").unwrap();

                                    let _ = read_recording_id(&filename, &mut data);
                                }
                            }

                            if data.scrobble {
                                let duration = mpv.get_property::<f64>("duration").unwrap();
                                let speed = mpv.get_property::<f64>("speed").unwrap();

                                data.scrobble_deadline = Instant::now()
                                    + Duration::from_secs_f64(scrobble_duration!(duration, speed));
                                data.payload
                                    .track_metadata
                                    .as_mut()
                                    .unwrap()
                                    .with_mut(move |tm| {
                                        tm.additional_info.duration_ms = (duration * 1000.0) as u64
                                    });
                                data.timeout = true;

                                data.payload.listened_at = None;
                                scrobble(
                                    "playing_now",
                                    &data.payload,
                                    &data.token,
                                    &data.cache_path,
                                );
                            }
                        }
                    }
                    None => break,
                    _ => {}
                }
            },
            Err(flume::RecvTimeoutError::Timeout) => {
                data.payload.listened_at = NonZeroU64::new(
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                );
                scrobble("single", &data.payload, &data.token, &data.cache_path);
                data.scrobble = false;
            }
            e @ Err(_) => e.unwrap(),
        }
    }

    return 0;
}

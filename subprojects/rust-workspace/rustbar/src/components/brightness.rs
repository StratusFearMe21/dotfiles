use nix::sys::inotify::{AddWatchFlags, InitFlags, Inotify};
use smithay_client_toolkit::reexports::{
    calloop::{generic::Generic, Interest, LoopHandle, Mode, PostAction, RegistrationToken},
    client::QueueHandle,
};
use std::{
    io::{BufWriter, Read, Write},
    os::unix::net::UnixStream,
    rc::Rc,
};

use crate::SimpleLayer;

macro_rules! match_brightness {
    ($b:expr) => {
        match $b {
            0..=15 => "󰃚 ",
            16..=31 => "󰃛 ",
            32..=47 => "󰃜 ",
            48..=63 => "󰃝 ",
            64..=79 => "󰃞 ",
            80..=95 => "󰃟 ",
            _ => "󰃠 ",
        }
    };
}

pub struct BrightnessBlock {
    brightness: usize,
    max_brightness: f32,
    handle: RegistrationToken,
    pub x_at: f32,
    pub width: f32,
}

impl BrightnessBlock {
    pub fn new(handle: &LoopHandle<SimpleLayer>, qh: Rc<QueueHandle<SimpleLayer>>) -> Self {
        let brightness_path = {
            std::fs::read_dir("/sys/class/backlight")
                .unwrap()
                .next()
                .map(|f| f.unwrap().path())
        };

        let mut max_brightness = 0.0;
        let brightness: usize;
        if let Some(ref brightness_path) = brightness_path {
            max_brightness = std::fs::read_to_string(brightness_path.join("max_brightness"))
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap() as f32;

            let mut brightness_file =
                std::fs::File::open(brightness_path.join("brightness")).unwrap();

            let mut br_string = String::new();
            brightness_file.read_to_string(&mut br_string).unwrap();
            brightness =
                ((br_string.trim().parse::<f32>().unwrap() / max_brightness) * 100.0) as usize;
        } else {
            brightness = 0;
        }

        let notify_instance = Inotify::init(InitFlags::empty()).unwrap();

        if let Some(ref brightness) = brightness_path {
            let _brightness_watch = notify_instance
                .add_watch(brightness.as_path(), AddWatchFlags::IN_CLOSE_WRITE)
                .unwrap();
        }

        let brightness_path = unsafe { brightness_path.unwrap_unchecked() };
        let handle = handle
            .insert_source(
                Generic::new(notify_instance, Interest::BOTH, Mode::Level),
                move |_, notify, data| unsafe {
                    for _ in notify.read_events().unwrap() {
                        let br_string =
                            std::fs::read_to_string(brightness_path.join("brightness")).unwrap();
                        data.shared_data
                            .brightness
                            .as_mut()
                            .unwrap_unchecked()
                            .brightness = ((br_string.trim().parse::<f32>().unwrap()
                            / data
                                .shared_data
                                .brightness
                                .as_mut()
                                .unwrap_unchecked()
                                .max_brightness)
                            * 100.0) as _;

                        data.write_bar(Rc::clone(&qh).as_ref())
                    }
                    Ok(PostAction::Continue)
                },
            )
            .unwrap();

        Self {
            brightness,
            max_brightness,
            handle,
            x_at: 0.0,
            width: 0.0,
        }
    }

    pub fn unregister(&self, handle: &LoopHandle<SimpleLayer>) {
        handle.remove(self.handle);
    }

    pub fn fmt(&self, f: &mut String) {
        std::fmt::Write::write_fmt(
            f,
            format_args!(
                " {}{}% ",
                match_brightness!(self.brightness),
                self.brightness
            ),
        )
        .unwrap()
    }

    pub fn fmt_table(&self, f: &mut BufWriter<UnixStream>) -> std::io::Result<()> {
        write!(
            f,
            concat!(include_str!("../table.txt"), "{}%\n"),
            match_brightness!(self.brightness),
            self.brightness
        )
    }
}

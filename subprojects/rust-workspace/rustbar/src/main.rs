use std::cell::RefCell;
use std::collections::HashMap;
use std::io::BufWriter;
use std::io::Write;
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::os::fd::AsRawFd;
use std::os::unix::net::UnixListener;
use std::os::unix::net::UnixStream;
use std::ptr::NonNull;
use std::sync::Arc;
use std::time::Duration;
use std::{ffi::CString, rc::Rc};

use components::{
    battery::BatteryBlock,
    brightness::BrightnessBlock,
    connman::ConnmanBlock,
    playback::PlaybackBlock,
    time::{TimeBlock, NTP_SERVERS},
};

use calloop::generic::Generic;
use calloop::signals::{Signal, Signals};
use calloop::EventLoop;
use calloop::Interest;
use calloop::LoopHandle;
use calloop::LoopSignal;
use calloop_dbus::SyncDBusSource;
use client::{
    globals::registry_queue_init,
    protocol::{wl_output, wl_pointer, wl_seat, wl_shm, wl_surface},
    Connection, Proxy, QueueHandle,
};
use cursor_shape::wp_cursor_shape_manager_v1::WpCursorShapeManagerV1;
use dbus::message::MatchRule;
use dconf_sys::dconf_client_new;
use dconf_sys::dconf_client_read;
use dconf_sys::DConfClient;
use freedesktop_desktop_entry::default_paths;
use freedesktop_desktop_entry::DesktopEntry;
use fuzzy_match::FuzzyQuery;
use fuzzy_matcher::skim::SkimMatcherV2;
use glib::log_writer_is_journald;
use glib::FromVariant;
use iced_tiny_skia::core::font::Family;
use iced_tiny_skia::core::Background;
use iced_tiny_skia::{
    core::{
        alignment::{Horizontal, Vertical},
        text::{LineHeight, Shaping},
        Color, Font, Rectangle, Size,
    },
    graphics::{backend::Text, Primitive, Viewport},
};
use memchr::memchr;
use smithay_client_toolkit::delegate_keyboard;
use smithay_client_toolkit::globals::GlobalData;
use smithay_client_toolkit::reexports::calloop::timer::TimeoutAction;
use smithay_client_toolkit::reexports::calloop::timer::Timer;
use smithay_client_toolkit::reexports::calloop::Mode;
use smithay_client_toolkit::reexports::calloop::RegistrationToken;
use smithay_client_toolkit::reexports::client::backend::ObjectId;
use smithay_client_toolkit::reexports::client::protocol::wl_keyboard;
use smithay_client_toolkit::seat::keyboard::keysyms;
use smithay_client_toolkit::seat::keyboard::KeyEvent;
use smithay_client_toolkit::seat::keyboard::KeyboardHandler;
use smithay_client_toolkit::seat::keyboard::Modifiers;
use smithay_client_toolkit::seat::pointer::BTN_LEFT;
use smithay_client_toolkit::shell::wlr_layer::KeyboardInteractivity;
use smithay_client_toolkit::{
    compositor::{CompositorHandler, CompositorState},
    delegate_compositor, delegate_layer, delegate_output, delegate_pointer, delegate_registry,
    delegate_seat, delegate_shm,
    output::{OutputHandler, OutputState},
    reexports::{calloop, client},
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    seat::{
        pointer::{PointerEvent, PointerEventKind, PointerHandler},
        Capability, SeatHandler, SeatState,
    },
    shell::{
        wlr_layer::{
            Anchor, Layer, LayerShell, LayerShellHandler, LayerSurface, LayerSurfaceConfigure,
        },
        WaylandSurface,
    },
    shm::{
        slot::{Buffer, SlotPool},
        Shm, ShmHandler,
    },
};
use tags::Tags;
use tiny_skia::{Mask, PixmapMut};
use znet_dwl::znet_tapesoftware_dwl_wm_monitor_v1::ZnetTapesoftwareDwlWmMonitorV1;
use znet_dwl::znet_tapesoftware_dwl_wm_v1::WobCommand;
use znet_dwl::znet_tapesoftware_dwl_wm_v1::ZnetTapesoftwareDwlWmV1;

mod connman;
mod dconf;
mod mpris;
mod upower;

mod components;
mod fuzzy_match;
mod tags;

pub mod znet_dwl {
    use smithay_client_toolkit::reexports::client as wayland_client;
    use wayland_client::protocol::*;

    pub mod __interfaces {
        use smithay_client_toolkit::reexports::client as wayland_client;
        use wayland_client::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!(
            "../../dwl/protocols/net-tapesoftware-dwl-wm-unstable-v1.xml"
        );
    }

    use self::__interfaces::*;

    wayland_scanner::generate_client_code!(
        "../../dwl/protocols/net-tapesoftware-dwl-wm-unstable-v1.xml"
    );
}

pub mod zwp_tablet_tool {
    use smithay_client_toolkit::reexports::client as wayland_client;
    use wayland_client::protocol::*;

    pub mod __interfaces {
        use smithay_client_toolkit::reexports::client as wayland_client;
        use wayland_client::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!(
            "/usr/share/wayland-protocols/unstable/tablet/tablet-unstable-v2.xml"
        );
    }

    use self::__interfaces::*;

    wayland_scanner::generate_client_code!(
        "/usr/share/wayland-protocols/unstable/tablet/tablet-unstable-v2.xml"
    );
}

pub mod cursor_shape {
    use smithay_client_toolkit::reexports::client as wayland_client;
    use wayland_client::protocol::*;

    pub mod __interfaces {
        use super::super::zwp_tablet_tool::__interfaces::*;
        use smithay_client_toolkit::reexports::client as wayland_client;
        use wayland_client::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("src/cursor-shape-v1.xml");
    }

    use self::__interfaces::*;
    use super::zwp_tablet_tool::zwp_tablet_tool_v2;

    wayland_scanner::generate_client_code!("src/cursor-shape-v1.xml");
}

#[macro_export]
macro_rules! add_match {
    ($bus:expr,$sender:expr) => {
        $bus.add_match::<crate::upower::OrgFreedesktopDBusPropertiesPropertiesChanged, _>(
            dbus::message::MatchRule::new_signal(
                "org.freedesktop.DBus.Properties",
                "PropertiesChanged",
            )
            .with_sender($sender),
            |_, _, _| true,
        )
        .unwrap()
    };
    ($bus:expr,$sender:expr,$signal:expr) => {
        $bus.add_match::<crate::upower::OrgFreedesktopDBusPropertiesPropertiesChanged, _>(
            dbus::message::MatchRule::new_signal($sender, $signal).with_sender($sender),
            |_, _, _| true,
        )
        .unwrap()
    };
    ($bus:expr,$sender:expr,$interface:expr,$signal:expr) => {
        $bus.add_match::<crate::upower::OrgFreedesktopDBusPropertiesPropertiesChanged, _>(
            dbus::message::MatchRule::new_signal($interface, $signal).with_sender($sender),
            |_, _, _| true,
        )
        .unwrap()
    };
}

const DIVIDER: &str = "";
const DIVIDER_HARD: &str = "";

struct DesktopCommand {
    name: String,
    command: String,
    score: Option<i64>,
}

enum BarState {
    Normal,
    ProgressBar {
        percentage: f32,
        icon: char,
    },
    AppLauncher {
        apps: Vec<DesktopCommand>,
        layout: Vec<Primitive>,
        current_input: String,
        matcher: SkimMatcherV2,
        selected: usize,
    },
}

impl BarState {}

impl From<WobCommand> for char {
    fn from(value: WobCommand) -> char {
        match value {
            WobCommand::VolumeUp | WobCommand::VolumeDown => '󰕾',
            WobCommand::LightUp | WobCommand::LightDown => '󰃠',
            _ => ' ',
        }
    }
}

pub struct SharedData {
    time: Option<TimeBlock>,
    playback: Option<PlaybackBlock>,
    connman: Option<ConnmanBlock>,
    brightness: Option<BrightnessBlock>,
    bat_block: Option<BatteryBlock>,
    time_handle: RegistrationToken,
    pub selected: Option<usize>,
}

impl SharedData {
    pub fn new(
        handle: &LoopHandle<SimpleLayer>,
        qh: Rc<QueueHandle<SimpleLayer>>,
        dconf: *mut DConfClient,
    ) -> Self {
        unsafe {
            let loop_handle: LoopHandle<'static, SimpleLayer> = std::mem::transmute(handle.clone());

            let (user_connection, _): (calloop_dbus::SyncDBusSource<()>, _) =
                calloop_dbus::SyncDBusSource::new_session().unwrap();
            let (system_connection, _): (calloop_dbus::SyncDBusSource<()>, _) =
                calloop_dbus::SyncDBusSource::new_system().unwrap();

            let user_connection_ptr = Box::into_raw(Box::new(user_connection));
            let system_connection_ptr = Box::into_raw(Box::new(system_connection));

            let user_connection: &'static mut SyncDBusSource<()> = &mut *user_connection_ptr;
            let system_connection: &'static mut SyncDBusSource<()> = &mut *system_connection_ptr;

            user_connection
                .add_match::<upower::OrgFreedesktopDBusPropertiesPropertiesChanged, _>(
                    MatchRule::new_signal("ca.desrt.dconf.Writer", "Notify"),
                    |_, _, _| true,
                )
                .unwrap();
            let mut time = None;
            if dconf_read_variant(dconf, "/dotfiles/somebar/time-block").unwrap_or(true) {
                time = Some(TimeBlock::new(
                    handle,
                    dconf_read_variant(dconf, "/dotfiles/somebar/time-show-day").unwrap_or(true),
                    dconf_read_variant(dconf, "/dotfiles/somebar/update-time-ntp").unwrap_or(true),
                    dconf_read_variant(dconf, "/dotfiles/somebar/time-servers")
                        .unwrap_or(NTP_SERVERS.into_iter().map(|s| s.to_string()).collect()),
                    Rc::clone(&qh),
                ));
            }

            let mut brightness = None;
            if dconf_read_variant(dconf, "/dotfiles/somebar/brightness-block").unwrap_or(true) {
                brightness = Some(BrightnessBlock::new(handle, Rc::clone(&qh)))
            }

            let mut battery = None;
            if dconf_read_variant(dconf, "/dotfiles/somebar/battery-block").unwrap_or(true) {
                battery = Some(BatteryBlock::new(system_connection))
            }

            let mut playback = None;
            if dconf_read_variant(dconf, "/dotfiles/somebar/media-block").unwrap_or(true) {
                playback = Some(PlaybackBlock::new(user_connection))
            }

            let mut connman = None;
            if dconf_read_variant(dconf, "/dotfiles/somebar/connman-block").unwrap_or(true) {
                connman = Some(ConnmanBlock::new(system_connection, time.as_mut()))
            }

            let sys_qh = Rc::clone(&qh);

            handle
                .insert_source(user_connection, move |event, user_con, shared_data| {
                    let qh = Rc::clone(&qh);
                    let Some(member) = event.member() else {
                        return None;
                    };
                    if &*member == "PropertiesChanged" {
                        if let Some(ref mut media) = shared_data.shared_data.playback {
                            if media.query_media(event) {
                                shared_data.write_bar(&qh);
                            }
                        }
                    } else if &*member == "Notify" {
                        let property: dconf::CaDesrtDconfWriterNotify = event.read_all().unwrap();

                        for p in property.changes {
                            let mut new_prop = property.prefix.clone();
                            new_prop.push_str(&p);
                            match new_prop.as_str() {
                                "/dotfiles/somebar/font" => {
                                    let new_font: String = dconf_read_variant(
                                        shared_data.dconf,
                                        "/dotfiles/somebar/font",
                                    )
                                    .unwrap_or(String::from("FiraCode Nerd Font 14"));

                                    let split = new_font.rsplit_once(' ').unwrap();

                                    let font = split.0;
                                    let font_size: f32 = split.1.parse().unwrap();

                                    shared_data.iced =
                                        iced_tiny_skia::Backend::new(iced_tiny_skia::Settings {
                                            default_text_size: font_size,
                                            default_font: Font {
                                                monospaced: true,
                                                family: Family::Name(std::mem::transmute(font)),
                                                ..Default::default()
                                            },
                                        });
                                    shared_data.bar_settings.default_font = new_font;
                                    shared_data.ascii_font_width = shared_data
                                        .iced
                                        .measure(
                                            DIVIDER,
                                            shared_data.iced.default_size(),
                                            LineHeight::Relative(1.0),
                                            shared_data.iced.default_font(),
                                            Size {
                                                width: f32::INFINITY,
                                                height: f32::INFINITY,
                                            },
                                            Shaping::Basic,
                                        )
                                        .0;

                                    shared_data.relayout(Rc::clone(&qh));
                                }
                                "/dotfiles/somebar/time-block" => {
                                    if dconf_read_variant(
                                        shared_data.dconf,
                                        "/dotfiles/somebar/time-block",
                                    )
                                    .unwrap_or(true)
                                    {
                                        shared_data.shared_data.time = Some(TimeBlock::new(
                                            &loop_handle,
                                            dconf_read_variant(
                                                shared_data.dconf,
                                                "/dotfiles/somebar/time-show-day",
                                            )
                                            .unwrap_or(true),
                                            dconf_read_variant(
                                                dconf,
                                                "/dotfiles/somebar/update-time-ntp",
                                            )
                                            .unwrap_or(true),
                                            dconf_read_variant(
                                                dconf,
                                                "/dotfiles/somebar/time-servers",
                                            )
                                            .unwrap_or(
                                                NTP_SERVERS
                                                    .into_iter()
                                                    .map(|s| s.to_string())
                                                    .collect(),
                                            ),
                                            Rc::clone(&qh),
                                        ));
                                    } else {
                                        if let Some(time) = shared_data.shared_data.time.take() {
                                            time.unregister(&loop_handle);
                                        }
                                    }
                                    shared_data.write_bar(&qh);
                                }
                                "/dotfiles/somebar/time-show-day" => {
                                    if let Some(ref mut time) = shared_data.shared_data.time {
                                        time.show_day = dconf_read_variant(
                                            shared_data.dconf,
                                            "/dotfiles/somebar/time-show-day",
                                        )
                                        .unwrap_or(true);
                                        shared_data.write_bar(&qh);
                                    }
                                }
                                "/dotfiles/somebar/update-time-ntp" => {
                                    if let Some(ref mut time) = shared_data.shared_data.time {
                                        time.update_time_ntp = dconf_read_variant(
                                            shared_data.dconf,
                                            "/dotfiles/somebar/update-time-ntp",
                                        )
                                        .unwrap_or(true);
                                        shared_data.write_bar(&qh);
                                    }
                                }
                                "/dotfiles/somebar/brightness-block" => {
                                    if dconf_read_variant(
                                        shared_data.dconf,
                                        "/dotfiles/somebar/brightness-block",
                                    )
                                    .unwrap_or(true)
                                    {
                                        shared_data.shared_data.brightness = Some(
                                            BrightnessBlock::new(&loop_handle, Rc::clone(&qh)),
                                        );
                                    } else {
                                        if let Some(brightness) =
                                            shared_data.shared_data.brightness.take()
                                        {
                                            brightness.unregister(&loop_handle);
                                        }
                                    }
                                    shared_data.write_bar(&qh);
                                }
                                "/dotfiles/somebar/battery-block" => {
                                    if dconf_read_variant(
                                        shared_data.dconf,
                                        "/dotfiles/somebar/battery-block",
                                    )
                                    .unwrap_or(true)
                                    {
                                        shared_data.shared_data.bat_block =
                                            Some(BatteryBlock::new(system_connection));
                                    } else {
                                        if let Some(bat_block) =
                                            shared_data.shared_data.bat_block.take()
                                        {
                                            bat_block.unregister(system_connection);
                                        }
                                    }
                                    shared_data.write_bar(&qh);
                                }
                                "/dotfiles/somebar/connman-block" => {
                                    if dconf_read_variant(
                                        shared_data.dconf,
                                        "/dotfiles/somebar/connman-block",
                                    )
                                    .unwrap_or(true)
                                    {
                                        shared_data.shared_data.connman = Some(ConnmanBlock::new(
                                            system_connection,
                                            shared_data.shared_data.time.as_mut(),
                                        ));
                                    } else {
                                        if let Some(connman) =
                                            shared_data.shared_data.connman.take()
                                        {
                                            connman.unregister(system_connection);
                                        }
                                    }
                                    shared_data.write_bar(&qh);
                                }
                                "/dotfiles/somebar/media-block" => {
                                    if dconf_read_variant(
                                        shared_data.dconf,
                                        "/dotfiles/somebar/media-block",
                                    )
                                    .unwrap_or(true)
                                    {
                                        shared_data.shared_data.playback =
                                            Some(PlaybackBlock::new(user_con));
                                    } else {
                                        if let Some(media) = shared_data.shared_data.playback.take()
                                        {
                                            media.unregister(user_con);
                                        }
                                    }
                                    shared_data.write_bar(&qh);
                                }
                                "/dotfiles/somebar/color-active" => {
                                    shared_data
                                        .bar_settings
                                        .update_color_active(shared_data.dconf);
                                    shared_data.relayout(Rc::clone(&qh));
                                }
                                "/dotfiles/somebar/color-inactive" => {
                                    shared_data
                                        .bar_settings
                                        .update_color_inactive(shared_data.dconf);
                                    shared_data.relayout(Rc::clone(&qh));
                                }
                                "/dotfiles/somebar/padding-x" => {
                                    shared_data.bar_settings.padding_x = dconf_read_variant::<f64>(
                                        shared_data.dconf,
                                        "/dotfiles/somebar/padding-x",
                                    )
                                    .unwrap_or(10.0)
                                        as f32;

                                    shared_data.relayout(Rc::clone(&qh));
                                }
                                "/dotfiles/somebar/padding-y" => {
                                    shared_data.bar_settings.padding_y = dconf_read_variant::<f64>(
                                        shared_data.dconf,
                                        "/dotfiles/somebar/padding-y",
                                    )
                                    .unwrap_or(5.0)
                                        as f32;

                                    shared_data.relayout(Rc::clone(&qh));
                                }
                                "/dotfiles/somebar/top-bar" => {
                                    shared_data.bar_settings.top_bar = dconf_read_variant(
                                        shared_data.dconf,
                                        "/dotfiles/somebar/top-bar",
                                    )
                                    .unwrap_or(true);
                                    for monitor in shared_data.monitors.values_mut() {
                                        monitor.output.layer_surface.set_anchor(
                                            if shared_data.bar_settings.top_bar {
                                                Anchor::TOP
                                            } else {
                                                Anchor::BOTTOM
                                            } | Anchor::LEFT
                                                | Anchor::RIGHT,
                                        );
                                        monitor.output.layer_surface.commit();
                                    }
                                }
                                "/dotfiles/somebar/time-servers" => {
                                    if let Some(ref mut time) = shared_data.shared_data.time {
                                        time.time_servers = dconf_read_variant(
                                            dconf,
                                            "/dotfiles/somebar/time-servers",
                                        )
                                        .unwrap_or(
                                            NTP_SERVERS
                                                .into_iter()
                                                .map(|s| s.to_string())
                                                .collect(),
                                        );
                                        time.update_time();
                                        shared_data.write_bar(&qh);
                                    }
                                }
                                "/dotfiles/somebar/bar-show-time" => {
                                    shared_data.bar_settings.bar_show_time = dconf_read_variant(
                                        shared_data.dconf,
                                        "/dotfiles/somebar/bar-show-time",
                                    )
                                    .unwrap_or(500);
                                }
                                _ => {}
                            }
                        }
                    }
                    None
                })
                .unwrap();

            let system_connection: &'static mut SyncDBusSource<()> = &mut *system_connection_ptr;

            handle
                .insert_source(system_connection, move |event, dbus, shared_data| {
                    let Some(member) = event.member() else {
                    return None;
                };
                    if &*member == "PropertiesChanged" {
                        if let Some(ref mut bat_block) = shared_data.shared_data.bat_block {
                            let property: upower::OrgFreedesktopDBusPropertiesPropertiesChanged =
                                event.read_all().unwrap();
                            bat_block.query_battery(event.path().unwrap().into_static(), property);
                            shared_data.write_bar(&sys_qh);
                        }
                    } else if &*member == "PropertyChanged" {
                        if let Some(ref mut connman) = shared_data.shared_data.connman {
                            if connman.query_connman(
                                event,
                                dbus,
                                shared_data.shared_data.time.as_mut(),
                            ) {
                                shared_data.write_bar(&sys_qh);
                            }
                        }
                    } else if &*member == "DeviceAdded" {
                        if let Some(ref mut bat_block) = shared_data.shared_data.bat_block {
                            bat_block.device_added(event, dbus);

                            shared_data.write_bar(&sys_qh);
                        }
                    } else if &*member == "DeviceRemoved" {
                        if let Some(ref mut bat_block) = shared_data.shared_data.bat_block {
                            bat_block.device_removed(event);

                            shared_data.write_bar(&sys_qh);
                        }
                    }
                    None
                })
                .unwrap();

            let socket_file = dirs::runtime_dir().unwrap().join("rustbar-0");
            let _ = std::fs::remove_file(&socket_file);
            let socket = UnixListener::bind(&socket_file).unwrap();

            handle
                .insert_source(
                    Generic::new(socket, Interest::READ, calloop::Mode::Level),
                    move |_event, socket, shared_data| {
                        let (file, _) = socket.accept().unwrap();
                        let mut file = BufWriter::new(file);
                        shared_data.shared_data.fmt_table(&mut file).unwrap();

                        Ok(calloop::PostAction::Continue)
                    },
                )
                .unwrap();

            handle
                .insert_source(
                    Signals::new(&[Signal::SIGINT, Signal::SIGTERM]).unwrap(),
                    move |_, _, data| {
                        std::fs::remove_file(&socket_file).unwrap();
                        data.exit.stop();
                    },
                )
                .unwrap();

            let time_handle = handle
                .insert_source(
                    Timer::from_duration(Duration::from_secs(31_536_000)),
                    |_, _, _| {
                        // hmmmmm
                        TimeoutAction::Drop
                    },
                )
                .unwrap();

            Self {
                time,
                time_handle,
                brightness,
                bat_block: battery,
                playback,
                connman,
                selected: None,
            }
        }
    }
}

impl SharedData {
    fn fmt(
        &mut self,
        color: Color,
        backend: &iced_tiny_skia::Backend,
        output: &Monitor,
        padding_x: f32,
        padding_y: f32,
    ) -> (Primitive, Size<f32>) {
        let divider_measurement = backend.measure(
            DIVIDER,
            backend.default_size() + padding_y,
            LineHeight::Relative(1.0),
            backend.default_font(),
            Size::INFINITY,
            Shaping::Basic,
        );
        let mut primitives = Vec::new();
        let logical_size = output.output.viewport.logical_size();
        let mut x = logical_size.width - padding_x;

        if let Some(ref mut media) = self.playback {
            let mut content = String::new();
            media.fmt(&mut content);
            let measurement = backend
                .measure(
                    &content,
                    backend.default_size(),
                    LineHeight::Relative(1.0),
                    backend.default_font(),
                    Size::INFINITY,
                    Shaping::Basic,
                )
                .0;
            x -= measurement;
            primitives.push(Primitive::Text {
                content,
                bounds: Rectangle {
                    x,
                    y: logical_size.height / 2.0,
                    width: logical_size.width,
                    height: logical_size.height / 2.0,
                },
                color,
                size: backend.default_size(),
                line_height: LineHeight::Relative(1.0),
                font: backend.default_font(),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Center,
                shaping: Shaping::Basic,
            });
            x -= divider_measurement.0;
            primitives.push(Primitive::Text {
                content: DIVIDER.to_owned(),
                bounds: Rectangle {
                    x,
                    y: 0.0,
                    width: logical_size.width,
                    height: logical_size.height,
                },
                color,
                size: backend.default_size() + padding_y,
                line_height: LineHeight::Relative(1.0),
                font: backend.default_font(),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Top,
                shaping: Shaping::Basic,
            });
            media.x_at = x;
            media.width = measurement + divider_measurement.0 + padding_x;
        }

        if let Some(ref mut connman) = self.connman {
            let mut content = String::new();
            connman.fmt(&mut content);
            let measurement = backend
                .measure(
                    &content,
                    backend.default_size(),
                    LineHeight::Relative(1.0),
                    backend.default_font(),
                    Size::INFINITY,
                    Shaping::Basic,
                )
                .0;
            x -= measurement;
            primitives.push(Primitive::Text {
                content,
                bounds: Rectangle {
                    x,
                    y: logical_size.height / 2.0,
                    width: logical_size.width,
                    height: logical_size.height / 2.0,
                },
                color,
                size: backend.default_size(),
                line_height: LineHeight::Relative(1.0),
                font: backend.default_font(),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Center,
                shaping: Shaping::Basic,
            });
            x -= divider_measurement.0;
            primitives.push(Primitive::Text {
                content: DIVIDER.to_owned(),
                bounds: Rectangle {
                    x,
                    y: 0.0,
                    width: logical_size.width,
                    height: logical_size.height,
                },
                color,
                size: backend.default_size() + padding_y,
                line_height: LineHeight::Relative(1.0),
                font: backend.default_font(),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Top,
                shaping: Shaping::Basic,
            });

            connman.x_at = x;
            connman.width = measurement + divider_measurement.0;
        }

        if let Some(ref mut bat_block) = self.bat_block {
            let mut content = String::new();
            bat_block.fmt(&mut content);
            let measurement = backend
                .measure(
                    &content,
                    backend.default_size(),
                    LineHeight::Relative(1.0),
                    backend.default_font(),
                    Size::INFINITY,
                    Shaping::Basic,
                )
                .0;
            x -= measurement;
            primitives.push(Primitive::Text {
                content,
                bounds: Rectangle {
                    x,
                    y: logical_size.height / 2.0,
                    width: logical_size.width,
                    height: logical_size.height / 2.0,
                },
                color,
                size: backend.default_size(),
                line_height: LineHeight::Relative(1.0),
                font: backend.default_font(),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Center,
                shaping: Shaping::Basic,
            });
            x -= divider_measurement.0;
            primitives.push(Primitive::Text {
                content: DIVIDER.to_owned(),
                bounds: Rectangle {
                    x,
                    y: 0.0,
                    width: logical_size.width,
                    height: logical_size.height,
                },
                color,
                size: backend.default_size() + padding_y,
                line_height: LineHeight::Relative(1.0),
                font: backend.default_font(),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Top,
                shaping: Shaping::Basic,
            });

            bat_block.x_at = x;
            bat_block.width = measurement + divider_measurement.0;
        }

        if let Some(ref mut brightness) = self.brightness {
            let mut content = String::new();
            brightness.fmt(&mut content);
            let measurement = backend
                .measure(
                    &content,
                    backend.default_size(),
                    LineHeight::Relative(1.0),
                    backend.default_font(),
                    Size::INFINITY,
                    Shaping::Basic,
                )
                .0;
            x -= measurement;
            primitives.push(Primitive::Text {
                content,
                bounds: Rectangle {
                    x,
                    y: logical_size.height / 2.0,
                    width: logical_size.width,
                    height: logical_size.height / 2.0,
                },
                color,
                size: backend.default_size(),
                line_height: LineHeight::Relative(1.0),
                font: backend.default_font(),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Center,
                shaping: Shaping::Basic,
            });
            x -= divider_measurement.0;
            primitives.push(Primitive::Text {
                content: DIVIDER.to_owned(),
                bounds: Rectangle {
                    x,
                    y: 0.0,
                    width: logical_size.width,
                    height: logical_size.height,
                },
                color,
                size: backend.default_size() + padding_y,
                line_height: LineHeight::Relative(1.0),
                font: backend.default_font(),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Top,
                shaping: Shaping::Basic,
            });

            brightness.x_at = x;
            brightness.width = measurement + divider_measurement.0;
        }

        let mut height = 0.0;

        if let Some(ref mut time) = self.time {
            let mut content = String::new();
            time.fmt(&mut content);
            let measurement = backend.measure(
                &content,
                backend.default_size(),
                LineHeight::Relative(1.0),
                backend.default_font(),
                Size::INFINITY,
                Shaping::Basic,
            );
            height = measurement.1;
            let measurement = measurement.0;
            x -= measurement;
            primitives.push(Primitive::Text {
                content,
                bounds: Rectangle {
                    x,
                    y: logical_size.height / 2.0,
                    width: logical_size.width,
                    height: logical_size.height / 2.0,
                },
                color,
                size: backend.default_size(),
                line_height: LineHeight::Relative(1.0),
                font: backend.default_font(),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Center,
                shaping: Shaping::Basic,
            });
            x -= padding_x;

            time.x_at = x;
            time.width = measurement + padding_x;
        }

        (
            Primitive::Group { primitives },
            Size {
                width: (logical_size.width - x) + (padding_x * 2.0),
                height,
            },
        )
    }
}

impl SharedData {
    fn fmt_table(&self, f: &mut BufWriter<UnixStream>) -> std::io::Result<()> {
        f.write_all(b"\n")?;
        if let Some(ref time) = self.time {
            time.fmt_table(f)?;
        }

        if let Some(ref brightness) = self.brightness {
            brightness.fmt_table(f)?;
        }

        if let Some(ref bat_block) = self.bat_block {
            bat_block.fmt_table(f)?;
        }

        if let Some(ref connman) = self.connman {
            connman.fmt_table(f)?;
        }

        if let Some(ref media) = self.playback {
            media.fmt_table(f)
        } else {
            Ok(())
        }
    }
}

fn dconf_read_variant<T: FromVariant>(dconf_client: *mut DConfClient, path: &str) -> Option<T> {
    let value: Option<glib::variant::Variant> = {
        let key = CString::new(path).unwrap();

        unsafe {
            NonNull::new(dconf_client_read(dconf_client, key.as_ptr()))
                .map(|nn| std::mem::transmute(nn))
        }
    };
    value.and_then(|v| v.get())
}

fn main() {
    let conn = Connection::connect_to_env().unwrap();

    let (globals, event_queue) = registry_queue_init(&conn).unwrap();
    let qh = Rc::new(event_queue.handle());

    let compositor = CompositorState::bind(&globals, &qh).unwrap();

    let layer_shell = LayerShell::bind(&globals, &qh).unwrap();

    let dwl: ZnetTapesoftwareDwlWmV1 = globals.bind(&qh, 1..=1, GlobalData).unwrap();
    let cursor_shape_manager: WpCursorShapeManagerV1 =
        globals.bind(&qh, 1..=1, GlobalData).unwrap();

    let shm = Shm::bind(&globals, &qh).unwrap();

    let mut event_loop = EventLoop::try_new().unwrap();
    let handle = event_loop.handle();

    let dconf = unsafe { dconf_client_new() };
    let shared_data = SharedData::new(&handle, Rc::clone(&qh), dconf);

    let new_font: String = dconf_read_variant(dconf, "/dotfiles/somebar/font")
        .unwrap_or(String::from("FiraCode Nerd Font 14"));

    let split = new_font.rsplit_once(' ').unwrap();

    let font = split.0;
    let font_size: f32 = split.1.parse().unwrap();

    let backend = iced_tiny_skia::Backend::new(iced_tiny_skia::Settings {
        default_text_size: font_size,
        default_font: Font {
            monospaced: true,
            family: Family::Name(unsafe { std::mem::transmute(font) }),
            ..Default::default()
        },
    });

    let measured_text = backend.measure(
        DIVIDER,
        backend.default_size(),
        LineHeight::Relative(1.0),
        backend.default_font(),
        Size {
            width: f32::INFINITY,
            height: f32::INFINITY,
        },
        Shaping::Basic,
    );

    let bar_settings = BarSettings::new(new_font, dconf);

    let bar_size = Size {
        width: 0.0,
        height: measured_text.1 + bar_settings.padding_y * 2.0,
    };

    let pool = SlotPool::new(1920 * bar_size.height as usize * 4, &shm).unwrap();

    let mut simple_layer = SimpleLayer::new(
        RegistryState::new(&globals),
        SeatState::new(&globals, &qh),
        OutputState::new(&globals, &qh),
        shm,
        event_loop.get_signal(),
        pool,
        bar_size,
        backend,
        shared_data,
        dwl,
        cursor_shape_manager,
        dconf,
        layer_shell,
        compositor,
        bar_settings,
        unsafe { std::mem::transmute(event_loop.handle()) },
    );

    let guard = event_queue.prepare_read().unwrap();
    let fd = Generic::new(
        guard.connection_fd().as_raw_fd(),
        Interest::READ,
        Mode::Level,
    );
    drop(guard);

    let event_queue = Rc::new(RefCell::new(event_queue));
    let event_queue_loop = Rc::clone(&event_queue);
    event_loop
        .handle()
        .insert_source(fd, move |_, _, data| {
            if event_queue_loop
                .borrow_mut()
                .blocking_dispatch(data)
                .is_err()
            {
                panic!("display_dispatch");
            }
            Ok(calloop::PostAction::Continue)
        })
        .unwrap();

    {
        let mut event_queue = event_queue.borrow_mut();
        event_queue.roundtrip(&mut simple_layer).unwrap();
        event_queue.flush().unwrap();
    }

    event_loop
        .run(None, &mut simple_layer, move |data| {
            let mut event_queue = event_queue.borrow_mut();
            event_queue.dispatch_pending(data).unwrap();
            event_queue.flush().unwrap();
        })
        .unwrap();
}

#[derive(Debug)]
pub struct Output {
    layer_surface: LayerSurface,
    viewport: Viewport,
    mask: Mask,
    frame_req: bool,
    first_configure: bool,
    buffers: Option<Buffers>,
}

pub struct Monitor {
    output: Output,
    info_output: Option<Output>,
    wl_output: wl_output::WlOutput,
    is_in_overlay: bool,
    dwl: ZnetTapesoftwareDwlWmMonitorV1,
    layout: usize,
    bar_state: BarState,
    window_title: String,
    tags: Tags,
    selected: bool,
    status_bar_primitives: Arc<Primitive>,
    status_bar_bg: Arc<Primitive>,
}

impl Output {
    fn frame(&mut self, qh: &QueueHandle<SimpleLayer>) {
        if !self.frame_req {
            self.layer_surface
                .wl_surface()
                .frame(qh, self.layer_surface.wl_surface().clone());
            self.frame_req = true;
        }
        self.layer_surface.commit();
    }
}

pub struct BarSettings {
    color_active: (Color, Color),
    color_inactive: (Color, Color),
    default_font: String,
    padding_x: f32,
    padding_y: f32,
    bar_show_time: u64,
    top_bar: bool,
}

impl BarSettings {
    fn new(default_font: String, dconf: *mut DConfClient) -> BarSettings {
        let color_active: ((f64, f64, f64), (f64, f64, f64)) =
            dconf_read_variant(dconf, "/dotfiles/somebar/color-active")
                .unwrap_or(((1.0, 0.56, 0.25), (0.2, 0.227, 0.25)));

        let color_inactive: ((f64, f64, f64), (f64, f64, f64)) =
            dconf_read_variant(dconf, "/dotfiles/somebar/color-inactive")
                .unwrap_or(((0.701, 0.694, 0.678), (0.039, 0.054, 0.078)));

        BarSettings {
            color_active: (
                Color::from_rgb(
                    color_active.0 .0 as f32,
                    color_active.0 .1 as f32,
                    color_active.0 .2 as f32,
                ),
                Color::from_rgb(
                    color_active.1 .0 as f32,
                    color_active.1 .1 as f32,
                    color_active.1 .2 as f32,
                ),
            ),
            color_inactive: (
                Color::from_rgb(
                    color_inactive.0 .0 as f32,
                    color_inactive.0 .1 as f32,
                    color_inactive.0 .2 as f32,
                ),
                Color::from_rgb(
                    color_inactive.1 .0 as f32,
                    color_inactive.1 .1 as f32,
                    color_inactive.1 .2 as f32,
                ),
            ),
            default_font,
            padding_x: dconf_read_variant::<f64>(dconf, "/dotfiles/somebar/padding-x")
                .unwrap_or(10.0) as f32,
            padding_y: dconf_read_variant::<f64>(dconf, "/dotfiles/somebar/padding-y")
                .unwrap_or(5.0) as f32,
            bar_show_time: dconf_read_variant(dconf, "/dotfiles/somebar/bar-show-time")
                .unwrap_or(500),
            top_bar: dconf_read_variant(dconf, "/dotfiles/somebar/top-bar").unwrap_or(true),
        }
    }

    fn update_color_active(&mut self, dconf: *mut DConfClient) {
        let color_active: ((f64, f64, f64), (f64, f64, f64)) =
            dconf_read_variant(dconf, "/dotfiles/somebar/color-active")
                .unwrap_or(((1.0, 0.56, 0.25), (0.2, 0.227, 0.25)));

        self.color_active = (
            Color::from_rgb(
                color_active.0 .0 as f32,
                color_active.0 .1 as f32,
                color_active.0 .2 as f32,
            ),
            Color::from_rgb(
                color_active.1 .0 as f32,
                color_active.1 .1 as f32,
                color_active.1 .2 as f32,
            ),
        );
    }

    fn update_color_inactive(&mut self, dconf: *mut DConfClient) {
        let color_inactive: ((f64, f64, f64), (f64, f64, f64)) =
            dconf_read_variant(dconf, "/dotfiles/somebar/color-inactive")
                .unwrap_or(((1.0, 0.56, 0.25), (0.2, 0.227, 0.25)));

        self.color_inactive = (
            Color::from_rgb(
                color_inactive.0 .0 as f32,
                color_inactive.0 .1 as f32,
                color_inactive.0 .2 as f32,
            ),
            Color::from_rgb(
                color_inactive.1 .0 as f32,
                color_inactive.1 .1 as f32,
                color_inactive.1 .2 as f32,
            ),
        );
    }
}

enum OutputType {
    Bar,
    Info(ObjectId),
}

pub struct SimpleLayer {
    registry_state: RegistryState,
    seat_state: SeatState,
    output_state: OutputState,
    compositor_state: CompositorState,
    shm: Shm,
    exit: LoopSignal,
    loop_handle: LoopHandle<'static, SimpleLayer>,
    pool: SlotPool,
    bar_size: Size<f32>,
    layer_shell: LayerShell,
    dwl: ZnetTapesoftwareDwlWmV1,
    cursor_shape_manager: WpCursorShapeManagerV1,
    monitors: HashMap<ObjectId, Monitor>,
    output_map: HashMap<ObjectId, ObjectId>,
    znet_map: HashMap<ObjectId, ObjectId>,
    output_type_map: HashMap<ObjectId, OutputType>,
    tag_count: usize,
    ascii_font_width: f32,
    layouts: Vec<String>,
    iced: iced_tiny_skia::Backend,
    keyboard: Option<wl_keyboard::WlKeyboard>,
    pointer: Option<wl_pointer::WlPointer>,
    dconf: *mut DConfClient,
    shared_data: SharedData,
    bar_settings: BarSettings,
}

impl SimpleLayer {
    fn new(
        registry_state: RegistryState,
        seat_state: SeatState,
        output_state: OutputState,
        shm: Shm,
        exit: LoopSignal,
        pool: SlotPool,
        bar_size: Size<f32>,
        iced: iced_tiny_skia::Backend,
        shared_data: SharedData,
        dwl: ZnetTapesoftwareDwlWmV1,
        cursor_shape_manager: WpCursorShapeManagerV1,
        dconf: *mut DConfClient,
        layer_shell: LayerShell,
        compositor_state: CompositorState,
        bar_settings: BarSettings,
        loop_handle: LoopHandle<'static, SimpleLayer>,
    ) -> SimpleLayer {
        Self {
            registry_state,
            seat_state,
            output_state,
            shm,
            exit,
            pool,
            dconf,
            dwl,
            cursor_shape_manager,
            shared_data,
            layer_shell,
            compositor_state,
            bar_size,
            loop_handle,
            ascii_font_width: iced
                .measure(
                    DIVIDER,
                    iced.default_size(),
                    LineHeight::Relative(1.0),
                    iced.default_font(),
                    Size {
                        width: f32::INFINITY,
                        height: f32::INFINITY,
                    },
                    Shaping::Basic,
                )
                .0,
            iced,
            tag_count: 9,
            bar_settings,
            keyboard: None,
            pointer: None,
            layouts: Vec::new(),
            monitors: HashMap::new(),
            output_map: HashMap::new(),
            znet_map: HashMap::new(),
            output_type_map: HashMap::new(),
        }
    }

    fn write_bar(&mut self, qh: &QueueHandle<Self>) {
        for monitor in self.monitors.values_mut() {
            let (status_bar_primitives, bar_size) = self.shared_data.fmt(
                if monitor.selected {
                    self.bar_settings.color_active.0
                } else {
                    self.bar_settings.color_inactive.0
                },
                &self.iced,
                monitor,
                self.bar_settings.padding_x,
                self.bar_settings.padding_y,
            );

            monitor.status_bar_primitives = Arc::new(status_bar_primitives);
            self.bar_size = bar_size;
            self.bar_size.height += self.bar_settings.padding_y * 2.0;

            monitor.output.frame(qh);
        }
    }

    fn layout_applauncher(&mut self) {
        let monitor = self.monitors.values_mut().find(|o| o.selected).unwrap();
        match &mut monitor.bar_state {
            BarState::AppLauncher {
                apps,
                layout,
                current_input,
                matcher,
                selected,
            } => {
                layout.clear();
                let logical_height = monitor.output.viewport.logical_size().height;
                let input_string = String::from("run: ") + current_input.as_str();

                let width = self
                    .iced
                    .measure(
                        &input_string,
                        self.iced.default_size(),
                        LineHeight::Relative(1.0),
                        self.iced.default_font(),
                        Size {
                            width: f32::INFINITY,
                            height: f32::INFINITY,
                        },
                        Shaping::Basic,
                    )
                    .0;

                layout.push(Primitive::Quad {
                    bounds: Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: (self.bar_settings.padding_x * 2.0) + width,
                        height: logical_height,
                    },
                    background: Background::Color(self.bar_settings.color_active.1),
                    border_radius: [0.0, 0.0, 0.0, 0.0],
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                });

                layout.push(Primitive::Text {
                    content: input_string,
                    bounds: Rectangle {
                        x: self.bar_settings.padding_x,
                        y: logical_height / 2.0,
                        width: f32::INFINITY,
                        height: logical_height,
                    },
                    color: self.bar_settings.color_active.0,
                    size: self.iced.default_size(),
                    line_height: LineHeight::Relative(1.0),
                    font: self.iced.default_font(),
                    horizontal_alignment: Horizontal::Left,
                    vertical_alignment: Vertical::Center,
                    shaping: Shaping::Basic,
                });

                let mut width_at = (self.bar_settings.padding_x * 2.0) + width;
                let query = FuzzyQuery::new(&current_input);
                for app in apps.iter_mut() {
                    app.score = query.fuzzy_match(&app.name, matcher);
                }
                apps.sort_unstable_by(|a, b| b.score.cmp(&a.score));

                for (index, item) in (&apps[..15]).into_iter().enumerate() {
                    if item.score.is_some() {
                        let measurement = self
                            .iced
                            .measure(
                                &item.name,
                                self.iced.default_size(),
                                LineHeight::Relative(1.0),
                                self.iced.default_font(),
                                Size {
                                    width: f32::INFINITY,
                                    height: f32::INFINITY,
                                },
                                Shaping::Basic,
                            )
                            .0;
                        if index == *selected {
                            layout.push(Primitive::Quad {
                                bounds: Rectangle {
                                    x: width_at,
                                    y: 0.0,
                                    width: (self.bar_settings.padding_x * 2.0) + measurement,
                                    height: logical_height,
                                },
                                background: Background::Color(self.bar_settings.color_active.1),
                                border_radius: [0.0, 0.0, 0.0, 0.0],
                                border_width: 0.0,
                                border_color: Color::TRANSPARENT,
                            });
                        }
                        width_at += self.bar_settings.padding_x;
                        layout.push(Primitive::Text {
                            content: item.name.clone(),
                            bounds: Rectangle {
                                x: width_at,
                                y: logical_height / 2.0,
                                width: f32::INFINITY,
                                height: logical_height,
                            },
                            color: if index == *selected {
                                self.bar_settings.color_active.0
                            } else {
                                self.bar_settings.color_inactive.0
                            },
                            size: self.iced.default_size(),
                            line_height: LineHeight::Relative(1.0),
                            font: self.iced.default_font(),
                            horizontal_alignment: Horizontal::Left,
                            vertical_alignment: Vertical::Center,
                            shaping: Shaping::Basic,
                        });
                        width_at += measurement;
                        width_at += self.bar_settings.padding_x;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

impl CompositorHandler for SimpleLayer {
    fn scale_factor_changed(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        surface: &wl_surface::WlSurface,
        new_factor: i32,
    ) {
        let output = match self.output_type_map.get(&surface.id()) {
            Some(OutputType::Bar) => {
                if let Some(monitor) = self.monitors.get_mut(&surface.id()) {
                    &mut monitor.output
                } else {
                    return;
                }
            }
            Some(OutputType::Info(id)) => {
                if let Some(monitor) = self.monitors.get_mut(id) {
                    if let Some(ref mut output) = monitor.info_output {
                        output
                    } else {
                        return;
                    }
                } else {
                    return;
                }
            }
            None => return,
        };

        output.viewport = Viewport::with_physical_size(
            Size {
                width: (output.viewport.logical_size().width * new_factor as f32) as u32,
                height: (output.viewport.logical_size().height * new_factor as f32) as u32,
            },
            new_factor as f64,
        );
        // Initializes our double buffer one we've configured the layer shell
        output.buffers = Some(Buffers::new(
            &mut self.pool,
            output.viewport.physical_width(),
            output.viewport.physical_height(),
            wl_shm::Format::Argb8888,
        ));
        output.mask = Mask::new(
            output.viewport.physical_width(),
            output.viewport.physical_height(),
        )
        .unwrap();
        output
            .layer_surface
            .set_buffer_scale(new_factor as u32)
            .unwrap();
        output.frame(qh);
    }

    fn frame(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        surface: &wl_surface::WlSurface,
        _time: u32,
    ) {
        match self.output_type_map.get(&surface.id()) {
            Some(OutputType::Bar) => self.draw(qh, &surface.id()),
            Some(OutputType::Info(id)) => self.draw_info_box(qh, &id.clone()),
            None => {}
        }
    }
}

impl OutputHandler for SimpleLayer {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        output: wl_output::WlOutput,
    ) {
        let surface = self.compositor_state.create_surface(&qh);
        let layer = self.layer_shell.create_layer_surface(
            &qh,
            surface,
            Layer::Bottom,
            None::<String>,
            Some(&output),
        );

        layer.set_anchor(
            if self.bar_settings.top_bar {
                Anchor::TOP
            } else {
                Anchor::BOTTOM
            } | Anchor::LEFT
                | Anchor::RIGHT,
        );
        layer.set_size(0, self.bar_size.height as u32);
        layer.set_keyboard_interactivity(KeyboardInteractivity::None);
        layer.set_exclusive_zone(self.bar_size.height as i32);

        layer.commit();

        let layer_id = layer.wl_surface().id();

        self.output_map.insert(output.id(), layer_id.clone());

        let monitor = self.dwl.get_monitor(&output, &qh, GlobalData);

        self.znet_map.insert(monitor.id(), layer_id.clone());
        let mut new_output = Monitor {
            output: Output {
                layer_surface: layer,
                viewport: Viewport::with_physical_size(
                    Size {
                        width: 0,
                        height: self.bar_size.height as u32,
                    },
                    1.0,
                ),
                frame_req: false,
                mask: Mask::new(1, self.bar_size.height as u32).unwrap(),
                first_configure: true,
                buffers: None,
            },
            wl_output: output,
            info_output: None,
            window_title: String::new(),
            layout: 0,
            dwl: monitor,
            selected: false,
            tags: Tags::new(
                self.tag_count,
                self.bar_settings.padding_x,
                self.bar_size.height,
                self.ascii_font_width,
                &self.iced,
            ),
            is_in_overlay: false,
            bar_state: BarState::Normal,
            status_bar_primitives: Arc::new(Primitive::Group {
                primitives: Vec::new(),
            }),
            status_bar_bg: Arc::new(Primitive::Group {
                primitives: Vec::new(),
            }),
        };

        let (primitives, bar_size) = self.shared_data.fmt(
            self.bar_settings.color_inactive.0,
            &self.iced,
            &new_output,
            self.bar_settings.padding_x,
            self.bar_settings.padding_y,
        );
        new_output.status_bar_primitives = Arc::new(primitives);
        self.output_type_map
            .insert(layer_id.clone(), OutputType::Bar);
        self.monitors.insert(layer_id, new_output);

        self.bar_size = bar_size;
        self.bar_size.height += self.bar_settings.padding_y * 2.0;
    }

    fn update_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn output_destroyed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        output: wl_output::WlOutput,
    ) {
        self.output_map
            .remove(&output.id())
            .and_then(|id| self.monitors.remove(&id));
        output.release();
    }
}

impl LayerShellHandler for SimpleLayer {
    fn closed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _layer: &LayerSurface) {
        self.exit.stop();
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        layer: &LayerSurface,
        configure: LayerSurfaceConfigure,
        _serial: u32,
    ) {
        let output_type = self.output_type_map.get(&layer.wl_surface().id());
        let output = match output_type {
            Some(OutputType::Bar) => {
                if let Some(monitor) = self.monitors.get_mut(&layer.wl_surface().id()) {
                    &mut monitor.output
                } else {
                    return;
                }
            }
            Some(OutputType::Info(id)) => {
                if let Some(monitor) = self.monitors.get_mut(id) {
                    if let Some(ref mut output) = monitor.info_output {
                        output
                    } else {
                        return;
                    }
                } else {
                    return;
                }
            }
            None => return,
        };

        if configure.new_size.0 == 0 || configure.new_size.1 == 0 {
            output.viewport = Viewport::with_physical_size(
                Size {
                    width: 0,
                    height: 16,
                },
                output.viewport.scale_factor(),
            );
            output.mask = Mask::new(1, 16).unwrap();
        } else {
            output.viewport = Viewport::with_physical_size(
                Size {
                    width: configure.new_size.0 * output.viewport.scale_factor() as u32,
                    height: configure.new_size.1 * output.viewport.scale_factor() as u32,
                },
                output.viewport.scale_factor(),
            );
            output.mask = Mask::new(
                output.viewport.physical_width(),
                output.viewport.physical_height(),
            )
            .unwrap();
        }

        // Initializes our double buffer one we've configured the layer shell
        output.buffers = Some(Buffers::new(
            &mut self.pool,
            output.viewport.physical_width(),
            output.viewport.physical_height(),
            wl_shm::Format::Argb8888,
        ));

        if output.first_configure {
            output.first_configure = false;
            match output_type {
                Some(OutputType::Bar) => self.draw(qh, &layer.wl_surface().id()),
                Some(OutputType::Info(id)) => self.draw_info_box(qh, &id.clone()),
                None => {}
            }
        }
    }
}

impl SeatHandler for SimpleLayer {
    fn seat_state(&mut self) -> &mut SeatState {
        &mut self.seat_state
    }

    fn new_seat(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_seat::WlSeat) {}

    fn new_capability(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        seat: wl_seat::WlSeat,
        capability: Capability,
    ) {
        if capability == Capability::Keyboard && self.keyboard.is_none() {
            println!("Set keyboard capability");
            let keyboard = self
                .seat_state
                .get_keyboard(qh, &seat, None)
                .expect("Failed to create keyboard");
            self.keyboard = Some(keyboard);
        }

        if capability == Capability::Pointer && self.pointer.is_none() {
            println!("Set pointer capability");
            let pointer = self
                .seat_state
                .get_pointer(qh, &seat)
                .expect("Failed to create pointer");
            self.pointer = Some(pointer);
        }
    }

    fn remove_capability(
        &mut self,
        _conn: &Connection,
        _: &QueueHandle<Self>,
        _: wl_seat::WlSeat,
        capability: Capability,
    ) {
        if capability == Capability::Keyboard && self.keyboard.is_some() {
            println!("Unset keyboard capability");
            self.keyboard.take().unwrap().release();
        }

        if capability == Capability::Pointer && self.pointer.is_some() {
            println!("Unset pointer capability");
            self.pointer.take().unwrap().release();
        }
    }

    fn remove_seat(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_seat::WlSeat) {}
}

impl KeyboardHandler for SimpleLayer {
    fn enter(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _: &wl_surface::WlSurface,
        _: u32,
        _: &[u32],
        _: &[u32],
    ) {
    }

    fn leave(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _: &wl_surface::WlSurface,
        _: u32,
    ) {
    }

    fn press_key(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        _keyboard: &wl_keyboard::WlKeyboard,
        _: u32,
        event: KeyEvent,
    ) {
        let monitor = self.monitors.values_mut().find(|o| o.selected).unwrap();
        match dbg!(event.keysym) {
            keysyms::XKB_KEY_Escape => {
                monitor.bar_state = BarState::Normal;
                monitor
                    .output
                    .layer_surface
                    .set_keyboard_interactivity(KeyboardInteractivity::None);
                if monitor.is_in_overlay {
                    monitor.output.layer_surface.set_layer(Layer::Bottom);
                    monitor.is_in_overlay = false;
                }
                if !monitor.output.frame_req {
                    monitor
                        .output
                        .layer_surface
                        .wl_surface()
                        .frame(qh, monitor.output.layer_surface.wl_surface().clone());
                    monitor.output.frame_req = true;
                }
                monitor.output.layer_surface.commit();
            }
            keysyms::XKB_KEY_BackSpace => match &mut monitor.bar_state {
                BarState::AppLauncher { current_input, .. } => {
                    current_input.pop();
                    monitor.output.frame(qh);
                    self.layout_applauncher();
                }
                _ => {}
            },
            keysyms::XKB_KEY_Down | keysyms::XKB_KEY_Right => match &mut monitor.bar_state {
                BarState::AppLauncher { selected, .. } => {
                    selected.add_assign(1);
                    monitor.output.frame(qh);
                    self.layout_applauncher();
                }
                _ => {}
            },
            keysyms::XKB_KEY_Up | keysyms::XKB_KEY_Left => match &mut monitor.bar_state {
                BarState::AppLauncher { selected, .. } => {
                    selected.sub_assign(1);
                    monitor.output.frame(qh);
                    self.layout_applauncher();
                }
                _ => {}
            },
            keysyms::XKB_KEY_Return => match &mut monitor.bar_state {
                BarState::AppLauncher { apps, selected, .. } => {
                    let app = apps.get(*selected).unwrap();

                    std::process::Command::new("sh")
                        .args(&["-c", &app.command])
                        .spawn()
                        .unwrap();

                    monitor.bar_state = BarState::Normal;
                    monitor
                        .output
                        .layer_surface
                        .set_keyboard_interactivity(KeyboardInteractivity::None);
                    if monitor.is_in_overlay {
                        monitor.output.layer_surface.set_layer(Layer::Bottom);
                        monitor.is_in_overlay = false;
                    }
                    if !monitor.output.frame_req {
                        monitor
                            .output
                            .layer_surface
                            .wl_surface()
                            .frame(qh, monitor.output.layer_surface.wl_surface().clone());
                        monitor.output.frame_req = true;
                    }
                    monitor.output.layer_surface.commit();
                }
                _ => {}
            },
            _ => match &mut monitor.bar_state {
                BarState::AppLauncher { current_input, .. } => {
                    if let Some(c) = event.utf8 {
                        current_input.push_str(&c);
                        monitor.output.frame(qh);
                        self.layout_applauncher();
                    }
                }
                _ => {}
            },
        }
    }

    fn release_key(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _: u32,
        _: KeyEvent,
    ) {
    }

    fn update_modifiers(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _serial: u32,
        _modifiers: Modifiers,
    ) {
    }
}

impl PointerHandler for SimpleLayer {
    fn pointer_frame(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        pointer: &wl_pointer::WlPointer,
        events: &[PointerEvent],
    ) {
        use PointerEventKind::*;
        for event in events {
            if let Some(monitor) = self.monitors.get_mut(&event.surface.id()) {
                match event.kind {
                    Enter { serial } => {
                        let cursor_device = self
                            .cursor_shape_manager
                            .get_pointer(pointer, qh, GlobalData);

                        cursor_device.set_shape(
                            serial,
                            cursor_shape::wp_cursor_shape_device_v1::Shape::Default,
                        );
                    }
                    Leave { .. } => {
                        monitor.status_bar_bg = Arc::new(Primitive::Group {
                            primitives: Vec::new(),
                        });
                        monitor.output.frame(qh);
                        if let Some(mut info) = monitor.info_output.take() {
                            // println!("info output destroy");
                            // let buffers = info.buffers.take().unwrap();
                            // buffers.buffers[0].wl_buffer().destroy();
                            // buffers.buffers[1].wl_buffer().destroy();
                            // info.layer_surface.wl_surface().destroy();
                        }
                        self.select_block(None, event.surface.id());
                    }
                    Motion { .. } => {
                        macro_rules! status_bar_bg {
                            ($x_at:expr,$width:expr,$self:expr,$output:expr,$qh:expr) => {
                                $output.status_bar_bg = Arc::new(Primitive::Quad {
                                    bounds: Rectangle {
                                        x: $x_at,
                                        y: 0.0,
                                        width: $width,
                                        height: $self.bar_size.height,
                                    },
                                    background: Background::Color(
                                        $self.bar_settings.color_active.0,
                                    ),
                                    border_radius: [0.0, 0.0, 0.0, 0.0],
                                    border_width: 0.0,
                                    border_color: Color::TRANSPARENT,
                                });
                            };
                        }
                        if let Some(ref media) = self.shared_data.playback {
                            if event.position.0 >= media.x_at as f64 {
                                if self.shared_data.selected != Some(0) {
                                    status_bar_bg!(media.x_at, media.width, self, monitor, qh);
                                    monitor.output.frame(qh);
                                    if monitor.info_output.is_none() {
                                        let surface = self.compositor_state.create_surface(&qh);
                                        let info_layer = self.layer_shell.create_layer_surface(
                                            &qh,
                                            surface,
                                            Layer::Overlay,
                                            None::<String>,
                                            Some(&monitor.wl_output),
                                        );

                                        info_layer.set_anchor(
                                            if self.bar_settings.top_bar {
                                                Anchor::TOP
                                            } else {
                                                Anchor::BOTTOM
                                            } | Anchor::RIGHT,
                                        );
                                        info_layer.set_size(
                                            256 + (self.bar_settings.padding_y as u32 * 2),
                                            512 + (self.bar_settings.padding_y as u32 * 2),
                                        );
                                        info_layer.set_keyboard_interactivity(
                                            KeyboardInteractivity::None,
                                        );

                                        info_layer.commit();
                                        self.output_type_map.insert(
                                            info_layer.wl_surface().id(),
                                            OutputType::Info(
                                                monitor.output.layer_surface.wl_surface().id(),
                                            ),
                                        );
                                        let viewport = Viewport::with_physical_size(
                                            Size {
                                                width: (256
                                                    + (self.bar_settings.padding_y as u32 * 2))
                                                    * monitor.output.viewport.scale_factor() as u32,
                                                height: (512
                                                    + (self.bar_settings.padding_y as u32 * 2))
                                                    * monitor.output.viewport.scale_factor() as u32,
                                            },
                                            monitor.output.viewport.scale_factor(),
                                        );
                                        monitor.info_output = Some(Output {
                                            layer_surface: info_layer,
                                            frame_req: false,
                                            mask: Mask::new(
                                                viewport.physical_width(),
                                                viewport.physical_height(),
                                            )
                                            .unwrap(),
                                            first_configure: true,
                                            buffers: None,
                                            viewport,
                                        });
                                    }
                                    self.select_block(Some(0), event.surface.id());
                                }
                                return;
                            }
                        }

                        if let Some(ref connman) = self.shared_data.connman {
                            if event.position.0 >= connman.x_at as f64 {
                                if self.shared_data.selected != Some(2) {
                                    status_bar_bg!(connman.x_at, connman.width, self, monitor, qh);
                                    monitor.output.frame(qh);
                                    self.select_block(Some(2), event.surface.id());
                                }
                                return;
                            }
                        }

                        if let Some(ref bat_block) = self.shared_data.bat_block {
                            if event.position.0 >= bat_block.x_at as f64 {
                                if self.shared_data.selected != Some(4) {
                                    status_bar_bg!(
                                        bat_block.x_at,
                                        bat_block.width,
                                        self,
                                        monitor,
                                        qh
                                    );
                                    monitor.output.frame(qh);
                                    self.select_block(Some(4), event.surface.id());
                                }
                                return;
                            }
                        }

                        if let Some(ref brightness) = self.shared_data.brightness {
                            if event.position.0 >= brightness.x_at as f64 {
                                if self.shared_data.selected != Some(6) {
                                    status_bar_bg!(
                                        brightness.x_at,
                                        brightness.width,
                                        self,
                                        monitor,
                                        qh
                                    );
                                    monitor.output.frame(qh);
                                    self.select_block(Some(6), event.surface.id());
                                }
                                return;
                            }
                        }

                        if let Some(ref time) = self.shared_data.time {
                            if event.position.0 >= time.x_at as f64 {
                                if self.shared_data.selected != Some(8) {
                                    status_bar_bg!(time.x_at, time.width, self, monitor, qh);
                                    monitor.output.frame(qh);
                                    self.select_block(Some(8), event.surface.id());
                                }
                                return;
                            }
                        }

                        monitor.status_bar_bg = Arc::new(Primitive::Group {
                            primitives: Vec::new(),
                        });
                        if let Some(mut info) = monitor.info_output.take() {
                            // println!("info output destroy");
                            // let buffers = info.buffers.take().unwrap();
                            // buffers.buffers[0].wl_buffer().destroy();
                            // buffers.buffers[1].wl_buffer().destroy();
                            // info.layer_surface.wl_surface().destroy();
                        }
                        monitor.output.frame(qh);

                        self.select_block(None, event.surface.id());
                    }
                    Press { button, .. } => match button {
                        BTN_LEFT => {
                            if event.position.0 < monitor.tags.width as f64 {
                                if let Some(tag) = (0..monitor.tags.tags.len()).find(|&tag| {
                                    (monitor.tags.num_width as f64 * ((tag + 1) as f64))
                                        > event.position.0
                                }) {
                                    monitor.dwl.set_tags(1 << tag, 1);
                                }
                            }
                        }
                        _ => {}
                    },
                    Release { .. } => {
                        // println!("Release {:x} @ {:?}", button, event.position);
                    }
                    Axis { .. } => {
                        // println!("Scroll H:{horizontal:?}, V:{vertical:?}");
                    }
                }
            }
        }
    }
}

impl ShmHandler for SimpleLayer {
    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm
    }
}

impl SimpleLayer {
    pub fn relayout(&mut self, qh: Rc<QueueHandle<Self>>) {
        self.write_bar(qh.as_ref());

        for monitor in self.monitors.values_mut() {
            monitor
                .output
                .layer_surface
                .set_size(0, self.bar_size.height as u32);
            monitor
                .output
                .layer_surface
                .set_exclusive_zone(self.bar_size.height as i32);
            monitor.tags.relayout(
                self.bar_settings.padding_x,
                self.bar_size.height,
                &self.iced,
                self.ascii_font_width,
                self.tag_count,
            );
            monitor.tags.relayout_bg(
                self.bar_settings.color_inactive,
                self.bar_settings.color_active,
                self.bar_size.height,
            );

            if !monitor.output.frame_req {
                monitor.output.layer_surface.wl_surface().frame(
                    qh.as_ref(),
                    monitor.output.layer_surface.wl_surface().clone(),
                );
                monitor.output.frame_req = true;
            }

            monitor.output.layer_surface.commit();
        }
    }
    pub fn draw(&mut self, _qh: &QueueHandle<Self>, output: &ObjectId) {
        if let Some(monitor) = self.monitors.get_mut(output) {
            monitor.output.frame_req = false;
            let width = monitor.output.viewport.physical_width();
            let height = monitor.output.viewport.physical_height();
            let logical_size = monitor.output.viewport.logical_size();

            // Draw to the window:
            if let Some(ref mut buffers) = monitor.output.buffers {
                let canvas = buffers.canvas(&mut self.pool).unwrap();
                let mut pixmap = PixmapMut::from_bytes(canvas, width, height).unwrap();
                pixmap.fill(tiny_skia::Color::WHITE);
                match &monitor.bar_state {
                    BarState::Normal => {
                        self.iced.draw::<String>(
                            &mut pixmap,
                            &mut monitor.output.mask,
                            &[
                                Primitive::Cache {
                                    content: Arc::clone(&monitor.tags.tags_background),
                                },
                                Primitive::Cache {
                                    content: Arc::clone(&monitor.tags.primitives),
                                },
                                Primitive::Cache {
                                    content: Arc::clone(&monitor.tags.tag_windows),
                                },
                                Primitive::Text {
                                    content: self.layouts[monitor.layout].clone(),
                                    bounds: Rectangle {
                                        x: monitor.tags.width + self.bar_settings.padding_x,
                                        y: logical_size.height / 2.0,
                                        width: logical_size.width,
                                        height: logical_size.height / 2.0,
                                    },
                                    color: if monitor.selected {
                                        self.bar_settings.color_active.0
                                    } else {
                                        self.bar_settings.color_inactive.0
                                    },
                                    size: self.iced.default_size(),
                                    line_height: LineHeight::Relative(1.0),
                                    font: self.iced.default_font(),
                                    horizontal_alignment: Horizontal::Left,
                                    vertical_alignment: Vertical::Center,
                                    shaping: Shaping::Basic,
                                },
                                Primitive::Text {
                                    content: monitor.window_title.clone(),
                                    bounds: Rectangle {
                                        x: monitor.tags.width
                                            + (self.bar_settings.padding_x * 2.0)
                                            + (self.ascii_font_width * 3.0),
                                        y: logical_size.height / 2.0,
                                        width: (logical_size.width - self.bar_size.width)
                                            - (monitor.tags.width + (self.ascii_font_width * 3.0)),
                                        height: logical_size.height / 2.0,
                                    },
                                    color: if monitor.selected {
                                        self.bar_settings.color_active.0
                                    } else {
                                        self.bar_settings.color_inactive.0
                                    },
                                    size: self.iced.default_size(),
                                    line_height: LineHeight::Relative(1.0),
                                    font: self.iced.default_font(),
                                    horizontal_alignment: Horizontal::Left,
                                    vertical_alignment: Vertical::Center,
                                    shaping: Shaping::Basic,
                                },
                                Primitive::Cache {
                                    content: Arc::clone(&monitor.status_bar_bg),
                                },
                                Primitive::Cache {
                                    content: Arc::clone(&monitor.status_bar_primitives),
                                },
                            ],
                            &monitor.output.viewport,
                            &[Rectangle {
                                x: 0.0,
                                y: 0.0,
                                width: width as f32,
                                height: height as f32,
                            }],
                            if monitor.selected {
                                self.bar_settings.color_active.1
                            } else {
                                self.bar_settings.color_inactive.1
                            },
                            &[],
                        );
                    }
                    BarState::ProgressBar { percentage, icon } => {
                        self.iced.draw::<String>(
                            &mut pixmap,
                            &mut monitor.output.mask,
                            &[
                                Primitive::Quad {
                                    bounds: Rectangle {
                                        x: 0.0,
                                        y: 0.0,
                                        width: monitor.tags.num_width,
                                        height: logical_size.height,
                                    },
                                    background: Background::Color(if monitor.selected {
                                        self.bar_settings.color_active.1
                                    } else {
                                        self.bar_settings.color_inactive.1
                                    }),
                                    border_radius: [0.0; 4],
                                    border_width: 0.0,
                                    border_color: Color::TRANSPARENT,
                                },
                                Primitive::Quad {
                                    bounds: Rectangle {
                                        x: monitor.tags.num_width,
                                        y: 0.0,
                                        width: (logical_size.width - monitor.tags.num_width)
                                            * percentage,
                                        height: logical_size.height,
                                    },
                                    background: Background::Color(if monitor.selected {
                                        self.bar_settings.color_active.0
                                    } else {
                                        self.bar_settings.color_inactive.0
                                    }),
                                    border_radius: [0.0; 4],
                                    border_width: 0.0,
                                    border_color: Color::TRANSPARENT,
                                },
                                Primitive::Text {
                                    content: icon.to_string(),
                                    bounds: Rectangle {
                                        x: monitor.tags.num_width / 2.0,
                                        y: logical_size.height / 2.0,
                                        width: monitor.tags.num_width,
                                        height: logical_size.height,
                                    },
                                    color: if monitor.selected {
                                        self.bar_settings.color_active.0
                                    } else {
                                        self.bar_settings.color_inactive.0
                                    },
                                    size: self.iced.default_size(),
                                    line_height: LineHeight::Relative(1.0),
                                    font: self.iced.default_font(),
                                    horizontal_alignment: Horizontal::Center,
                                    vertical_alignment: Vertical::Center,
                                    shaping: Shaping::Basic,
                                },
                            ],
                            &monitor.output.viewport,
                            &[Rectangle {
                                x: 0.0,
                                y: 0.0,
                                width: width as f32,
                                height: height as f32,
                            }],
                            self.bar_settings.color_inactive.1,
                            &[],
                        );
                    }
                    BarState::AppLauncher { layout, .. } => {
                        self.iced.draw::<String>(
                            &mut pixmap,
                            &mut monitor.output.mask,
                            layout.as_ref(),
                            &monitor.output.viewport,
                            &[Rectangle {
                                x: 0.0,
                                y: 0.0,
                                width: width as f32,
                                height: height as f32,
                            }],
                            self.bar_settings.color_inactive.1,
                            &[],
                        );
                    }
                }
                // Damage the entire window
                monitor.output.layer_surface.wl_surface().damage_buffer(
                    0,
                    0,
                    width as i32,
                    height as i32,
                );

                buffers
                    .buffer()
                    .attach_to(monitor.output.layer_surface.wl_surface())
                    .expect("buffer attach");
                /*
                // Request our next frame
                self.layer
                    .wl_surface()
                    .frame(qh, self.layer.wl_surface().clone());
                */

                // Attach and commit to present.
                monitor.output.layer_surface.commit();
                buffers.flip();
            }
        }
    }
    pub fn draw_info_box(&mut self, _qh: &QueueHandle<Self>, surface_id: &ObjectId) {
        if let Some(monitor) = self.monitors.get_mut(surface_id) {
            if let Some(ref mut output) = monitor.info_output {
                output.frame_req = false;
                let width = output.viewport.physical_width();
                let height = output.viewport.physical_height();
                let logical_size = output.viewport.logical_size();

                // Draw to the window:
                if let Some(ref mut buffers) = output.buffers {
                    let canvas = buffers.canvas(&mut self.pool).unwrap();
                    let mut pixmap = PixmapMut::from_bytes(canvas, width, height).unwrap();
                    self.iced.draw::<String>(
                        &mut pixmap,
                        &mut output.mask,
                        &[Primitive::Text {
                            content: "Work in Progress".to_owned(),
                            bounds: Rectangle {
                                x: 8.0,
                                y: 8.0,
                                width: logical_size.width,
                                height: logical_size.height,
                            },
                            color: self.bar_settings.color_active.0,
                            size: self.iced.default_size(),
                            line_height: LineHeight::Relative(1.0),
                            font: self.iced.default_font(),
                            horizontal_alignment: Horizontal::Left,
                            vertical_alignment: Vertical::Top,
                            shaping: Shaping::Basic,
                        }],
                        &output.viewport,
                        &[Rectangle {
                            x: 0.0,
                            y: 0.0,
                            width: width as f32,
                            height: height as f32,
                        }],
                        self.bar_settings.color_active.1,
                        &[],
                    );
                    // Damage the entire window
                    output.layer_surface.wl_surface().damage_buffer(
                        0,
                        0,
                        width as i32,
                        height as i32,
                    );

                    buffers
                        .buffer()
                        .attach_to(output.layer_surface.wl_surface())
                        .expect("buffer attach");
                    /*
                    // Request our next frame
                    self.layer
                        .wl_surface()
                        .frame(qh, self.layer.wl_surface().clone());
                    */

                    // Attach and commit to present.
                    output.layer_surface.commit();
                    buffers.flip();
                }
            }
        }
    }
    pub fn select_block(&mut self, block: Option<usize>, output: ObjectId) {
        macro_rules! change_text_color {
            ($b:expr,$color:expr,$primitives:expr) => {
                match $primitives.get_mut($b) {
                    Some(Primitive::Text { color, .. }) => {
                        *color = $color;
                    }
                    None => {}
                    _ => unreachable!(),
                }
            };
        }
        macro_rules! change_text_content {
            ($b:expr,$text:expr,$primitives:expr) => {
                match $primitives.get_mut($b) {
                    Some(Primitive::Text { content, .. }) => {
                        *content = $text;
                    }
                    None => {}
                    _ => unreachable!(),
                }
            };
        }
        if let Some(output) = self.monitors.get_mut(&output) {
            if let Some(block) = block {
                match Arc::make_mut(&mut output.status_bar_primitives) {
                    Primitive::Group { primitives } => {
                        if let Some(b) = self.shared_data.selected {
                            change_text_color!(b + 1, self.bar_settings.color_active.0, primitives);
                            change_text_color!(b, self.bar_settings.color_active.0, primitives);
                            // change_text_color!(b, self.bar_settings.color_active.0, primitives);
                            change_text_content!(b - 1, DIVIDER.to_owned(), primitives);
                            change_text_content!(b + 1, DIVIDER.to_owned(), primitives);
                            // change_text_color!(b, self.bar_settings.color_active.0, primitives);
                        }
                        change_text_color!(block + 1, self.bar_settings.color_active.1, primitives);
                        change_text_color!(block, self.bar_settings.color_inactive.1, primitives);
                        // change_text_color!(block, self.bar_settings.color_inactive.1, primitives);
                        change_text_content!(block - 1, DIVIDER_HARD.to_owned(), primitives);
                        change_text_content!(block + 1, DIVIDER_HARD.to_owned(), primitives);
                        // change_text_color!(block, self.bar_settings.color_inactive.1, primitives);
                    }
                    _ => unreachable!(),
                }
            } else {
                match Arc::make_mut(&mut output.status_bar_primitives) {
                    Primitive::Group { primitives } => {
                        if let Some(b) = self.shared_data.selected {
                            change_text_color!(b + 1, self.bar_settings.color_active.0, primitives);
                            change_text_color!(b, self.bar_settings.color_active.0, primitives);
                            // change_text_color!(b, self.bar_settings.color_active.0, primitives);
                            change_text_content!(b - 1, DIVIDER.to_owned(), primitives);
                            change_text_content!(b + 1, DIVIDER.to_owned(), primitives);
                            // change_text_color!(b, self.bar_settings.color_active.0, primitives);
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        self.shared_data.selected = block;
    }
}

delegate_compositor!(SimpleLayer);
delegate_output!(SimpleLayer);
delegate_shm!(SimpleLayer);

delegate_seat!(SimpleLayer);
delegate_keyboard!(SimpleLayer);
delegate_pointer!(SimpleLayer);

delegate_layer!(SimpleLayer);

delegate_registry!(SimpleLayer);

impl
    client::Dispatch<
        znet_dwl::znet_tapesoftware_dwl_wm_v1::ZnetTapesoftwareDwlWmV1,
        smithay_client_toolkit::globals::GlobalData,
    > for SimpleLayer
{
    fn event(
        state: &mut Self,
        _proxy: &znet_dwl::znet_tapesoftware_dwl_wm_v1::ZnetTapesoftwareDwlWmV1,
        event: znet_dwl::znet_tapesoftware_dwl_wm_v1::Event,
        _data: &smithay_client_toolkit::globals::GlobalData,
        _conn: &client::Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            znet_dwl::znet_tapesoftware_dwl_wm_v1::Event::Tag { count } => {
                state.tag_count = count as usize;
            }
            znet_dwl::znet_tapesoftware_dwl_wm_v1::Event::Layout { name } => {
                state.layouts.push(name);
            }
            znet_dwl::znet_tapesoftware_dwl_wm_v1::Event::ExecWobCommand { command } => {
                if let Ok(command) = command.into_result() {
                    let monitor = state.monitors.values_mut().find(|o| o.selected).unwrap();
                    if !monitor.is_in_overlay {
                        monitor.output.layer_surface.set_layer(Layer::Overlay);
                    }
                    state.loop_handle.remove(state.shared_data.time_handle);
                    let number = String::from_utf8(match command {
                        WobCommand::VolumeUp => {
                            std::process::Command::new("pamixer")
                                .args(&["-i", "5", "--get-volume"])
                                .output()
                                .unwrap()
                                .stdout
                        }
                        WobCommand::VolumeDown => {
                            std::process::Command::new("pamixer")
                                .args(&["-d", "5", "--get-volume"])
                                .output()
                                .unwrap()
                                .stdout
                        }
                        WobCommand::LightUp => {
                            std::process::Command::new("light")
                                .args(&["-A", "5"])
                                .status()
                                .unwrap();
                            std::process::Command::new("light")
                                .arg("-G")
                                .output()
                                .unwrap()
                                .stdout
                        }
                        WobCommand::LightDown => {
                            std::process::Command::new("light")
                                .args(&["-U", "5"])
                                .status()
                                .unwrap();
                            std::process::Command::new("light")
                                .arg("-G")
                                .output()
                                .unwrap()
                                .stdout
                        }
                        WobCommand::LaunchApp => {
                            monitor
                                .output
                                .layer_surface
                                .set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
                            monitor.is_in_overlay = true;
                            let apps = freedesktop_desktop_entry::Iter::new(default_paths()).fold(
                                Vec::new(),
                                |mut items, entry| {
                                    if let Ok(bytes) = std::fs::read_to_string(&entry) {
                                        if let Ok(entry) = DesktopEntry::decode(&entry, &bytes) {
                                            if let Some(name) = entry.name(None) {
                                                if let Some(exec) = entry.exec() {
                                                    let mut command = exec.to_owned();
                                                    while let Some(index) =
                                                        memchr(b'%', command.as_bytes())
                                                    {
                                                        if index + 1 == command.len() {
                                                            command.pop();
                                                            command.pop();
                                                        } else {
                                                            command.remove(index + 1);
                                                            command.remove(index);
                                                        }
                                                    }
                                                    items.push(DesktopCommand {
                                                        name: name.into_owned(),
                                                        command,
                                                        score: None,
                                                    });
                                                }
                                            }
                                        }
                                    }
                                    items
                                },
                            );
                            monitor.bar_state = BarState::AppLauncher {
                                apps,
                                current_input: String::new(),
                                matcher: SkimMatcherV2::default(),
                                layout: Vec::new(),
                                selected: 0,
                            };
                            monitor.output.frame(qh);
                            state.layout_applauncher();
                            return;
                        }
                        WobCommand::Overlay => {
                            if monitor.is_in_overlay {
                                monitor.output.layer_surface.set_layer(Layer::Bottom);
                                monitor.is_in_overlay = false;
                            } else {
                                monitor.is_in_overlay = true;
                            }
                            monitor.output.frame(qh);
                            return;
                        }
                    })
                    .unwrap();

                    monitor.is_in_overlay = true;
                    monitor.bar_state = BarState::ProgressBar {
                        percentage: number.trim().parse::<f32>().unwrap() / 100.0,
                        icon: command.into(),
                    };
                    monitor.output.frame(qh);
                    let qh: &'static QueueHandle<Self> = unsafe { std::mem::transmute(qh) };
                    state.shared_data.time_handle = state
                        .loop_handle
                        .insert_source(
                            Timer::from_duration(Duration::from_millis(
                                state.bar_settings.bar_show_time,
                            )),
                            move |_, _, data| {
                                let monitor =
                                    data.monitors.values_mut().find(|o| o.selected).unwrap();
                                monitor.bar_state = BarState::Normal;
                                if monitor.is_in_overlay {
                                    monitor.output.layer_surface.set_layer(Layer::Bottom);
                                    monitor.is_in_overlay = false;
                                }
                                monitor.output.frame(qh);
                                TimeoutAction::Drop
                            },
                        )
                        .unwrap();
                }
            }
        }
    }
}

impl client::Dispatch<cursor_shape::wp_cursor_shape_manager_v1::WpCursorShapeManagerV1, GlobalData>
    for SimpleLayer
{
    fn event(
        _: &mut Self,
        _: &cursor_shape::wp_cursor_shape_manager_v1::WpCursorShapeManagerV1,
        _: <cursor_shape::wp_cursor_shape_manager_v1::WpCursorShapeManagerV1 as Proxy>::Event,
        _: &GlobalData,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl client::Dispatch<cursor_shape::wp_cursor_shape_device_v1::WpCursorShapeDeviceV1, GlobalData>
    for SimpleLayer
{
    fn event(
        _: &mut Self,
        _: &cursor_shape::wp_cursor_shape_device_v1::WpCursorShapeDeviceV1,
        _: <cursor_shape::wp_cursor_shape_device_v1::WpCursorShapeDeviceV1 as Proxy>::Event,
        _: &GlobalData,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl
    client::Dispatch<
        znet_dwl::znet_tapesoftware_dwl_wm_monitor_v1::ZnetTapesoftwareDwlWmMonitorV1,
        smithay_client_toolkit::globals::GlobalData,
    > for SimpleLayer
{
    fn event(
        state: &mut Self,
        proxy: &znet_dwl::znet_tapesoftware_dwl_wm_monitor_v1::ZnetTapesoftwareDwlWmMonitorV1,
        event: znet_dwl::znet_tapesoftware_dwl_wm_monitor_v1::Event,
        _data: &smithay_client_toolkit::globals::GlobalData,
        _conn: &client::Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        match event {
            znet_dwl::znet_tapesoftware_dwl_wm_monitor_v1::Event::Selected { selected } => {
                if let Some(output) = state
                    .znet_map
                    .get(&proxy.id())
                    .and_then(|id| state.monitors.get_mut(id))
                {
                    output.selected = selected == 1;
                    match Arc::make_mut(&mut output.status_bar_primitives) {
                        Primitive::Group { primitives } => {
                            for primitive in primitives.iter_mut() {
                                match primitive {
                                    Primitive::Text { color, .. } => {
                                        if output.selected {
                                            *color = state.bar_settings.color_active.0;
                                        } else {
                                            *color = state.bar_settings.color_inactive.0;
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                    state.write_bar(qhandle);
                }
            }
            znet_dwl::znet_tapesoftware_dwl_wm_monitor_v1::Event::Tag {
                tag,
                state: tag_state,
                num_clients,
                focused_client,
            } => {
                if let Some(output) = state
                    .znet_map
                    .get(&proxy.id())
                    .and_then(|id| state.monitors.get_mut(id))
                {
                    if let Ok(tag_state) = tag_state.into_result() {
                        output.tags.tag_event(
                            tag,
                            tag_state,
                            num_clients,
                            focused_client,
                            state.bar_settings.color_inactive,
                            state.bar_settings.color_active,
                            state.bar_size.height,
                            state.bar_settings.padding_x,
                        );
                    }
                }
            }
            znet_dwl::znet_tapesoftware_dwl_wm_monitor_v1::Event::Layout { layout } => {
                if let Some(output) = state
                    .znet_map
                    .get(&proxy.id())
                    .and_then(|id| state.monitors.get_mut(id))
                {
                    output.layout = layout as usize;
                }
            }
            znet_dwl::znet_tapesoftware_dwl_wm_monitor_v1::Event::Title { title } => {
                if let Some(output) = state
                    .znet_map
                    .get(&proxy.id())
                    .and_then(|id| state.monitors.get_mut(id))
                {
                    output.window_title = title;
                }
            }
            znet_dwl::znet_tapesoftware_dwl_wm_monitor_v1::Event::Frame => {
                if let Some(monitor) = state
                    .znet_map
                    .get(&proxy.id())
                    .and_then(|id| state.monitors.get_mut(id))
                {
                    if !monitor.output.frame_req {
                        monitor
                            .output
                            .layer_surface
                            .wl_surface()
                            .frame(qhandle, monitor.output.layer_surface.wl_surface().clone());
                        monitor.output.frame_req = true;
                    }
                    monitor.output.layer_surface.commit();
                }
            }
        }
    }
}

impl ProvidesRegistryState for SimpleLayer {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }

    registry_handlers![OutputState, SeatState];
}

#[derive(Debug)]
struct Buffers {
    buffers: [Buffer; 2],
    current: usize,
}

impl Buffers {
    fn new(pool: &mut SlotPool, width: u32, height: u32, format: wl_shm::Format) -> Buffers {
        Self {
            buffers: [
                pool.create_buffer(width as i32, height as i32, width as i32 * 4, format)
                    .expect("create buffer")
                    .0,
                pool.create_buffer(width as i32, height as i32, width as i32 * 4, format)
                    .expect("create buffer")
                    .0,
            ],
            current: 0,
        }
    }

    fn flip(&mut self) {
        self.current = 1 - self.current
    }

    fn buffer(&self) -> &Buffer {
        &self.buffers[self.current]
    }

    fn canvas<'a>(&'a self, pool: &'a mut SlotPool) -> Option<&mut [u8]> {
        self.buffers[self.current].canvas(pool)
    }
}

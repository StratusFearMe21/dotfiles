use std::cell::RefCell;
use std::collections::HashMap;
use std::io::BufWriter;
use std::io::Write;
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::os::fd::AsRawFd;
use std::os::fd::BorrowedFd;
use std::os::unix::net::UnixListener;
use std::os::unix::net::UnixStream;
use std::ptr::NonNull;
use std::sync::Arc;
use std::time::Duration;
use std::{ffi::CString, rc::Rc};

use color::DefaultColorParser;
use components::{
    battery::BatteryBlock,
    brightness::BrightnessBlock,
    connman::ConnmanBlock,
    playback::PlaybackBlock,
    time::{TimeBlock, NTP_SERVERS},
};

use calloop::generic::Generic;
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
use cssparser::{Parser, ParserInput};
use dbus::message::MatchRule;
use dconf_sys::dconf_client_new;
use dconf_sys::dconf_client_read;
use dconf_sys::DConfClient;
use freedesktop_desktop_entry::default_paths;
use freedesktop_desktop_entry::DesktopEntry;
use glib::FromVariant;
use iced_tiny_skia::core::font::Family;
use iced_tiny_skia::core::Background;
use iced_tiny_skia::{
    core::{
        alignment::{Horizontal, Vertical},
        text::{LineHeight, Shaping},
        Color, Font, Rectangle, Size,
    },
    graphics::{backend::Text, Viewport},
    Primitive,
};
use memchr::memchr;
use nucleo_matcher::pattern::Pattern;
use palette::IntoColor;
use rusqlite::OpenFlags;
use smithay_client_toolkit::delegate_keyboard;
use smithay_client_toolkit::globals::GlobalData;
use smithay_client_toolkit::reexports::calloop::timer::TimeoutAction;
use smithay_client_toolkit::reexports::calloop::timer::Timer;
use smithay_client_toolkit::reexports::calloop::Mode;
use smithay_client_toolkit::reexports::calloop::RegistrationToken;
use smithay_client_toolkit::reexports::client::backend::ObjectId;
use smithay_client_toolkit::reexports::client::protocol::wl_keyboard;
use smithay_client_toolkit::reexports::protocols::wp::cursor_shape::v1::client::wp_cursor_shape_device_v1;
use smithay_client_toolkit::reexports::protocols::wp::fractional_scale::v1::client::wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1;
use smithay_client_toolkit::reexports::protocols::wp::fractional_scale::v1::client::wp_fractional_scale_v1;
use smithay_client_toolkit::reexports::protocols::wp::fractional_scale::v1::client::wp_fractional_scale_v1::WpFractionalScaleV1;
use smithay_client_toolkit::reexports::protocols::wp::viewporter::client::wp_viewport::WpViewport;
use smithay_client_toolkit::reexports::protocols::wp::viewporter::client::wp_viewporter::WpViewporter;
use smithay_client_toolkit::seat::keyboard::KeyEvent;
use smithay_client_toolkit::seat::keyboard::KeyboardHandler;
use smithay_client_toolkit::seat::keyboard::Keysym;
use smithay_client_toolkit::seat::keyboard::Modifiers;
use smithay_client_toolkit::seat::pointer::cursor_shape::CursorShapeManager;
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
use yoke::Yokeable;
use znet_dwl::znet_tapesoftware_dwl_wm_monitor_v1::ZnetTapesoftwareDwlWmMonitorV1;
use znet_dwl::znet_tapesoftware_dwl_wm_v1::WobCommand;
use znet_dwl::znet_tapesoftware_dwl_wm_v1::ZnetTapesoftwareDwlWmV1;

mod connman;
mod dconf;
mod logind;
mod mpris;
mod upower;

mod components;
mod tags;

include!(concat!(env!("OUT_DIR"), "/kinds.rs"));

#[allow(non_camel_case_types)]
pub mod znet_dwl {
    use smithay_client_toolkit::reexports::client as wayland_client;
    use wayland_client::protocol::*;

    #[allow(non_upper_case_globals)]
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

struct DesktopCommand {
    name: String,
    command: String,
}

impl AsRef<str> for DesktopCommand {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

#[derive(Yokeable)]
pub struct Commands<'a>(&'a [DesktopCommand], Vec<(&'a DesktopCommand, u32)>);

enum BarState {
    Normal,
    ProgressBar {
        percentage: f32,
        icon: char,
    },
    AppLauncher {
        apps: yoke::Yoke<Commands<'static>, Vec<DesktopCommand>>,
        layout: Vec<Primitive>,
        current_input: String,
        default: String,
        selected: usize,
        prompt: &'static str,
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
            let sys_handle: LoopHandle<'static, SimpleLayer> = std::mem::transmute(handle.clone());

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
                    dconf_read_variant(dconf, "/dotfiles/somebar/update-time-ntp").unwrap_or(true),
                    dconf_read_variant(dconf, "/dotfiles/somebar/time-servers")
                        .unwrap_or(NTP_SERVERS.into_iter().map(|s| s.to_string()).collect()),
                    dconf_read_variant(dconf, "/dotfiles/somebar/time-fmt")
                        .unwrap_or("%I:%M".to_owned()),
                    dconf_read_variant(dconf, "/dotfiles/somebar/date-fmt")
                        .unwrap_or("%m/%d/%y %A".to_owned()),
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

            system_connection
                .add_match(
                    MatchRule::new_signal("org.freedesktop.login1.Manager", "PrepareForShutdown"),
                    |_: (), _, _| true,
                )
                .unwrap();

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
                            match shared_data
                                .settings_parser
                                .parse(new_prop, None)
                                .as_ref()
                                .and_then(|t| t.root_node().child(0))
                                .map(|n| NodeKind::from(n.kind_id()))
                            {
                                Some(NodeKind::Font) => {
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
                                            &shared_data.bar_settings.divider,
                                            shared_data.iced.default_size(),
                                            LineHeight::Relative(1.0),
                                            shared_data.iced.default_font(),
                                            Size {
                                                width: f32::INFINITY,
                                                height: f32::INFINITY,
                                            },
                                            Shaping::Basic,
                                        )
                                        .width;

                                    shared_data.relayout(Rc::clone(&qh));
                                }
                                Some(NodeKind::TimeBlock) => {
                                    if dconf_read_variant(
                                        shared_data.dconf,
                                        "/dotfiles/somebar/time-block",
                                    )
                                    .unwrap_or(true)
                                    {
                                        shared_data.shared_data.time = Some(TimeBlock::new(
                                            &loop_handle,
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
                                            dconf_read_variant(dconf, "/dotfiles/somebar/time-fmt")
                                                .unwrap_or("%I:%M".to_owned()),
                                            dconf_read_variant(dconf, "/dotfiles/somebar/date-fmt")
                                                .unwrap_or("%m/%d/%y %A".to_owned()),
                                            Rc::clone(&qh),
                                        ));
                                    } else {
                                        if let Some(time) = shared_data.shared_data.time.take() {
                                            time.unregister(&loop_handle);
                                        }
                                    }
                                    shared_data.write_bar(&qh);
                                }
                                Some(NodeKind::Divider) => {
                                    shared_data.bar_settings.divider = dconf_read_variant(
                                        shared_data.dconf,
                                        "/dotfiles/somebar/divider",
                                    )
                                    .unwrap_or_else(|| "".to_owned());
                                    shared_data.write_bar(&qh);
                                }
                                Some(NodeKind::DividerHard) => {
                                    shared_data.bar_settings.divider_hard = dconf_read_variant(
                                        shared_data.dconf,
                                        "/dotfiles/somebar/divider-hard",
                                    )
                                    .unwrap_or_else(|| "".to_owned());
                                    shared_data.write_bar(&qh);
                                }
                                Some(NodeKind::DateFmt) => {
                                    if let Some(ref mut time) = shared_data.shared_data.time {
                                        time.date_fmt = dconf_read_variant(
                                            shared_data.dconf,
                                            "/dotfiles/somebar/date-fmt",
                                        )
                                        .unwrap_or_else(|| "%m/%d/%y %A".to_owned());
                                        shared_data.write_bar(&qh);
                                    }
                                }
                                Some(NodeKind::BrowserPath) => {
                                    shared_data.bar_settings.browser_path = dconf_read_variant(
                                        shared_data.dconf,
                                        "/dotfiles/somebar/browser-path",
                                    )
                                    .unwrap_or_else(|| ".firedragon".to_owned());
                                }
                                Some(NodeKind::Browser) => {
                                    shared_data.bar_settings.browser = format!(
                                        "{} ",
                                        dconf_read_variant(
                                            shared_data.dconf,
                                            "/dotfiles/somebar/browser",
                                        )
                                        .unwrap_or_else(|| "firedragon".to_owned())
                                    );
                                }
                                Some(NodeKind::TimeFmt) => {
                                    if let Some(ref mut time) = shared_data.shared_data.time {
                                        time.time_fmt = dconf_read_variant(
                                            shared_data.dconf,
                                            "/dotfiles/somebar/time-fmt",
                                        )
                                        .unwrap_or_else(|| "%I:%M".to_owned());
                                        shared_data.write_bar(&qh);
                                    }
                                }
                                Some(NodeKind::UpdateTimeNtp) => {
                                    if let Some(ref mut time) = shared_data.shared_data.time {
                                        time.update_time_ntp = dconf_read_variant(
                                            shared_data.dconf,
                                            "/dotfiles/somebar/update-time-ntp",
                                        )
                                        .unwrap_or(true);
                                        shared_data.write_bar(&qh);
                                    }
                                }
                                Some(NodeKind::BrightnessBlock) => {
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
                                Some(NodeKind::BatteryBlock) => {
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
                                Some(NodeKind::ConnmanBlock) => {
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
                                Some(NodeKind::MediaBlock) => {
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
                                Some(NodeKind::ColorActive) => {
                                    shared_data
                                        .bar_settings
                                        .update_color_active(shared_data.dconf);
                                    shared_data.relayout(Rc::clone(&qh));
                                }
                                Some(NodeKind::ColorInactive) => {
                                    shared_data
                                        .bar_settings
                                        .update_color_inactive(shared_data.dconf);
                                    shared_data.relayout(Rc::clone(&qh));
                                }
                                Some(NodeKind::PaddingX) => {
                                    shared_data.bar_settings.padding_x = dconf_read_variant::<f64>(
                                        shared_data.dconf,
                                        "/dotfiles/somebar/padding-x",
                                    )
                                    .unwrap_or(10.0)
                                        as f32;

                                    shared_data.relayout(Rc::clone(&qh));
                                }
                                Some(NodeKind::PaddingY) => {
                                    shared_data.bar_settings.padding_y = dconf_read_variant::<f64>(
                                        shared_data.dconf,
                                        "/dotfiles/somebar/padding-y",
                                    )
                                    .unwrap_or(3.0)
                                        as f32;

                                    shared_data.relayout(Rc::clone(&qh));
                                }
                                Some(NodeKind::TopBar) => {
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
                                Some(NodeKind::TimeServers) => {
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
                                Some(NodeKind::BarShowTime) => {
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
                    } else if &*member == "PrepareForShutdown" {
                        let prepare: logind::OrgFreedesktopLogin1ManagerPrepareForShutdown =
                            event.read_all().unwrap();

                        if !prepare.start {
                            if let Some(ref mut time) = shared_data.shared_data.time {
                                time.unregister(&sys_handle);
                                *time = TimeBlock::new(
                                    &sys_handle,
                                    dconf_read_variant(dconf, "/dotfiles/somebar/update-time-ntp")
                                        .unwrap_or(true),
                                    dconf_read_variant(dconf, "/dotfiles/somebar/time-servers")
                                        .unwrap_or(
                                            NTP_SERVERS
                                                .into_iter()
                                                .map(|s| s.to_string())
                                                .collect(),
                                        ),
                                    dconf_read_variant(dconf, "/dotfiles/somebar/time-fmt")
                                        .unwrap_or("%I:%M".to_owned()),
                                    dconf_read_variant(dconf, "/dotfiles/somebar/date-fmt")
                                        .unwrap_or("%m/%d/%y %A".to_owned()),
                                    Rc::clone(&sys_qh),
                                );
                            }
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

            // handle
            //     .insert_source(
            //         Signals::new(&[Signal::SIGINT, Signal::SIGTERM]).unwrap(),
            //         move |signal, _, data| match signal.signal() {
            //             Signal::SIGINT | Signal::SIGTERM => {
            //                 std::fs::remove_file(&socket_file).unwrap();
            //                 data.exit.stop();
            //             }
            //             _ => unreachable!(),
            //         },
            //     )
            //     .unwrap();

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
        divider: &str,
    ) -> (Primitive, Size<f32>) {
        let divider_measurement = backend.measure(
            divider,
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
                .width;
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
            x -= divider_measurement.width;
            primitives.push(Primitive::Text {
                content: divider.to_owned(),
                bounds: Rectangle {
                    x,
                    y: logical_size.height / 2.0,
                    width: logical_size.width,
                    height: logical_size.height,
                },
                color,
                size: backend.default_size() + padding_y * 2.0,
                line_height: LineHeight::Relative(1.0),
                font: backend.default_font(),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Center,
                shaping: Shaping::Basic,
            });
            media.x_at = x;
            media.width = measurement + divider_measurement.width + padding_x;
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
                .width;
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
            x -= divider_measurement.width;
            primitives.push(Primitive::Text {
                content: divider.to_owned(),
                bounds: Rectangle {
                    x,
                    y: logical_size.height / 2.0,
                    width: logical_size.width,
                    height: logical_size.height,
                },
                color,
                size: backend.default_size() + padding_y * 2.0,
                line_height: LineHeight::Relative(1.0),
                font: backend.default_font(),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Center,
                shaping: Shaping::Basic,
            });

            connman.x_at = x;
            connman.width = measurement + divider_measurement.width;
        }

        if let Some(ref mut bat_block) = self.bat_block {
            bat_block.xs_at.clear();
            bat_block.widths.clear();
            for content in
                unsafe { std::mem::transmute::<&BatteryBlock, &'static BatteryBlock>(&bat_block) }
                    .fmt()
            {
                let current_measurement = backend
                    .measure(
                        &content,
                        backend.default_size(),
                        LineHeight::Relative(1.0),
                        backend.default_font(),
                        Size::INFINITY,
                        Shaping::Basic,
                    )
                    .width;

                x -= current_measurement;
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
                x -= divider_measurement.width;
                primitives.push(Primitive::Text {
                    content: divider.to_owned(),
                    bounds: Rectangle {
                        x,
                        y: logical_size.height / 2.0,
                        width: logical_size.width,
                        height: logical_size.height,
                    },
                    color,
                    size: backend.default_size() + padding_y * 2.0,
                    line_height: LineHeight::Relative(1.0),
                    font: backend.default_font(),
                    horizontal_alignment: Horizontal::Left,
                    vertical_alignment: Vertical::Center,
                    shaping: Shaping::Basic,
                });

                bat_block.xs_at.push(x);
                bat_block
                    .widths
                    .push(current_measurement + divider_measurement.width);
            }
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
                .width;
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
            x -= divider_measurement.width;
            primitives.push(Primitive::Text {
                content: divider.to_owned(),
                bounds: Rectangle {
                    x,
                    y: logical_size.height / 2.0,
                    width: logical_size.width,
                    height: logical_size.height,
                },
                color,
                size: backend.default_size() + padding_y * 2.0,
                line_height: LineHeight::Relative(1.0),
                font: backend.default_font(),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Center,
                shaping: Shaping::Basic,
            });

            brightness.x_at = x;
            brightness.width = measurement + divider_measurement.width;
        }

        let mut height = 0.0;

        if let Some(ref mut time) = self.time {
            {
                let mut content = String::new();
                time.fmt_date(&mut content);
                let measurement = backend.measure(
                    &content,
                    backend.default_size(),
                    LineHeight::Relative(1.0),
                    backend.default_font(),
                    Size::INFINITY,
                    Shaping::Basic,
                );
                height = measurement.height;
                let measurement = measurement.width;
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
                x -= divider_measurement.width;
                primitives.push(Primitive::Text {
                    content: divider.to_owned(),
                    bounds: Rectangle {
                        x,
                        y: logical_size.height / 2.0,
                        width: logical_size.width,
                        height: logical_size.height,
                    },
                    color,
                    size: backend.default_size() + padding_y * 2.0,
                    line_height: LineHeight::Relative(1.0),
                    font: backend.default_font(),
                    horizontal_alignment: Horizontal::Left,
                    vertical_alignment: Vertical::Center,
                    shaping: Shaping::Basic,
                });

                time.xs_at[0] = x;
                time.widths[0] = measurement + padding_x;
            }

            {
                let mut content = String::new();
                time.fmt_time(&mut content);
                let measurement = backend.measure(
                    &content,
                    backend.default_size(),
                    LineHeight::Relative(1.0),
                    backend.default_font(),
                    Size::INFINITY,
                    Shaping::Basic,
                );
                let measurement = measurement.width;
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

                time.xs_at[1] = x;
                time.widths[1] = measurement + padding_x;
            }
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
            time.fmt_time_table(f)?;
            time.fmt_date_table(f)?;
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
    if std::env::args().nth(1).map(|a| a == "check") == Some(true) {
        freedesktop_desktop_entry::Iter::new(default_paths()).for_each(|entry| {
            if let Ok(bytes) = std::fs::read_to_string(&entry) {
                if let Ok(entry) = DesktopEntry::decode(&entry, &bytes) {
                    if let Some(exec) = entry.exec() {
                        let mut command = exec.to_owned();
                        while let Some(index) = memchr(b'%', command.as_bytes()) {
                            if index + 1 == command.len() {
                                command.pop();
                                command.pop();
                            } else {
                                command.remove(index + 1);
                                command.remove(index);
                            }
                        }
                        println!("{}", entry.path.display());
                        let _ = std::process::Command::new("sh")
                            .args([
                                "-c",
                                &format!("which {}", command.split(' ').next().unwrap()),
                            ])
                            .status();
                    }
                }
            }
        });
        return;
    }

    let conn = Connection::connect_to_env().unwrap();

    let (globals, event_queue) = registry_queue_init(&conn).unwrap();
    let qh = Rc::new(event_queue.handle());

    let compositor = CompositorState::bind(&globals, &qh).unwrap();

    let layer_shell = LayerShell::bind(&globals, &qh).unwrap();

    let dwl: ZnetTapesoftwareDwlWmV1 = globals.bind(&qh, 1..=1, GlobalData).unwrap();
    let cursor_shape_manager: CursorShapeManager = CursorShapeManager::bind(&globals, &qh).unwrap();

    let fractional_scale: WpFractionalScaleManagerV1 =
        globals.bind(&qh, 1..=1, GlobalData).unwrap();

    let viewporter: WpViewporter = globals.bind(&qh, 1..=1, GlobalData).unwrap();

    let shm = Shm::bind(&globals, &qh).unwrap();

    let dconf = unsafe { dconf_client_new() };

    let new_font: String = dconf_read_variant(dconf, "/dotfiles/somebar/font")
        .unwrap_or(String::from("FiraCode Nerd Font 14"));
    let divider: String =
        dconf_read_variant(dconf, "/dotfiles/somebar/divider").unwrap_or(String::from(""));
    let divider_hard: String =
        dconf_read_variant(dconf, "/dotfiles/somebar/divider-hard").unwrap_or(String::from(""));

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
        &divider,
        backend.default_size(),
        LineHeight::Relative(1.0),
        backend.default_font(),
        Size {
            width: f32::INFINITY,
            height: f32::INFINITY,
        },
        Shaping::Basic,
    );

    let bar_settings = BarSettings::new(new_font, dconf, divider, divider_hard);

    let bar_size = Size {
        width: 0.0,
        height: measured_text.height + bar_settings.padding_y * 2.0,
    };

    let pool = SlotPool::new(1920 * bar_size.height as usize * 4, &shm).unwrap();

    let guard = event_queue.prepare_read().unwrap();
    let fd = Generic::new(
        unsafe { BorrowedFd::borrow_raw(guard.connection_fd().as_raw_fd()) },
        Interest::READ,
        Mode::Level,
    );
    drop(guard);

    let mut event_loop = EventLoop::try_new().unwrap();
    let handle = event_loop.handle();

    let shared_data = SharedData::new(&handle, Rc::clone(&qh), dconf);

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
        fractional_scale,
        viewporter,
    );

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
    viewporter_vp: WpViewport,
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
    browser_path: String,
    browser: String,
    divider: String,
    divider_hard: String,
    top_bar: bool,
}

fn parse_color(
    color_input: &mut [palette::Srgba; 2],
    dconf_path: &str,
    dconf_client: *mut DConfClient,
) {
    if let Some((color_one, color_two)) =
        dconf_read_variant::<(String, String)>(dconf_client, dconf_path)
    {
        if let Ok((_, color)) = color::parse_color_with(
            &mut DefaultColorParser::new(Some(&mut color::Color::LinSrgb(
                color_input[0].into_linear(),
            ))),
            &mut Parser::new(&mut ParserInput::new(&color_one)),
        ) {
            let color: palette::LinSrgba = color.into_color();
            color_input[0] = palette::Srgba::from_linear(color);
        }

        if let Ok((_, color)) = color::parse_color_with(
            &mut DefaultColorParser::new(Some(&mut color::Color::LinSrgb(
                color_input[1].into_linear(),
            ))),
            &mut Parser::new(&mut ParserInput::new(&color_two)),
        ) {
            let color: palette::LinSrgba = color.into_color();
            color_input[1] = palette::Srgba::from_linear(color);
        }
    }
}

impl BarSettings {
    fn new(
        default_font: String,
        dconf: *mut DConfClient,
        divider: String,
        divider_hard: String,
    ) -> BarSettings {
        let mut color_active: [palette::Srgba; 2] = [
            palette::Srgba::from_components((1.0, 0.56, 0.25, 1.0)),
            palette::Srgba::from_components((0.2, 0.227, 0.25, 1.0)),
        ];

        parse_color(&mut color_active, "/dotfiles/somebar/color-active", dconf);

        let mut color_inactive: [palette::Srgba; 2] = [
            palette::Srgba::from_components((0.701, 0.694, 0.678, 1.0)),
            palette::Srgba::from_components((0.039, 0.054, 0.078, 1.0)),
        ];

        parse_color(
            &mut color_inactive,
            "/dotfiles/somebar/color-inactive",
            dconf,
        );

        BarSettings {
            color_active: (
                Color::from_rgba(
                    color_active[0].red,
                    color_active[0].green,
                    color_active[0].blue,
                    color_active[0].alpha,
                ),
                Color::from_rgba(
                    color_active[1].red,
                    color_active[1].green,
                    color_active[1].blue,
                    color_active[1].alpha,
                ),
            ),
            color_inactive: (
                Color::from_rgba(
                    color_inactive[0].red,
                    color_inactive[0].green,
                    color_inactive[0].blue,
                    color_inactive[0].alpha,
                ),
                Color::from_rgba(
                    color_inactive[1].red,
                    color_inactive[1].green,
                    color_inactive[1].blue,
                    color_inactive[1].alpha,
                ),
            ),
            default_font,
            padding_x: dconf_read_variant::<f64>(dconf, "/dotfiles/somebar/padding-x")
                .unwrap_or(10.0) as f32,
            padding_y: dconf_read_variant::<f64>(dconf, "/dotfiles/somebar/padding-y")
                .unwrap_or(3.0) as f32,
            bar_show_time: dconf_read_variant(dconf, "/dotfiles/somebar/bar-show-time")
                .unwrap_or(500),
            top_bar: dconf_read_variant(dconf, "/dotfiles/somebar/top-bar").unwrap_or(true),
            browser_path: dconf_read_variant(dconf, "/dotfiles/somebar/browser-path")
                .unwrap_or_else(|| ".firedragon".to_owned()),
            browser: format!(
                "{} ",
                dconf_read_variant(dconf, "/dotfiles/somebar/browser")
                    .unwrap_or_else(|| "firedragon".to_owned())
            ),
            divider,
            divider_hard,
        }
    }

    fn update_color_active(&mut self, dconf: *mut DConfClient) {
        let mut color_active = [
            palette::Srgba::from_components((1.0, 0.56, 0.25, 1.0)),
            palette::Srgba::from_components((0.2, 0.227, 0.25, 1.0)),
        ];

        parse_color(&mut color_active, "/dotfiles/somebar/color-active", dconf);

        self.color_active = (
            Color::from_rgba(
                color_active[0].red,
                color_active[0].green,
                color_active[0].blue,
                color_active[0].alpha,
            ),
            Color::from_rgba(
                color_active[1].red,
                color_active[1].green,
                color_active[1].blue,
                color_active[1].alpha,
            ),
        );
    }

    fn update_color_inactive(&mut self, dconf: *mut DConfClient) {
        let mut color_inactive = [
            palette::Srgba::from_components((0.701, 0.694, 0.678, 1.0)),
            palette::Srgba::from_components((0.039, 0.054, 0.078, 1.0)),
        ];

        parse_color(
            &mut color_inactive,
            "/dotfiles/somebar/color-inactive",
            dconf,
        );

        self.color_inactive = (
            Color::from_rgba(
                color_inactive[0].red,
                color_inactive[0].green,
                color_inactive[0].blue,
                color_inactive[0].alpha,
            ),
            Color::from_rgba(
                color_inactive[1].red,
                color_inactive[1].green,
                color_inactive[1].blue,
                color_inactive[1].alpha,
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
    fractional_scaling: WpFractionalScaleManagerV1,
    viewporter: WpViewporter,
    cursor_shape_manager: CursorShapeManager,
    monitors: HashMap<ObjectId, Monitor>,
    output_map: HashMap<ObjectId, ObjectId>,
    znet_map: HashMap<ObjectId, ObjectId>,
    fractional_map: HashMap<ObjectId, ObjectId>,
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
    matcher: nucleo_matcher::Matcher,
    settings_parser: tree_sitter::Parser,
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
        cursor_shape_manager: CursorShapeManager,
        dconf: *mut DConfClient,
        layer_shell: LayerShell,
        compositor_state: CompositorState,
        bar_settings: BarSettings,
        loop_handle: LoopHandle<'static, SimpleLayer>,
        fractional_scaling: WpFractionalScaleManagerV1,
        viewporter: WpViewporter,
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
                    &bar_settings.divider,
                    iced.default_size(),
                    LineHeight::Relative(1.0),
                    iced.default_font(),
                    Size {
                        width: f32::INFINITY,
                        height: f32::INFINITY,
                    },
                    Shaping::Basic,
                )
                .width,
            iced,
            tag_count: 9,
            bar_settings,
            keyboard: None,
            pointer: None,
            layouts: Vec::new(),
            monitors: HashMap::new(),
            output_map: HashMap::new(),
            znet_map: HashMap::new(),
            fractional_map: HashMap::new(),
            output_type_map: HashMap::new(),
            matcher: nucleo_matcher::Matcher::new({
                let mut config = nucleo_matcher::Config::DEFAULT;
                config.prefer_prefix = true;
                config
            }),
            settings_parser: {
                let mut parser = tree_sitter::Parser::new();
                parser
                    .set_language(tree_sitter_dconfsomebar::language())
                    .unwrap();
                parser.set_timeout_micros(500_000);
                parser
            },
            fractional_scaling,
            viewporter,
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
                &self.bar_settings.divider,
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
                selected,
                prompt,
                ..
            } => {
                layout.clear();
                let logical_height = monitor.output.viewport.logical_size().height;
                let input_string = format!("{}: {}", prompt, current_input);

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
                    .width;

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
                let query = Pattern::parse(
                    &current_input,
                    nucleo_matcher::pattern::CaseMatching::Ignore,
                );

                #[inline(always)]
                fn map_project_query(
                    cart: yoke::Yoke<Commands<'static>, Vec<DesktopCommand>>,
                    matcher: &mut nucleo_matcher::Matcher,
                    query: Pattern,
                ) -> yoke::Yoke<Commands<'static>, Vec<DesktopCommand>> {
                    cart.map_project(|cart, _| {
                        Commands(cart.0, query.match_list(cart.0.iter(), matcher))
                    })
                }

                let mut apps_old: yoke::Yoke<Commands<'static>, Vec<DesktopCommand>> =
                    yoke::Yoke::attach_to_cart(Vec::new(), |cart| Commands(cart, Vec::new()));
                core::mem::swap(&mut apps_old, apps);
                *apps = map_project_query(apps_old, &mut self.matcher, query);
                let apps = apps.get();
                for (index, (item, _)) in (&apps.1[..15.min(apps.1.len())]).into_iter().enumerate()
                {
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
                        .width;
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
            _ => unreachable!(),
        }
    }
}

impl CompositorHandler for SimpleLayer {
    fn transform_changed(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_surface::WlSurface,
        _: wl_output::Transform,
    ) {
    }

    fn scale_factor_changed(
        &mut self,
        _conn: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_surface::WlSurface,
        _: i32,
    ) {
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
        let fractional_scaler = self
            .fractional_scaling
            .get_fractional_scale(&surface, qh, GlobalData);
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
        self.fractional_map
            .insert(fractional_scaler.id(), layer_id.clone());

        let monitor = self.dwl.get_monitor(&output, &qh, GlobalData);

        self.znet_map.insert(monitor.id(), layer_id.clone());
        let mut new_output = Monitor {
            output: Output {
                viewporter_vp: self
                    .viewporter
                    .get_viewport(layer.wl_surface(), qh, GlobalData),
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
                // fractional_scaler,
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
            &self.bar_settings.divider,
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
                    width: (configure.new_size.0 as f64 * output.viewport.scale_factor()) as u32,
                    height: (configure.new_size.1 as f64 * output.viewport.scale_factor()) as u32,
                },
                output.viewport.scale_factor(),
            );
            output.mask = Mask::new(
                output.viewport.physical_width(),
                output.viewport.physical_height(),
            )
            .unwrap();
            output.viewporter_vp.set_destination(
                output.viewport.logical_size().width as _,
                output.viewport.logical_size().height as _,
            );
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
        _: &[Keysym],
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
        match event.keysym {
            Keysym::Escape => {
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
            Keysym::BackSpace => match &mut monitor.bar_state {
                BarState::AppLauncher { current_input, .. } => {
                    current_input.pop();
                    monitor.output.frame(qh);
                    self.layout_applauncher();
                }
                _ => {}
            },
            Keysym::Down | Keysym::Right => match &mut monitor.bar_state {
                BarState::AppLauncher { selected, .. } => {
                    selected.add_assign(1);
                    monitor.output.frame(qh);
                    self.layout_applauncher();
                }
                _ => {}
            },
            Keysym::Up | Keysym::Left => match &mut monitor.bar_state {
                BarState::AppLauncher { selected, .. } => {
                    selected.sub_assign(1);
                    monitor.output.frame(qh);
                    self.layout_applauncher();
                }
                _ => {}
            },
            Keysym::Return => match &mut monitor.bar_state {
                BarState::AppLauncher {
                    apps,
                    selected,
                    default,
                    current_input,
                    ..
                } => {
                    if let Some((app, _)) = apps.get().1.get(*selected) {
                        std::process::Command::new("sh")
                            .args(["-c", &app.command])
                            .spawn()
                            .unwrap();
                    } else {
                        let _ = std::process::Command::new("sh")
                            .args(["-c", &format!("{}{}", default, current_input)])
                            .spawn();
                    }

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
                        let cursor_device = self.cursor_shape_manager.get_shape_device(pointer, qh);

                        cursor_device.set_shape(serial, wp_cursor_shape_device_v1::Shape::Default);
                    }
                    Leave { .. } => {
                        monitor.status_bar_bg = Arc::new(Primitive::Group {
                            primitives: Vec::new(),
                        });
                        monitor.output.frame(qh);
                        monitor.info_output.take();
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
                        let mut block = 0;
                        if let Some(ref media) = self.shared_data.playback {
                            if event.position.0 >= media.x_at as f64 {
                                if self.shared_data.selected != Some(block) {
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
                                                width: (256.0
                                                    + (self.bar_settings.padding_y as f64 * 2.0)
                                                        * monitor.output.viewport.scale_factor())
                                                    as u32,
                                                height: (512.0
                                                    + (self.bar_settings.padding_y as f64 * 2.0)
                                                        * monitor.output.viewport.scale_factor())
                                                    as u32,
                                            },
                                            monitor.output.viewport.scale_factor(),
                                        );
                                        let fractional_scaler =
                                            self.fractional_scaling.get_fractional_scale(
                                                info_layer.wl_surface(),
                                                qh,
                                                GlobalData,
                                            );
                                        self.fractional_map.insert(
                                            fractional_scaler.id(),
                                            info_layer.wl_surface().id(),
                                        );
                                        monitor.info_output = Some(Output {
                                            // fractional_scaler,
                                            viewporter_vp: self.viewporter.get_viewport(
                                                info_layer.wl_surface(),
                                                qh,
                                                GlobalData,
                                            ),
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
                                    self.select_block(Some(block), event.surface.id());
                                }
                                return;
                            }
                        }

                        if let Some(ref connman) = self.shared_data.connman {
                            block += 2;
                            if event.position.0 >= connman.x_at as f64 {
                                if self.shared_data.selected != Some(block) {
                                    status_bar_bg!(connman.x_at, connman.width, self, monitor, qh);
                                    monitor.output.frame(qh);
                                    self.select_block(Some(block), event.surface.id());
                                }
                                return;
                            }
                        }

                        if let Some(ref bat_block) = self.shared_data.bat_block {
                            for (x_at, width) in bat_block
                                .xs_at
                                .iter()
                                .copied()
                                .zip(bat_block.widths.iter().copied())
                            {
                                block += 2;
                                if event.position.0 >= x_at as f64 {
                                    if self.shared_data.selected != Some(block) {
                                        status_bar_bg!(x_at, width, self, monitor, qh);
                                        monitor.output.frame(qh);
                                        self.select_block(Some(block), event.surface.id());
                                    }
                                    return;
                                }
                            }
                        }

                        if let Some(ref brightness) = self.shared_data.brightness {
                            block += 2;
                            if event.position.0 >= brightness.x_at as f64 {
                                if self.shared_data.selected != Some(block) {
                                    status_bar_bg!(
                                        brightness.x_at,
                                        brightness.width,
                                        self,
                                        monitor,
                                        qh
                                    );
                                    monitor.output.frame(qh);
                                    self.select_block(Some(block), event.surface.id());
                                }
                                return;
                            }
                        }

                        if let Some(ref time) = self.shared_data.time {
                            for i in 0..2 {
                                block += 2;
                                if event.position.0 >= time.xs_at[i] as f64 {
                                    if self.shared_data.selected != Some(block) {
                                        status_bar_bg!(
                                            time.xs_at[i],
                                            time.widths[i],
                                            self,
                                            monitor,
                                            qh
                                        );
                                        monitor.output.frame(qh);
                                        self.select_block(Some(block), event.surface.id());
                                    }
                                    return;
                                }
                            }
                        }

                        monitor.status_bar_bg = Arc::new(Primitive::Group {
                            primitives: Vec::new(),
                        });
                        monitor.info_output.take();
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
            monitor.tags.relayout_windows(
                self.bar_settings.color_active.0,
                self.bar_settings.color_inactive.0,
                self.bar_settings.padding_x,
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
                            change_text_content!(
                                b - 1,
                                self.bar_settings.divider.clone(),
                                primitives
                            );
                            change_text_content!(
                                b + 1,
                                self.bar_settings.divider.clone(),
                                primitives
                            );
                            // change_text_color!(b, self.bar_settings.color_active.0, primitives);
                        }
                        change_text_color!(block + 1, self.bar_settings.color_active.1, primitives);
                        change_text_color!(block, self.bar_settings.color_inactive.1, primitives);
                        // change_text_color!(block, self.bar_settings.color_inactive.1, primitives);
                        change_text_content!(
                            block - 1,
                            self.bar_settings.divider_hard.clone(),
                            primitives
                        );
                        change_text_content!(
                            block + 1,
                            self.bar_settings.divider_hard.clone(),
                            primitives
                        );
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
                            change_text_content!(
                                b - 1,
                                self.bar_settings.divider.clone(),
                                primitives
                            );
                            change_text_content!(
                                b + 1,
                                self.bar_settings.divider.clone(),
                                primitives
                            );
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
                for monitor in state.monitors.values_mut() {
                    monitor.tags.relayout(
                        state.bar_settings.padding_x,
                        state.bar_size.height,
                        &state.iced,
                        state.ascii_font_width,
                        state.tag_count,
                    );

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
                                                    });
                                                }
                                            }
                                        }
                                    }
                                    items
                                },
                            );
                            monitor.bar_state = BarState::AppLauncher {
                                apps: yoke::Yoke::attach_to_cart(apps, |cart| {
                                    Commands(cart, Vec::new())
                                }),
                                default: String::new(),
                                current_input: String::new(),
                                layout: Vec::new(),
                                selected: 0,
                                prompt: "run"
                            };
                            monitor.output.frame(qh);
                            state.layout_applauncher();
                            return;
                        }
                        WobCommand::LaunchBrowser => {
                            monitor
                                .output
                                .layer_surface
                                .set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
                            monitor.is_in_overlay = true;
                            let path = format!(
                                "echo file:$(realpath ~/{}/*.default-release/places.sqlite)?immutable=true",
                                state.bar_settings.browser_path
                            );
                            let cmd = std::process::Command::new("sh")
                                .args(["-c", &path])
                                .output()
                                .unwrap()
                                .stdout;
                            let conn =
                                rusqlite::Connection::open_with_flags(String::from_utf8_lossy(&cmd).trim(), OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI)
                                    .unwrap();
                            let mut stmt = conn
                                .prepare(
                                    "SELECT \
                                moz_bookmarks.title, \
                                url \
                                FROM moz_bookmarks \
                                INNER JOIN \
                                moz_places on \
                                moz_bookmarks.fk = moz_places.id \
                                ORDER BY frecency DESC;",
                                )
                                .unwrap();

                            let apps = stmt
                                .query_map([], |row| {
                                    Ok(DesktopCommand {
                                        name: row.get(0).unwrap(),
                                        command: format!(
                                            "{}{}",
                                            state.bar_settings.browser,
                                            row.get::<_, String>(1).unwrap()
                                        ),
                                    })
                                })
                                .unwrap()
                                .map(|d| d.unwrap())
                                .collect();

                            monitor.bar_state = BarState::AppLauncher {
                                apps: yoke::Yoke::attach_to_cart(apps, |cart| {
                                    Commands(cart, Vec::new())
                                }),
                                default: state.bar_settings.browser.clone(),
                                current_input: String::new(),
                                layout: Vec::new(),
                                selected: 0,
                                prompt: "browser"
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
                        WobCommand::PowerButton => {
                            monitor.output.frame(qh);
                            if monitor.info_output.is_none() {
                                let surface = state.compositor_state.create_surface(&qh);
                                let info_layer = state.layer_shell.create_layer_surface(
                                    &qh,
                                    surface,
                                    Layer::Overlay,
                                    None::<String>,
                                    Some(&monitor.wl_output),
                                );

                                info_layer.set_anchor(Anchor::all());
                                info_layer.set_size(
                                    512 + (state.bar_settings.padding_y as u32 * 2),
                                    256 + (state.bar_settings.padding_y as u32 * 2),
                                );
                                info_layer.set_keyboard_interactivity(KeyboardInteractivity::None);

                                info_layer.commit();
                                state.output_type_map.insert(
                                    info_layer.wl_surface().id(),
                                    OutputType::Info(
                                        monitor.output.layer_surface.wl_surface().id(),
                                    ),
                                );
                                let viewport = Viewport::with_physical_size(
                                    Size {
                                        width: (512.0 + (state.bar_settings.padding_y as f64 * 2.0)
                                            * monitor.output.viewport.scale_factor()) as u32,
                                        height: (256.0 + (state.bar_settings.padding_y as f64 * 2.0)
                                            * monitor.output.viewport.scale_factor()) as u32,
                                    },
                                    monitor.output.viewport.scale_factor(),
                                );
                                let fractional_scaler = state
                                        .fractional_scaling
                                        .get_fractional_scale(
                                            info_layer.wl_surface(),
                                            qh,
                                            GlobalData,
                                        );
                                state.fractional_map.insert(fractional_scaler.id(), info_layer.wl_surface().id());
                                monitor.info_output = Some(Output {
                                    // fractional_scaler,
                                    viewporter_vp: state.viewporter.get_viewport(info_layer.wl_surface(), qh, GlobalData),
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

impl client::Dispatch<WpFractionalScaleManagerV1, GlobalData> for SimpleLayer {
    fn event(
        _: &mut Self,
        _: &WpFractionalScaleManagerV1,
        _: <WpFractionalScaleManagerV1 as Proxy>::Event,
        _: &GlobalData,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl client::Dispatch<WpFractionalScaleV1, GlobalData> for SimpleLayer {
    fn event(
        state: &mut Self,
        proxy: &WpFractionalScaleV1,
        event: <WpFractionalScaleV1 as Proxy>::Event,
        _: &GlobalData,
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        let wp_fractional_scale_v1::Event::PreferredScale { scale } = event else {
            unreachable!()
        };
        let new_factor = scale as f32 / 120.0;
        let surface_id = state.fractional_map.get(&proxy.id()).unwrap();
        let output = match state.output_type_map.get(&surface_id) {
            Some(OutputType::Bar) => {
                if let Some(monitor) = state.monitors.get_mut(&surface_id) {
                    &mut monitor.output
                } else {
                    return;
                }
            }
            Some(OutputType::Info(id)) => {
                if let Some(monitor) = state.monitors.get_mut(id) {
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
                width: (output.viewport.logical_size().width * new_factor) as u32,
                height: (output.viewport.logical_size().height * new_factor) as u32,
            },
            new_factor as f64,
        );
        output.viewporter_vp.set_destination(
            output.viewport.logical_size().width as _,
            output.viewport.logical_size().height as _,
        );
        // // Initializes our double buffer one we've configured the layer shell
        output.buffers = Some(Buffers::new(
            &mut state.pool,
            output.viewport.physical_width(),
            output.viewport.physical_height(),
            wl_shm::Format::Argb8888,
        ));
        output.mask = Mask::new(
            output.viewport.physical_width(),
            output.viewport.physical_height(),
        )
        .unwrap();
        // output
        //     .layer_surface
        //     .set_buffer_scale(new_factor as u32)
        //     .unwrap();
        output.frame(qh);
    }
}

impl client::Dispatch<WpViewporter, GlobalData> for SimpleLayer {
    fn event(
        _: &mut Self,
        _: &WpViewporter,
        _: <WpViewporter as Proxy>::Event,
        _: &GlobalData,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl client::Dispatch<WpViewport, GlobalData> for SimpleLayer {
    fn event(
        _: &mut Self,
        _: &WpViewport,
        _: <WpViewport as Proxy>::Event,
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

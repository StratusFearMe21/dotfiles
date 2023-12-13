use nix::fcntl::{FcntlArg, OFlag};
use smithay_client_toolkit as sctk;

use std::borrow::Cow;
use std::cell::RefCell;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::mem;
use std::os::unix::io::{AsRawFd, RawFd};
use std::rc::Rc;

use sctk::data_device_manager::data_device::{DataDevice, DataDeviceHandler};
use sctk::data_device_manager::data_offer::{DataOfferError, DataOfferHandler, DragOffer};
use sctk::data_device_manager::data_source::{CopyPasteSource, DataSourceHandler};
use sctk::data_device_manager::{DataDeviceManagerState, WritePipe};
use sctk::primary_selection::device::{PrimarySelectionDevice, PrimarySelectionDeviceHandler};
use sctk::primary_selection::selection::{PrimarySelectionSource, PrimarySelectionSourceHandler};
use sctk::primary_selection::PrimarySelectionManagerState;
use sctk::seat::pointer::{PointerEvent, PointerEventKind};
use sctk::seat::Capability;

use sctk::reexports::calloop::{LoopHandle, PostAction};
use sctk::reexports::client::globals::GlobalList;
use sctk::reexports::client::protocol::wl_data_device::WlDataDevice;
use sctk::reexports::client::protocol::wl_data_device_manager::DndAction;
use sctk::reexports::client::protocol::wl_data_source::WlDataSource;
use sctk::reexports::client::protocol::wl_seat::WlSeat;
use sctk::reexports::client::{Connection, QueueHandle};
use sctk::reexports::protocols::wp::primary_selection::zv1::client::{
    zwp_primary_selection_device_v1::ZwpPrimarySelectionDeviceV1,
    zwp_primary_selection_source_v1::ZwpPrimarySelectionSourceV1,
};

use crate::push_str::PushString;
use crate::SimpleLayer;

use super::mime::{normalize_to_lf, MimeType, ALLOWED_MIME_TYPES};

pub struct State {
    pub primary_selection_manager_state: Option<PrimarySelectionManagerState>,
    pub data_device_manager_state: Option<DataDeviceManagerState>,
    pub exit: bool,

    seat: ClipboardSeatState,

    loop_handle: LoopHandle<'static, SimpleLayer>,
    queue_handle: QueueHandle<SimpleLayer>,

    primary_sources: Vec<PrimarySelectionSource>,
    primary_selection_content: Rc<[u8]>,

    data_sources: Vec<CopyPasteSource>,
    data_selection_content: Rc<[u8]>,
}

impl State {
    #[must_use]
    pub fn new(
        globals: &GlobalList,
        queue_handle: &QueueHandle<SimpleLayer>,
        loop_handle: LoopHandle<'static, SimpleLayer>,
    ) -> Option<Self> {
        let data_device_manager_state = DataDeviceManagerState::bind(globals, queue_handle).ok();
        let primary_selection_manager_state =
            PrimarySelectionManagerState::bind(globals, queue_handle).ok();

        // When both globals are not available nothing could be done.
        if data_device_manager_state.is_none() && primary_selection_manager_state.is_none() {
            return None;
        }

        Some(Self {
            seat: ClipboardSeatState::default(),
            primary_selection_content: Rc::from([]),
            data_selection_content: Rc::from([]),
            queue_handle: queue_handle.clone(),
            primary_selection_manager_state,
            primary_sources: Vec::new(),
            data_device_manager_state,
            data_sources: Vec::new(),
            loop_handle,
            exit: false,
        })
    }

    /// Store selection for the given target.
    ///
    /// Selection source is only created when `Some(())` is returned.
    pub fn store_selection(&mut self, ty: SelectionTarget, contents: String) -> Option<()> {
        if !self.seat.has_focus {
            return None;
        }

        let contents = Rc::from(contents.into_bytes());

        match ty {
            SelectionTarget::Clipboard => {
                let mgr = self.data_device_manager_state.as_ref()?;
                self.data_selection_content = contents;
                let source =
                    mgr.create_copy_paste_source(&self.queue_handle, ALLOWED_MIME_TYPES.iter());
                source.set_selection(
                    self.seat.data_device.as_ref().unwrap(),
                    self.seat.latest_serial,
                );
                self.data_sources.push(source);
            }
            SelectionTarget::Primary => {
                let mgr = self.primary_selection_manager_state.as_ref()?;
                self.primary_selection_content = contents;
                let source =
                    mgr.create_selection_source(&self.queue_handle, ALLOWED_MIME_TYPES.iter());
                source.set_selection(
                    self.seat.primary_device.as_ref().unwrap(),
                    self.seat.latest_serial,
                );
                self.primary_sources.push(source);
            }
        }

        Some(())
    }

    /// Load selection for the given target.
    pub fn load_selection(
        &mut self,
        ty: SelectionTarget,
        string: Rc<RefCell<PushString>>,
    ) -> Result<()> {
        if !self.seat.has_focus {
            return Err(Error::new(ErrorKind::Other, "client doesn't have focus"));
        }

        let (read_pipe, mime_type) = match ty {
            SelectionTarget::Clipboard => {
                let selection = self
                    .seat
                    .data_device
                    .as_ref()
                    .and_then(|data| data.data().selection_offer())
                    .ok_or_else(|| Error::new(ErrorKind::Other, "selection is empty"))?;

                let mime_type = selection
                    .with_mime_types(MimeType::find_allowed)
                    .ok_or_else(|| {
                        Error::new(ErrorKind::NotFound, "supported mime-type is not found")
                    })?;

                (
                    selection
                        .receive(mime_type.to_string())
                        .map_err(|err| match err {
                            DataOfferError::InvalidReceive => {
                                Error::new(ErrorKind::Other, "offer is not ready yet")
                            }
                            DataOfferError::Io(err) => err,
                        })?,
                    mime_type,
                )
            }
            SelectionTarget::Primary => {
                let selection = self
                    .seat
                    .primary_device
                    .as_ref()
                    .and_then(|data| data.data().selection_offer())
                    .ok_or_else(|| Error::new(ErrorKind::Other, "selection is empty"))?;

                let mime_type = selection
                    .with_mime_types(MimeType::find_allowed)
                    .ok_or_else(|| {
                        Error::new(ErrorKind::NotFound, "supported mime-type is not found")
                    })?;

                (selection.receive(mime_type.to_string())?, mime_type)
            }
        };

        // Mark FD as non-blocking so we won't block ourselves.
        unsafe {
            set_non_blocking(read_pipe.as_raw_fd())?;
        }

        let qh = self.queue_handle.clone();
        let mut reader_buffer = [0; 4096];
        let mut content = Vec::new();
        let _ = self
            .loop_handle
            .insert_source(read_pipe, move |_, file, state| {
                let file = unsafe { file.get_mut() };
                loop {
                    match file.read(&mut reader_buffer) {
                        Ok(0) => {
                            let utf8 = String::from_utf8_lossy(&content);
                            let content = match utf8 {
                                Cow::Borrowed(_) => {
                                    // Don't clone the read data.
                                    let mut to_send = Vec::new();
                                    mem::swap(&mut content, &mut to_send);
                                    String::from_utf8(to_send).unwrap()
                                }
                                Cow::Owned(content) => content,
                            };

                            // Post-process the content according to mime type.
                            let content = match mime_type {
                                MimeType::TextPlainUtf8 | MimeType::TextPlain => {
                                    normalize_to_lf(content)
                                }
                                MimeType::Utf8String => content,
                            };

                            string.borrow_mut().push_str(&content);
                            state.layout_applauncher();
                            state
                                .monitors
                                .values_mut()
                                .find(|o| o.selected)
                                .unwrap()
                                .output
                                .frame(&qh);
                            break PostAction::Remove;
                        }
                        Ok(n) => content.extend_from_slice(&reader_buffer[..n]),
                        Err(err) if err.kind() == ErrorKind::WouldBlock => {
                            break PostAction::Continue
                        }
                        err @ Err(_) => {
                            err.unwrap();
                        }
                    };
                }
            });

        Ok(())
    }

    fn send_request(&mut self, ty: SelectionTarget, write_pipe: WritePipe, mime: String) {
        // We can only send strings, so don't do anything with the mime-type.
        if MimeType::find_allowed(&[mime]).is_none() {
            return;
        }

        // Mark FD as non-blocking so we won't block ourselves.
        unsafe {
            if set_non_blocking(write_pipe.as_raw_fd()).is_err() {
                return;
            }
        }

        // Don't access the content on the state directly, since it could change during
        // the send.
        let contents = match ty {
            SelectionTarget::Clipboard => self.data_selection_content.clone(),
            SelectionTarget::Primary => self.primary_selection_content.clone(),
        };

        let mut written = 0;
        let _ = self
            .loop_handle
            .insert_source(write_pipe, move |_, file, _| {
                let file = unsafe { file.get_mut() };
                loop {
                    match file.write(&contents[written..]) {
                        Ok(n) if written + n == contents.len() => {
                            written += n;
                            break PostAction::Remove;
                        }
                        Ok(n) => written += n,
                        Err(err) if err.kind() == ErrorKind::WouldBlock => {
                            break PostAction::Continue
                        }
                        Err(_) => break PostAction::Remove,
                    }
                }
            });
    }
}

impl State {
    pub fn new_capability(
        &mut self,
        qh: &QueueHandle<SimpleLayer>,
        seat: WlSeat,
        capability: Capability,
    ) {
        match capability {
            Capability::Keyboard => {
                // Selection sources are tied to the keyboard, so add/remove decives
                // when we gain/loss capability.

                if self.seat.data_device.is_none() && self.data_device_manager_state.is_some() {
                    self.seat.data_device = self
                        .data_device_manager_state
                        .as_ref()
                        .map(|mgr| mgr.get_data_device(qh, &seat));
                }

                if self.seat.primary_device.is_none()
                    && self.primary_selection_manager_state.is_some()
                {
                    self.seat.primary_device = self
                        .primary_selection_manager_state
                        .as_ref()
                        .map(|mgr| mgr.get_selection_device(qh, &seat));
                }
            }
            _ => (),
        }
    }

    pub fn remove_capability(&mut self, _: WlSeat, capability: Capability) {
        match capability {
            Capability::Keyboard => {
                self.seat.data_device = None;
                self.seat.primary_device = None;
            }
            _ => (),
        }
    }
}

impl State {
    pub fn pointer_frame(&mut self, events: &[PointerEvent]) {
        for event in events {
            match event.kind {
                PointerEventKind::Press { serial, .. }
                | PointerEventKind::Release { serial, .. } => {
                    self.seat.latest_serial = serial;
                }
                _ => (),
            }
        }
    }
}

impl DataDeviceHandler for SimpleLayer {
    fn enter(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &WlDataDevice) {}

    fn leave(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &WlDataDevice) {}

    fn motion(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &WlDataDevice) {}

    fn drop_performed(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &WlDataDevice) {}

    // The selection is finished and ready to be used.
    fn selection(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &WlDataDevice) {}
}

impl DataSourceHandler for SimpleLayer {
    fn send_request(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &WlDataSource,
        mime: String,
        write_pipe: WritePipe,
    ) {
        self.clipboard_state
            .send_request(SelectionTarget::Clipboard, write_pipe, mime)
    }

    fn cancelled(&mut self, _: &Connection, _: &QueueHandle<Self>, deleted: &WlDataSource) {
        self.clipboard_state
            .data_sources
            .retain(|source| source.inner() != deleted)
    }

    fn accept_mime(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &WlDataSource,
        _: Option<String>,
    ) {
    }

    fn dnd_dropped(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &WlDataSource) {}

    fn action(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &WlDataSource, _: DndAction) {}

    fn dnd_finished(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &WlDataSource) {}
}

impl DataOfferHandler for SimpleLayer {
    fn source_actions(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &mut DragOffer,
        _: DndAction,
    ) {
    }

    fn selected_action(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &mut DragOffer,
        _: DndAction,
    ) {
    }
}

impl PrimarySelectionDeviceHandler for SimpleLayer {
    fn selection(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &ZwpPrimarySelectionDeviceV1,
    ) {
    }
}

impl PrimarySelectionSourceHandler for SimpleLayer {
    fn send_request(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &ZwpPrimarySelectionSourceV1,
        mime: String,
        write_pipe: WritePipe,
    ) {
        self.clipboard_state
            .send_request(SelectionTarget::Primary, write_pipe, mime);
    }

    fn cancelled(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        deleted: &ZwpPrimarySelectionSourceV1,
    ) {
        self.clipboard_state
            .primary_sources
            .retain(|source| source.inner() != deleted)
    }
}

impl State {
    pub fn keyboard_key(&mut self, serial: u32) {
        self.seat.latest_serial = serial;
    }

    pub fn keyboard_enter(&mut self, serial: u32) {
        self.seat.latest_serial = serial;
        self.seat.has_focus = true;
    }

    pub fn keyboard_leave(&mut self) {
        self.seat.latest_serial = 0;
        self.seat.has_focus = false;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SelectionTarget {
    /// The target is clipboard selection.
    Clipboard,
    /// The target is primary selection.
    Primary,
}

#[derive(Debug, Default)]
struct ClipboardSeatState {
    data_device: Option<DataDevice>,
    primary_device: Option<PrimarySelectionDevice>,
    has_focus: bool,

    /// The latest serial used to set the selection content.
    latest_serial: u32,
}

unsafe fn set_non_blocking(raw_fd: RawFd) -> std::io::Result<()> {
    let flags = nix::fcntl::fcntl(raw_fd, FcntlArg::F_GETFL).unwrap();

    if flags < 0 {
        return Err(std::io::Error::last_os_error());
    }

    let result = nix::fcntl::fcntl(
        raw_fd,
        FcntlArg::F_SETFL(OFlag::from_bits(flags).unwrap() | OFlag::O_NONBLOCK),
    )
    .unwrap();
    if result < 0 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(())
}

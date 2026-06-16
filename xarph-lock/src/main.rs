//! Xarph Lock Screen
//!
//! Uses ext-session-lock-v1 for secure session locking.
//! Renders via SHM + Cairo, authenticates via PAM.

use cairo::{Context, Format, ImageSurface};
use smithay_client_toolkit::{
    compositor::{CompositorHandler, CompositorState},
    output::{OutputHandler, OutputState},
    reexports::calloop::{EventLoop, LoopHandle},
    reexports::calloop_wayland_source::WaylandSource,
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    seat::{SeatHandler, SeatState},
    seat::keyboard::{KeyboardHandler, KeyEvent, Keysym, Modifiers},
    session_lock::{
        SessionLock, SessionLockHandler, SessionLockState, SessionLockSurface,
        SessionLockSurfaceConfigure,
    },
    shm::{raw::RawPool, Shm, ShmHandler},
};
use wayland_client::{
    globals::registry_queue_init,
    protocol::{wl_buffer, wl_keyboard, wl_output, wl_seat, wl_shm, wl_surface},
    Connection, QueueHandle,
};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const BG_COLOR: (f64, f64, f64) = (0.047, 0.047, 0.071);
const TEXT_COLOR: (f64, f64, f64) = (0.9, 0.9, 0.94);
const ACCENT_COLOR: (f64, f64, f64) = (0.667, 0.549, 1.0);
const ERROR_COLOR: (f64, f64, f64) = (1.0, 0.47, 0.47);

struct AppData {
    loop_handle: LoopHandle<'static, Self>,
    conn: Connection,
    compositor_state: CompositorState,
    output_state: OutputState,
    registry_state: RegistryState,
    seat_state: SeatState,
    shm: Shm,
    session_lock_state: SessionLockState,
    session_lock: Option<SessionLock>,
    lock_surfaces: Vec<SessionLockSurface>,
    exit: bool,
    password: Arc<Mutex<String>>,
    error_message: Arc<Mutex<Option<String>>>,
    authenticated: Arc<Mutex<bool>>,
}

fn main() {
    env_logger::init();

    let conn = Connection::connect_to_env().expect("Failed to connect to Wayland");
    let (globals, event_queue) = registry_queue_init(&conn).expect("Failed to init globals");
    let qh: QueueHandle<AppData> = event_queue.handle();
    let mut event_loop: EventLoop<AppData> =
        EventLoop::try_new().expect("Failed to create event loop");

    let password = Arc::new(Mutex::new(String::new()));
    let error_message = Arc::new(Mutex::new(None::<String>));
    let authenticated = Arc::new(Mutex::new(false));

    let mut app_data = AppData {
        loop_handle: event_loop.handle(),
        conn: conn.clone(),
        compositor_state: CompositorState::bind(&globals, &qh).expect("Compositor bind failed"),
        output_state: OutputState::new(&globals, &qh),
        registry_state: RegistryState::new(&globals),
        seat_state: SeatState::new(&globals, &qh),
        shm: Shm::bind(&globals, &qh).expect("SHM bind failed"),
        session_lock_state: SessionLockState::new(&globals, &qh),
        session_lock: None,
        lock_surfaces: Vec::new(),
        exit: false,
        password: password.clone(),
        error_message: error_message.clone(),
        authenticated: authenticated.clone(),
    };

    // Initiate the session lock.
    app_data.session_lock = Some(
        app_data
            .session_lock_state
            .lock(&qh)
            .expect("ext-session-lock-v1 not supported by compositor"),
    );

    WaylandSource::new(conn.clone(), event_queue)
        .insert(event_loop.handle())
        .expect("Failed to insert Wayland source");

    log::info!("xarph-lock: session lock initiated, waiting for surfaces...");

    loop {
        event_loop
            .dispatch(Duration::from_millis(16), &mut app_data)
            .expect("Event loop dispatch failed");

        if app_data.exit {
            break;
        }
    }

    log::info!("xarph-lock: exiting");
}

impl SessionLockHandler for AppData {
    fn locked(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        session_lock: SessionLock,
    ) {
        log::info!("xarph-lock: session locked, creating lock surfaces");

        for output in self.output_state.outputs() {
            let surface = self.compositor_state.create_surface(qh);
            let lock_surface = session_lock.create_lock_surface(surface, &output, qh);
            self.lock_surfaces.push(lock_surface);
        }
    }

    fn finished(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _session_lock: SessionLock,
    ) {
        log::info!("xarph-lock: session lock finished (compositor cancelled)");
        self.exit = true;
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        session_lock_surface: SessionLockSurface,
        configure: SessionLockSurfaceConfigure,
        _serial: u32,
    ) {
        let (width, height) = configure.new_size;
        let width = if width == 0 { 1920 } else { width };
        let height = if height == 0 { 1080 } else { height };

        render_lock_surface(
            &session_lock_surface,
            &self.shm,
            qh,
            width,
            height,
            &self.password,
            &self.error_message,
        );
    }
}

impl CompositorHandler for AppData {
    fn scale_factor_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _new_factor: i32,
    ) {
    }

    fn transform_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _new_transform: wl_output::Transform,
    ) {
    }

    fn frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _time: u32,
    ) {
    }

    fn surface_enter(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _output: &wl_output::WlOutput,
    ) {
    }

    fn surface_leave(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _output: &wl_output::WlOutput,
    ) {
    }
}

impl OutputHandler for AppData {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
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
        _output: wl_output::WlOutput,
    ) {
    }
}

impl SeatHandler for AppData {
    fn seat_state(&mut self) -> &mut SeatState {
        &mut self.seat_state
    }

    fn new_seat(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: wl_seat::WlSeat,
    ) {
    }

    fn new_capability(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        _seat: wl_seat::WlSeat,
        capability: smithay_client_toolkit::seat::Capability,
    ) {
        if capability == smithay_client_toolkit::seat::Capability::Keyboard {
            self.seat_state
                .get_keyboard(qh, &_seat, None)
                .expect("Failed to get keyboard");
        }
    }

    fn remove_capability(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: wl_seat::WlSeat,
        _capability: smithay_client_toolkit::seat::Capability,
    ) {
    }

    fn remove_seat(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: wl_seat::WlSeat,
    ) {
    }
}

impl KeyboardHandler for AppData {
    fn enter(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &wl_keyboard::WlKeyboard,
        _surface: &wl_surface::WlSurface,
        _serial: u32,
        _raw: &[u32],
        _keysyms: &[Keysym],
    ) {
    }

    fn leave(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &wl_keyboard::WlKeyboard,
        _surface: &wl_surface::WlSurface,
        _serial: u32,
    ) {
    }

    fn press_key(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        _keyboard: &wl_keyboard::WlKeyboard,
        _serial: u32,
        event: KeyEvent,
    ) {
        // Handle special keys.
        match event.keysym {
            Keysym::Return | Keysym::KP_Enter => {
                let password = self.password.lock().unwrap().clone();
                if password.is_empty() {
                    return;
                }
                if authenticate_pam(&password) {
                    log::info!("xarph-lock: authentication successful, unlocking");
                    *self.authenticated.lock().unwrap() = true;
                    if let Some(lock) = self.session_lock.take() {
                        lock.unlock();
                        self.conn.roundtrip().ok();
                        self.exit = true;
                    }
                } else {
                    log::info!("xarph-lock: authentication failed");
                    *self.error_message.lock().unwrap() =
                        Some("Incorrect password".to_string());
                    self.password.lock().unwrap().clear();
                    // Re-render all lock surfaces with error.
                    self.render_all(qh);
                }
                return;
            }
            Keysym::BackSpace => {
                self.password.lock().unwrap().pop();
                self.render_all(qh);
                return;
            }
            Keysym::Escape => {
                self.password.lock().unwrap().clear();
                *self.error_message.lock().unwrap() = None;
                self.render_all(qh);
                return;
            }
            _ => {}
        }

        // Handle text input.
        if let Some(text) = &event.utf8 {
            if !text.is_empty() && !text.chars().any(|c| c.is_control()) {
                self.password.lock().unwrap().push_str(text);
                *self.error_message.lock().unwrap() = None;
                self.render_all(qh);
            }
        }
    }

    fn release_key(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &wl_keyboard::WlKeyboard,
        _serial: u32,
        _event: KeyEvent,
    ) {
    }

    fn update_modifiers(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &wl_keyboard::WlKeyboard,
        _serial: u32,
        _modifiers: Modifiers,
        _layout: u32,
    ) {
    }
}

impl ProvidesRegistryState for AppData {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState, SeatState,];
}

impl ShmHandler for AppData {
    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm
    }
}

smithay_client_toolkit::delegate_compositor!(AppData);
smithay_client_toolkit::delegate_output!(AppData);
smithay_client_toolkit::delegate_seat!(AppData);
smithay_client_toolkit::delegate_keyboard!(AppData);
smithay_client_toolkit::delegate_session_lock!(AppData);
smithay_client_toolkit::delegate_shm!(AppData);
smithay_client_toolkit::delegate_registry!(AppData);
wayland_client::delegate_noop!(AppData: ignore wl_buffer::WlBuffer);

// --- Rendering ---

impl AppData {
    fn render_all(&mut self, qh: &QueueHandle<Self>) {
        let surfaces: Vec<_> = self.lock_surfaces.clone();
        for surface in &surfaces {
            render_lock_surface(
                surface,
                &self.shm,
                qh,
                1920,
                1080,
                &self.password,
                &self.error_message,
            );
        }
    }
}

fn render_lock_surface(
    lock_surface: &SessionLockSurface,
    shm: &Shm,
    qh: &QueueHandle<AppData>,
    width: u32,
    height: u32,
    password: &Arc<Mutex<String>>,
    error_message: &Arc<Mutex<Option<String>>>,
) {
    let stride = width as i32 * 4;
    let size = stride as usize * height as usize;

    let mut pool = match RawPool::new(size, shm) {
        Ok(pool) => pool,
        Err(_) => {
            log::error!("xarph-lock: failed to create SHM pool");
            return;
        }
    };

    // Render with Cairo into an ImageSurface, then copy to SHM.
    let mut surface =
        ImageSurface::create(Format::ARgb32, width as i32, height as i32)
            .expect("Failed to create Cairo surface");
    {
        let cr = Context::new(&surface).expect("Failed to create Cairo context");

        // Draw background.
        cr.set_source_rgb(BG_COLOR.0, BG_COLOR.1, BG_COLOR.2);
        cr.paint().expect("Failed to paint background");

        let center_x = width as f64 / 2.0;
        let center_y = height as f64 / 2.0;

        // Lock icon.
        cr.set_source_rgb(ACCENT_COLOR.0, ACCENT_COLOR.1, ACCENT_COLOR.2);
        cr.select_font_face("sans-serif", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
        cr.set_font_size(64.0);
        let icon_text = "\u{1F512}";
        let icon_ext = cr.text_extents(icon_text).unwrap();
        cr.move_to(center_x - icon_ext.width() / 2.0, center_y - 60.0);
        cr.show_text(icon_text).ok();

        // Title.
        cr.set_source_rgb(TEXT_COLOR.0, TEXT_COLOR.1, TEXT_COLOR.2);
        cr.set_font_size(32.0);
        let title_ext = cr.text_extents("Locked").unwrap();
        cr.move_to(center_x - title_ext.width() / 2.0, center_y + 10.0);
        cr.show_text("Locked").ok();

        // Password mask.
        let pwd = password.lock().unwrap();
        let mask = if pwd.is_empty() {
            String::new()
        } else {
            "*".repeat(pwd.len())
        };
        drop(pwd);

        cr.set_font_size(20.0);
        let mask_ext = cr.text_extents(&mask).unwrap();
        cr.move_to(center_x - mask_ext.width() / 2.0, center_y + 60.0);
        cr.show_text(&mask).ok();

        // Error message.
        if let Some(err) = error_message.lock().unwrap().as_ref() {
            cr.set_source_rgb(ERROR_COLOR.0, ERROR_COLOR.1, ERROR_COLOR.2);
            cr.set_font_size(14.0);
            let err_ext = cr.text_extents(err).unwrap();
            cr.move_to(center_x - err_ext.width() / 2.0, center_y + 100.0);
            cr.show_text(err).ok();
        }

        // Hint text.
        cr.set_source_rgba(TEXT_COLOR.0, TEXT_COLOR.1, TEXT_COLOR.2, 0.4);
        cr.set_font_size(12.0);
        let hint = "Type your password and press Enter";
        let hint_ext = cr.text_extents(hint).unwrap();
        cr.move_to(center_x - hint_ext.width() / 2.0, center_y + 140.0);
        cr.show_text(hint).ok();
    }
    surface.flush();

    // Copy rendered pixels to SHM pool.
    {
        let data = surface.data().expect("Failed to read Cairo surface data");
        let canvas = pool.mmap();
        canvas[..size].copy_from_slice(&data);
    }

    // Submit the buffer.
    let buffer = pool.create_buffer(
        0,
        width as i32,
        height as i32,
        stride,
        wl_shm::Format::Argb8888,
        (),
        qh,
    );

    lock_surface.wl_surface().attach(Some(&buffer), 0, 0);
    lock_surface.wl_surface().commit();
    buffer.destroy();
}

/// Authenticate using PAM with the "login" service.
/// Falls back to false if PAM is unavailable.
fn authenticate_pam(password: &str) -> bool {
    let user = match std::env::var("USER") {
        Ok(u) => u,
        Err(_) => return false,
    };

    let mut auth = match pam::Authenticator::with_password("login") {
        Ok(a) => a,
        Err(e) => {
            eprintln!("xarph-lock: PAM unavailable ({e}), denying unlock");
            return false;
        }
    };

    auth.get_handler().set_credentials(&user, password);
    auth.authenticate().is_ok()
}

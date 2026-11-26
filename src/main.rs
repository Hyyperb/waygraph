#![allow(unused_variables)]
#![allow(dead_code)]

use std::{collections::HashMap, ffi::c_void};

use wayland_client::{
    Dispatch, Proxy,
    protocol::{
        wl_buffer,
        wl_compositor::{self, WlCompositor},
        wl_display, wl_pointer, wl_registry, wl_seat, wl_shm, wl_shm_pool, wl_surface,
    },
};

use wayland_protocols_wlr::layer_shell::v1::client::{zwlr_layer_shell_v1, zwlr_layer_surface_v1};

use fonts::FontRenderer;

use crate::{desktop::DesktopItem, layout::DummyLayout, renderer::quad::QuadRenderer};

extern crate khronos_egl as egl;

mod desktop;
mod fonts;
mod input;
mod layout;
mod renderer;
mod ui;

const FONT: &str = "NotoSans-Regular";

#[derive(Default)]
struct Flags {
    configured: bool,
    drawshadow: bool,
    setexclusivezone: bool,
}

#[derive(Default)]
struct PointerData {
    last_x: f64,
    last_y: f64,
    last_time: u32,
}

struct WaygraphConfig {
    double_click_timeout: u32,
}

impl Default for WaygraphConfig {
    fn default() -> Self {
        WaygraphConfig {
            double_click_timeout: 200,
        }
    }
}

struct DesktopLayout {
    items: Vec<DesktopItem>,
    layout: Box<dyn layout::Layout>,
    width: u32,
    height: u32,
    icon_renderer: Option<QuadRenderer>,
}

impl Default for DesktopLayout {
    fn default() -> Self {
        let items = vec![];
        let layout = Box::new(DummyLayout {});
        Self {
            items,
            layout,
            width: 1920,
            height: 1080,
            icon_renderer: None,
        }
    }
}

#[derive(Default)]
struct FontData {
    font_renderer: Option<FontRenderer>,
}

#[derive(Default)]
struct WayGraph {
    flags: Flags,
    config: WaygraphConfig,
    pointer: PointerData,
    desktop: DesktopLayout,
    font: FontData,
}

struct AppData {
    compositor: Option<WlCompositor>,
    layer_shell: Option<zwlr_layer_shell_v1::ZwlrLayerShellV1>,
    shm: Option<wl_shm::WlShm>,
    seat: Option<wl_seat::WlSeat>,
    app: WayGraph,
}

impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: <wl_registry::WlRegistry as wayland_client::Proxy>::Event,
        _data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            match interface.as_str() {
                "wl_compositor" => {
                    state.compositor = Some(registry.bind(name, version.min(4), qhandle, ()));
                }
                "zwlr_layer_shell_v1" => {
                    state.layer_shell = Some(registry.bind(name, 1, qhandle, ()));
                }
                "wl_shm" => {
                    state.shm = Some(registry.bind(name, 1, qhandle, ()));
                }
                "wl_seat" => {
                    state.seat = Some(registry.bind(name, 1, qhandle, ()));
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<wl_display::WlDisplay, ()> for AppData {
    fn event(
        state: &mut Self,
        proxy: &wl_display::WlDisplay,
        event: <wl_display::WlDisplay as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_surface::WlSurface, ()> for AppData {
    fn event(
        state: &mut Self,
        proxy: &wl_surface::WlSurface,
        event: <wl_surface::WlSurface as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_compositor::WlCompositor, ()> for AppData {
    fn event(
        state: &mut Self,
        proxy: &wl_compositor::WlCompositor,
        event: <wl_compositor::WlCompositor as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<zwlr_layer_shell_v1::ZwlrLayerShellV1, ()> for AppData {
    fn event(
        state: &mut Self,
        layer_shell: &zwlr_layer_shell_v1::ZwlrLayerShellV1,
        event: <zwlr_layer_shell_v1::ZwlrLayerShellV1 as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<zwlr_layer_surface_v1::ZwlrLayerSurfaceV1, ()> for AppData {
    fn event(
        state: &mut Self,
        layer_surface: &zwlr_layer_surface_v1::ZwlrLayerSurfaceV1,
        event: <zwlr_layer_surface_v1::ZwlrLayerSurfaceV1 as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        if let zwlr_layer_surface_v1::Event::Configure { serial, .. } = event {
            layer_surface.ack_configure(serial);
            state.app.flags.configured = true;
        }
    }
}

impl Dispatch<wl_shm::WlShm, ()> for AppData {
    fn event(
        state: &mut Self,
        proxy: &wl_shm::WlShm,
        event: <wl_shm::WlShm as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_seat::WlSeat, ()> for AppData {
    fn event(
        state: &mut Self,
        seat: &wl_seat::WlSeat,
        event: <wl_seat::WlSeat as Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_pointer::WlPointer, ()> for AppData {
    fn event(
        state: &mut Self,
        pointer: &wl_pointer::WlPointer,
        event: <wl_pointer::WlPointer as Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        // println!("{event:?}");
        match event {
            wl_pointer::Event::Motion {
                time,
                surface_x,
                surface_y,
            } => input::pointer::handle_motion(state, surface_x, surface_y, time),

            wl_pointer::Event::Button {
                serial,
                time,
                button,
                state: button_state,
            } => input::pointer::handle_button(state, button, button_state, time),

            _ => {}
        }
    }
}

impl Dispatch<wl_buffer::WlBuffer, ()> for AppData {
    fn event(
        state: &mut Self,
        proxy: &wl_buffer::WlBuffer,
        event: <wl_buffer::WlBuffer as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_shm_pool::WlShmPool, ()> for AppData {
    fn event(
        state: &mut Self,
        proxy: &wl_shm_pool::WlShmPool,
        event: <wl_shm_pool::WlShmPool as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
    }
}

fn main() {
    let conn = wayland_client::Connection::connect_to_env().unwrap();
    let display = conn.display();

    let mut event_queue = conn.new_event_queue();

    let qh = event_queue.handle();

    let _registry = display.get_registry(&qh, ());

    let mut state = AppData {
        compositor: None,
        layer_shell: None,
        shm: None,
        seat: None,
        app: WayGraph::default(),
    };

    state.app.desktop.width = 1920;
    state.app.desktop.height = 1000;

    type FlagCallback = Box<dyn Fn(&mut AppData)>;
    let mut flag_callbacks: HashMap<&str, FlagCallback> = HashMap::new();
    flag_callbacks.insert(
        "--draw-shadow",
        Box::new(|state: &mut AppData| {
            state.app.flags.drawshadow = true;
        }),
    );

    flag_callbacks.insert(
        "--set-exclusive-zone",
        Box::new(|state: &mut AppData| {
            state.app.flags.setexclusivezone = true;
        }),
    );

    for arg in std::env::args().skip(1) {
        match flag_callbacks.get(arg.as_str()) {
            Some(callback) => callback(&mut state),
            None => {
                println!("Skipping unrecognised argument: {}", arg)
            }
        }

        if let Some(callback) = flag_callbacks.get(arg.as_str()) {
            callback(&mut state);
        }
        // TODO: replace with a arg parsing library.
    }

    event_queue.roundtrip(&mut state).unwrap();

    let compositor = state.compositor.as_ref().unwrap();
    let layer_shell = state.layer_shell.as_ref().unwrap();
    let shm = state.shm.as_ref().unwrap();
    let seat = state.seat.as_ref().unwrap();
    let pointer = seat.get_pointer(&qh, ());

    let surface = compositor.create_surface(&qh, ());

    let layer_surface = layer_shell.get_layer_surface(
        &surface,
        None,
        zwlr_layer_shell_v1::Layer::Bottom,
        "waygraph".into(),
        &qh,
        (),
    );

    layer_surface.set_size(state.app.desktop.width, state.app.desktop.height);
    layer_surface
        .set_anchor(zwlr_layer_surface_v1::Anchor::Bottom | zwlr_layer_surface_v1::Anchor::Left);

    if state.app.flags.setexclusivezone {
        layer_surface.set_exclusive_zone(-1);
    }

    surface.commit();

    while !state.app.flags.configured {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }

    let egl = egl::Instance::new(egl::Static);
    let egl_display = unsafe { egl.get_display(display.id().as_ptr() as *mut c_void) }.unwrap();
    egl.initialize(egl_display).unwrap();
    egl.bind_api(egl::OPENGL_API).unwrap();

    #[rustfmt::skip]
    let attributes = [
        egl::RED_SIZE, 8,
        egl::GREEN_SIZE, 8,
        egl::BLUE_SIZE, 8,
        egl::ALPHA_SIZE, 8,
        egl::RENDERABLE_TYPE, egl::OPENGL_ES3_BIT,
        egl::NONE,
    ];

    #[rustfmt::skip]
    let context_attributes = [
        egl::CONTEXT_MAJOR_VERSION, 3,
        egl::CONTEXT_MINOR_VERSION, 2,
        egl::NONE,
    ];

    let config = egl
        .choose_first_config(egl_display, &attributes)
        .unwrap()
        .unwrap();

    let context = egl
        .create_context(egl_display, config, None, &context_attributes)
        .unwrap();

    let egl_window = wayland_egl::WlEglSurface::new(
        surface.id(),
        state.app.desktop.width as i32,
        state.app.desktop.height as i32,
    )
    .unwrap();

    let egl_surface = unsafe {
        egl.create_window_surface(egl_display, config, egl_window.ptr() as *mut _, None)
            .unwrap()
    };

    egl.make_current(
        egl_display,
        Some(egl_surface),
        Some(egl_surface),
        Some(context),
    )
    .unwrap();
    gl::load_with(|name| {
        let c = std::ffi::CString::new(name).unwrap();
        egl.get_proc_address(name).unwrap() as *const c_void
    });

    let desktop_entry_size = 160;
    let desktop_entries = desktop::get_desktop_entries();
    state.app.desktop.items = desktop_entries;

    let row_count = state.app.desktop.height / desktop_entry_size;

    let layoutgrid = layout::simplegridlayout::SimpleGridLayout::new(
        state.app.desktop.width as f32,
        state.app.desktop.height as f32,
        FONT,
        18,
        desktop_entry_size as f32,
    );

    state.app.desktop.icon_renderer = Some(renderer::quad::QuadRenderer::new());
    state.app.desktop.layout = Box::new(layoutgrid);

    loop {
        event_queue.dispatch_pending(&mut state).unwrap();
        unsafe {
            gl::Viewport(
                0,
                0,
                state.app.desktop.width as i32,
                state.app.desktop.height as i32,
            );

            if state.app.flags.drawshadow {
                gl::ClearColor(0.0, 0.0, 0.0, 0.7);
            } else {
                gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            }
            gl::Clear(gl::COLOR_BUFFER_BIT);

            state
                .app
                .desktop
                .layout
                .arrange_items(&mut state.app.desktop.items);
            state.app.desktop.layout.render(&state);
        }
        egl.swap_buffers(egl_display, egl_surface).unwrap();
    }

    // event_queue.blocking_dispatch(&mut state).unwrap();
}

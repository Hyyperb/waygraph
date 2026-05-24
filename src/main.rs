#![allow(unused_variables)]
#![allow(dead_code)]

use std::os::fd::AsFd;

use wayland_client::{
    Dispatch,
    protocol::{
        wl_buffer,
        wl_compositor::{self, WlCompositor},
        wl_display, wl_registry, wl_shm, wl_shm_pool, wl_surface,
    },
};

use wayland_protocols_wlr::layer_shell::v1::client::{zwlr_layer_shell_v1, zwlr_layer_surface_v1};

struct AppData {
    compositor: Option<WlCompositor>,
    layer_shell: Option<zwlr_layer_shell_v1::ZwlrLayerShellV1>,
    shm: Option<wl_shm::WlShm>,

    configured: bool,
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
            state.configured = true;
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
        configured: false,
    };

    event_queue.roundtrip(&mut state).unwrap();

    let compositor = state.compositor.as_ref().unwrap();
    let layer_shell = state.layer_shell.as_ref().unwrap();
    let shm = state.shm.as_ref().unwrap();

    let surface = compositor.create_surface(&qh, ());

    let layer_surface = layer_shell.get_layer_surface(
        &surface,
        None,
        zwlr_layer_shell_v1::Layer::Background,
        "waygraph".into(),
        &qh,
        (),
    );

    layer_surface.set_size(400, 300);
    surface.commit();

    let file = std::fs::File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("/dev/shm/waygraph")
        .unwrap();

    let width = 400;
    let height = 300;
    let stride = width * 4;
    let size = stride * width;

    file.set_len(size).unwrap();

    let mut data = unsafe { memmap2::MmapMut::map_mut(&file).unwrap() };

    for chunk in data.chunks_exact_mut(4) {
        chunk[0] = 0xFF;
        chunk[1] = 0x00;
        chunk[2] = 0x00;
        chunk[3] = 0xFF;
    }

    let pool = shm.create_pool(file.as_fd(), size as i32, &qh, ());

    let buffer = pool.create_buffer(
        0,
        width as i32,
        height,
        stride as i32,
        wl_shm::Format::Argb8888,
        &qh,
        (),
    );

    while !state.configured {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }

    surface.attach(Some(&buffer), 0, 0);
    surface.commit();

    loop {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }
}

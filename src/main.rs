mod composition_host;

use bindings::windows::{
    foundation::numerics::Vector2,
    ui::composition::Compositor,
    win32::{
        system_services::{
            CreateDispatcherQueueController, DispatcherQueueOptions,
            DISPATCHERQUEUE_THREAD_APARTMENTTYPE, DISPATCHERQUEUE_THREAD_TYPE,
        },
        windows_and_messaging::HWND,
        winrt::{ICompositorDesktopInterop, RoInitialize, RO_INIT_TYPE},
    },
};
use composition_host::CompositionHost;
use raw_window_handle::HasRawWindowHandle;
use windows::Interface;
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn run() -> windows::Result<()> {
    // Ensure dispatcher queue.
    unsafe {
        RoInitialize(RO_INIT_TYPE::RO_INIT_SINGLETHREADED).ok()?;
    }

    let options = DispatcherQueueOptions {
        dw_size: std::mem::size_of::<DispatcherQueueOptions>() as u32,
        thread_type: DISPATCHERQUEUE_THREAD_TYPE::DQTYPE_THREAD_CURRENT,
        apartment_type: DISPATCHERQUEUE_THREAD_APARTMENTTYPE::DQTAT_COM_NONE,
    };
    let _controller = unsafe {
        let mut result = None;
        CreateDispatcherQueueController(options, &mut result).and_some(result)?
    };

    // Create window.
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Click to add composition elements...")
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    // Create desktop window target.
    let compositor = Compositor::new()?;
    let window_handle = window.raw_window_handle();
    let window_handle = match window_handle {
        raw_window_handle::RawWindowHandle::Windows(window_handle) => window_handle.hwnd,
        _ => panic!("Unsupported platform!"),
    };

    let compositor_desktop: ICompositorDesktopInterop = compositor.cast()?;
    let mut result = None;

    let target = compositor_desktop
        .CreateDesktopWindowTarget(HWND(window_handle as isize), false.into(), &mut result)
        .and_some(result)?;

    // Create composition root.
    let container_visual = compositor.create_container_visual()?;
    container_visual.set_relative_size_adjustment(Vector2 { x: 1.0, y: 1.0 })?;
    target.set_root(&container_visual)?;

    // Create composition host.
    let window_size = window.inner_size();
    let comp_host = CompositionHost::new(&container_visual, window_size.width, window_size.height)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, .. },
                ..
            } => {
                if state == ElementState::Pressed {
                    comp_host.add_element().unwrap();
                }
            }
            _ => (),
        }
    });
}

fn main() {
    let result = run();

    // We do this for nicer HRESULT printing when errors occur.
    if let Err(error) = result {
        error.code().unwrap();
    }
}

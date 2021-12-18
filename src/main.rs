mod composition_host;

use composition_host::CompositionHost;
use raw_window_handle::HasRawWindowHandle;
use windows::{
    core::{Interface, Result},
    Foundation::Numerics::Vector2,
    Win32::System::{
        Com::{CoInitializeEx, COINIT_APARTMENTTHREADED},
        WinRT::{
            Composition::ICompositorDesktopInterop,
            CreateDispatcherQueueController, DispatcherQueueOptions,
            DQTAT_COM_NONE, DQTYPE_THREAD_CURRENT,
        },
    },
    UI::Composition::Compositor,
};
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn run() -> Result<()> {
    // Ensure dispatcher queue.
    unsafe { CoInitializeEx(std::ptr::null_mut(), COINIT_APARTMENTTHREADED)? };

    let options = DispatcherQueueOptions {
        dwSize: std::mem::size_of::<DispatcherQueueOptions>() as u32,
        threadType: DQTYPE_THREAD_CURRENT,
        apartmentType: DQTAT_COM_NONE,
    };
    let _controller = unsafe { CreateDispatcherQueueController(options)? };

    // Create window.
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Left click to add composition elements...")
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    // Create desktop window target.
    let compositor = Compositor::new()?;
    let window_handle = window.raw_window_handle();
    let hwnd = match window_handle {
        raw_window_handle::RawWindowHandle::Win32(windows_handle) => {
            windows_handle.hwnd
        },
        _ => panic!("Unsupported platform!"),
    };

    let compositor_desktop: ICompositorDesktopInterop = compositor.cast()?;

    let target = unsafe {
        compositor_desktop.CreateDesktopWindowTarget(hwnd as isize, false)?
    };

    // Create composition root.
    let container_visual = compositor.CreateContainerVisual()?;
    container_visual.SetRelativeSizeAdjustment(Vector2 { X: 1.0, Y: 1.0 })?;
    target.SetRoot(&container_visual)?;

    // Create composition host.
    let window_size = window.inner_size();
    let comp_host = CompositionHost::new(
        &container_visual,
        window_size.width,
        window_size.height,
    )?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        button: MouseButton::Left,
                        ..
                    },
                ..
            } => {
                comp_host.add_element().unwrap();
            },
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

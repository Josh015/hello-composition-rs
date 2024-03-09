mod composition_host;

use composition_host::CompositionHost;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::{
    core::*,
    Foundation::Numerics::Vector2,
    Win32::{
        Foundation::HWND,
        System::{
            Com::{CoInitializeEx, COINIT_APARTMENTTHREADED},
            WinRT::{
                Composition::ICompositorDesktopInterop,
                CreateDispatcherQueueController, DispatcherQueueOptions,
                DQTAT_COM_NONE, DQTYPE_THREAD_CURRENT,
            },
        },
    },
    UI::Composition::Compositor,
};
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() -> anyhow::Result<()> {
    // Ensure dispatcher queue.
    unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()? };

    let options = DispatcherQueueOptions {
        dwSize: std::mem::size_of::<DispatcherQueueOptions>() as u32,
        threadType: DQTYPE_THREAD_CURRENT,
        apartmentType: DQTAT_COM_NONE,
    };
    let _controller = unsafe { CreateDispatcherQueueController(options)? };

    // Create window and obtain handle.
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Left click to add composition elements...")
        .with_resizable(false)
        .build(&event_loop)?;
    let raw_window_handle = window.window_handle()?.into();
    let hwnd = match raw_window_handle {
        RawWindowHandle::Win32(window_handle) => {
            HWND(window_handle.hwnd.into())
        },
        _ => panic!("Unsupported platform!"),
    };

    // Create compositor.
    let compositor = Compositor::new()?;
    let compositor_desktop: ICompositorDesktopInterop = compositor.cast()?;
    let target =
        unsafe { compositor_desktop.CreateDesktopWindowTarget(hwnd, false)? };
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

    event_loop.run(move |event, event_loop| {
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => event_loop.exit(),
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
    })?;

    Ok(())
}

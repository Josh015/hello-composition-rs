mod composition_host;
mod interop;
mod window_target;

use bindings::windows::{foundation::numerics::Vector2, ui::composition::Compositor};
use composition_host::CompositionHost;
use interop::{create_dispatcher_queue_controller_for_current_thread, ro_initialize, RoInitType};
use window_target::CompositionDesktopWindowTargetSource;
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn run() -> winrt::Result<()> {
    // Ensure dispatcher queue.
    ro_initialize(RoInitType::MultiThreaded)?;
    let _controller = create_dispatcher_queue_controller_for_current_thread()?;

    // Create window.
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Click to add composition elements...");
    window.set_resizable(false);

    // Create desktop window target.
    let compositor = Compositor::new()?;
    let target = window.create_window_target(&compositor, false)?;

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

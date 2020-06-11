mod composition_host;
mod interop;
mod window_target;

use composition_host::CompositionHost;
use interop::{create_dispatcher_queue_controller_for_current_thread, ro_initialize, RoInitType};
use window_target::CompositionDesktopWindowTargetSource;
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use bindings::windows::{foundation::numerics::Vector2, ui::composition::Compositor};

fn run() -> winrt::Result<()> {
    ro_initialize(RoInitType::MultiThreaded)?;
    let _controller = create_dispatcher_queue_controller_for_current_thread()?;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Click to add composition elements...");

    let compositor = Compositor::new()?;
    let target = window.create_window_target(&compositor, false)?;

    let root = compositor.create_container_visual()?;
    root.set_relative_size_adjustment(Vector2 { x: 1.0, y: 1.0 })?;
    target.set_root(&root)?;

    let window_size = window.inner_size();
    let window_size = Vector2 {
        x: window_size.width as f32,
        y: window_size.height as f32,
    };
    let mut host = CompositionHost::new(&root, &window_size)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                if state == ElementState::Pressed {
                    host.on_pointer_pressed(button == MouseButton::Right, false)
                        .unwrap();
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

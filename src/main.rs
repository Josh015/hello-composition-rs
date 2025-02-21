use application::Application;
use winit::event_loop::{ControlFlow, EventLoop};

mod application;

fn main() -> anyhow::Result<()> {
    // Create window and obtain handle.
    let mut app = Application::default();
    let event_loop = EventLoop::new()?;

    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut app)?;
    Ok(())
}

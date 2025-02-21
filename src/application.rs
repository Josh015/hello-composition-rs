use rand::{distr::Uniform, prelude::*};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::{ffi::c_void, time::Duration};
use windows::{
    Foundation::Numerics::{Vector2, Vector3},
    System::DispatcherQueueController,
    UI::{
        Color,
        Composition::{
            Compositor, ContainerVisual, Desktop::DesktopWindowTarget,
        },
    },
    Win32::{
        Foundation::HWND,
        System::{
            Com::{COINIT_APARTMENTTHREADED, CoInitializeEx},
            WinRT::{
                Composition::ICompositorDesktopInterop,
                CreateDispatcherQueueController, DQTAT_COM_NONE,
                DQTYPE_THREAD_CURRENT, DispatcherQueueOptions,
            },
        },
    },
    core::*,
};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

pub struct Application {
    _controller: DispatcherQueueController,
    compositor: Compositor,
    target: Option<DesktopWindowTarget>,
    window: Option<Window>,
}

impl Default for Application {
    fn default() -> Self {
        // Ensure dispatcher queue.
        unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok().unwrap() };

        let options = DispatcherQueueOptions {
            dwSize: std::mem::size_of::<DispatcherQueueOptions>() as u32,
            threadType: DQTYPE_THREAD_CURRENT,
            apartmentType: DQTAT_COM_NONE,
        };
        let controller =
            unsafe { CreateDispatcherQueueController(options).unwrap() };

        // Create compositor and container.
        let compositor = Compositor::new().unwrap();

        Self {
            _controller: controller,
            compositor,
            target: None,
            window: None,
        }
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window.
        let window_attributes = WindowAttributes::default()
            .with_title("Left click to add composition elements...")
            .with_resizable(false);
        let window = event_loop.create_window(window_attributes).unwrap();

        let raw_window_handle = window.window_handle().unwrap().into();
        let hwnd = match raw_window_handle {
            RawWindowHandle::Win32(window_handle) => {
                HWND(window_handle.hwnd.get() as *mut c_void)
            },
            _ => panic!("Unsupported platform!"),
        };

        // Create desktop window target.
        let compositor_desktop: ICompositorDesktopInterop =
            self.compositor.cast().unwrap();
        let target = unsafe {
            compositor_desktop
                .CreateDesktopWindowTarget(hwnd, false)
                .unwrap()
        };

        // Create composition host.
        let container_visual = self.compositor.CreateContainerVisual().unwrap();

        container_visual
            .SetRelativeSizeAdjustment(Vector2 { X: 1.0, Y: 1.0 })
            .unwrap();

        target.SetRoot(&container_visual).unwrap();

        self.window = Some(window);
        self.target = Some(target);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                self.add_element().unwrap();
            },
            _ => (),
        }
    }
}

impl Application {
    fn add_element(&self) -> Result<()> {
        if let Ok(visual) = self.target.as_ref().unwrap().Root() {
            let visuals = visual.cast::<ContainerVisual>()?.Children()?;
            let window_size = self.window.as_ref().unwrap().inner_size();

            // Create randomized squares.
            let element = self.compositor.CreateSpriteVisual()?;
            let mut rng = rand::rng();
            let size = rng.random_range(50..150);
            let x = rng.random_range(0..window_size.width - size) as f32;
            let y = rng.random_range(0..(window_size.height / 2) - size) as f32;

            element.SetBrush(
                &self
                    .compositor
                    .CreateColorBrushWithColor(get_random_color())?,
            )?;
            element.SetSize(Vector2 {
                X: size as f32,
                Y: size as f32,
            })?;
            element.SetOffset(Vector3 { X: x, Y: y, Z: 0.0 })?;

            // Set square falling animations.
            let animation = self.compositor.CreateVector3KeyFrameAnimation()?;
            let bottom = window_size.height - size;

            animation.InsertKeyFrame(
                1.0,
                Vector3 {
                    X: x,
                    Y: bottom as f32,
                    Z: 0.0,
                },
            )?;
            animation.SetDuration(Duration::from_secs(2).into())?;
            animation.SetDelayTime(Duration::from_secs(3).into())?;
            element.StartAnimation(h!("Offset"), &animation)?;

            visuals.InsertAtTop(&element)?;
        }

        Ok(())
    }
}

fn get_random_color() -> Color {
    let mut rng = rand::rng();
    let die = Uniform::try_from(0..=255).unwrap();
    let r = die.sample(&mut rng);
    let g = die.sample(&mut rng);
    let b = die.sample(&mut rng);

    Color {
        A: 255,
        R: r,
        G: g,
        B: b,
    }
}

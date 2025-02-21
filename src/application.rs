use rand::{distr::Uniform, prelude::*};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::time::Duration;
use windows::{
    Foundation::Numerics::{Vector2, Vector3},
    System::DispatcherQueueController,
    UI::{
        Color,
        Composition::{
            Compositor, ContainerVisual, Desktop::DesktopWindowTarget,
            SpriteVisual,
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
    window::{Window, WindowAttributes},
};

pub struct Application {
    #[allow(dead_code)]
    controller: DispatcherQueueController,
    container_visual: ContainerVisual,
    compositor: Compositor,
    window: Option<Window>,
    #[allow(dead_code)]
    target: Option<DesktopWindowTarget>,
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

        // Create compositor.
        let compositor = Compositor::new().unwrap();
        let container_visual = compositor.CreateContainerVisual().unwrap();

        Self {
            controller,
            container_visual,
            compositor,
            window: None,
            target: None,
        }
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title("Left click to add composition elements...")
            .with_resizable(false);
        let window = event_loop.create_window(window_attributes).unwrap();

        let raw_window_handle = window.window_handle().unwrap().into();
        let hwnd = match raw_window_handle {
            RawWindowHandle::Win32(window_handle) => {
                HWND(window_handle.hwnd.into())
            },
            _ => panic!("Unsupported platform!"),
        };

        // Create compositor.
        let compositor_desktop: ICompositorDesktopInterop =
            self.compositor.cast().unwrap();
        let target = unsafe {
            compositor_desktop
                .CreateDesktopWindowTarget(hwnd, false)
                .unwrap()
        };
        self.container_visual
            .SetRelativeSizeAdjustment(Vector2 { X: 1.0, Y: 1.0 })
            .unwrap();
        target.SetRoot(&self.container_visual).unwrap();

        self.window = Some(window);
        self.target = Some(target);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
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
    pub fn add_element(&self) -> Result<()> {
        let window_size = self.window.as_ref().unwrap().inner_size();
        let mut rng = rand::rng();
        let size = rng.random_range(50..150);
        let offset_x = rng.random_range(0..window_size.width - size);
        let offset_y = rng.random_range(0..(window_size.height / 2) - size);
        let visual = self.compositor.CreateSpriteVisual()?;

        visual.SetSize(Vector2 {
            X: size as f32,
            Y: size as f32,
        })?;
        visual.SetBrush(
            &self
                .compositor
                .CreateColorBrushWithColor(get_random_color())?,
        )?;
        visual.SetOffset(Vector3 {
            X: offset_x as f32,
            Y: offset_y as f32,
            Z: 0.0,
        })?;
        self.container_visual.Children()?.InsertAtTop(&visual)?;
        self.animate_square(&visual)?;

        Ok(())
    }

    fn animate_square(&self, visual: &SpriteVisual) -> Result<()> {
        let window_size = self.window.as_ref().unwrap().inner_size();
        let offset_x = visual.Offset()?.X;
        let animation = self.compositor.CreateVector3KeyFrameAnimation()?;
        let bottom = window_size.height as f32 - visual.Size()?.Y;

        animation.InsertKeyFrame(
            1.0,
            Vector3 {
                X: offset_x,
                Y: bottom,
                Z: 0.0,
            },
        )?;
        animation.SetDuration(Duration::from_secs(2).into())?;
        animation.SetDelayTime(Duration::from_secs(3).into())?;
        visual.StartAnimation(h!("Offset"), &animation)?;

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

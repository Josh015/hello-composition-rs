fn main() {
    windows::build!(
        windows::foundation::numerics::{Vector2, Vector3}
        windows::system::DispatcherQueueController
        windows::ui::composition::{
            Compositor,
            ContainerVisual,
            SpriteVisual,
        }
        windows::ui::composition::desktop::DesktopWindowTarget
        windows::ui::Color
        windows::win32::system_services::CreateDispatcherQueueController
        windows::win32::windows_and_messaging::HWND
        windows::win32::winrt::{ICompositorDesktopInterop, RoInitialize}
    );
}
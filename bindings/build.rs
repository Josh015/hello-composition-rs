winrt::build!(
    dependencies
        os
    types
        windows::foundation::numerics::{Vector2, Vector3}
        windows::foundation::TimeSpan
        windows::system::DispatcherQueueController
        windows::ui::composition::{
            Compositor,
            ContainerVisual,
            SpriteVisual,
        }
        windows::ui::composition::desktop::DesktopWindowTarget
        windows::ui::Color
);

fn main() {
    build();
}

use bindings::windows::{
    foundation::{
        numerics::{Vector2, Vector3},
        TimeSpan,
    },
    ui::{
        composition::{Compositor, ContainerVisual, SpriteVisual},
        Color,
    },
};
use rand::{
    distributions::{Distribution, Uniform},
    prelude::*,
};
use std::time::Duration;

pub struct CompositionHost {
    container_visual: ContainerVisual,
    compositor: Compositor,
    parent_size: Vector2,
}

impl CompositionHost {
    pub fn new(container_visual: &ContainerVisual, parent_size: &Vector2) -> winrt::Result<Self> {
        let result = Self {
            container_visual: container_visual.clone(),
            compositor: container_visual.compositor()?.clone(),
            parent_size: parent_size.clone(),
        };

        Ok(result)
    }

    pub fn on_pointer_pressed(
        &mut self,
        _is_right_button: bool,
        _is_eraser: bool,
    ) -> winrt::Result<()> {
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(50, 150);
        let offset_x = rng.gen_range(0, (self.parent_size.x as u32) - size);
        let offset_y = rng.gen_range(0, ((self.parent_size.y / 2.0) as u32) - size);
        self.add_element(size as f32, offset_x as f32, offset_y as f32)?;

        Ok(())
    }

    fn add_element(&self, size: f32, offset_x: f32, offset_y: f32) -> winrt::Result<()> {
        let visual = self.compositor.create_sprite_visual()?;
        visual.set_size(Vector2 { x: size, y: size })?;
        visual.set_brush(
            self.compositor
                .create_color_brush_with_color(self.get_random_color()?)?,
        )?;
        visual.set_offset(Vector3 {
            x: offset_x,
            y: offset_y,
            z: 0.0,
        })?;
        self.container_visual.children()?.insert_at_top(&visual)?;
        self.animate_square(&visual, 3)?;

        Ok(())
    }

    fn animate_square(&self, visual: &SpriteVisual, delay: u64) -> winrt::Result<()> {
        let offset_x = visual.offset()?.x;
        let animation = self.compositor.create_vector3_key_frame_animation()?;
        let bottom = self.parent_size.y - visual.size()?.y;

        animation.insert_key_frame(
            1.0,
            Vector3 {
                x: offset_x,
                y: bottom,
                z: 0.0,
            },
        )?;
        animation.set_duration(TimeSpan::from(Duration::from_secs(2)))?;
        animation.set_delay_time(TimeSpan::from(Duration::from_secs(delay)))?;
        visual.start_animation("Offset", animation)?;

        Ok(())
    }

    fn get_random_color(&self) -> winrt::Result<Color> {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(0..=255);
        let r = die.sample(&mut rng);
        let g = die.sample(&mut rng);
        let b = die.sample(&mut rng);

        Ok(Color { r, g, b, a: 255 })
    }
}

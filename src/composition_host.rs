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
    width: u32,
    height: u32,
}

impl CompositionHost {
    pub fn new(container_visual: &ContainerVisual, width: u32, height: u32) -> winrt::Result<Self> {
        Ok(Self {
            container_visual: container_visual.clone(),
            compositor: container_visual.compositor()?.clone(),
            width,
            height,
        })
    }

    pub fn add_element(&self) -> winrt::Result<()> {
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(50, 150);
        let offset_x = rng.gen_range(0, self.width - size);
        let offset_y = rng.gen_range(0, (self.height / 2) - size);
        let visual = self.compositor.create_sprite_visual()?;

        visual.set_size(Vector2 {
            x: size as f32,
            y: size as f32,
        })?;
        visual.set_brush(
            self.compositor
                .create_color_brush_with_color(self.get_random_color())?,
        )?;
        visual.set_offset(Vector3 {
            x: offset_x as f32,
            y: offset_y as f32,
            z: 0.0,
        })?;
        self.container_visual.children()?.insert_at_top(&visual)?;
        self.animate_square(&visual)?;

        Ok(())
    }

    fn animate_square(&self, visual: &SpriteVisual) -> winrt::Result<()> {
        let offset_x = visual.offset()?.x;
        let animation = self.compositor.create_vector3_key_frame_animation()?;
        let bottom = self.height as f32 - visual.size()?.y;
        let duration = Duration::from_secs(2);
        let delay = Duration::from_secs(3);

        animation.insert_key_frame(
            1.0,
            Vector3 {
                x: offset_x,
                y: bottom,
                z: 0.0,
            },
        )?;
        animation.set_duration(TimeSpan::from(duration))?;
        animation.set_delay_time(TimeSpan::from(delay))?;
        visual.start_animation("Offset", animation)?;

        Ok(())
    }

    fn get_random_color(&self) -> Color {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(0..=255);
        let r = die.sample(&mut rng);
        let g = die.sample(&mut rng);
        let b = die.sample(&mut rng);

        Color { a: 255, r, g, b }
    }
}

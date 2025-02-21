use rand::{distr::Uniform, prelude::*};
use std::time::Duration;
use windows::{
    Foundation::Numerics::{Vector2, Vector3},
    UI::{
        Color,
        Composition::{Compositor, ContainerVisual, SpriteVisual},
    },
    core::{Result, h},
};

pub struct CompositionHost {
    container_visual: ContainerVisual,
    compositor: Compositor,
    width: u32,
    height: u32,
}

impl CompositionHost {
    pub fn new(
        container_visual: &ContainerVisual,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        Ok(Self {
            container_visual: container_visual.clone(),
            compositor: container_visual.Compositor()?.clone(),
            width,
            height,
        })
    }

    pub fn add_element(&self) -> Result<()> {
        let mut rng = rand::rng();
        let size = rng.random_range(50..150);
        let offset_x = rng.random_range(0..self.width - size);
        let offset_y = rng.random_range(0..(self.height / 2) - size);
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
        let offset_x = visual.Offset()?.X;
        let animation = self.compositor.CreateVector3KeyFrameAnimation()?;
        let bottom = self.height as f32 - visual.Size()?.Y;

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

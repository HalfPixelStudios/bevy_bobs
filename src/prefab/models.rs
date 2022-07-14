use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
pub struct ColorRGB {
    r: f32,
    g: f32,
    b: f32,
}

impl From<ColorRGB> for Color {
    fn from(c: ColorRGB) -> Self {
        Color::rgb(c.r, c.g, c.b)
    }
}

#[derive(Deserialize, Clone, Copy)]
pub struct ColorRGBA {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl From<ColorRGBA> for Color {
    fn from(c: ColorRGBA) -> Self {
        Color::rgba(c.r, c.g, c.b, c.a)
    }
}

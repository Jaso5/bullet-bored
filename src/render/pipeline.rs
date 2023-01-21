use macroquad::{window::InternalGlContext, prelude::Camera3D};

use super::{Renderable, base::BaseUniform};
use std::rc::Weak;

pub trait Pipeline<U> {
    fn add_object(&mut self, object: Weak<dyn Renderable<U>>);
    fn render(&mut self, gl: &mut InternalGlContext, dt: f32, camera: &Camera3D);
}

pub enum PipelineObject {
    Base ( Weak<dyn Renderable<BaseUniform>> )
}
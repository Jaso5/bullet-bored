use macroquad::{window::InternalGlContext, prelude::Camera3D};
use miniquad::Bindings;

use self::{base::BasePipeline, pipeline::{Pipeline, PipelineObject}};

pub mod base;
mod pipeline;


pub struct Renderer {
    base: base::BasePipeline   
}

impl Renderer {
    pub fn new(ctx: &mut miniquad::GraphicsContext) -> Self {
        Self {
            base: BasePipeline::new(ctx)
        }
    }
    
    pub fn add_object<U>(&mut self, object: PipelineObject) {
        match object {
            PipelineObject::Base(obj) => self.base.add_object(obj),
        }
    }
    
    pub fn render(&mut self, gl: &mut InternalGlContext, dt: f32, camera: &Camera3D) {
        self.base.render(gl, dt, &camera);
    }
}



pub trait Renderable<U> {
    fn uniforms(&self) -> &U;
    fn render_info(&self) -> &RenderInfo;
}

pub struct RenderInfo<'a> {
    pub bindings: &'a Bindings,
    pub base_element: i32,
    pub num_elements: i32,
    pub num_instances: i32,
}
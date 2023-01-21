use std::{fs, rc::{Weak, self}, sync::RwLock};

use macroquad::{prelude::{Vec2, Vec4, Mat4, Camera3D, Camera}, window::InternalGlContext, rand};
use miniquad::{Pipeline, Shader, ShaderMeta, UniformBlockLayout, UniformDesc, UniformType, ShaderError, BufferLayout, VertexAttribute, VertexFormat, PipelineParams};

use crate::util::crash_and_burn;

use super::{Renderable, pipeline};

pub struct BasePipeline {
    pipeline: Pipeline,
    objects: Vec<Weak<RwLock<dyn Renderable<BaseUniform>>>>,
    t: f32,
    buzz_r: u32,
    buzz_mag: f32
}

impl BasePipeline {
    pub fn new(ctx: &mut miniquad::GraphicsContext) -> Self {
        let vtx_shader = fs::read_to_string("content/builtin/shader/player_vtx.glsl").unwrap();
        let frg_shader = fs::read_to_string("content/builtin/shader/board_frg.glsl").unwrap();
        
        let shader = match Shader::new(ctx, &vtx_shader, &frg_shader, shader_meta()) {
            Ok(shader) => shader,
            Err(e) => {
                match e {
                    ShaderError::CompilationError { shader_type, error_message } => {
                        println!("{:?} Shader compilation error: \n{}", shader_type, error_message)
                    }
                    e => crash_and_burn::crash_and_burn(e.into())
                }
                
                panic!()
            }
        };
        let pipeline = Pipeline::with_params(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2)
            ],
            shader,
            PipelineParams {
                primitive_type: miniquad::PrimitiveType::Lines,
                ..Default::default()
            }
        );
        
        
        Self {
            pipeline,
            objects: Vec::new(),
            t: 0.0,
            buzz_r: 0,
            buzz_mag: 0.0025
        }
    }
    
    fn render_obj(gl: &mut InternalGlContext, buzz_r: u32, buzz_mag: f32, obj: rc::Rc<RwLock<dyn Renderable<BaseUniform>>>, camera: &Camera3D) {
        let obj = obj.read().unwrap();
        
        let info = obj.render_info();
        let external_uniforms = obj.uniforms();
        
        let internal_uniforms = InternalUniform::from_external(
            external_uniforms,
            camera,
            buzz_mag,
            buzz_r
        );
        
        gl.quad_context.apply_bindings(&info.bindings);
        gl.quad_context.apply_uniforms(&internal_uniforms);
        gl.quad_context.draw(info.base_element, info.num_elements, info.num_instances);
    }
}

impl pipeline::Pipeline<BaseUniform> for BasePipeline {
    fn add_object(&mut self, object: Weak<RwLock<dyn Renderable<BaseUniform>>>) {
        self.objects.push(object);
    }
    
    fn render(&mut self, gl: &mut InternalGlContext, dt: f32, camera: &Camera3D) {
        self.t += dt;
        
        if self.t >= 70.0 {
            self.buzz_r = rand::gen_range(0, 256);
            self.t = 0.0;
        }
        
        let mut count = 0;
    
        gl.quad_context.apply_pipeline(&self.pipeline);
        
        self.objects
            .drain_filter(|ptr| {
                count += 1;
                match ptr.upgrade() {
                    Some(obj) => {
                        Self::render_obj(gl, self.buzz_r + count, self.buzz_mag, obj, camera);
                        
                        false
                    }
                    None => true
                }
            });
    }
}

#[repr(C)]
pub struct BaseVertex {
    pub pos: Vec2
}

pub struct BaseUniform {
    pub color: Vec4,
    
    pub model_matrix: Mat4,
}

#[repr(C)]
struct InternalUniform {
    color: Vec4,
    
    model_matrix: Mat4,
    view_matrix: Mat4,
    
    buzz_mag: f32,
    buzz_r: u32
}

impl InternalUniform {
    pub fn from_external(other: BaseUniform, camera: &Camera3D, buzz_mag: f32, buzz_r: u32) -> Self {
        Self {
            color: other.color,
            model_matrix: other.model_matrix,
            
            view_matrix: camera.matrix(),
            buzz_mag,
            buzz_r,
        }
    }
}

fn shader_meta() -> ShaderMeta {
    ShaderMeta {
        images: vec![],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("color", UniformType::Float4),
                
                UniformDesc::new("model_matrix", UniformType::Mat4),                
                UniformDesc::new("view_matrix", UniformType::Mat4),
                
                UniformDesc::new("buzz_mag", UniformType::Float1),
                UniformDesc::new("buzz_r", UniformType::Int1),                
            ]
        }
    }
}
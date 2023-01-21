use std::{f32::consts::PI};

use macroquad::{prelude::{Vec2, Vec4, Mat4, Vec3, Rect}};
use miniquad::{Bindings, Buffer, BufferType};

use crate::{math::{vec2_to_angle, ClampToRect}, render::{base::{BaseVertex, BaseUniform}, Renderable, RenderInfo}};


const ACCELERATION: f32 = 0.05;
const DEACCELERATION: f32 = 0.1;
const SPEED_CAP: f32 = 0.5;

pub struct Player {
    render_info: RenderInfo,
    
    pub pos: Vec2,
    pub facing: Vec2,
    pub target_direction: Vec2,
    pub scale: f32,
    pub momentum: Vec2
}

impl Player {
    pub fn new(ctx: &mut miniquad::GraphicsContext) -> Self {
        let vtxs = [
            BaseVertex { pos: Vec2::new(0.0, 7.0) },
            BaseVertex { pos: Vec2::new(5.0, 0.0) }, 
            BaseVertex { pos: Vec2::new(0.0, -5.0) },
            BaseVertex { pos: Vec2::new(-5.0, 0.0) },
            BaseVertex { pos: Vec2::new(7.5, -8.0) },
            BaseVertex { pos: Vec2::new(-7.5, -8.0) },
        ];
        
        let vtx_buf = Buffer::immutable(ctx, BufferType::VertexBuffer, &vtxs);
        
        let idxs: [u16; 8*2] = [
            0, 1,
            1, 2,
            2, 3,
            3, 0,
            1, 4,
            4, 2,
            2, 5,
            5, 3
        ];
        
        let idx_buf = Buffer::immutable(ctx, BufferType::IndexBuffer, &idxs);
        
        let bindings = Bindings {
            vertex_buffers: vec![vtx_buf],
            index_buffer: idx_buf,
            images: vec![]
        };
        
        let render_info = RenderInfo {
            bindings,
            base_element: 0,
            num_elements: 16,
            num_instances: 1
        };
        
        Self {
            render_info,
            
            pos: Vec2::new(50.0, 50.0),
            facing: Vec2::new(0.0, 1.0),
            target_direction: Vec2::new(0.0, 1.0),
            scale: 0.5,
            momentum: Vec2::ZERO
        }
    }    
    
    pub fn movement(&mut self, vec: Vec2, mut bounds: Rect) {
        if vec.length_squared() == 0.0 {
            self.momentum *= DEACCELERATION;
        } else {
            self.momentum += vec * ACCELERATION;
            self.target_direction = vec.normalize();
        }
        self.momentum = self.momentum.clamp_length(0.0, SPEED_CAP);
        self.pos += self.momentum;
        
        // Correct bounds for size of player
        bounds.x += 4.0;
        bounds.y += 4.0;
        bounds.w -= 8.0;
        bounds.h -= 8.0;
        
        self.pos.clamp_inside_rect(bounds);
        
        let angle = vec2_to_angle(self.facing);
        
        let difference = self.facing.angle_between(self.target_direction);
        
        let turn_angle = difference / 16.0;
        
        self.facing = Vec2::from_angle(angle + turn_angle);
    }
}

impl Renderable<BaseUniform> for Player {
    fn uniforms(&self) -> BaseUniform {
        BaseUniform {
            color: Vec4::ONE,
            model_matrix: 
                Mat4::from_translation(self.pos.extend(0.0))
                // Correct by 90 deg because model faces upwards
                * Mat4::from_rotation_z(vec2_to_angle(self.facing) - (PI/2.0))
                * Mat4::from_scale(Vec3::splat(self.scale)),
        }
    }
    
    fn render_info(&self) -> &RenderInfo {
        &self.render_info
    }
}

// const buzz: [u8; 256] = [186, 139, 54, 199, 28, 107, 173, 232, 33, 224, 249, 81, 229, 203, 41, 202, 147, 209, 184, 148, 53, 155, 186, 255, 222, 38, 201, 33, 110, 128, 129, 248, 77, 77, 114, 15, 253, 206, 215, 87, 205, 91, 56, 135, 183, 68, 144, 175, 163, 25, 202, 225, 224, 72, 72, 75, 174, 232, 45, 120, 239, 89, 206, 200, 205, 244, 18, 95, 148, 147, 215, 158, 150, 86, 205, 236, 239, 42, 173, 174, 242, 127, 242, 103, 196, 73, 142, 77, 81, 217, 116, 9, 156, 156, 210, 65, 247, 224, 171, 148, 210, 102, 173, 59, 215, 51, 88, 103, 100, 183, 100, 81, 90, 65, 235, 73, 148, 181, 174, 215, 64, 144, 10, 74, 225, 42, 229, 153, 195, 86, 95, 37, 240, 26, 198, 127, 107, 256, 123, 50, 205, 184, 210, 97, 211, 187, 104, 199, 57, 124, 220, 127, 133, 11, 44, 55, 92, 15, 86, 194, 9, 152, 28, 181, 68, 65, 200, 54, 109, 30, 138, 38, 160, 67, 186, 142, 39, 2, 93, 63, 249, 170, 14, 108, 106, 67, 43, 222, 204, 242, 63, 175, 129, 6, 242, 165, 137, 15, 246, 152, 97, 256, 48, 197, 197, 74, 215, 105, 239, 91, 94, 220, 230, 205, 198, 186, 29, 37, 183, 33, 191, 222, 241, 112, 231, 113, 134, 55, 218, 186, 63, 230, 103, 237, 39, 62, 126, 94, 32, 200, 112, 176, 32, 202, 52, 71, 140, 5, 119, 157, 183, 218, 105, 189, 140, 192];

#![feature(drain_filter)]

mod math;
mod scripting;
mod game;
mod util;
mod render;

use std::{time::Instant, rc::{Rc, Weak}, sync::RwLock};

use game::{board::Board, bullet::BulletType};
use macroquad::prelude::*;
use game::player::Player;
use render::{Renderer, pipeline::PipelineObject, Renderable, base::BaseUniform};

#[macroquad::main("Bullet Bored")]
async fn main() {
    let cam = Camera3D {
        // position: Vec3::new(50.0, -50.0, 75.0),
        position: Vec3::new(50.0, 50.0, 150.0),
        target: Vec3::new(50.0, 50.0, 0.0),
        up: Vec3::new(0.0,1.0,0.0),
        fovy: 70.0,
        projection: Projection::Perspective,
        ..Default::default()
    };

    
    let (board, player, mut renderer) = {
        let InternalGlContext {
            quad_context: ctx, ..
        } = unsafe { get_internal_gl() };

        (Board::new(ctx), Player::new(ctx), Renderer::new(ctx))
    };
    
    let player = Rc::new(RwLock::new(player));
    let board = Rc::new(RwLock::new(board));
    
    renderer.add_object::<BaseUniform>(PipelineObject::Base(
        Rc::downgrade(&player) as Weak<RwLock<dyn Renderable<BaseUniform>>>
    ));
    renderer.add_object::<BaseUniform>(PipelineObject::Base(
        Rc::downgrade(&board) as Weak<RwLock<dyn Renderable<BaseUniform>>>
    ));
    
    // let test = BulletType::init(String::from("basic"), true);
    
    let mut prev_time = Instant::now();
    
    loop {
        let now = Instant::now();
        let dt = now.duration_since(prev_time).as_millis() as f32;
        prev_time = now;
        
        {
            let mut player = player.write().unwrap();
            
            let axis = get_movement_vec();
        
            player.movement(axis, board.read().unwrap().bounds);
        }
        
        clear_background(BLACK);
        
            let mut gl = unsafe { get_internal_gl() };

            // Ensure that macroquad's shapes are not going to be lost
            gl.flush();

         
            renderer.render(&mut gl, dt, &cam);
            
            
            draw_text(get_fps().to_string().as_str(), 20.0, 20.0, 20.0, BLUE);
            
            // visualize_vec(Vec2::new(50.0, 50.0), player.target_direction * 50.0, RED);
            // visualize_vec(Vec2::new(50.0, 50.0), player.facing * 50.0, BLUE);
            // visualize_vec(Vec2::new(50.0, 50.0), temp * 50.0, GREEN);
        
        next_frame().await
    }
}

fn get_movement_vec() -> Vec2 {
    let mut axis = Vec2::ZERO;
    
    if is_key_down(KeyCode::Up) {
        axis.y += 1.0;
    }
        if is_key_down(KeyCode::Down) {
        axis.y -= 1.0;
    }
        if is_key_down(KeyCode::Left) {
        axis.x -= 1.0;
    }
        if is_key_down(KeyCode::Right) {
        axis.x += 1.0;
    }
    
    axis.normalize_or_zero()
}
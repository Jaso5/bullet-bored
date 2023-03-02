use macroquad::prelude::Vec2;
use rhai::{Scope, Engine, AST, Map};

use crate::{util::crash_and_burn::{crash_and_burn, FinalError}, scripting::util::pull_fatal};

use super::BulletInfo;

#[allow(dead_code)]
pub struct BulletInstance {
    pos: Vec2,
    scale: Vec2,
    rot: f32,
    scope: Scope<'static> // Contains positioning, and program state
}

impl BulletInstance {
    fn init(pos: Vec2, scale: Vec2, rot: f32, ast: &AST) -> Self {
        let mut scope = Scope::new();
        
        let engine = Engine::new_raw();
        
        engine.eval_ast_with_scope::<()>(&mut scope, ast).unwrap();
        
        Self {
            pos,
            scale,
            rot,
            scope,
        }
    }
    
    fn tick(&mut self, engine: &mut Engine, ast: &AST, world: Map, info: &BulletInfo) -> bool {
        self.scope.push_constant("world", world);
        
        let res = engine.call_fn::<bool>(&mut self.scope, ast, "tick", ()).unwrap_or(false);
        
        self.pos = pull_fatal::<[f32; 2]>("pos", &self.scope, info).into();
        self.rot = pull_fatal::<f32>("rot", &self.scope, info).into();
        self.scale = pull_fatal::<[f32; 2]>("scale", &self.scope, info).into();

        res
    }
    
    
}
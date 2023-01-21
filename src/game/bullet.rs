use macroquad::prelude::Vec2;
use rhai::{Scope, AST, Map, Engine};

use crate::util::crash_and_burn::{crash_and_burn, FinalError};

// use super::world::WorldState;

// Bullets have a global scope
pub struct BulletType {
    ast: AST,
    instances: Vec<BulletInstance>
}

pub struct BulletInstance {
    pos: Vec2,
    scale: Vec2,
    rot: f32,
    scope: Scope<'static>
}

impl BulletInstance {
    fn init(pos: Vec2, scale: Vec2, rot: f32, scope: Scope<'static>) -> Self {
        Self {
            pos,
            scale,
            rot,
            scope
        }
    }
    
    fn tick(&mut self, engine: &mut Engine, ast: &AST, world: Map) -> bool {
        self.scope.push_constant("world", world);
        
        let res = engine.call_fn::<bool>(&mut self.scope, ast, "tick", ()).unwrap_or(false);
        
        self.pos = self.scope.get_value::<[f32; 2]>("pos").unwrap_or_else(|| { crash_and_burn(FinalError::Scripting) } ).into();
        self.rot = self.scope.get_value::<f32>("rot").unwrap_or_else(|| { crash_and_burn(FinalError::Scripting) } );
        self.scale = self.scope.get_value::<[f32; 2]>("scale").unwrap_or_else(|| { crash_and_burn(FinalError::Scripting) } ).into();
        
        res
    }
}
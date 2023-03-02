pub mod instance;


use std::{path::{PathBuf, self}, fs};

use rhai::{Scope, AST, Map, Engine};

use crate::util::crash_and_burn::{crash_and_burn, FinalError};

use self::instance::BulletInstance;

// use super::world::WorldState;

pub struct BulletType {
    ast: AST,
    instances: Vec<BulletInstance>,
    info: BulletInfo
}

pub struct BulletInfo {
    pub name: String
}

impl BulletType {
    pub fn init(name: String, is_builtin: bool) -> Result<Self, String> {
        let mut path = PathBuf::from("content/");
        
        path.push(match is_builtin {
            true => "builtin",
            false => "mods"
        });
        
        path.push(format!("bullets/{}.bullet.rhai", name));
        
        match path.exists() {
            false => Err(format!("Bullet type {} does not exist", name)),
            true => {
                // let script = fs::read_to_string(path);
                let engine = Engine::new();
                
                let ast = engine.compile_file(path).unwrap();
                
                let info = BulletInfo {
                    name,
                };
                
                Ok(Self{
                    ast,
                    instances: Vec::new(),
                    info,
                })
            }
        }
    }
}
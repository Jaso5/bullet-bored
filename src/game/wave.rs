// use std::{path::PathBuf, fs::File};

// use miniquad::info;
// use rhai::{Scope};

// use crate::util::RhaiType;



// pub struct Wave {
//     program: RhaiType,
//     scope: Scope<'static>
// }

// impl Wave {
//     pub fn load(name: String, is_mod: bool) -> Option<Self> {
//         let base_path = match is_mod {
//             true => PathBuf::from("content/builtin/waves"),
//             false => PathBuf::from("content/mods/waves")
//         };
        
//         let path = base_path.join(&name).join("wave.rhai");
        
//         if !path.exists() {
//             info!("Wave does not exist: {}", name);
//             return None
//         }
        
//         // Unwrap should be safe since we check if the file exists first
//         let file = File::open(path).unwrap();
        
        
        
//         None
//     }
    
//     pub fn tick() -> Vec<u32> {
        
        
//         todo!()
//     }
// }
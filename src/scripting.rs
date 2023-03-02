pub mod vec_interop;
pub mod validation;
pub mod error;
pub mod util;

// use rhai::{AST, Engine, plugin::RhaiResult, Scope};

// pub enum RhaiType {
//     AST (AST),
//     Uncompiled (String)
// }

// impl RhaiType {
//     pub fn execute<T>(&self, engine: &mut Engine, scope: Option<&'static mut Scope>, func: &str) -> T {
//         let result = match (self, scope) {
//             (Self::Uncompiled(script), None) => engine.call_fn(&script),
//             (Self::Uncompiled(script), Some(scope)) => engine.run_with_scope(scope, &script),
//             (Self::AST(ast), None) => engine.run_ast(ast),
//             (Self::AST(ast), Some(scope)) => engine.run_ast_with_scope(scope, ast)
//         };
        
        
        
//         todo!()
//     }
// }
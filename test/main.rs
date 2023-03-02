use std::fs;

use rhai::{Scope, Engine, Map};

fn main() {
    let mut scope = Scope::new();
    // let mut state = Map::new();
    let engine = Engine::new();
    
    let script =
    r#"
        let x = 0;
        
        fn bar() {
            x = x + 1;
            x
        }
    "#;
 
    let ast = engine.compile_with_scope(&scope, script).unwrap();
    
    scope.push("x", 0_i64);

    // engine.call_fn::<i64>(&mut scope, &ast, "bar", ()).unwrap();
    // engine.call_fn::<i64>(&mut scope, &ast, "bar", ()).unwrap();
    // engine.call_fn::<i64>(&mut scope, &ast, "bar", ()).unwrap();
        
        
            
    println!("{}", engine.call_fn::<i64>(&mut scope, &ast, "bar", ()).unwrap());
    println!("{}", engine.call_fn::<i64>(&mut scope, &ast, "bar", ()).unwrap());
    println!("{}", engine.call_fn::<i64>(&mut scope, &ast, "bar", ()).unwrap());
    println!("{}", engine.call_fn::<i64>(&mut scope, &ast, "bar", ()).unwrap());
}
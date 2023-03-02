use rhai::AST;

use super::error::ScriptError;

pub fn validate_bullet(ast: &AST, path: String) -> Result<(), Vec<ScriptError>> {
    let mut errors = Vec::new();
    
    let mut iter = ast.iter_functions();
    
    let mut funcs: [(&str, bool, Vec<&str>, bool); 2] = [
        ("init", false, vec![], false),
        ("tick", false, vec![], false)
    ];
    
    // Check for functions
    for (name, found, args, mut args_correct) in funcs.iter_mut() {
        if let Some(func) = iter.find(|item| item.name == *name) {
            // Check function args
            if func.params == *args { args_correct = true }
            else {
                errors.push(ScriptError::IncompatibleFunc {
                    name: String::from(*name),
                    expected: args.iter().map(|s| s.to_string()).collect(),
                    got: func.params.iter().map(|s| s.to_string()).collect(),
                    file: path.clone()
                })
            }
            
            *found = true;
        }
    }
    
    // 
    errors.append(&mut
        funcs.iter()
            .filter(|item| !item.1)
            .map(|item| ScriptError::MissingFunc { name: item.0, file: path.clone() })
            .collect()
        );
    
    Ok(())
}


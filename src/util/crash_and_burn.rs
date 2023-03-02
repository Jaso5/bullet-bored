use miniquad::ShaderError;
use native_dialog::MessageDialog;

use crate::scripting::error::ScriptError;


pub fn crash_and_burn(error: FinalError) -> ! {
    MessageDialog::new()
        .set_type(native_dialog::MessageType::Error)
        .set_title(error.get_error_type())
        .set_text(error.get_error_text())
        .show_alert()
        .unwrap();
    
    panic!();
}

pub enum FinalError {
    Scripting { reason: String },
    Shader { reason: String },
    
    Unknown
}

impl FinalError {
    pub fn get_error_type(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match self {
            Self::Scripting { reason: _ }=> "Script Error",
            Self::Unknown => "Unknown Error",
            Self::Shader { reason: _} => "Shader Error: ",
            _ => "Unimplemented Error"
        }
    }
    
    pub fn get_error_text(&self) -> &str {
        match self {
            Self::Shader { reason } => reason,
            _ => "Unimplemented Dialogue"
        }
    }
}

impl From<ShaderError> for FinalError {
    fn from(err: ShaderError) -> Self {
        Self::Shader { reason: err.to_string() }
    }
}

impl From<ScriptError> for FinalError {
    fn from(err: ScriptError) -> Self {
        match err {
            ScriptError::MissingFunc { name, file } => {
                Self::Scripting { reason: format!("Expected function {name}\nSouce: {file}") }
            },
            ScriptError::IncompatibleFunc { name, expected, got, file } => {
                Self::Scripting { reason: format!("Function {name} is incompatible\nExpected: {expected:?}\nGot: {got:?}\nSource: {file}") }
            },
            ScriptError::BadValue { name, file } => {
                Self::Scripting { reason: format!("Value {name} is void/incorrect type\nSource: {file}") }
            }
        }
    }
}
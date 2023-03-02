pub enum ScriptError {
    MissingFunc { name: &'static str, file: String },
    IncompatibleFunc {
        name: String,
        expected: Vec<String>,
        got: Vec<String>,
        file: String
    },
    BadValue {
        name: String,
        file: String
    }
}

impl ScriptError {
    // fn to_string(self) -> String {
    //     match self {
    //         Self::MissingFunc { name, file } => format!("Missing function: {name:?}\nSource: {file}"),
    //         Self::IncompatibleFunc { name, expected, got, file } => {
    //             format!("Incompatible Function")
    //         }
    //         Self::BadValue { name, file } =>             
    //     }
    // }
}
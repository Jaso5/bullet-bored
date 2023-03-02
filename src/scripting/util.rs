use rhai::Scope;

use crate::{util::crash_and_burn::crash_and_burn, game::bullet::BulletInfo};

use super::error::ScriptError;

pub fn pull_fatal<T: Clone + 'static>(name: &str, scope: &Scope, info: &BulletInfo) -> T {
    scope.get_value::<T>(name).unwrap_or_else(|| {
        let error = ScriptError::BadValue {
            name: name.to_string(),
            file: info.name.clone()
        };
        
        crash_and_burn(error.into())
    })
}
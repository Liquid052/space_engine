use crate::components::naming::UniquelyNamed;
use bevy::core::Name;
use bevy::prelude::{default, Bundle};
use std::borrow::Cow;

#[derive(Bundle, Default)]
pub struct NameBundle {
    pub name:   Name,
    pub marker: UniquelyNamed
}

impl NameBundle {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: Name::new(name),
            ..default()
        }
    }
}

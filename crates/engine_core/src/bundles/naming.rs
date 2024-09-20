use std::borrow::Cow;
use bevy::core::Name;
use bevy::prelude::{Bundle, default};
use crate::components::UniquelyNamed;

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

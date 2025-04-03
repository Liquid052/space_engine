use bevy::prelude::*;
use crate::prelude::*;


#[derive(SystemSet, Default, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct NamingSet;


pub(crate) fn update_unique_names(
    names: Query<(Entity, &Name), Changed<Name>>,
    mut names_reg: ResMut<NameReg>
    ) {

    names.iter().for_each(|(entity, name)| {
        names_reg.update(entity, name.as_ref());
    });

}

pub(crate) fn check_removed_components(
    mut removed: RemovedComponents<Name>,
    mut names_reg: ResMut<NameReg>
) {
    for entity in removed.read() {
        names_reg.remove(entity);
    }
}
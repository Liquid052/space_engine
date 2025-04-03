use bevy::prelude::*;
use engine_core::prelude::Running;
pub use last::*;
pub use orbits::*;
pub use physics::*;
pub use post_update::*;
pub use run_conditions::*;
pub use transfer::*;
pub(crate) use update::*;

mod last;
mod post_update;
mod run_conditions;
mod update;

/// System sets defining the order of execution for the space plugin.
#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpaceSystemSet {
    /// Update
    Physics,
    /// Update
    Transfers,

    /// PostUpdate
    InstallDraw,
    /// PostUpdate
    UpdateDraw,
    /// PostUpdate
    Camera,
}

pub(crate) fn config_set(app: &mut App) {
    // Update
    app.configure_sets(
        Update,
        SpaceSystemSet::Transfers.after(SpaceSystemSet::Physics),
    );

    // Post-update
    app.configure_sets(
        PostUpdate,
        SpaceSystemSet::UpdateDraw.after(SpaceSystemSet::InstallDraw),
    )
    .configure_sets(
        PostUpdate,
        SpaceSystemSet::Camera.after(SpaceSystemSet::UpdateDraw),
    );
}

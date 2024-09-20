use bevy::prelude::*;
use core_assets::prelude::*;



#[derive(SubStates, Reflect, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(BuildingStates = BuildingStates::Finished)]
pub enum AppState {
    #[default]
    Menu,
    Loading,
    InGame {
        paused: bool,
    }
}



// Computed states for easier
#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub struct InGame;

impl ComputedStates for InGame {
    // Computed states can be calculated from one or many source states.
    type SourceStates = AppState;

    // Now, we define the rule that determines the value of our computed state.
    fn compute(sources: AppState) -> Option<InGame> {
        match sources {
            AppState::InGame { .. } => Some(InGame),
            _ => None,
        }
    }
}



#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub struct Running;

impl ComputedStates for Running {
    // Computed states can be calculated from one or many source states.
    type SourceStates = AppState;

    // Now, we define the rule that determines the value of our computed state.
    fn compute(sources: AppState) -> Option<Running> {
        match sources {
            AppState::InGame { paused: true } => Some(Running),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub struct Paused;

impl ComputedStates for Paused {
    // Computed states can be calculated from one or many source states.
    type SourceStates = AppState;

    // Now, we define the rule that determines the value of our computed state.
    fn compute(sources: AppState) -> Option<Paused> {
        match sources {
            AppState::InGame { paused: false } => Some(Paused),
            _ => None,
        }
    }
}

/// Todo document. For type-state pattern
pub mod plugins {
    pub struct Building;
    pub struct Finished;

    mod private {
        pub trait Sealed {}
    }

    pub trait PluginState: private::Sealed {}

    impl PluginState for Building {}
    impl PluginState for Finished {}

    impl private::Sealed for Building {}
    impl private::Sealed for Finished {}
}

use bevy::prelude::*;
use core_assets::prelude::*;



/// Enum representing the various states of the application.
#[derive(SubStates, Reflect, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(BuildingStates = BuildingStates::Finished)]
pub enum AppState {
    #[default]
    Menu,            // State representing the main menu.
    Loading,         // State representing the loading screen.
    InGame {         // State representing the gameplay.
        paused: bool, // Indicates if the game is currently paused.
    }
}

/// Computed state representing that the game is currently in the "InGame" state.
#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub struct InGame;

impl ComputedStates for InGame {
    // Computed states can be calculated from one or many source states.
    type SourceStates = AppState;

    // Defines the rule that determines the value of our computed state.
    fn compute(sources: AppState) -> Option<InGame> {
        match sources {
            AppState::InGame { .. } => Some(InGame), // Returns Some(InGame) if the AppState is InGame.
            _ => None, // Otherwise, returns None.
        }
    }
}

/// Computed state representing that the game is currently running.
#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub struct Running;

impl ComputedStates for Running {
    // Computed states can be calculated from one or many source states.
    type SourceStates = AppState;

    // Defines the rule that determines the value of our computed state.
    fn compute(sources: AppState) -> Option<Running> {
        match sources {
            AppState::InGame { paused: false } => Some(Running), // Returns Some(Running) if the game is not paused.
            _ => None, // Otherwise, returns None.
        }
    }
}

/// Computed state representing that the game is currently paused.
#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub struct Paused;

impl ComputedStates for Paused {
    // Computed states can be calculated from one or many source states.
    type SourceStates = AppState;

    // Defines the rule that determines the value of our computed state.
    fn compute(sources: AppState) -> Option<Paused> {
        match sources {
            AppState::InGame { paused: true } => Some(Paused), // Returns Some(Paused) if the game is paused.
            _ => None, // Otherwise, returns None.
        }
    }
}


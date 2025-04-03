use bevy::prelude::*;

/// Enum representing the various states of the asset loading process.
#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Reflect, Debug)]
pub enum LoadingStates {
    #[default]
    CoreAssets,      // Initial state for loading core assets.
    ModIndex,        // State for loading mod indexes.
    ModsMeta,        // State for loading metadata about mods.
    ContentLoading,  // State for loading additional content.
    ContentProcessing, // State for processing assets which could be added later in development.
    Finished         // Indicates that the loading process is complete.
}

/// Enum representing the states related to building processes.
#[derive(SubStates, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
#[source(LoadingStates = LoadingStates::Finished)]
pub enum BuildingStates {
    #[default]
    Building,        // The state during the building process.
    Finished         // Indicates that the building process is complete.
}

/// Computed state representing that the content has been fully loaded.
#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub struct ContentLoaded;

impl ComputedStates for ContentLoaded {
    // Computed states can be calculated from one or many source states.
    type SourceStates = LoadingStates;

    // Defines the rule that determines the value of our computed state.
    fn compute(sources: LoadingStates) -> Option<ContentLoaded> {
        match sources {
            LoadingStates::Finished => Some(ContentLoaded),
            _ => None,
        }
    }
}

/// Computed state representing that the building process is finished.
#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub struct BuildFinished;

impl ComputedStates for BuildFinished {
    // Computed states can be calculated from one or many source states.
    type SourceStates = BuildingStates;

    // Defines the rule that determines the value of our computed state.
    fn compute(sources: BuildingStates) -> Option<BuildFinished> {
        match sources {
            BuildingStates::Finished => Some(BuildFinished),
            _ => None,
        }
    }
}

/// Computed state representing the loading process.
#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub struct Loading;

impl ComputedStates for Loading {
    // Computed states can be calculated from one or many source states.
    type SourceStates = LoadingStates;

    // Defines the rule that determines the value of our computed state.
    fn compute(sources: LoadingStates) -> Option<Loading> {
        match sources {
            LoadingStates::Finished   => None,
            LoadingStates::CoreAssets => None,
            _ => Some(Loading),
        }
    }
}
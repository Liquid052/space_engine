use bevy::prelude::*;

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Reflect, Debug)]
pub enum LoadingStates {
    #[default]
    CoreAssets,
    ModIndex,
    ModsMeta,
    ContentLoading,
    // for processing of assets which could be added later in development
    ContentProcessing,
    Finished
}

#[derive(SubStates, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
#[source(LoadingStates = LoadingStates::Finished)]
pub enum BuildingStates {
    #[default]
    Building,
    Finished
}

// Computed states
#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub struct ContentLoaded;

impl ComputedStates for ContentLoaded {
    // Computed states can be calculated from one or many source states.
    type SourceStates = LoadingStates;

    // Now, we define the rule that determines the value of our computed state.
    fn compute(sources: LoadingStates) -> Option<ContentLoaded> {
        match sources {
            LoadingStates::Finished => Some(ContentLoaded),
            _ => None,
        }
    }
}

#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub struct BuildFinished;

impl ComputedStates for BuildFinished {
    // Computed states can be calculated from one or many source states.
    type SourceStates = BuildingStates;

    // Now, we define the rule that determines the value of our computed state.
    fn compute(sources: BuildingStates) -> Option<BuildFinished> {
        match sources {
            BuildingStates::Finished => Some(BuildFinished),
            _ => None,
        }
    }
}


#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub struct Loading;

impl ComputedStates for Loading {
    // Computed states can be calculated from one or many source states.
    type SourceStates = LoadingStates;

    // Now, we define the rule that determines the value of our computed state.
    fn compute(sources: LoadingStates) -> Option<Loading> {
        match sources {
            LoadingStates::Finished   => None,
            LoadingStates::CoreAssets => None,
            _ => Some(Loading),
        }
    }
}
use bevy::prelude::*;

use crate::GameLoading;

use self::{
    copier::{despawn_copier, spawn_copier},
    houses::{despawn_houses, spawn_houses, spawn_player_houses},
    kitchen::{despawn_kitchen, spawn_kitchen, spawn_player_kitchen},
    shower::{despawn_shower, spawn_shower},
    urban::{despawn_urban, spawn_player_urban, spawn_urban},
};

pub mod copier;
pub mod houses;
pub mod kitchen;
pub mod shower;
pub mod urban;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameLevel {
    #[default]
    Houses,
    Kitchen,
    Urban,
    Shower,
    Copier,
}

pub struct LevelsPlugin;
impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            spawn_houses
                .in_schedule(OnEnter(GameLevel::Houses))
                .run_if(in_state(GameLoading::Loaded)),
            spawn_kitchen
                .in_schedule(OnEnter(GameLevel::Kitchen))
                .run_if(in_state(GameLoading::Loaded)),
            spawn_urban
                .in_schedule(OnEnter(GameLevel::Urban))
                .run_if(in_state(GameLoading::Loaded)),
            spawn_shower
                .in_schedule(OnEnter(GameLevel::Shower))
                .run_if(in_state(GameLoading::Loaded)),
            spawn_copier
                .in_schedule(OnEnter(GameLevel::Copier))
                .run_if(in_state(GameLoading::Loaded)),
        ))
        .add_systems((
            despawn_houses.in_schedule(OnExit(GameLevel::Houses)),
            despawn_kitchen.in_schedule(OnExit(GameLevel::Kitchen)),
            despawn_urban.in_schedule(OnExit(GameLevel::Urban)),
            despawn_shower.in_schedule(OnExit(GameLevel::Shower)),
            despawn_copier.in_schedule(OnExit(GameLevel::Copier)),
        ));
        //.add_systems((
        //    spawn_player_houses.in_schedule(OnEnter(GameLevel::Houses)),
        //    spawn_player_kitchen.in_schedule(OnEnter(GameLevel::Kitchen)),
        //    spawn_player_urban.in_schedule(OnEnter(GameLevel::Urban)),
        //    spawn_player_shower.in_schedule(OnEnter(GameLevel::Shower)),
        //    spawn_player_copier.in_schedule(OnEnter(GameLevel::Copier)),
        //));
    }
}

use bevy::{math::vec3, prelude::*};

use crate::GameLoading;

use self::{
    bf1::{despawn_bf1, spawn_bf1},
    bf_start::{despawn_bfstart, spawn_bfstart},
    bfa::{despawn_bfa1, despawn_bfa2, despawn_bfa3, spawn_bfa1, spawn_bfa2, spawn_bfa3},
    copier::{despawn_copier, spawn_copier},
    houses::{despawn_houses, spawn_houses},
    kitchen::{despawn_kitchen, spawn_kitchen},
    shower::{despawn_shower, spawn_shower},
    urban::{despawn_urban, spawn_urban},
};

pub mod bf1;
pub mod bf_start;
pub mod bfa;
pub mod copier;
pub mod houses;
pub mod kitchen;
pub mod shower;
pub mod urban;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameLevel {
    #[default]
    Kitchen,
    Houses,
    Urban,
    Shower,
    Copier,
    Bathroom,
    BFStart,
    BF1,
    BFA1,
    BFA2,
    BFA3,
    ControlRoom,
}

impl GameLevel {
    pub fn spawn_pos(&self) -> Vec3 {
        //TODO don't hardcode
        match self {
            GameLevel::Houses => vec3(0.0, 0.1, 0.0),
            GameLevel::Kitchen => vec3(0.0, 0.1, 0.0),
            GameLevel::Urban => vec3(0.0, 0.1, 0.0),
            GameLevel::Shower => vec3(0.0, 0.1, 0.0),
            GameLevel::Copier => vec3(0.0, 0.1, 0.0),
            GameLevel::BFStart => vec3(0.0, 0.1, 0.0),
            GameLevel::Bathroom => vec3(0.0, 0.1, 0.0),
            GameLevel::BF1 => vec3(0.0, 0.1, 0.0),
            GameLevel::BFA1 => vec3(-271.0, 0.2, 44.0),
            GameLevel::BFA2 => vec3(-1.35, 0.2, 23.0),
            GameLevel::BFA3 => vec3(372.0, 0.2, -9.5),
            GameLevel::ControlRoom => vec3(0.0, 0.1, 0.0),
        }
    }
    pub fn teleporter_pos_close_enough(&self, pos: Vec3) -> bool {
        //TODO don't hardcode
        match self {
            GameLevel::Houses => false,
            GameLevel::Kitchen => false,
            GameLevel::Urban => false,
            GameLevel::Shower => false,
            GameLevel::Copier => false,
            GameLevel::Bathroom => false,
            GameLevel::BFStart => vec3(-106.31, -35.4, -44.725).distance(pos) < 15.0,
            GameLevel::BF1 => vec3(58.6389, -453.065, -640.837).distance(pos) < 8.0,
            GameLevel::BFA1 => vec3(-350.534, 0.0, -79.1253).distance(pos) < 8.0,
            GameLevel::BFA2 => vec3(-1.53535, 0.0, -195.286).distance(pos) < 8.0,
            GameLevel::BFA3 => vec3(372.869, 0.0, -91.5084).distance(pos) < 8.0,
            GameLevel::ControlRoom => false,
        }
    }
    pub fn clock_position_close_enough(&self, pos: Vec3) -> bool {
        //TODO don't hardcode
        match self {
            GameLevel::Houses => vec3(-50.3636, 0.0, -12.6446).distance(pos) < 1.5,
            GameLevel::Kitchen => vec3(-0.139801, 0.0, 2.02768).distance(pos) < 1.5,
            GameLevel::Urban => vec3(-4.65948, 0.0, -7.78706).distance(pos) < 1.5,
            GameLevel::Shower => vec3(1.00083, 0.0, -0.709787).distance(pos) < 0.8,
            GameLevel::Copier => vec3(-0.213056, 0.0, -0.637783).distance(pos) < 0.8,
            GameLevel::Bathroom => vec3(0.3016, 0.0, 0.410778).distance(pos) < 1.5,
            GameLevel::BFStart => false,
            GameLevel::BF1 => false,
            GameLevel::BFA1 => false,
            GameLevel::BFA2 => false,
            GameLevel::BFA3 => false,
            GameLevel::ControlRoom => false,
        }
    }
    pub fn teleporter_dest(&self) -> GameLevel {
        //TODO don't hardcode
        match self {
            GameLevel::Kitchen => GameLevel::BFStart,
            GameLevel::BFStart => GameLevel::Shower,
            GameLevel::Shower => GameLevel::BFA1,
            GameLevel::BFA1 => GameLevel::Copier,
            GameLevel::Copier => GameLevel::BFA2,
            GameLevel::BFA2 => GameLevel::Bathroom,
            GameLevel::Bathroom => GameLevel::BFA3,
            GameLevel::BFA3 => GameLevel::Houses,
            GameLevel::Houses => GameLevel::BF1,
            GameLevel::BF1 => GameLevel::Urban,
            GameLevel::Urban => GameLevel::ControlRoom,
            GameLevel::ControlRoom => GameLevel::Kitchen,
        }
    }
    pub fn teleporter_code(code: &str) -> Option<GameLevel> {
        //TODO don't hardcode
        match code {
            "0625" => Some(GameLevel::Kitchen),
            "1332" => Some(GameLevel::BFStart),
            "0719" => Some(GameLevel::Shower),
            "1512" => Some(GameLevel::BFA1),
            "1514" => Some(GameLevel::Copier),
            "0655" => Some(GameLevel::BFA2),
            "1207" => Some(GameLevel::Bathroom),
            "0201" => Some(GameLevel::BFA3),
            "2142" => Some(GameLevel::Houses),
            "0722" => Some(GameLevel::BF1),
            "2306" => Some(GameLevel::Urban),
            "0121" => Some(GameLevel::ControlRoom),
            _ => None,
        }
    }
    pub fn player_can_jump(&self) -> bool {
        match self {
            GameLevel::Kitchen => false,
            GameLevel::BFStart => true,
            GameLevel::Shower => false,
            GameLevel::BFA1 => true,
            GameLevel::Copier => false,
            GameLevel::BFA2 => true,
            GameLevel::Bathroom => false,
            GameLevel::BFA3 => true,
            GameLevel::Houses => false,
            GameLevel::BF1 => true,
            GameLevel::Urban => false,
            GameLevel::ControlRoom => true,
        }
    }
    pub fn show_gun(&self) -> bool {
        match self {
            GameLevel::Kitchen => false,
            GameLevel::BFStart => true,
            GameLevel::Shower => false,
            GameLevel::BFA1 => true,
            GameLevel::Copier => false,
            GameLevel::BFA2 => true,
            GameLevel::Bathroom => false,
            GameLevel::BFA3 => true,
            GameLevel::Houses => false,
            GameLevel::BF1 => true,
            GameLevel::Urban => false,
            GameLevel::ControlRoom => true,
        }
    }
    pub fn show_drones_dead_msg(&self) -> bool {
        match self {
            GameLevel::Kitchen => false,
            GameLevel::BFStart => false,
            GameLevel::Shower => false,
            GameLevel::BFA1 => true,
            GameLevel::Copier => false,
            GameLevel::BFA2 => true,
            GameLevel::Bathroom => false,
            GameLevel::BFA3 => true,
            GameLevel::Houses => false,
            GameLevel::BF1 => true,
            GameLevel::Urban => false,
            GameLevel::ControlRoom => true,
        }
    }
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
            spawn_bfstart
                .in_schedule(OnEnter(GameLevel::BFStart))
                .run_if(in_state(GameLoading::Loaded)),
            spawn_bf1
                .in_schedule(OnEnter(GameLevel::BF1))
                .run_if(in_state(GameLoading::Loaded)),
            spawn_bfa1
                .in_schedule(OnEnter(GameLevel::BFA1))
                .run_if(in_state(GameLoading::Loaded)),
            spawn_bfa2
                .in_schedule(OnEnter(GameLevel::BFA2))
                .run_if(in_state(GameLoading::Loaded)),
            spawn_bfa3
                .in_schedule(OnEnter(GameLevel::BFA3))
                .run_if(in_state(GameLoading::Loaded)),
        ))
        .add_systems((
            despawn_houses.in_schedule(OnExit(GameLevel::Houses)),
            despawn_kitchen.in_schedule(OnExit(GameLevel::Kitchen)),
            despawn_urban.in_schedule(OnExit(GameLevel::Urban)),
            despawn_shower.in_schedule(OnExit(GameLevel::Shower)),
            despawn_copier.in_schedule(OnExit(GameLevel::Copier)),
            despawn_bfstart.in_schedule(OnExit(GameLevel::BFStart)),
            despawn_bf1.in_schedule(OnExit(GameLevel::BF1)),
            despawn_bfa1.in_schedule(OnExit(GameLevel::BFA1)),
            despawn_bfa2.in_schedule(OnExit(GameLevel::BFA2)),
            despawn_bfa3.in_schedule(OnExit(GameLevel::BFA3)),
        ));
    }
}

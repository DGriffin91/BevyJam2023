use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::*;

use crate::{assets::AudioAssets, levels::GameLevel, ui::AudioVolumes, GameLoading};

pub struct GameAudioPlugin;
impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(set_music.run_if(in_state(GameLoading::Loaded)));
    }
}

fn set_music(
    level: Res<State<GameLevel>>,
    audio: Res<bevy_kira_audio::Audio>,
    audio_assets: Res<AudioAssets>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut current_handle: Local<Option<Handle<AudioInstance>>>,
    audio_volumes: Res<AudioVolumes>,
) {
    if audio_volumes.is_changed() {
        if let Some(current_handle) = &*current_handle {
            if let Some(instance) = audio_instances.get_mut(&current_handle) {
                instance.set_volume(
                    audio_volumes.music as f64 * 0.9,
                    AudioTween::linear(Duration::from_secs_f32(0.1)),
                );
            }
        }
    }
    if !level.is_changed() {
        return;
    }
    if let Some(current_handle) = &*current_handle {
        if let Some(instance) = audio_instances.get_mut(&current_handle) {
            instance.stop(AudioTween::new(
                Duration::from_secs_f32(6.5),
                AudioEasing::InOutPowi(2),
            ));
        }
    }
    let clip = match level.0 {
        GameLevel::Kitchen => &audio_assets.kitchen,
        GameLevel::Houses => &audio_assets.theme3,
        GameLevel::Urban => &audio_assets.theme3,
        GameLevel::Shower => &audio_assets.shower,
        GameLevel::Copier => &audio_assets.theme3,
        GameLevel::Bathroom => &audio_assets.theme3,
        GameLevel::BFStart => &audio_assets.theme1,
        GameLevel::BF1 => &audio_assets.theme1,
        GameLevel::BFA1 => &audio_assets.theme1,
        GameLevel::BFA2 => &audio_assets.theme1,
        GameLevel::BFA3 => &audio_assets.theme1,
        GameLevel::ControlRoom => &audio_assets.theme3,
    };

    *current_handle = Some(
        audio
            .play(clip.clone())
            .looped()
            .fade_in(AudioTween::new(
                Duration::from_secs_f32(0.5),
                AudioEasing::OutPowi(2),
            ))
            .with_volume(audio_volumes.music as f64 * 0.9)
            .handle(),
    );
}

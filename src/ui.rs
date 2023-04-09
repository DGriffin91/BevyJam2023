use bevy::{prelude::*, window::CursorGrabMode};
use bevy_egui::*;
use bevy_fps_controller::controller::{FpsController, RenderPlayer};

use crate::{levels::GameLevel, GameLoading, Health};

pub struct GameUiPlugin;
impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system(ui_system.run_if(in_state(GameLoading::Loaded)));
    }
}

pub fn ui_system(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameLevel>>,
    mut fps_controller: Query<&mut FpsController>,
    windows: Query<&Window>,
    health: Query<&Health, With<RenderPlayer>>,
) {
    egui::Window::new("Hello2").show(contexts.ctx_mut(), |ui| {
        for health in &health {
            ui.label(format!("health {}", health.0));
        }
    });
    let window = windows.single();
    if window.cursor.grab_mode == CursorGrabMode::Locked
        && !contexts.ctx_mut().wants_pointer_input()
    {
        return;
    }
    let mut fps_controler = fps_controller.single_mut();
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        if ui.button("Resume").clicked() {
            fps_controler.enable_input = true;
        }
        if ui.button("Urban").clicked() {
            dbg!("urban");
            next_state.set(GameLevel::Urban);
        }
        if ui.button("Houses").clicked() {
            next_state.set(GameLevel::Houses);
        }
        if ui.button("Kitchen").clicked() {
            next_state.set(GameLevel::Kitchen);
        }
        if ui.button("Shower").clicked() {
            next_state.set(GameLevel::Shower);
        }
        if ui.button("Copier").clicked() {
            next_state.set(GameLevel::Copier);
        }
    });
}

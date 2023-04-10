use bevy::{prelude::*, window::CursorGrabMode};
use bevy_egui::{
    egui::{
        epaint::Shadow, style::WidgetVisuals, vec2, Color32, FontFamily, FontId, Margin, Rounding,
        Slider, Stroke,
    },
    *,
};
use bevy_fps_controller::controller::{FpsController, RenderPlayer};
use bevy_rapier3d::prelude::Velocity;
use iyes_progress::ProgressCounter;

use crate::ui::egui::TextStyle::Heading;
use crate::ui::egui::TextStyle::Monospace;
use crate::ui::egui::TextStyle::Small;
use crate::{character_controller::JUMP_SPEED, ui::egui::TextStyle::Body};
use crate::{levels::GameLevel, units::UnitData, GameLoading, Health};
use crate::{ui::egui::TextStyle::Button, units::Difficulty};

pub struct GameUiPlugin;
impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system(ui_system.run_if(in_state(GameLoading::Loaded)))
            .add_system(loading_ui.run_if(in_state(GameLoading::AssetLoading)))
            .insert_resource(PlayerCode::default())
            .insert_resource(SettingClock::default())
            .insert_resource(TextFeed::default())
            .insert_resource(GameElapsedTime::default())
            .insert_resource(HasEnteredControlRoom::default())
            .insert_resource(FinishedGame::default());
    }
}

#[derive(Resource, Default)]
pub struct PlayerCode(pub String);
#[derive(Resource, Default)]
pub struct SettingClock(pub bool);

#[derive(Resource, Default)]
pub struct TextFeed(pub String);

#[derive(Resource, Default)]
pub struct GameElapsedTime(pub Option<f32>);

#[derive(Resource, Default)]
pub struct HasEnteredControlRoom(pub bool);

#[derive(Resource, Default)]
pub struct FinishedGame(pub (bool, f32));

impl TextFeed {
    pub fn push(&mut self, text: &str) {
        self.0 = format!("{}\n\n> {}", self.0, text)
    }
}

pub fn ui_system(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut next_level: ResMut<NextState<GameLevel>>,
    level: Res<State<GameLevel>>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut FpsController)>,
    mut windows: Query<&mut Window>,
    mut health: Query<&mut Health, With<RenderPlayer>>,
    mut player_code: ResMut<PlayerCode>,
    units: Query<Entity, With<UnitData>>,
    mut setting_clock: ResMut<SettingClock>,
    mut text_feed: ResMut<TextFeed>,
    keys: Res<Input<KeyCode>>,
    mut one_bot_left: Local<bool>,
    mut difficulty: ResMut<Difficulty>,
    end_game: (Res<GameElapsedTime>, Res<FinishedGame>),
    time: Res<Time>,
) {
    let (game_time, game_finished) = end_game;
    let drones_remaining = units.iter().count();
    if drones_remaining == 1 {
        *one_bot_left = true;
    }
    if *one_bot_left && drones_remaining == 0 && level.0.show_drones_dead_msg() {
        text_feed.push("Looks like all the drones have been eliminated, find the teleporter and continue to the next sector.");
        *one_bot_left = false;
    }
    let was_setting_clock = setting_clock.0;
    let mut window = windows.single_mut();
    if let Some((mut transform, mut velocity, mut fps_controller)) = player.iter_mut().next() {
        let ctx = contexts.ctx_mut();
        ctx.set_visuals(get_visuals());
        let frame = egui::Frame {
            rounding: Rounding::none(),
            shadow: Shadow::NONE,
            fill: Color32::from_rgba_unmultiplied(0, 0, 0, 64),
            stroke: Stroke::NONE,
            inner_margin: Margin::symmetric(8.0, 8.0),
            outer_margin: Margin::symmetric(2.0, 2.0),
        };
        set_text_styles(ctx);
        egui::Window::new("win1")
            .title_bar(false)
            .collapsible(false)
            .movable(false)
            .resizable(false)
            .interactable(false)
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(0.0, 0.0))
            .frame(frame)
            .show(contexts.ctx_mut(), |ui| {
                ui.vertical_centered_justified(|ui| {
                    if let Some(health) = &health.iter().next() {
                        if game_finished.0 .0 {
                            if let Some(game_time) = game_time.0 {
                                ui.label(format!(
                                    "TIME ELAPSED {:.1}",
                                    game_finished.0 .1 - game_time
                                ));
                            }
                        } else {
                            if let Some(game_time) = game_time.0 {
                                ui.label(format!(
                                    "TIME ELAPSED {:.1}",
                                    time.elapsed_seconds() - game_time
                                ));
                            }
                            ui.label(format!("HEALTH {}", (health.0 * 100.0).round() as i32));
                            ui.label(format!("{} DRONES REMAINING", drones_remaining));
                        }
                    }
                })
            });
        let mut teleport_dest = None;
        if setting_clock.0 {
            egui::Window::new("set clock win")
                .title_bar(false)
                .collapsible(false)
                .movable(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .frame(frame)
                .show(contexts.ctx_mut(), |ui| {
                    ui.vertical_centered_justified(|ui| {
                        if ui.text_edit_singleline(&mut player_code.0).changed() {
                            player_code.0 = strip_non_numeric(&player_code.0);
                        }
                        if ui.button("SET CLOCK").clicked() {
                            if let Some(level) = GameLevel::teleporter_code(&player_code.0) {
                                dbg!(&level, &player_code.0);
                                player_code.0 = String::new();
                                setting_clock.0 = false;
                                teleport_dest = Some(level);
                            }
                        }
                        if ui.button("CLOSE").clicked() {
                            setting_clock.0 = false;
                        }
                    })
                });
        }
        if window.cursor.grab_mode != CursorGrabMode::Locked {
            egui::Window::new("tab win")
                .title_bar(false)
                .collapsible(false)
                .movable(false)
                .resizable(false)
                .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(0.0, 0.0))
                .frame(frame)
                .show(contexts.ctx_mut(), |ui| {
                    ui.vertical_centered_justified(|ui| {
                        if ui.button("RESTART LEVEL").clicked() {
                            next_level.set(level.0.clone());
                            teleport_dest = Some(level.0.clone());
                        }
                        let easy = Difficulty::Easy == *difficulty;
                        let medium = Difficulty::Medium == *difficulty;
                        let hard = Difficulty::Hard == *difficulty;
                        let ultra = Difficulty::Ultra == *difficulty;
                        if ui.radio(easy, "EASY").clicked() {
                            *difficulty = Difficulty::Easy;
                        }
                        if ui.radio(medium, "MEDIUM").clicked() {
                            *difficulty = Difficulty::Medium;
                        }
                        if ui.radio(hard, "HARD").clicked() {
                            *difficulty = Difficulty::Hard;
                        }
                        if ui.radio(ultra, "ULTRA").clicked() {
                            *difficulty = Difficulty::Ultra;
                        }
                        let mut sens = fps_controller.sensitivity * 1000.0;
                        if ui
                            .add(Slider::new(&mut sens, 0.1..=5.0).text("Mouse Sensitivity"))
                            .changed()
                        {
                            fps_controller.sensitivity = sens / 1000.0;
                        }
                    })
                });
        } else {
            if level.0.clock_position_close_enough(transform.translation) {
                egui::Window::new("set clock text")
                    .title_bar(false)
                    .collapsible(false)
                    .movable(false)
                    .resizable(false)
                    .interactable(false)
                    .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                    .frame(frame)
                    .show(contexts.ctx_mut(), |ui| {
                        ui.vertical_centered_justified(|ui| {
                            ui.label("PRESS E TO SET CLOCK");
                            #[cfg(not(debug_assertions))]
                            if keys.just_pressed(KeyCode::E) {
                                setting_clock.0 = !setting_clock.0
                            }
                        })
                    });
            }
            if drones_remaining == 0 && level.0.teleporter_pos_close_enough(transform.translation) {
                teleport_dest = Some(level.0.teleporter_dest());
            }
            let ctx = contexts.ctx_mut();
            let mut visuals = get_visuals();
            visuals.override_text_color = Some(Color32::from_rgba_unmultiplied(255, 255, 255, 96));
            ctx.set_visuals(visuals);
            egui::Window::new("win3")
                .title_bar(false)
                .collapsible(false)
                .movable(false)
                .resizable(false)
                .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(0.0, 0.0))
                .frame(frame)
                .show(contexts.ctx_mut(), |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.label("PRESS TAB FOR MENU");
                    })
                });
        }
        #[cfg(debug_assertions)]
        if keys.just_pressed(KeyCode::E) {
            setting_clock.0 = !setting_clock.0;
        }
        if was_setting_clock && !setting_clock.0 {
            fps_controller.enable_input = true;
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        } else if !was_setting_clock && setting_clock.0 {
            fps_controller.enable_input = false;
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
        if let Some(teleport_dest) = teleport_dest {
            teleport(
                &mut fps_controller,
                &mut health,
                teleport_dest,
                &mut next_level,
                &mut velocity,
                &mut transform,
                &units,
                &mut commands,
            );
        }
        egui::Window::new("text_feed")
            .title_bar(false)
            .collapsible(false)
            .movable(false)
            .resizable(false)
            .fixed_size(vec2(600.0, 35.0))
            .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(0.0, 0.0))
            .frame(frame)
            .show(contexts.ctx_mut(), |ui| {
                egui::ScrollArea::vertical()
                    .max_height(40.0)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        ui.label(&text_feed.0);
                    });
            });
    }
}

fn teleport(
    fps_controller: &mut FpsController,
    health: &mut Query<&mut Health, With<RenderPlayer>>,
    level: GameLevel,
    next_level: &mut NextState<GameLevel>,
    velocity: &mut Velocity,
    transform: &mut Transform,
    units: &Query<Entity, With<UnitData>>,
    commands: &mut Commands,
) {
    fps_controller.gravity = 0.0;
    if let Some(mut health) = health.iter_mut().next() {
        health.0 = 1.0;
    }
    if level.player_can_jump() {
        fps_controller.jump_speed = JUMP_SPEED;
    } else {
        fps_controller.jump_speed = 0.0;
    }
    next_level.set(level.clone());

    velocity.linvel = Vec3::ZERO;
    transform.translation = level.spawn_pos();
    for unit in units {
        if commands.get_entity(unit).is_some() {
            commands.entity(unit).despawn_recursive();
        }
    }
}

fn set_text_styles(ctx: &mut egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (Heading, FontId::new(30.0, FontFamily::Monospace)),
        (Body, FontId::new(13.0, FontFamily::Monospace)),
        (Monospace, FontId::new(13.0, FontFamily::Monospace)),
        (Button, FontId::new(13.0, FontFamily::Monospace)),
        (Small, FontId::new(13.0, FontFamily::Monospace)),
    ]
    .into();
    ctx.set_style(style);
}

fn get_visuals() -> egui::Visuals {
    let mut visuals = egui::Visuals::dark();
    visuals.override_text_color = Some(Color32::WHITE);
    visuals.extreme_bg_color = Color32::from_rgba_unmultiplied(0, 0, 0, 192);
    visuals.widgets.inactive = WidgetVisuals {
        bg_fill: Color32::from_rgba_unmultiplied(0, 0, 0, 192),
        weak_bg_fill: Color32::from_rgba_unmultiplied(0, 0, 0, 192),
        bg_stroke: Stroke::NONE,
        rounding: Rounding::none(),
        fg_stroke: Stroke::new(5.0, Color32::from_rgba_unmultiplied(255, 255, 255, 128)),
        expansion: 0.0,
    };
    visuals.widgets.hovered = WidgetVisuals {
        bg_fill: Color32::from_rgba_unmultiplied(255, 255, 255, 48),
        weak_bg_fill: Color32::from_rgba_unmultiplied(255, 255, 255, 48),
        bg_stroke: Stroke::NONE,
        rounding: Rounding::none(),
        fg_stroke: Stroke::NONE,
        expansion: 0.0,
    };
    visuals
}

fn strip_non_numeric(input: &str) -> String {
    input.chars().filter(|c| c.is_numeric()).collect()
}

fn loading_ui(
    mut contexts: EguiContexts,
    progress: Option<Res<ProgressCounter>>,
    mut last_done: Local<u32>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        if progress.done > *last_done {
            *last_done = progress.done;
        }
        let ctx = contexts.ctx_mut();
        ctx.set_visuals(get_visuals());
        egui::Window::new("win3")
            .title_bar(false)
            .collapsible(false)
            .movable(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(contexts.ctx_mut(), |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.label("LOADING");
                    ui.label(format!("{}/{}", *last_done, progress.total));
                })
            });
    }
}

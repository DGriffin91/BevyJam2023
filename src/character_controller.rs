use std::f32::consts::TAU;

use bevy::{
    core_pipeline::{fxaa::Fxaa, tonemapping::Tonemapping},
    math::{vec2, Vec3Swizzles},
    prelude::*,
    window::CursorGrabMode,
};
use bevy_egui::EguiContexts;
use bevy_polyline::prelude::{Polyline, PolylineBundle, PolylineMaterial};
use bevy_rapier3d::prelude::*;

use bevy_fps_controller::controller::*;

use crate::{ui::ui_system, Health};

pub const SPAWN_POINT: Vec3 = Vec3::new(0.0, 0.0, 0.0);

pub struct CharacterController;
impl Plugin for CharacterController {
    fn build(&self, app: &mut App) {
        app.add_plugin(FpsControllerPlugin)
            .add_startup_system(setup)
            .add_systems((manage_cursor, display_text, respawn).after(ui_system));
    }
}

#[derive(Component)]
pub struct ShootableByUnit;

#[derive(Component)]
pub struct LogicalPlayerEntity(pub Entity);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    let polyline = polylines.add(Polyline {
        vertices: vec![Vec3::ZERO, Vec3::ZERO],
    });
    commands.spawn_empty().insert(PolylineBundle {
        polyline: polyline.clone(),
        material: polyline_materials.add(PolylineMaterial {
            width: 10.0,
            color: Color::RED,
            perspective: true,
            depth_bias: 0.0,
        }),
        ..default()
    });

    // Note that we have two entities for the player
    // One is a "logical" player that handles the physics computation and collision
    // The other is a "render" player that is what is displayed to the user
    // This distinction is useful for later on if you want to add multiplayer,
    // where often time these two ideas are not exactly synced up
    let logical_player_entity = commands
        .spawn((
            //Collider::capsule(Vec3::Y * 0.2, Vec3::Y * 1.0, 0.2),
            Collider::capsule(Vec3::Y * 0.1, Vec3::Y * 1.0, 0.2),
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            RigidBody::Dynamic,
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            AdditionalMassProperties::Mass(1.0),
            GravityScale(0.0),
            Ccd { enabled: true }, // Prevent clipping when going fast
            TransformBundle::from_transform(Transform::from_translation(SPAWN_POINT)),
            LogicalPlayer(0),
            FpsControllerInput {
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController {
                enable_input: false,
                air_acceleration: 80.0,
                height: 1.1,
                upright_height: 1.7,
                crouch_height: 1.0,
                walk_speed: 6.0,
                run_speed: 12.0,
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            Visibility::default(),
            ComputedVisibility::default(),
            Camera3dBundle {
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: TAU / 5.0,
                    ..default()
                }),
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                tonemapping: Tonemapping::TonyMcMapface,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            RenderPlayer(0),
            FogSettings {
                color: Color::rgba(0.1, 0.1, 0.1, 1.0),
                falloff: FogFalloff::Exponential { density: 0.0003 },
                ..default()
            },
        ))
        .insert(EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        })
        .insert(Fxaa::default())
        .insert(polyline)
        .insert(LogicalPlayerEntity(logical_player_entity))
        .insert(ShootableByUnit)
        .insert(Health(1.0));

    commands.spawn(
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/fira_mono.ttf"),
                font_size: 24.0,
                color: Color::BLACK,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
    );
}

fn respawn(mut query: Query<(&mut Transform, &mut Velocity), With<FpsController>>) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.y > -50.0 {
            continue;
        }

        velocity.linvel = Vec3::ZERO;
        transform.translation = SPAWN_POINT;
    }
}

fn manage_cursor(
    keys: Res<Input<KeyCode>>,
    mut fps_controller: Query<&mut FpsController>,
    btn: Res<Input<MouseButton>>,
    //#[cfg(debug_assertions)] editor_state: Res<EditorState>,
    mut windows: Query<&mut Window>,
    mut contexts: EguiContexts,
) {
    if contexts.ctx_mut().wants_pointer_input() {
        return;
    }
    let mut window = windows.single_mut();
    let mut fps_controller = fps_controller.single_mut();
    let cursor_locked = window.cursor.grab_mode == CursorGrabMode::Locked;
    let mut lock = None;
    if keys.just_pressed(KeyCode::Tab) {
        lock = Some(!cursor_locked);
    }
    if keys.just_pressed(KeyCode::Escape) || (!cursor_locked && fps_controller.enable_input) {
        // Unlock
        lock = Some(false);
    }

    #[allow(unused_assignments, unused_mut)]
    let mut editor_active = false;

    //#[cfg(debug_assertions)]
    //{
    //    editor_active = editor_state.active;
    //}

    if btn.just_pressed(MouseButton::Left)
        && (!fps_controller.enable_input || window.cursor.visible || !cursor_locked)
        && !editor_active
    {
        // Lock
        lock = Some(true);
    }

    if let Some(lock) = lock {
        if lock {
            // Unlock
            fps_controller.enable_input = true;
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        } else {
            // Lock
            fps_controller.enable_input = false;
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
    if cursor_locked {
        let (w, h) = (window.width(), window.height());
        window.set_cursor_position(Some(vec2(w / 2.0, h / 2.0)));
    }
}

fn display_text(
    mut controller_query: Query<(&Transform, &Velocity)>,
    mut text_query: Query<&mut Text>,
) {
    for (transform, velocity) in &mut controller_query {
        for mut text in &mut text_query {
            text.sections[0].value = format!(
                "vel: {:.2}, {:.2}, {:.2}\npos: {:.2}, {:.2}, {:.2}\nspd: {:.2}",
                velocity.linvel.x,
                velocity.linvel.y,
                velocity.linvel.z,
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
                velocity.linvel.xz().length()
            );
        }
    }
}

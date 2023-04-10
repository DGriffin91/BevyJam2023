use crate::{
    assets::PropAssets,
    character_controller::LogicalPlayerEntity,
    levels::GameLevel,
    materials::pbr_material::{EnvSettings, MaterialsSet},
    ui::ui_system,
    units::UnitData,
    GameLoading, Health, LevelsStarted,
};
use bevy::{math::vec3, prelude::*};
use bevy_egui::EguiContexts;
use bevy_fps_controller::controller::{FpsController, RenderPlayer};
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                respawn,
                player_shoot,
                add_gun,
                add_crosshair,
                progress_projectiles,
                gun_visibility,
            )
                .chain()
                .distributive_run_if(in_state(GameLoading::Loaded))
                .before(MaterialsSet::MaterialSwap)
                .after(ui_system),
        );
    }
}

#[derive(Component)]
pub struct GunRef(pub Entity);

#[derive(Component)]
pub struct GunFlash;

#[derive(Component)]
pub struct GunModel;

#[derive(Component)]
pub struct PlayerGun {
    pub attack_damage: f32,
    pub fire_cooldown: f32,
    pub fire_rate: f32,
}

fn add_gun(
    mut commands: Commands,
    props: Res<PropAssets>,
    player: Query<Entity, (With<LogicalPlayerEntity>, Without<GunRef>)>,
) {
    if let Some(player) = player.iter().next() {
        let env_settings = EnvSettings {
            env_spec: 0.1,
            env_diff: 0.1,
            emit_mult: 0.1,
        };
        let trans = Transform::from_translation(vec3(0.36, -0.1, -0.38));
        let gun_emit = commands
            .spawn(SceneBundle {
                scene: props.gun_emit.clone(),
                transform: trans,
                ..default()
            })
            .insert(GunModel)
            .id();
        let gun = commands
            .spawn(SceneBundle {
                scene: props.gun.clone(),
                transform: trans,
                ..default()
            })
            .insert(env_settings)
            .insert(GunModel)
            .id();
        let flash = commands
            .spawn(SceneBundle {
                scene: props.gun_flash.clone(),
                transform: trans,
                visibility: Visibility::Hidden,
                ..default()
            })
            .insert(GunFlash)
            .insert(EnvSettings {
                env_spec: 0.0,
                env_diff: 0.0,
                emit_mult: 10.0, //not working, even if I chain and apply_system_buffers before mat swap
            })
            .id();
        commands
            .entity(player)
            .insert(GunRef(gun.clone()))
            .add_child(gun)
            .add_child(gun_emit)
            .add_child(flash)
            .insert(PlayerGun {
                attack_damage: 0.3,
                fire_cooldown: 1.0,
                fire_rate: 10.0,
            });
    }
}

#[derive(Component)]
pub struct CrosshairRef(pub Entity);

fn add_crosshair(
    mut commands: Commands,
    props: Res<PropAssets>,
    player: Query<Entity, (With<LogicalPlayerEntity>, Without<CrosshairRef>)>,
) {
    if let Some(player) = player.iter().next() {
        let trans = Transform::from_translation(vec3(0.0, 0.0, -0.2));
        let crosshair = commands
            .spawn(SceneBundle {
                scene: props.crosshair.clone(),
                transform: trans,
                ..default()
            })
            .id();
        commands
            .entity(player)
            .insert(CrosshairRef(crosshair.clone()))
            .add_child(crosshair);
    }
}

fn player_shoot(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut player: Query<(
        Entity,
        &GlobalTransform,
        &LogicalPlayerEntity,
        &mut PlayerGun,
    )>,
    buttons: Res<Input<MouseButton>>,
    props: Res<PropAssets>,
    mut contexts: EguiContexts,
    mut gun_flash: Query<&mut Visibility, With<GunFlash>>,
    mut healths: Query<&mut Health>,
    time: Res<Time>,
    state: Res<State<GameLevel>>,
) {
    // We will color in read the colliders hovered by the mouse.
    for (entity, camera_transform, logical_player_entity, mut gun) in &mut player {
        gun.fire_cooldown -= gun.fire_rate * time.delta_seconds();

        if !buttons.pressed(MouseButton::Left)
            || contexts.ctx_mut().wants_pointer_input()
            || gun.fire_cooldown > 0.0
            || !state.0.show_gun()
        {
            for mut flash in &mut gun_flash {
                *flash = Visibility::Hidden;
            }
            continue;
        } else {
            for mut flash in &mut gun_flash {
                *flash = Visibility::Visible;
            }
        }
        gun.fire_cooldown = 1.0;
        // First, compute a ray from the mouse position.
        let origin = camera_transform.translation();
        let direction = camera_transform.forward();

        let ct = camera_transform;
        let mut projectile_trans = ct.compute_transform();
        let look_at = origin + ct.forward() * 1000.0;
        projectile_trans.translation = origin + ct.right() * 0.2 - ct.up() * 0.1;
        projectile_trans.look_at(look_at, Vec3::Y);
        commands
            .spawn(SceneBundle {
                scene: props.projectile_lite.clone(),
                transform: projectile_trans,
                ..default()
            })
            .insert(Projectile {
                speed: 250.0,
                max_dist: 1000.0,
                dist_trav: 0.0,
            })
            .insert(EnvSettings {
                env_spec: 0.0,
                env_diff: 0.0,
                emit_mult: 10.0, //not working
            });
        // Then cast the ray.
        let hit = rapier_context.cast_ray(
            origin,
            direction,
            f32::MAX,
            false,
            QueryFilter::default()
                .exclude_collider(entity)
                .exclude_collider(logical_player_entity.0)
                .exclude_sensors(),
        );

        if let Some((hit_entity, toi)) = hit {
            if commands.get_entity(hit_entity).is_some() {
                if let Ok(mut health) = healths.get_mut(hit_entity) {
                    let dmg_mult = 1.0 / (toi - 35.0).clamp(1.0, 100.0).powf(0.5);
                    health.0 -= gun.attack_damage * dmg_mult;
                }
            }
        }
    }
}

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
    pub max_dist: f32,
    pub dist_trav: f32,
}

fn progress_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(Entity, &mut Transform, &mut Projectile)>,
) {
    for (entity, mut trans, mut projectile) in &mut projectiles {
        if projectile.dist_trav >= projectile.max_dist {
            commands.entity(entity).despawn_recursive();
        }
        let dist = time.delta_seconds() * projectile.speed;
        trans.translation = trans.translation + trans.forward() * dist;
        projectile.dist_trav += dist;
    }
}

fn respawn(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Velocity, &mut FpsController)>,
    mut health: Query<&mut Health, With<RenderPlayer>>,
    state: Res<State<GameLevel>>,
    mut next_state: ResMut<NextState<GameLevel>>,
    units: Query<Entity, With<UnitData>>,
    levels_started: Res<LevelsStarted>,
) {
    if !levels_started.0 {
        return;
    }
    if let Some(mut health) = health.iter_mut().next() {
        if let Some((mut transform, mut velocity, mut fps_controller)) = query.iter_mut().next() {
            if transform.translation.y < -500.0 || health.0 <= 0.0 {
                fps_controller.gravity = 0.0;
                health.0 = 1.0;
                next_state.set(state.0.clone());

                velocity.linvel = Vec3::ZERO;
                transform.translation = state.0.spawn_pos();
                for unit in &units {
                    if commands.get_entity(unit).is_some() {
                        commands.entity(unit).despawn_recursive();
                    }
                }
            }
        }
    }
}

fn gun_visibility(
    mut gun_models: Query<&mut Visibility, With<GunModel>>,
    level: Res<State<GameLevel>>,
) {
    for mut gun_vis in &mut gun_models {
        if level.0.show_gun() {
            *gun_vis = Visibility::Visible
        } else {
            *gun_vis = Visibility::Hidden
        }
    }
}

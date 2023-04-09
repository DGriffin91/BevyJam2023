use crate::{
    assets::PropAssets,
    character_controller::LogicalPlayerEntity,
    materials::pbr_material::{EnvSettings, MaterialsSet},
    GameLoading, Health,
};
use bevy::{math::vec3, prelude::*};
use bevy_egui::EguiContexts;
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                cast_ray.run_if(in_state(GameLoading::Loaded)),
                add_gun.run_if(in_state(GameLoading::Loaded)),
                add_crosshair.run_if(in_state(GameLoading::Loaded)),
                progress_projectiles.run_if(in_state(GameLoading::Loaded)),
            )
                .chain()
                .before(MaterialsSet::MaterialSwap),
        );
    }
}

#[derive(Component)]
pub struct GunRef(pub Entity);

#[derive(Component)]
pub struct GunFlash;

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
            .id();
        let gun = commands
            .spawn(SceneBundle {
                scene: props.gun.clone(),
                transform: trans,
                ..default()
            })
            .insert(env_settings)
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
                attack_damage: 0.4,
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

fn cast_ray(
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
) {
    // We will color in read the colliders hovered by the mouse.
    for (entity, camera_transform, logical_player_entity, mut gun) in &mut player {
        gun.fire_cooldown -= gun.fire_rate * time.delta_seconds();
        for mut flash in &mut gun_flash {
            *flash = Visibility::Visible;
        }
        if !buttons.pressed(MouseButton::Left)
            || contexts.ctx_mut().wants_pointer_input()
            || gun.fire_cooldown > 0.0
        {
            for mut flash in &mut gun_flash {
                *flash = Visibility::Hidden;
            }
            continue;
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
                speed: 150.0,
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

        if let Some((hit_entity, _toi)) = hit {
            if commands.get_entity(hit_entity).is_some() {
                dbg!(hit_entity);
                if let Ok(mut health) = healths.get_mut(hit_entity) {
                    dbg!(&health.0);
                    health.0 -= gun.attack_damage;
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

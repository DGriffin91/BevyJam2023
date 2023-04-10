use std::time::Duration;

use bevy::{math::vec3, prelude::*};
use bevy_rapier3d::prelude::{Collider, QueryFilter, RapierContext};
use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::Rng;

use crate::assets::PropAssets;
use crate::character_controller::{LogicalPlayerEntity, ShootableByUnit};
use crate::player::Projectile;
use crate::util::all_children;
use crate::Health;
use crate::{assets::UnitAssets, GameLoading, GameRng};
pub struct UnitsPlugin;
impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                spawn_enemies,
                play_animations,
                roam,
                setup_anim_player_refs,
                face_dest_pos,
                move_to_dest,
                target_shootables,
                shoot_stuff,
                blowup,
                damage_player,
            )
                .distributive_run_if(in_state(GameLoading::Loaded)),
        )
        .init_resource::<Difficulty>();
        //.add_system(spawn_some_units.in_schedule(OnEnter(GameLoading::Loaded)));
    }
}

pub fn setup_anim_player_refs(
    mut commands: Commands,
    unit_entities: Query<Entity, (With<UnitData>, Without<ChildAnimEntity>)>,
    children_query: Query<&Children>,
    animation_players: Query<&mut AnimationPlayer>,
) {
    for unit_entity in unit_entities.iter() {
        if let Ok(children) = children_query.get(unit_entity) {
            all_children(children, &children_query, &mut |child_entity| {
                if animation_players.get(child_entity).is_ok() {
                    commands
                        .entity(unit_entity)
                        .insert(ChildAnimEntity(child_entity));
                }
            });
        }
    }
}

#[derive(Copy, Clone)]
pub enum UnitsStates {
    Walk,
    Idle,
    Bob,
    Bonk,
    Fire,
    WalkLazy,
    Stop,
}

impl UnitsStates {
    pub fn rng_pick(rng: &mut GameRng) -> UnitsStates {
        let items = [
            (UnitsStates::Walk, 100),
            (UnitsStates::Idle, 15),
            (UnitsStates::Bob, 7),
            (UnitsStates::Bonk, 1),
            (UnitsStates::Fire, 1),
            (UnitsStates::WalkLazy, 20),
            (UnitsStates::Stop, 100),
        ];
        let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
        items[dist.sample(&mut rng.0)].0
    }

    pub fn rng_pick_attacking(rng: &mut GameRng) -> UnitsStates {
        let items = [
            (UnitsStates::Walk, 40),
            (UnitsStates::Bob, 7),
            (UnitsStates::Fire, 100),
            (UnitsStates::WalkLazy, 20),
        ];
        let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
        items[dist.sample(&mut rng.0)].0
    }

    pub fn get_anim(&self, ass: &UnitAssets) -> Option<Handle<AnimationClip>> {
        Some(match self {
            UnitsStates::Walk => ass.walk.clone(),
            UnitsStates::Idle => ass.idle.clone(),
            UnitsStates::Bob => ass.bob.clone(),
            UnitsStates::Bonk => ass.bonk.clone(),
            UnitsStates::Fire => ass.fire.clone(),
            UnitsStates::WalkLazy => ass.walk_lazy.clone(),
            UnitsStates::Stop => return None,
        })
    }

    pub fn does_loop(&self) -> bool {
        match self {
            UnitsStates::Walk => true,
            UnitsStates::Idle => true,
            UnitsStates::Bob => true,
            UnitsStates::Bonk => false,
            UnitsStates::Fire => true,
            UnitsStates::WalkLazy => true,
            UnitsStates::Stop => false,
        }
    }

    pub fn use_unit_speed(&self) -> bool {
        match self {
            UnitsStates::Walk => true,
            UnitsStates::Idle => false,
            UnitsStates::Bob => true,
            UnitsStates::Bonk => false,
            UnitsStates::Fire => false,
            UnitsStates::WalkLazy => true,
            UnitsStates::Stop => false,
        }
    }
}

#[derive(Component, Clone)]
pub struct UnitData {
    pub spawn: Vec3,
    pub max_radius: f32,
    pub dest: Vec3,
    pub target_to_shoot: Option<Vec3>,
    pub target_to_apply_damage: Option<Entity>,
    pub current_state: UnitsStates,
    pub state_timer: f32,
    pub current_clip: Option<Handle<AnimationClip>>,
    pub speed: f32,
    pub arrived: bool,
    pub init: bool, // for some reason they disappear if they don't walk first
    pub range: f32,
    pub fire_cooldown: f32,
    pub fire_rate: f32,
}

#[derive(Component)]
pub struct ChildAnimEntity(pub Entity);

fn roam(time: Res<Time>, mut units: Query<&mut UnitData>, mut rng: ResMut<GameRng>) {
    for mut unit in &mut units {
        unit.state_timer -= time.delta_seconds();
        if unit.state_timer < 0.0 {
            unit.current_state = if unit.init {
                if unit.target_to_shoot.is_some() {
                    UnitsStates::rng_pick_attacking(&mut rng)
                } else {
                    UnitsStates::rng_pick(&mut rng)
                }
            } else {
                unit.init = true;
                UnitsStates::Walk
            };
            match unit.current_state {
                UnitsStates::Walk => {
                    unit.state_timer = rng.gen_range(1.0..2.0);
                    let r = unit.max_radius;
                    unit.dest = vec3(
                        unit.spawn.x + rng.gen_range(-r..r),
                        unit.spawn.y,
                        unit.spawn.z + rng.gen_range(-r..r),
                    );
                    unit.arrived = false;
                }
                UnitsStates::Idle => {
                    unit.state_timer = rng.gen_range(1.0..2.0);
                    unit.arrived = true;
                }
                UnitsStates::Bob => {
                    unit.state_timer = rng.gen_range(1.0..2.0);
                    unit.arrived = true;
                }
                UnitsStates::Bonk => {
                    unit.state_timer = rng.gen_range(2.0..3.0);
                    unit.arrived = true;
                }
                UnitsStates::Fire => {
                    unit.state_timer = rng.gen_range(1.0..2.0);
                    unit.arrived = true;
                }
                UnitsStates::WalkLazy => {
                    unit.state_timer = rng.gen_range(1.0..2.0);
                    let r = unit.max_radius;
                    unit.dest = vec3(
                        unit.spawn.x + rng.gen_range(-r..r),
                        unit.spawn.y,
                        unit.spawn.z + rng.gen_range(-r..r),
                    );
                    unit.arrived = false;
                }
                UnitsStates::Stop => {
                    unit.state_timer = rng.gen_range(1.0..2.0);
                    unit.arrived = true;
                }
            }
        }
    }
}

#[derive(Component)]
pub struct EnemySpawns;

#[derive(Resource, Default, PartialEq, Eq)]
pub enum Difficulty {
    #[default]
    Easy,
    Medium,
    Hard,
    Ultra,
}

impl Difficulty {
    fn bot_dmg(&self) -> f32 {
        match self {
            Difficulty::Easy => 0.006,
            Difficulty::Medium => 0.012,
            Difficulty::Hard => 0.024,
            Difficulty::Ultra => 0.048,
        }
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    scene_entities: Query<Entity, With<EnemySpawns>>,
    children_query: Query<&Children>,
    transforms: Query<(&Transform, &Name)>,
    unit_assets: Res<UnitAssets>,
    mut rng: ResMut<GameRng>,
) {
    for entity in scene_entities.iter() {
        if let Ok(children) = children_query.get(entity) {
            all_children(children, &children_query, &mut |entity| {
                if let Ok((trans, name)) = transforms.get(entity) {
                    if !name.to_lowercase().contains("enemyspawn") {
                        return; //return from closure
                    }
                    let spawn_pos = trans.translation + Vec3::Y * 0.05;
                    commands
                        .spawn(SceneBundle {
                            scene: unit_assets.unit1.clone(),
                            transform: Transform::from_translation(spawn_pos),
                            ..default()
                        })
                        .insert(UnitData {
                            spawn: spawn_pos,
                            max_radius: 2.0,
                            dest: spawn_pos,
                            current_state: UnitsStates::Stop,
                            state_timer: rng.gen_range(5.5..6.5),
                            current_clip: None,
                            speed: rng.gen_range(0.8..2.5),
                            arrived: false,
                            init: false,
                            target_to_shoot: None,
                            target_to_apply_damage: None,
                            range: 43.0,
                            fire_cooldown: 1.0,
                            fire_rate: 4.0,
                        })
                        .insert(Collider::capsule(
                            vec3(0.0, 0.0, 0.0),
                            vec3(0.0, 1.6, 0.0),
                            0.4,
                        ))
                        .insert(Health(1.0));
                }
            });
            commands.entity(entity).remove::<EnemySpawns>();
        }
    }
}

fn _spawn_some_units(
    mut commands: Commands,
    unit_assets: Res<UnitAssets>,
    mut rng: ResMut<GameRng>,
) {
    for x in 0..0 {
        for z in 0..0 {
            let spawn_pos = vec3(x as f32 * 4.0 - 20.0, 0.0, z as f32 * 4.0 - 10.0);
            commands
                .spawn(SceneBundle {
                    scene: unit_assets.unit1.clone(),
                    transform: Transform::from_translation(spawn_pos),
                    ..default()
                })
                .insert(UnitData {
                    spawn: spawn_pos,
                    max_radius: 2.0,
                    dest: spawn_pos,
                    current_state: UnitsStates::Stop,
                    state_timer: rng.gen_range(2.5..4.5),
                    current_clip: None,
                    speed: rng.gen_range(0.8..2.5),
                    arrived: false,
                    init: false,
                    target_to_shoot: None,
                    target_to_apply_damage: None,
                    range: 50.0,
                    fire_cooldown: 1.0,
                    fire_rate: 1.0,
                })
                .insert(Collider::capsule(
                    vec3(0.0, 0.0, 0.0),
                    vec3(0.0, 1.6, 0.0),
                    0.4,
                ))
                .insert(Health(1.0));
        }
    }
}

fn play_animations(
    mut units: Query<(&mut UnitData, &ChildAnimEntity)>,
    unit_assets: Res<UnitAssets>,
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    for (unit, anim) in &mut units {
        if let Ok(mut player) = animation_players.get_mut(anim.0) {
            let clip = match unit.current_state {
                UnitsStates::Walk | UnitsStates::WalkLazy => {
                    if !unit.arrived {
                        unit.current_state.get_anim(&unit_assets)
                    } else {
                        None
                    }
                }
                _ => unit.current_state.get_anim(&unit_assets),
            };
            if let Some(clip) = clip {
                let play = if let Some(current_clip) = unit.current_clip.clone() {
                    if current_clip != clip {
                        true
                    } else {
                        if unit.current_state.does_loop() {
                            //if the animation loops, don't restart it
                            false
                        } else {
                            true
                        }
                    }
                } else {
                    true
                };
                if play {
                    let a = player.play_with_transition(clip, Duration::from_secs_f32(0.1));
                    if unit.current_state.does_loop() {
                        a.repeat();
                    }
                    if unit.current_state.use_unit_speed() {
                        a.set_speed(unit.speed);
                    }
                }
            } else {
                player.stop_repeating();
            }
        }
    }
}

pub fn face_dest_pos(mut unit_entities: Query<(&mut Transform, &UnitData)>) {
    for (mut trans, unit) in &mut unit_entities {
        match unit.current_state {
            UnitsStates::Fire => {
                if let Some(target_to_shoot) = unit.target_to_shoot {
                    let look = vec3(target_to_shoot.x, unit.spawn.y, target_to_shoot.z);
                    let new_trans = trans.looking_at(look, Vec3::Y);
                    trans.rotation = new_trans.rotation;
                }
            }
            UnitsStates::Walk | UnitsStates::WalkLazy => {
                let look = vec3(unit.dest.x, unit.spawn.y, unit.dest.z);
                let new_trans = trans.looking_at(look, Vec3::Y);
                trans.rotation = new_trans.rotation;
            }
            _ => {}
        }
    }
}

pub fn move_to_dest(mut unit_entities: Query<(&mut Transform, &mut UnitData)>) {
    for (mut trans, mut unit) in &mut unit_entities {
        match unit.current_state {
            UnitsStates::Walk | UnitsStates::WalkLazy => {
                if trans.translation.distance(unit.dest) > 0.1 {
                    let dir = (unit.dest - trans.translation).normalize();
                    trans.translation += dir * unit.speed * 0.005;
                } else {
                    unit.arrived = true;
                }
            }
            _ => (),
        }
    }
}

pub fn target_shootables(
    mut unit_entities: Query<(Entity, &GlobalTransform, &mut UnitData)>,
    shootables: Query<
        (Entity, &GlobalTransform, Option<&LogicalPlayerEntity>),
        With<ShootableByUnit>,
    >,
    rapier_context: Res<RapierContext>,
) {
    for (unit_entity, unit_trans, mut unit) in &mut unit_entities {
        let unit_trans = unit_trans.compute_transform();
        let mut closest_entity = None;
        let mut closest_pos = vec3(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut closest_dist = f32::INFINITY;
        let mut logical_player = None;
        for (shootable, shootable_trans, logical_player_shootable) in &shootables {
            let shootable_trans = shootable_trans.compute_transform();
            let this_dist = shootable_trans.translation.distance(unit_trans.translation);
            if this_dist < closest_dist {
                closest_dist = this_dist;
                closest_pos = shootable_trans.translation;
                closest_entity = Some(shootable.clone());
                logical_player = logical_player_shootable;
            }
        }
        unit.target_to_shoot = None;
        unit.target_to_apply_damage = None;
        if let Some(closest_entity) = closest_entity {
            if closest_dist < unit.range {
                let origin = unit_trans.translation + Vec3::Y * 1.65; // head level
                let hit = rapier_context.cast_ray(
                    origin,
                    (closest_pos - origin).normalize(),
                    f32::MAX,
                    false,
                    QueryFilter::default()
                        .exclude_collider(unit_entity)
                        .exclude_sensors(),
                );
                if let Some(hit) = hit {
                    if hit.0 == closest_entity {
                        unit.target_to_shoot = Some(closest_pos);
                        unit.target_to_apply_damage = Some(closest_entity);
                    } else if let Some(logical_player) = logical_player {
                        if hit.0 == logical_player.0 {
                            unit.target_to_shoot = Some(closest_pos);
                            unit.target_to_apply_damage = Some(closest_entity);
                        }
                    }
                }
            }
        }
    }
}

pub fn shoot_stuff(
    mut commands: Commands,
    mut unit_entities: Query<(Entity, &GlobalTransform, &mut UnitData)>,
    mut target: Query<&GlobalTransform>,
    time: Res<Time>,
    props: Res<PropAssets>,
    difficulty: Res<Difficulty>,
) {
    for (_unit_entity, unit_trans, mut unit) in &mut unit_entities {
        unit.fire_cooldown -= unit.fire_rate * time.delta_seconds();
        if unit.fire_cooldown > 0.0 {
            continue;
        }
        let unit_trans = unit_trans.compute_transform();
        if let UnitsStates::Fire = unit.current_state {
            if let Some(target_to_apply_damage) = unit.target_to_apply_damage {
                if let Some(target_to_shoot) = unit.target_to_shoot {
                    if let Ok(player_trans) = target.get_mut(target_to_apply_damage) {
                        unit.fire_cooldown = 1.0; //reset cooldown

                        // head level
                        let start_pos =
                            unit_trans.translation + Vec3::Y * 1.65 + unit_trans.right() * 0.2;
                        // XD (so we don't come directly at players camera)
                        let target =
                            target_to_shoot - vec3(0.01, 0.1, 0.01) + player_trans.left() * 0.01;

                        commands
                            .spawn(SceneBundle {
                                scene: props.projectile_lite_red.clone(),
                                transform: Transform::from_translation(start_pos)
                                    .looking_at(target, Vec3::Y),
                                ..default()
                            })
                            .insert(Projectile {
                                speed: 100.0,
                                max_dist: 1000.0,
                                dist_trav: 0.0,
                            })
                            .insert(DamagePlayer(difficulty.bot_dmg()));
                    }
                }
            }
        }
    }
}

pub fn blowup(
    mut commands: Commands,
    mut unit_entities: Query<(Entity, &GlobalTransform, &mut UnitData, &Health)>,
    mut rng: ResMut<GameRng>,
    props: Res<PropAssets>,
) {
    for (entity, trans, _unit, health) in &mut unit_entities {
        if health.0 <= 0.0 {
            if commands.get_entity(entity).is_some() {
                commands.entity(entity).despawn_recursive();
            }
            for _ in 0..16 {
                let origin = trans.translation() + trans.up();
                commands
                    .spawn(SceneBundle {
                        scene: props.projectile_lite_red.clone(),
                        transform: Transform::from_translation(origin).looking_at(
                            origin
                                + vec3(
                                    rng.gen_range(-1.0..1.0),
                                    rng.gen_range(0.4..1.0),
                                    rng.gen_range(-1.0..1.0),
                                ),
                            Vec3::Y,
                        ),
                        ..default()
                    })
                    .insert(Projectile {
                        speed: 35.0,
                        max_dist: 20.0,
                        dist_trav: 0.0,
                    });
            }
        }
    }
}

#[derive(Component)]
struct DamagePlayer(f32);

fn damage_player(
    mut player: Query<(&GlobalTransform, &LogicalPlayerEntity, &mut Health)>,
    mut projectiles: Query<(&GlobalTransform, &mut Projectile, &DamagePlayer)>,
) {
    for (player_trans, _, mut health) in &mut player {
        for (proj_trans, _proj, damage) in &mut projectiles {
            if player_trans
                .translation()
                .distance(proj_trans.translation())
                < 1.0
            {
                health.0 -= damage.0;
            }
        }
    }
}

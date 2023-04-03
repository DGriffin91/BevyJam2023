use crate::util::all_children;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PhysicsStuff;
impl Plugin for PhysicsStuff {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            //.add_plugin(RapierDebugRenderPlugin::default())
            .add_system(setup_trimeshe_colliders);
    }
}

#[derive(Component)]
pub struct AddTrimeshPhysics;

pub fn setup_trimeshe_colliders(
    mut commands: Commands,
    scene_entities: Query<Entity, With<AddTrimeshPhysics>>,
    children_query: Query<&Children>,
    mesh_handles: Query<&Handle<Mesh>>,
    meshes: Res<Assets<Mesh>>,
) {
    for entity in scene_entities.iter() {
        if let Ok(children) = children_query.get(entity) {
            all_children(children, &children_query, &mut |entity| {
                if let Ok(mesh_h) = mesh_handles.get_component(entity) {
                    let mesh = meshes.get(mesh_h).unwrap();
                    // TODO seems inefficient if there are multiple instances of the same trimesh collider
                    commands.entity(entity).insert((
                        Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap(),
                        RigidBody::Fixed,
                    ));
                }
            });
            commands.entity(entity).remove::<AddTrimeshPhysics>();
        }
    }
}

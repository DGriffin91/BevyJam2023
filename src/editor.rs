use bevy::prelude::*;
use bevy_basic_camera::CameraController;
use bevy_editor_pls::{
    controls::{self, EditorControls},
    default_windows::cameras::EditorCamera,
    editor::EditorEvent,
    prelude::*,
};

pub struct GameEditorPlugin;
impl Plugin for GameEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EditorPlugin::new())
            .add_startup_system(set_cam3d_controls)
            .insert_resource(editor_controls())
            .add_system(sync_editor_free_camera);
    }
}

fn editor_controls() -> EditorControls {
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(controls::Action::PlayPauseEditor);

    editor_controls.insert(
        controls::Action::PlayPauseEditor,
        controls::Binding {
            input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::Escape)),
            conditions: vec![controls::BindingCondition::ListeningForText(false)],
        },
    );

    editor_controls
}

fn set_cam3d_controls(
    mut query: Query<
        &mut bevy_editor_pls::default_windows::cameras::camera_3d_free::FlycamControls,
    >,
) {
    let mut controls = query.single_mut();
    controls.key_up = KeyCode::E;
    controls.key_down = KeyCode::Q;
}

fn sync_editor_free_camera(
    mut d3_cam: Query<&mut Transform, (With<EditorCamera>, Without<CameraController>)>,
    player_cam: Query<&Transform, With<CameraController>>,
    mut editor_events: EventReader<EditorEvent>,
) {
    for editor_event in editor_events.iter() {
        if let EditorEvent::Toggle { now_active } = editor_event {
            if *now_active {
                for mut cam in d3_cam.iter_mut() {
                    *cam = *player_cam.single();
                }
            }
        }
    }
}

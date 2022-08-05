use bevy::prelude::*;
use bevy_mod_picking::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod board;
mod pieces;
mod ui;
mod movement;

use board::*;
use pieces::*;
use ui::*;
use movement::*;

fn main() {
    App::new()
    // set antialiasing to use 4 samples.
    .insert_resource(Msaa { samples: 4 })
    // set WindowsDescriptor Resource and change title and size
    .insert_resource(WindowDescriptor {
        title: "Chess!".to_string(),
        width: 1600.,
        height: 1600.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugins(DefaultPickingPlugins)
    .add_plugin(DebugCursorPickingPlugin)
    .add_plugin(BoardPlugin)
    .add_plugin(PiecesPlugin)
    .add_plugin(UIPlugin)
    .add_plugin(MovementPlugin)
    .add_plugin(WorldInspectorPlugin::new())
    .add_startup_system(setup_system)
    .add_system(bevy::input::system::exit_on_esc_system)
    .run();
}

fn setup_system(mut commands: Commands) {
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(), 
            Vec3::new(-4.0, 15.0, 4.0),
        )),
        ..Default::default()
    })
    .insert_bundle(PickingCameraBundle::default());
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}

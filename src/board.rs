use bevy::{prelude::*};
use bevy_mod_picking::*;

#[derive(Default, Component)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}
#[derive(Default)]
pub struct SelectedSquare {
    pub entity: Option<Entity>,
}
#[derive(Default)]
pub struct SelectedPiece {
    pub entity: Option<Entity>,
}


pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<Square>()
            .init_resource::<SelectedPiece>()
            .add_startup_system(create_board);
    }
}

fn create_board(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));
    // plane

    let white_color = materials.add(Color::rgb(1.0, 0.9, 0.9).into());
    let black_color = materials.add(Color::rgb(0.0, 0.1, 0.1).into());
    for i in 0..8 {
        for j in 0..8 {
            let mut name = String::from("Square ");
            name.push_str(&i.to_string());
            name.push_str("-");
            name.push_str(&j.to_string());
            commands.spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                // Change material according to position to get alternating pattern
                material: if (i + j + 1) % 2 == 0 {
                    white_color.clone()
                } else {
                    black_color.clone()
                },
                transform: Transform::from_translation(Vec3::new(i as f32, 0.0, j as f32)),
                ..Default::default()
            })
            .insert_bundle(PickableBundle::default())
            .insert(Square {x: i, y: j})
            .insert(Name::new(name));
        }
    }      
}

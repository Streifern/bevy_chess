use crate::{pieces::*, movement::*};
use bevy::prelude::*;

// Component to mark the Text entity
#[derive(Component)]
struct NextMoveText;

/// Initialize UiCamera and text
fn init_next_move_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    // with one section
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.0),
                    top: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            // use the text::with_section constructor
            text: Text::with_section(
                "Next move: White",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.8, 0.8, 0.8),
                    ..default()
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                }
            ),
        ..default()
        })
        .insert(NextMoveText);
}
 
/// Update text with the correct turn
fn next_move_text_update(
    turn: Res<PlayerTurn>,
    mut query: Query<&mut Text, With<NextMoveText>>,
) {
    if turn.is_changed() == false {
        return ;
    }

    for mut text in query.iter_mut() {
        text.sections[0].value = format!(
            "Next move: {}",
            match turn.0 {
                PieceColor::White => "White",
                PieceColor::Black => "Black",
            }
        );
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_next_move_text)
            .add_system(next_move_text_update);
    }
}
use bevy::{prelude::*, ecs::event::Events, app::AppExit};
use bevy_mod_picking::*;

use crate::pieces::*;
use crate::board::*;

pub struct PlayerTurn(pub PieceColor);

pub struct MovementPlugin;

pub struct ResetSelectedEvent;

impl Default for PlayerTurn {
    fn default() -> Self {
        Self(PieceColor::White)
    }
}

impl PlayerTurn {
    pub fn change(&mut self) {
        self.0 = match self.0 {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
            app
            .init_resource::<PlayerTurn>()
            .add_event::<ResetSelectedEvent>()
            .add_system(select_square)
            .add_system(select_piece)
            .add_system(initiate_move)
            .add_system(move_piece)
            .add_system(despawn_taken_pieces)
            .add_system(reset_selected);
    }
}

fn select_square(
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut events: EventReader<PickingEvent>, 
    square_query: Query<&Square>,
) {

    for event in events.iter() {

        match event {
            PickingEvent::Selection(_e) => {},
            PickingEvent::Hover(_e) => {},
            PickingEvent::Clicked(clicked_entity) => {
                // get the actual square. This ensures it exists and is a square
                if let Ok(_square) =  square_query.get(*clicked_entity) {
                    // mark it as selected
                    selected_square.entity = Some(*clicked_entity);

                } else {
                    // player clicked outside the board, deselect everything
                    selected_piece.entity = None;
                    selected_square.entity = None;
                }
            }
        }  
    } 
}

fn select_piece(
    selected_square: Res<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    turn: Res<PlayerTurn>,
    square_query: Query<&Square>,
    pieces_query: Query<(Entity, &Piece)>,
) {
    // only run if selected square is chaged.
    if !selected_square.is_changed() {
        return;
    }
    // check for entity of the selected square and get it
    let square_entity = if let Some(entity) = selected_square.entity {
        entity
    } else {
        return;
    };
    // check for square component and get it
    let square = if let Ok(square) = square_query.get(square_entity) {
        square
    } else {
        return;
    };

    if selected_piece.entity.is_none() {
        // find piece in the currently selected square and check for correct turn
        for (piece_entity, piece) in pieces_query.iter() {
            if piece.x == square.x && piece.y == square.y && piece.color == turn.0 {
                // pass entity for piece in selected_square to selected_piece
                selected_piece.entity = Some(piece_entity);
                break;
            }
        }
    }
}

fn initiate_move(
    mut commands: Commands,
    selected_square: Res<SelectedSquare>,
    selected_piece: Res<SelectedPiece>,
    mut turn: ResMut<PlayerTurn>,
    square_query: Query<&Square>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
    mut reset_selected_event: EventWriter<ResetSelectedEvent>,
) {
    // only run if selected square is chaged.
    if !selected_square.is_changed() {
        return;
    }
    // check for entity of the selected square and get it
    let square_entity = if let Some(entity) = selected_square.entity {
        entity
    } else {
        return;
    };
    // check for square component and get it
    let square = if let Ok(square) = square_query.get(square_entity) {
        square
    } else {
        return;
    };

    if let Some(selected_piece_entity) = selected_piece.entity {
        // map all pieces to vecs for passing to functions
        let pieces_vec = pieces_query
                            .iter_mut()
                            .map(|(_, piece)| *piece)
                            .collect();
        
        let pieces_entity_vec = pieces_query
                            .iter_mut()
                            .map(|(entity, piece)| (entity, *piece))
                            .collect::<Vec<(Entity, Piece)>>();
        // move the selected piece to the selected square
        let mut piece = if let Ok((_piece_entity, piece)) = pieces_query.get_mut(selected_piece_entity) {
            piece
        } else {
            return;
        };

        if piece.is_move_valid((square.x, square.y), pieces_vec) {
            // check if a piece of the opposite color exists in this square and despawn it.
            for (other_entity, other_pieces) in pieces_entity_vec {
                if other_pieces.x == square.x
                    && other_pieces.y == square.y
                    && other_pieces.color != piece.color
                {
                    // mark the piece as taken
                    commands.entity(other_entity)
                        .insert(Taken);
                }
            }

            // move piece
            piece.x = square.x;
            piece.y = square.y;
            // change turn;
            turn.change();
        }
        reset_selected_event.send(ResetSelectedEvent);
    }      
}

fn move_piece(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Piece)>
) {
    for (mut transform, piece) in query.iter_mut() {
        // get the direction to move in
        let direction = Vec3::new(piece.x as f32, 0.0, piece.y as f32) - transform.translation;
        
        // only move if the piece isn't already there (distance is big)
        if direction.length() > 0.05 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    } 
}

fn reset_selected(
    mut event_reader: EventReader<ResetSelectedEvent>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
) {
    for _event in event_reader.iter() {
        selected_square.entity = None;
        selected_piece.entity = None;
    }
}

fn despawn_taken_pieces(
    mut commands: Commands,
    mut app_exit_events: ResMut<Events<AppExit>>,
    query: Query<(Entity, &Piece, &Taken)>,
) {
    for (entity, piece, _taken) in query.iter() {
        // if king is taken, exit the game
        if piece.piece_type == PieceType::King {
            println!("{} won! Thanks for playing!", match piece.color {
                PieceColor::White => "Black",
                PieceColor::Black => "White",
            }
        );
        app_exit_events.send(AppExit);
    }
    // despawn piece and children
    commands.entity(entity).despawn_recursive();
    }
}
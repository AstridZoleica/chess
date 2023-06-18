// HashMap for easily accessing named things with ids, including the starting positions.
use std::collections::HashMap;

// Declare some modules.
mod configuration;
use crate::configuration::*;
mod game;
use crate::game::*;

fn main() {
    println!("Hello, world!");
    // Set global variables, namely the game_counter and the hashmaps.
    let mut game_counter: u64 = 0;
    // Load in the pieces from the configuration file.
    let piece_list: PieceList = load_piece_list().unwrap();
    // Create the hashmap which pairs PieceTypes and their symbols for recognition.
    let mut piece_symbol_map: HashMap<char, &PieceType> = piece_list.map_piecetypes_to_symbols();
    // dbg!(&piece_symbol_map); // Had some issues before, this was for debugging.
    // piece_list_console_diagnostics(piece_list);
    // Load in the positions provided in the configuration file. Must be done after loading in the PieceTypes since the FEN has symbols that correspond to pieces.
    let position_list: PositionListIntermediateRepresentation =
        PositionListIntermediateRepresentation::new();
    // Create the hashmap which pairs the names of positions with their FENs.
    let mut position_name_map: HashMap<String, String> = position_list.map_positions_to_names();
    // Initialize a game.
    let game: Game = Game::new(
        game_counter,
        String::from("standard"),
        piece_symbol_map,
        position_name_map,
    );
    // Fill a HashMap with the piece IDs and references to the pieces.
    let mut piece_id_map: HashMap<u8, &Piece> = game.map_pieces_to_ids();
    game.print_piece_id_map();
    game.print_piece_symbol_map();
}

// Debugging Functions

fn piece_list_console_diagnostics(piece_list: PieceList) {
    for i in piece_list.pieces {
        println!("{}", i.name.to_uppercase());
        println!("{}", i.white_id);
        println!("{}", i.black_id);
        for j in i.moveset {
            println!("Move ID {}", j.id);
            println!("Translation {} {}", j.translation.0, j.translation.1);
            println!("Moves? {}", j.moves);
            println!("Captures? {}", j.captures);
            println!("Requires Previous move? {}", j.piece_makes_previous_move);
            if j.piece_makes_previous_move {
                println!("What is the previous move? {}", j.previous_move.unwrap());
            }
            println!("Conditional based on position? {}", j.requires_target_piece);
            if j.requires_target_piece {
                println!(
                    "Target piece player {} and ID {}",
                    j.target_piece_player, j.target_piece_id
                );
            }
            println!("Castles? {}", j.castles);
            if j.castles {
                println!(
                    "Castling target piece player {} and ID {}",
                    j.castle_target_piece_player, j.castle_target_piece_id
                );
                println!(
                    "Castling target relative position {} {} {} {}",
                    j.castle_target_piece_relative_location.0,
                    j.castle_target_piece_relative_location.1,
                    j.castle_target_piece_relative_location.2,
                    j.castle_target_piece_relative_location.3
                );
                println!(
                    "Castling target movement {} {} {} {}",
                    j.castle_target_piece_movement.0,
                    j.castle_target_piece_movement.1,
                    j.castle_target_piece_movement.2,
                    j.castle_target_piece_movement.3
                );
            }
            println!("En passant? {}", j.enpassant);
            if j.enpassant {
                println!(
                    "EnPassant target piece player {} and ID {}",
                    j.enpassant_target_piece_player.unwrap(),
                    j.enpassant_target_piece_id.unwrap()
                );
                println!(
                    "EnPassant target piece relative location {} {} {} {}",
                    j.enpassant_target_piece_relative_location.unwrap().0,
                    j.enpassant_target_piece_relative_location.unwrap().1,
                    j.enpassant_target_piece_relative_location.unwrap().2,
                    j.enpassant_target_piece_relative_location.unwrap().3
                );
                println!(
                    "EnPassant target piece previous move: {}",
                    j.enpassant_target_piece_previous_move.unwrap()
                );
            }
            println!("--");
        }
        println!("Promotable: {}", i.promotable);
        println!("Promotes to: {}", i.promotes_to);
        println!("- - - - - - - - - - - -");
    }
}

const BOARDSIZE: u8 = 8;
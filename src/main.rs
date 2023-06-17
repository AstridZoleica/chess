// HashMap for easily accessing named things with ids, including the starting positions.
use std::collections::HashMap;

// Declare some modules.
mod configuration;
use crate::configuration::*;

fn main() {
    println!("Hello, world!");
    // Set global variables, namely the game_counter and the hashmaps.
    let mut game_counter:u64 = 0;
    // Load in the pieces from the configuration file.
    let piece_list: PieceList = load_piece_list().unwrap();
    // Create the hashmap which pairs PieceTypes and their symbols for recognition.
    let mut piece_symbol_map: HashMap<char, &PieceType> = piece_list.map_piecetypes_to_symbols();
    // dbg!(&piece_symbol_map); // Had some issues before, this was for debugging.
    // piece_list_console_diagnostics(piece_list);
    // Load in the positions provided in the configuration file. Must be done after loading in the PieceTypes since the FEN has symbols that correspond to pieces.
    let position_list: PositionListIntermediateRepresentation = PositionListIntermediateRepresentation::new();
    // Create the hashmap which pairs the names of positions with their FENs.
    let mut position_name_map: HashMap<String, String> = position_list.map_positions_to_names();
    // Initialize a game.
    let game: Game = Game::new(game_counter, String::from("standard"), piece_symbol_map, position_name_map);
    // Fill a HashMap with the piece IDs and references to the pieces.
    let mut piece_id_map: HashMap<u8, &Piece> = game.map_pieces_to_ids();
    print_piece_id_map(game);
}

// Debugging Functions
fn print_piece_id_map(game: Game<'_>) {
    println!("Piece ID Map");
    println!("{}  {}  {}  {}  {}  {}  {}  {}", game.position[0], game.position[1], game.position[2], game.position[3], game.position[4], game.position[5], game.position[6], game.position[7]);
    println!("{}  {}  {}  {}  {}  {}  {}  {}", game.position[8], game.position[9], game.position[10], game.position[11], game.position[12], game.position[13], game.position[14], game.position[15]);
    println!("{}  {}  {}  {}  {}  {}  {}  {}", game.position[16], game.position[17], game.position[18], game.position[19], game.position[20], game.position[21], game.position[22], game.position[23]);
    println!("{}  {}  {}  {}  {}  {}  {}  {}", game.position[24], game.position[25], game.position[26], game.position[27], game.position[27], game.position[28], game.position[29], game.position[30]);
    println!("{}  {}  {}  {}  {}  {}  {}  {}", game.position[32], game.position[33], game.position[34], game.position[35], game.position[36], game.position[37], game.position[38], game.position[39]);
    println!("{}  {}  {}  {}  {}  {}  {}  {}", game.position[40], game.position[41], game.position[42], game.position[43], game.position[44], game.position[45], game.position[46], game.position[47]);
    println!("{}  {}  {}  {}  {}  {}  {}  {}", game.position[48], game.position[49], game.position[50], game.position[51], game.position[52], game.position[53], game.position[54], game.position[55]);
    println!("{}  {}  {}  {}  {}  {}  {}  {}", game.position[56], game.position[57], game.position[58], game.position[59], game.position[60], game.position[61], game.position[62], game.position[63]);
}

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
                println!("Target piece player {} and ID {}", j.target_piece_player, j.target_piece_id);
            }
            println!("Castles? {}", j.castles);
            if j.castles {
                println!("Castling target piece player {} and ID {}", j.castle_target_piece_player, j.castle_target_piece_id);
                println!("Castling target relative position {} {} {} {}", j.castle_target_piece_relative_location.0, j.castle_target_piece_relative_location.1, j.castle_target_piece_relative_location.2, j.castle_target_piece_relative_location.3);
                println!("Castling target movement {} {} {} {}", j.castle_target_piece_movement.0, j.castle_target_piece_movement.1, j.castle_target_piece_movement.2, j.castle_target_piece_movement.3);
            }
            println!("En passant? {}", j.enpassant);
            if j.enpassant {
                println!("EnPassant target piece player {} and ID {}", j.enpassant_target_piece_player.unwrap(), j.enpassant_target_piece_id.unwrap());
                println!("EnPassant target piece relative location {} {} {} {}", j.enpassant_target_piece_relative_location.unwrap().0, j.enpassant_target_piece_relative_location.unwrap().1, j.enpassant_target_piece_relative_location.unwrap().2, j.enpassant_target_piece_relative_location.unwrap().3);
                println!("EnPassant target piece previous move: {}", j.enpassant_target_piece_previous_move.unwrap());
            }
            println!("--");
        }
        println!("Promotable: {}", i.promotable);
        println!("Promotes to: {}", i.promotes_to);
        println!("- - - - - - - - - - - -");
    }
}

const BOARDSIZE: u8 = 8;

// Game Data Structure
struct Game<'a> {
    id: u64,
    active_color: char,
    check: bool,
    position: Vec<u8>,
    list_of_pieces_ingame: Vec<Piece<'a>>,
    list_of_moves: Vec<(u8, String)>
}

impl<'a> Game<'a> {
    pub fn new(game_id: u64, starting_position_key: String, piece_hashmap: HashMap<char, &'a PieceType>, position_hashmap: HashMap<String, String>) -> Game<'a> {
        let mut piece_counter:u8 = 1;
        // Check that all the IDs are found in the piece hashmap, otherwise give up lol
        // Hold the pieces and collect them.
        let mut temp_pieces: Vec<Piece> = Vec::new();
        // Position vector as a list of 64 numbers.
        let mut position_vector: Vec<u8> = Vec::new();
        // Fill in empty spaces in FEN notation.
        for i in position_hashmap.get(&starting_position_key).unwrap().chars() {
            // println!("{}", i.clone()); This helps to debug.
            match i {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    let mut c: u32 = 0;
                    while c < i.to_digit(10).unwrap() {
                        position_vector.push(0);
                        c += 1;
                    }
                },
                // Ignore slashes in FEN notation.
                '/' => {},
                // Place pieces into the position vector as their IDs, place them into the piece collection, and increment the piece counter.
                a => {
                    let p: Piece = Piece {
                        id: piece_counter,
                        player: if a.is_uppercase() {
                            'w'
                        } else {
                            'b'
                        },
                        piece_type: piece_hashmap.get(&a).unwrap(),
                        has_castled: false,
                        list_of_moves: Vec::new()
                    };
                    piece_counter += 1;
                    position_vector.push(p.id);
                    temp_pieces.push(p);
                }
            }
        }
        let mut game: Game = Game {
            id: game_id,
            active_color: 'w',
            check: false,
            position: position_vector,
            list_of_pieces_ingame: temp_pieces,
            list_of_moves: Vec::new(),
        };
        game
    }

    pub fn map_pieces_to_ids(&self) -> HashMap<u8, &Piece> {
        let mut output: HashMap<u8, &Piece> = HashMap::new();
        for i in &self.list_of_pieces_ingame {
            output.insert(i.id, &i);
        }
        // for i in &game.list_of_pieces_ingame {
            // println!("{}", i.id);
            // println!("{}", i.player);
            // println!("{}", i.piece_type.black_id);
        // }
        output
    }
}
// HashMap for easily accessing named things with ids, including the starting positions.
use std::collections::HashMap;

// Declare some modules.
mod configuration;
use crate::configuration::*;

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

// Game Data Structure
struct Game<'a> {
    id: u64,
    active_color: char,
    check: bool,
    position: Vec<u8>,
    list_of_pieces_ingame: Vec<Piece<'a>>,
    list_of_moves: Vec<(u8, String)>,
}

impl<'a> Game<'a> {
    pub fn new(
        game_id: u64,
        starting_position_key: String,
        piece_hashmap: HashMap<char, &'a PieceType>,
        position_hashmap: HashMap<String, String>,
    ) -> Game<'a> {
        let mut piece_counter: u8 = 1;
        // Check that all the IDs are found in the piece hashmap, otherwise give up lol
        // Hold the pieces and collect them.
        let mut temp_pieces: Vec<Piece> = Vec::new();
        // Position vector as a list of 64 numbers.
        let mut position_vector: Vec<u8> = Vec::new();
        // Fill in empty spaces in FEN notation.
        for i in position_hashmap
            .get(&starting_position_key)
            .unwrap()
            .chars()
        {
            // println!("{}", i.clone()); This helps to debug.
            match i {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    let mut c: u32 = 0;
                    while c < i.to_digit(10).unwrap() {
                        position_vector.push(0);
                        c += 1;
                    }
                }
                // Ignore slashes in FEN notation.
                '/' => {}
                // Place pieces into the position vector as their IDs, place them into the piece collection, and increment the piece counter.
                a => {
                    let p: Piece = Piece {
                        id: piece_counter,
                        player: if a.is_uppercase() { 'w' } else { 'b' },
                        symbol: a,
                        piece_type: piece_hashmap.get(&a).unwrap(),
                        has_castled: false,
                        list_of_moves: Vec::new(),
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

    pub fn print_piece_id_map(&self) {
        println!("Piece ID Map");
        println!(
            "{}  {}  {}  {}  {}  {}  {}  {}",
            self.position[0],
            self.position[1],
            self.position[2],
            self.position[3],
            self.position[4],
            self.position[5],
            self.position[6],
            self.position[7]
        );
        println!(
            "{}  {}  {}  {}  {}  {}  {}  {}",
            self.position[8],
            self.position[9],
            self.position[10],
            self.position[11],
            self.position[12],
            self.position[13],
            self.position[14],
            self.position[15]
        );
        println!(
            "{}  {}  {}  {}  {}  {}  {}  {}",
            self.position[16],
            self.position[17],
            self.position[18],
            self.position[19],
            self.position[20],
            self.position[21],
            self.position[22],
            self.position[23]
        );
        println!(
            "{}  {}  {}  {}  {}  {}  {}  {}",
            self.position[24],
            self.position[25],
            self.position[26],
            self.position[27],
            self.position[27],
            self.position[28],
            self.position[29],
            self.position[30]
        );
        println!(
            "{}  {}  {}  {}  {}  {}  {}  {}",
            self.position[32],
            self.position[33],
            self.position[34],
            self.position[35],
            self.position[36],
            self.position[37],
            self.position[38],
            self.position[39]
        );
        println!(
            "{}  {}  {}  {}  {}  {}  {}  {}",
            self.position[40],
            self.position[41],
            self.position[42],
            self.position[43],
            self.position[44],
            self.position[45],
            self.position[46],
            self.position[47]
        );
        println!(
            "{}  {}  {}  {}  {}  {}  {}  {}",
            self.position[48],
            self.position[49],
            self.position[50],
            self.position[51],
            self.position[52],
            self.position[53],
            self.position[54],
            self.position[55]
        );
        println!(
            "{}  {}  {}  {}  {}  {}  {}  {}",
            self.position[56],
            self.position[57],
            self.position[58],
            self.position[59],
            self.position[60],
            self.position[61],
            self.position[62],
            self.position[63]
        );
    }

    pub fn print_piece_symbol_map(&self) {
        let temp = self.map_pieces_to_ids();
        println!("Current Board");
        let mut output: Vec<char> = Vec::new();
        for i in &self.position {
            if *i != 0 {
                output.push(temp.get(i).unwrap().symbol);
            } else {
                output.push('0');
            }
        }
        println!(
            "{}  {}  {}  {}  {}  {}  {}  {}",
            output[0],
            output[1],
            output[2],
            output[3],
            output[4],
            output[5],
            output[6],
            output[7]
        );
        println!(
            "{}  {}  {}  {}  {}  {}  {}  {}",
            output[8],
            output[9],
            output[10],
            output[11],
            output[12],
            output[13],
            output[14],
            output[15]
        );
        println!(
            "{}  {}  {}  {}  {}  {}  {}  {}",
            output[16],
            output[17],
            output[18],
            output[19],
            output[20],
            output[21],
            output[22],
            output[23]
        );
        println!(
            "{}  {}  {}  {}  {}  {}  {}  {}",
            output[24],
            output[25],
            output[26],
            output[27],
            output[27],
            output[28],
            output[29],
            output[30]
        );
        println!(
            "{}  {}  {}  {}  {}  {}  {}  {}",
            output[32],
            output[33],
            output[34],
            output[35],
            output[36],
            output[37],
            output[38],
            output[39]
        );
        println!(
            "{}  {}  {}  {}  {}  {}  {}  {}",
            output[40],
            output[41],
            output[42],
            output[43],
            output[44],
            output[45],
            output[46],
            output[47]
        );
        println!(
            "{}  {}  {}  {}  {}  {}  {}  {}",
            output[48],
            output[49],
            output[50],
            output[51],
            output[52],
            output[53],
            output[54],
            output[55]
        );
        println!(
            "{}  {}  {}  {}  {}  {}  {}  {}",
            output[56],
            output[57],
            output[58],
            output[59],
            output[60],
            output[61],
            output[62],
            output[63]
        );
    }
}

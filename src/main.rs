use serde::{Deserialize, Serialize};
// use serde_json::{Deserialize, Value} see AA
use serde_json::{Result};

// File Handling.
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Convert characters to u8 in moveset parsing.
use std::convert::TryInto;

// HashMap for easily accessing named things with ids, including the starting positions.
use std::collections::HashMap;


fn main() {
    println!("Hello, world!");
    let mut game_counter:u64 = 0;
    let piece_list: PieceList = load_piece_list().unwrap();
    let mut piece_symbol_map: HashMap<char, &PieceType> = HashMap::new();
    for i in &piece_list.pieces {
        piece_symbol_map.insert(i.white_id, i);
        piece_symbol_map.insert(i.black_id, i);
    }
    // dbg!(&piece_symbol_map); Had some issues before, this was for debugging.
    // piece_list_console_diagnostics(piece_list);
    let position_list: PositionListIntermediateRepresentation = PositionListIntermediateRepresentation {
        positions: parse_startingPositions_json().unwrap()
    };
    let mut position_name_map: HashMap<String, String> = HashMap::new();
    for i in &position_list.positions {
        position_name_map.insert(i.name.clone(), i.fen.clone());
        println!("{}", i.name.clone());
        println!("{}", i.fen.clone());
    }
    let game: Game = Game::new(game_counter, String::from("standard"), piece_symbol_map, position_name_map);
    let mut piece_id_map: HashMap<u8, &Piece> = HashMap::new();
    for i in &game.list_of_pieces_ingame {
        piece_id_map.insert(i.id, &i);
    }
    for i in game.list_of_pieces_ingame {
        println!("{}", i.id);
        println!("{}", i.player);
        println!("{}", i.piece_type.black_id);
    }
    // Refactor this lol.
    println!("Piece ID Map                           Piece Location Map");
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

// Load in the piece list based on the appropriate json file.
fn load_piece_list() -> Result<PieceList> {
    let mut output_piece_list: PieceList = PieceList {
        pieces: Vec::new()
    };
    match parse_pieces_json() {
        Err(why) => panic!("Failed to parse pieces.json because: {}", why),
        Ok(piece_intermediate_representation_vector) => {
            for piece_intermediate_representation in piece_intermediate_representation_vector {
                let temp: Vec<char> = piece_intermediate_representation.id.chars().collect();
                let piece = PieceType {
                    name: piece_intermediate_representation.name,
                    white_id: temp[0],
                    black_id: temp[1],
                    moveset: parse_moveset(piece_intermediate_representation.moves).unwrap(),
                    promotable: piece_intermediate_representation.promotable,
                    promotes_to: piece_intermediate_representation.promotes_to
                };
                output_piece_list.pieces.push(piece);
            }
        },
    }
    Ok(output_piece_list)
}


const BOARDSIZE: u8 = 8;

struct Game<'a> {
    id: u64,
    active_color: char,
    position: Vec<u8>,
    list_of_pieces_ingame: Vec<Piece<'a>>,
    list_of_moves: Vec<(u8, u8, String)>
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
                        player: if (a.is_uppercase()) {
                            'w'
                        } else {
                            'b'
                        },
                        piece_type: piece_hashmap.get(&a).unwrap(),
                        has_castled: false,
                        on_board: true,
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
            position: position_vector,
            list_of_pieces_ingame: temp_pieces,
            list_of_moves: Vec::new(),
        };
        game
    }
}

// ###### HANDLING PIECES ######

//An actual piece on the board.
struct Piece<'a>{
    id: u8,
    player: char,
    piece_type: &'a PieceType,
    has_castled: bool,
    on_board: bool,
    list_of_moves: Vec<String>
}

//All of the pieces types loaded into the program.
#[derive(Debug)]
struct PieceList {
    pieces: Vec<PieceType>
}

//"Piece Type". Stores data about a piece, distinct from the actual pieces on a board which are a different data structure.
#[derive(Debug)]
struct PieceType {
    name: String,
    white_id: char,
    black_id: char,
    moveset: Vec<Move>,
    promotable: bool,
    promotes_to: String
}

//Intermediate Piece List, again for handling serde's output.
#[derive(Deserialize, Debug)]
struct PiecesListIntermediate {
    pieces: Vec<PieceIntermediateRepresentation>
}

//Piece Intermediate Representation. This is what serde plugs its values into.
#[derive(Serialize, Deserialize, Debug)]
struct PieceIntermediateRepresentation {
    name: String,
    id: String,
    moves: Vec<String>,
    promotable: bool,
    promotes_to: String
}

// Data Structure for Each Move
#[derive(Debug)]
struct Move {
    //Moves have IDs in the following format: HV1234cmjnfolmMOVEIDrFI1234srFI1234t1234perFI1234mMOVEID!
    id: String,
    translation: (u8, u8),
    reflections: (bool, bool, bool, bool),
    captures: bool,
    moves: bool,
    jump: bool,
    any_multiple: bool,
    only_first_move: bool,
    once: bool,
    piece_makes_previous_move: bool,
    previous_move: Option<String>,
    requires_target_piece: bool,
    target_piece_player: char,
    target_piece_id: char,
    target_piece_relative_location: (u8, u8, u8, u8),
    castles: bool,
    castle_target_piece_player: char,
    castle_target_piece_id: char,
    castle_target_piece_relative_location: (u8, u8, u8, u8),
    castle_target_piece_movement: (u8, u8, u8, u8),
    castle_target_piece_cannot_move: bool,
    enpassant: bool,
    enpassant_target_piece_player: Option<char>,
    enpassant_target_piece_id: Option<char>,
    enpassant_target_piece_relative_location: Option<(u8, u8, u8, u8)>,
    enpassant_target_piece_previous_move: Option<String>
}

// Accept the Pieces.json file, reading all of the pieces contained within.
fn parse_pieces_json() -> Result<Vec<PieceIntermediateRepresentation>> {
    // Create a path to the desired file
    let path = Path::new("pieces.json");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        // Uncomment this to check what the file contains.
        Ok(_) => (), //print!("{} contains:\n{}", display, s),
    }
    //Convert that String into a str.
    s = s.to_owned();
    let s_slice: &str = &s[..];

    //Make a PiecesListIntermediate out of the string using serde's json parsing.
    let pieces_list: PiecesListIntermediate = serde_json::from_str::<PiecesListIntermediate>(s_slice).unwrap();
    // Iniialize output vector and fill it.
    let mut output: Vec<PieceIntermediateRepresentation> = Vec::new();
    for i in pieces_list.pieces {
        output.push(i);
    }
    
    // AA First attempt to process multiple objects failed
    // let stream = Deserializer::from_str(s_slice).into_iter::<Value>();
    // let mut output: Vec<PieceIntermediateRepresentation> = Vec::new();
    // for object in stream {
    //     let o = object.unwrap();
    //     let p: PieceIntermediateRepresentation = serde_json::from_value(o).unwrap();
    //     output.push(p);
    // }

    //Report the name of each item in the output vector.
    for i in &output {
        println!("{}", i.name)
    }
    Ok(output)
}

// Function for handling the moveset of each piece.
fn parse_moveset(moveslist: Vec<String>) -> Result<Vec<Move>> {
    let mut output: Vec<Move> = Vec::new();
    for move_string in moveslist {
        if move_string.len() == 47 { // This means the move is not an en passant move nor a move which requires certain previous moves and we can sleep well at night. These options allow for recursion, and I will cry at night because this is Rust.
            // Collect the string into a Vector.
            let m_s_chars: Vec<char> = move_string.chars().collect();
            let m:Move = Move {
                // Read char as digit, unwrap the result as u32, convert u32 to u8, unwrap the result.
                id: m_s_chars.clone().into_iter().collect(),
                translation: (m_s_chars[0].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[1].to_digit(10).unwrap().try_into().unwrap()),
                reflections: (
                        match m_s_chars[2] {
                            '0' => Ok(false),
                            '1' => Ok(true),
                            _ => Err("Invalid character in Reflections portion of moveID. Must be 0 or 1!"),
                        }.unwrap(),
                        match m_s_chars[3] {
                            '0' => Ok(false),
                            '2' => Ok(true),
                            _ => Err("Invalid character in Reflections portion of moveID. Must be 0 or 2!"),
                        }.unwrap(),
                        match m_s_chars[4] {
                            '0' => Ok(false),
                            '3' => Ok(true),
                            _ => Err("Invalid character in Reflections portion of moveID. Must be 0 or 3!"),
                        }.unwrap(),
                        match m_s_chars[5] {
                            '0' => Ok(false),
                            '4' => Ok(true),
                            _ => Err("Invalid character in Reflections portion of moveID. Must be 0 or 4!"),
                        }.unwrap()
                    ),
                captures: match m_s_chars[6] {
                    '0' => Ok(false),
                    'c' => Ok(true),
                    _ => Err("Invalid character in Captures slot of moveID. Must be 0 or c")
                }.unwrap(),
                moves: match m_s_chars[7] {
                    '0' => Ok(false),
                    'm' => Ok(true),
                    _ => Err("Invalid character in Moves slot of moveID. Must be 0 or m")
                }.unwrap(),
                jump: match m_s_chars[8] {
                    '0' => Ok(false),
                    'j' => Ok(true),
                    _ => Err("Invalid character in Jumps slot of moveID. Must be 0 or j")
                }.unwrap(),
                any_multiple: match m_s_chars[9] {
                    '0' => Ok(false),
                    'n' => Ok(true),
                    _ => Err("Invalid character in aNy multiples slot of moveID. Must be 0 or n")
                }.unwrap(),
                only_first_move: match m_s_chars[10] {
                    '0' => Ok(false),
                    'f' => Ok(true),
                    _ => Err("Invalid character in only First move slot of moveID. Must be 0 or f")
                }.unwrap(),
                once: match m_s_chars[11] {
                    '0' => Ok(false),
                    'o' => Ok(true),
                    _ => Err("Invalid character in only Once slot of moveID. Must be 0 or o")
                }.unwrap(),
                piece_makes_previous_move: false, // We can say this because of the length of the string.
                previous_move: None,
                requires_target_piece: if (m_s_chars[16], m_s_chars[17], m_s_chars[18], m_s_chars[19], m_s_chars[20], m_s_chars[21]) == ('0','0', '0', '0', '0', '0') {
                    false
                } else {
                    true
                },
                target_piece_player: m_s_chars[16],
                target_piece_id: m_s_chars[17],
                target_piece_relative_location: (m_s_chars[0].to_digit(18).unwrap().try_into().unwrap(), m_s_chars[1].to_digit(19).unwrap().try_into().unwrap(), m_s_chars[1].to_digit(20).unwrap().try_into().unwrap(), m_s_chars[1].to_digit(21).unwrap().try_into().unwrap()),
                castles: if (m_s_chars[24], m_s_chars[25], m_s_chars[26], m_s_chars[27], m_s_chars[28], m_s_chars[29]) == ('0','0', '0', '0', '0', '0') {
                    false
                } else {
                    true
                },
                castle_target_piece_player: m_s_chars[24],
                castle_target_piece_id: m_s_chars[25],
                castle_target_piece_relative_location: (m_s_chars[0].to_digit(26).unwrap().try_into().unwrap(), m_s_chars[1].to_digit(27).unwrap().try_into().unwrap(), m_s_chars[1].to_digit(28).unwrap().try_into().unwrap(), m_s_chars[1].to_digit(29).unwrap().try_into().unwrap()),
                castle_target_piece_movement: (m_s_chars[31].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[32].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[33].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[34].to_digit(10).unwrap().try_into().unwrap()),
                castle_target_piece_cannot_move: match m_s_chars[35] {
                    '0' => Ok(false),
                    'p' => Ok(true),
                    _ => Err("Invalid character in Previous moves slot of moveID. Must be 0 or p")
                }.unwrap(),
                enpassant: false, // We can say this because of the length of the string.
                enpassant_target_piece_player: None,
                enpassant_target_piece_id: None,
                enpassant_target_piece_relative_location: None,
                enpassant_target_piece_previous_move: None
            };
            output.push(m);
        } else { // Now we need to handle cases where another moveID is nested within this moveID, so either a previous move (self) or en passant (target piece previous move).
            let m_s_chars: Vec<char> = move_string.chars().collect();
            // First check if we only need to handle en passant. This will help a great deal.
            if (m_s_chars[12], m_s_chars[13], m_s_chars[14], m_s_chars[15]) == ('l', 'M', '0', 'r') {
                let m:Move = Move {
                    id: m_s_chars.clone().into_iter().collect(),
                    // Read char as digit, unwrap the result as u32, convert u32 to u8, unwrap the result.
                    translation: (m_s_chars[0].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[1].to_digit(10).unwrap().try_into().unwrap()),
                    reflections: (
                            match m_s_chars[2] {
                                '0' => Ok(false),
                                '1' => Ok(true),
                                _ => Err("Invalid character in Reflections portion of moveID. Must be 0 or 1!"),
                            }.unwrap(),
                            match m_s_chars[3] {
                                '0' => Ok(false),
                                '2' => Ok(true),
                                _ => Err("Invalid character in Reflections portion of moveID. Must be 0 or 2!"),
                            }.unwrap(),
                            match m_s_chars[4] {
                                '0' => Ok(false),
                                '3' => Ok(true),
                                _ => Err("Invalid character in Reflections portion of moveID. Must be 0 or 3!"),
                            }.unwrap(),
                            match m_s_chars[5] {
                                '0' => Ok(false),
                                '4' => Ok(true),
                                _ => Err("Invalid character in Reflections portion of moveID. Must be 0 or 4!"),
                            }.unwrap()
                        ),
                    captures: match m_s_chars[6] {
                        '0' => Ok(false),
                        'c' => Ok(true),
                        _ => Err("Invalid character in Captures slot of moveID. Must be 0 or c")
                    }.unwrap(),
                    moves: match m_s_chars[7] {
                        '0' => Ok(false),
                        'm' => Ok(true),
                        _ => Err("Invalid character in Moves slot of moveID. Must be 0 or m")
                    }.unwrap(),
                    jump: match m_s_chars[8] {
                        '0' => Ok(false),
                        'j' => Ok(true),
                        _ => Err("Invalid character in Jumps slot of moveID. Must be 0 or j")
                    }.unwrap(),
                    any_multiple: match m_s_chars[9] {
                        '0' => Ok(false),
                        'n' => Ok(true),
                        _ => Err("Invalid character in aNy multiples slot of moveID. Must be 0 or n")
                    }.unwrap(),
                    only_first_move: match m_s_chars[10] {
                        '0' => Ok(false),
                        'f' => Ok(true),
                        _ => Err("Invalid character in only First move slot of moveID. Must be 0 or f")
                    }.unwrap(),
                    once: match m_s_chars[11] {
                        '0' => Ok(false),
                        'o' => Ok(true),
                        _ => Err("Invalid character in only Once slot of moveID. Must be 0 or o")
                    }.unwrap(),
                    piece_makes_previous_move: false, // We can say this because of the length of the string.
                    previous_move: None,
                    requires_target_piece: if (m_s_chars[16], m_s_chars[17], m_s_chars[18], m_s_chars[19], m_s_chars[20], m_s_chars[21]) == ('0','0', '0', '0', '0', '0') {
                        false
                    } else {
                        true
                    },
                    target_piece_player: m_s_chars[16],
                    target_piece_id: m_s_chars[17],
                    target_piece_relative_location: (m_s_chars[18].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[19].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[20].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[21].to_digit(10).unwrap().try_into().unwrap()),
                    castles: if (m_s_chars[24], m_s_chars[25], m_s_chars[26], m_s_chars[27], m_s_chars[28], m_s_chars[29]) == ('0','0', '0', '0', '0', '0') {
                        false
                    } else {
                        true
                    },
                    castle_target_piece_player: m_s_chars[24],
                    castle_target_piece_id: m_s_chars[25],
                    castle_target_piece_relative_location: (m_s_chars[26].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[27].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[28].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[29].to_digit(10).unwrap().try_into().unwrap()),
                    castle_target_piece_movement: (m_s_chars[31].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[32].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[33].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[34].to_digit(10).unwrap().try_into().unwrap()),
                    castle_target_piece_cannot_move: match m_s_chars[35] {
                        '0' => Ok(false),
                        'p' => Ok(true),
                        _ => Err("Invalid character in Previous moves slot of moveID. Must be 0 or p")
                    }.unwrap(),
                    enpassant: true,
                    enpassant_target_piece_player: Some(m_s_chars[38]),
                    enpassant_target_piece_id: Some(m_s_chars[39]),
                    enpassant_target_piece_relative_location: Some((m_s_chars[40].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[41].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[42].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[43].to_digit(10).unwrap().try_into().unwrap())),
                    // Feed the remaining characters until the end of the string into a String.
                    enpassant_target_piece_previous_move: Some(
                        {
                            let mut temp_output: String = String::new();
                            let mut i = 45;
                            while i < m_s_chars.len() - 1 {
                                temp_output.push(m_s_chars[i]);
                                i += 1;
                            }
                            temp_output
                        }
                    )
                };
                output.push(m);
            } else if (m_s_chars[m_s_chars.len() - 3], m_s_chars[m_s_chars.len() - 2], m_s_chars[m_s_chars.len() - 1]) == ('M', '0', '!') { // Now handle cases where we only need to deal with a previous move. This means that the last characters are M0!, indicating no move specified in the en passant condition (would appear as M0!! or with more exclamation points depending on nesting).
                let mut end_index = 0; // Use this to determine the end of the previous move's string.
                let m:Move = Move {
                    id: m_s_chars.clone().into_iter().collect(),
                    // Read char as digit, unwrap the result as u32, convert u32 to u8, unwrap the result.
                    translation: (m_s_chars[0].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[1].to_digit(10).unwrap().try_into().unwrap()),
                    reflections: (
                            match m_s_chars[2] {
                                '0' => Ok(false),
                                '1' => Ok(true),
                                _ => Err("Invalid character in Reflections portion of moveID. Must be 0 or 1!"),
                            }.unwrap(),
                            match m_s_chars[3] {
                                '0' => Ok(false),
                                '2' => Ok(true),
                                _ => Err("Invalid character in Reflections portion of moveID. Must be 0 or 2!"),
                            }.unwrap(),
                            match m_s_chars[4] {
                                '0' => Ok(false),
                                '3' => Ok(true),
                                _ => Err("Invalid character in Reflections portion of moveID. Must be 0 or 3!"),
                            }.unwrap(),
                            match m_s_chars[5] {
                                '0' => Ok(false),
                                '4' => Ok(true),
                                _ => Err("Invalid character in Reflections portion of moveID. Must be 0 or 4!"),
                            }.unwrap()
                        ),
                    captures: match m_s_chars[6] {
                        '0' => Ok(false),
                        'c' => Ok(true),
                        _ => Err("Invalid character in Captures slot of moveID. Must be 0 or c")
                    }.unwrap(),
                    moves: match m_s_chars[7] {
                        '0' => Ok(false),
                        'm' => Ok(true),
                        _ => Err("Invalid character in Moves slot of moveID. Must be 0 or m")
                    }.unwrap(),
                    jump: match m_s_chars[8] {
                        '0' => Ok(false),
                        'j' => Ok(true),
                        _ => Err("Invalid character in Jumps slot of moveID. Must be 0 or j")
                    }.unwrap(),
                    any_multiple: match m_s_chars[9] {
                        '0' => Ok(false),
                        'n' => Ok(true),
                        _ => Err("Invalid character in aNy multiples slot of moveID. Must be 0 or n")
                    }.unwrap(),
                    only_first_move: match m_s_chars[10] {
                        '0' => Ok(false),
                        'f' => Ok(true),
                        _ => Err("Invalid character in only First move slot of moveID. Must be 0 or f")
                    }.unwrap(),
                    once: match m_s_chars[11] {
                        '0' => Ok(false),
                        'o' => Ok(true),
                        _ => Err("Invalid character in only Once slot of moveID. Must be 0 or o")
                    }.unwrap(),
                    piece_makes_previous_move: true,
                    previous_move: Some(
                        {
                            let mut clone_a_new_str: String = String::new();
                            for c in m_s_chars.clone().into_iter() {
                                clone_a_new_str.push(c);
                            };
                            //Use rmatch indices to find the second to last exclamation point.
                            let results: Vec<_> = clone_a_new_str.rmatch_indices('!').collect();
                            (end_index, _) = results[1];
                            let mut output_previous_move_string: String = String::new();
                            let mut counter = 14;
                            while counter < (end_index + 1) {
                                output_previous_move_string.push(m_s_chars[counter]);
                                counter += 1;
                            }
                            output_previous_move_string
                        }
                    ),
                    requires_target_piece: if (m_s_chars[end_index + 2], m_s_chars[end_index + 3], m_s_chars[end_index + 4], m_s_chars[end_index + 5], m_s_chars[end_index + 6], m_s_chars[end_index + 7]) == ('0','0', '0', '0', '0', '0') {
                        false
                    } else {
                        true
                    },
                    target_piece_player: m_s_chars[end_index + 2],
                    target_piece_id: m_s_chars[end_index + 3],
                    target_piece_relative_location: (m_s_chars[end_index + 4].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[end_index + 5].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[end_index + 6].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[end_index + 7].to_digit(10).unwrap().try_into().unwrap()),
                    castles: if (m_s_chars[end_index + 10], m_s_chars[end_index + 11], m_s_chars[end_index + 12], m_s_chars[end_index + 13], m_s_chars[end_index + 14], m_s_chars[end_index + 15]) == ('0','0', '0', '0', '0', '0') {
                        false
                    } else {
                        true
                    },
                    castle_target_piece_player: m_s_chars[end_index + 10],
                    castle_target_piece_id: m_s_chars[end_index + 11],
                    castle_target_piece_relative_location: (m_s_chars[end_index + 12].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[end_index + 13].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[end_index + 14].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[end_index + 15].to_digit(10).unwrap().try_into().unwrap()),
                    castle_target_piece_movement: (m_s_chars[end_index + 17].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[end_index + 18].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[end_index + 19].to_digit(10).unwrap().try_into().unwrap(), m_s_chars[end_index + 20].to_digit(10).unwrap().try_into().unwrap()),
                    castle_target_piece_cannot_move: match m_s_chars[end_index + 21] {
                        '0' => Ok(false),
                        'p' => Ok(true),
                        _ => Err("Invalid character in Previous moves slot of moveID. Must be 0 or p")
                    }.unwrap(),
                    enpassant: false,
                    enpassant_target_piece_player: None,
                    enpassant_target_piece_id: None,
                    enpassant_target_piece_relative_location: None,
                    enpassant_target_piece_previous_move: None
                };
                output.push(m);
            } else { // Final set of cases in which I cry myself to sleep. This covers situations in which you have both.
            }
        }
    }
    Ok(output)
}

// ###### HANDLING POSITIONS ######
//Intermediate Position List, again for handling serde's output.
#[derive(Deserialize, Debug)]
struct PositionListIntermediateRepresentation {
    positions: Vec<PositionIntermediateRepresentation>
}

//Position Intermediate Representation. This is what serde plugs its values into.
#[derive(Serialize, Deserialize, Debug)]
struct PositionIntermediateRepresentation {
    name: String,
    fen: String
}

// Accept the startingPositions.json file, reading all of the partial FEN notations contained within.
fn parse_startingPositions_json() -> Result<Vec<PositionIntermediateRepresentation>> {
    // Create a path to the desired file
    let path = Path::new("startingPositions.json");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        // Uncomment this to check what the file contains.
        Ok(_) => (), //print!("{} contains:\n{}", display, s),
    }
    //Convert that String into a str.
    s = s.to_owned();
    let s_slice: &str = &s[..];

    //Make a PiecesListIntermediate out of the string using serde's json parsing.
    let positions_list: PositionListIntermediateRepresentation = serde_json::from_str::<PositionListIntermediateRepresentation>(s_slice).unwrap();
    // Iniialize output vector and fill it.
    let mut output: Vec<PositionIntermediateRepresentation> = Vec::new();
    for i in positions_list.positions {
        output.push(i);
    }
    Ok(output)
}
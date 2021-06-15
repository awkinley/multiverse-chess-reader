extern crate process_memory;

// REMOVE THIS IF YOU WANT TO MAKE IT WORK ON SOMETHING THAT ISN'T WINDOWS
#[cfg(not(windows))]
fn main() {
    println!("windows it not supported, if this is a mistake, remove this main definition from main.rs");
}

mod game_interface;
use self::game_interface::*;

mod game_data;
use self::game_data::*;

pub fn get_field_name_from_offset(offset: u64) -> &'static str {
    for field in &BOARD_FIELDS {
        if offset == field.offset {
            return field.name;
        }
    }
    return "";
}

pub fn get_field_offset_from_name(name: &str) -> u64 {
    for field in &BOARD_FIELDS {
        if name == field.name {
            return field.offset;
        }
    }
    return 0;
}

pub fn print_board(board: &GameBoard) {
    let pieces = &board.pieces;
    for named_field in &board.fields {
        info!("{} = {}", named_field.name, named_field.value);
    }
    info!("player_to_move = {:?}", board.player_to_move);
    // println!("last move start (row, col) = ({}, {}), last move end (row, col) = ({}, {})", board.last_move_start_row, board.last_move_start_col, board.last_move_end_row, board.last_move_end_col);
    for row in (0..board.height).rev() {
        let mut to_print :String = "".to_string();
        for col in 0..board.width {
            to_print.push_str(&format!("|"));
            let piece = pieces[row as usize][col as usize];

            let type_char = char::from(piece.piece_type);
            let owner_char = char::from(piece.owner);
            to_print.push_str(&format!("{}{}", owner_char,type_char));

            if col == board.width - 1 {
                to_print.push_str("|");
                info!("{}", to_print);
                to_print = "".to_string();
            }
        }
    }
}


use bytes::Bytes;
fn save_array(
    process_handle: (*mut winapi::ctypes::c_void, process_memory::Architecture),
    array: game_data::ArrayWithCap,
    bytes_per_element: u32,
) -> game_data::SavedArrayWithCap {
    let array_len = array.array_len;
    let array_cap = array.array_cap;
    let byte_ptr: *mut u8 = array.array_ptr as *mut u8;
    let mut bytes = reading::read_bytes2(
        process_handle,
        array.array_ptr,
        (array_len * bytes_per_element) as u64,
    );

    let bbytes = Bytes::from(bytes.clone());
    game_data::SavedArrayWithCap {
        array_len,
        array_cap,
        bytes,
        bbytes,
    }
}

fn save_game(
    process_handle: (*mut winapi::ctypes::c_void, process_memory::Architecture),
    game_data: game_data::GameData,
) -> game_data::SavedGameData {
    let unknown1 = game_data.unknown1;
    let unknown2 = game_data.unknown2;
    let non_neg_universes = game_data.non_neg_universes;
    let neg_universes = game_data.neg_universes;
    let num_boards = game_data.num_boards;
    let unknown3 = game_data.unknown3;
    let unknown4 = game_data.unknown4;
    let unknown5 = game_data.unknown5;
    let unknown6 = game_data.unknown6;
    let unknown7 = game_data.unknown7;
    let unknown8 = game_data.unknown8;
    let unknown9 = game_data.unknown9;
    let board_width = game_data.board_width;
    let board_height = game_data.board_height;
    let unknown10 = game_data.unknown10;
    let unknown11 = game_data.unknown11;

    let board_array = save_array(
        process_handle,
        game_data.board_array,
        game_data::BOARD_ELEMENT_LENGTH,
    );
    let array_40 = save_array(
        process_handle,
        game_data.array_40,
        game_data::_40_ELEMENT_LENGTH,
    );
    let array_50 = save_array(
        process_handle,
        game_data.array_50,
        game_data::_50_ELEMENT_LENGTH,
    );
    let array_60 = save_array(
        process_handle,
        game_data.array_60,
        game_data::_60_ELEMENT_LENGTH,
    );
    let array_70 = save_array(
        process_handle,
        game_data.array_70,
        game_data::_70_ELEMENT_LENGTH,
    );
    let array_80 = save_array(
        process_handle,
        game_data.array_80,
        game_data::_80_ELEMENT_LENGTH,
    );
    let array_90 = save_array(
        process_handle,
        game_data.array_90,
        game_data::_90_ELEMENT_LENGTH,
    );
    let array_a0 = save_array(
        process_handle,
        game_data.array_a0,
        game_data::_A0_ELEMENT_LENGTH,
    );
    let array_b0 = save_array(
        process_handle,
        game_data.array_b0,
        game_data::_B0_ELEMENT_LENGTH,
    );
    let array_c0 = save_array(
        process_handle,
        game_data.array_c0,
        game_data::_C0_ELEMENT_LENGTH,
    );
    let array_d0 = save_array(
        process_handle,
        game_data.array_d0,
        game_data::_D0_ELEMENT_LENGTH,
    );

    game_data::SavedGameData {
        unknown1,
        unknown2,
        non_neg_universes,
        neg_universes,
        num_boards,
        unknown3,
        unknown4,
        unknown5,
        unknown6,
        unknown7,
        unknown8,
        unknown9,
        board_array,
        array_40,
        array_50,
        array_60,
        array_70,
        array_80,
        array_90,
        array_a0,
        array_b0,
        array_c0,
        array_d0,
        board_height,
        board_width,
        unknown10,
        unknown11,
    }
}

pub fn get_boards_where_field_has_value(
    game_boards: &Vec<GameBoard>,
    field_name: &str,
    value: i32,
) -> Option<GameBoard> {
    for board in game_boards {
        let fields = &board.fields;
        for f in fields {
            if f.name == field_name {
                if f.value == value {
                    return Some(board.clone());
                }
            }
        }
    }

    None
}

pub fn get_field_value_in_board_by_name(board: &GameBoard, field_name: &str) -> i32 {
    let fields = &board.fields;
    for f in fields {
        if f.name == field_name {
            return f.value;
        }
    }

    -1
}

mod notation;
use self::notation::*;
pub fn move_for_board(board: &GameBoard, game_boards: &Vec<GameBoard>) -> notation::Move {
    let next_move_start_row = get_field_value_in_board_by_name(&board, NEXT_MOVE_START_ROW);
    let next_move_start_col = get_field_value_in_board_by_name(&board, NEXT_MOVE_START_COL);
    let board_timeline = get_field_value_in_board_by_name(&board, TIMELINE_NUMBER);
    let board_time = get_field_value_in_board_by_name(&board, TIME_POSITION);

    let next_move_dest_row = get_field_value_in_board_by_name(&board, NEXT_MOVE_DEST_ROW);
    let next_move_dest_col = get_field_value_in_board_by_name(&board, NEXT_MOVE_DEST_COL);
    let next_move_dest_universe = get_field_value_in_board_by_name(&board, NEXT_MOVE_DEST_UNIVERSE);
    let next_move_dest_time = get_field_value_in_board_by_name(&board, NEXT_MOVE_DEST_TIME);

    let next_board = get_field_value_in_board_by_name(&board, NEXT_BOARD_NUMBER);
    let created_board_number = get_field_value_in_board_by_name(&board, CREATED_BOARD_NUMBER);
    let prev_board = get_field_value_in_board_by_name(&board, PREV_BOARD_NUMBER);

    let pieces = board.pieces.clone();
    let moving_piece = pieces[next_move_start_row as usize][next_move_start_col as usize];
    let notation_piece: Piece = Piece::from(moving_piece);

    let start_loc = Location {
        universe: board_timeline,
        time: board_time,
        row: next_move_start_row,
        col: next_move_start_col,
    };
    let end_loc = Location {
        universe: next_move_dest_universe,
        time: next_move_dest_time,
        row: next_move_dest_row,
        col: next_move_dest_col,
    };

    let is_jump: bool = start_loc.universe != end_loc.universe || start_loc.time != end_loc.time;
    let mut is_branching = false;
    let mut does_capture = false;
    if is_jump {
        let created_board =
            match get_boards_where_field_has_value(game_boards, BOARD_NUMBER, created_board_number)
            {
                Some(a) => a,
                None => panic!(
                    "field named {} with value {} not found",
                    BOARD_NUMBER, created_board_number
                ),
            };

        let created_board_timeline =
            get_field_value_in_board_by_name(&created_board, TIMELINE_NUMBER);
        let created_board_prev_board_num =
            get_field_value_in_board_by_name(&created_board, PREV_BOARD_NUMBER);
        let created_board_prev_board = match get_boards_where_field_has_value(
            game_boards,
            BOARD_NUMBER,
            created_board_prev_board_num,
        ) {
            Some(a) => a,
            None => panic!(
                "field named {} with value {} not found",
                BOARD_NUMBER, created_board_prev_board_num
            ),
        };
        let created_board_prev_timeline =
            get_field_value_in_board_by_name(&created_board_prev_board, TIMELINE_NUMBER);

        // the board created by the jump has a previous board, which is the start of the purple arrow that points to the
        // left side of that board on the UI
        // if the timeline that that previous board is in is different from the timeline that the board that was created by the jump
        // then the board branches, otherwise it does not
        if created_board_timeline != created_board_prev_timeline {
            is_branching = true;
        }

        let potentially_captured_pieces = created_board_prev_board.pieces;
        let potentially_captured_piece =
            potentially_captured_pieces[end_loc.row as usize][end_loc.col as usize];

        // check if we landed on a piece
        does_capture = match potentially_captured_piece.piece_type {
            PieceType::Nothing => false,
            _ => true,
        };
    } else {

    }

    let piece_move = Move {
        start_loc,
        end_loc,
        start_piece: notation_piece,
        end_piece: notation_piece,
        is_jump,
        is_branching,
        does_capture,
        moves_present: false,
    };

    return piece_move;
}
pub fn generate_turns(game_boards: &Vec<GameBoard>) -> Vec<notation::Turn> {
    let mut turns = Vec::<Turn>::new();

    // boards are numbered in the order they are created
    // this means that by going sequentially up through the board numbers and looking at the move that
    // was played on the previous board to make that board, then we can get the moves in the order they were made
    // this also makes it easier to deal with branching (more on that below)
    let mut board_num = 1i32;
    let mut player = Player::White;
    let mut moves = Vec::new();
    let mut board_height : u32 = 8;
    let mut prev_boardOption: Option<GameBoard> = None;
    while board_num < game_boards.len() as i32 {
        info!("board index: {}", board_num);
        let board = match get_boards_where_field_has_value(game_boards, BOARD_NUMBER, board_num) {
            Some(a) => a,
            None => {info!("field named {} with value {} not found", BOARD_NUMBER, board_num); 
                    return turns; }
        };
        print_board(&board);

        let prev_board_num = get_field_value_in_board_by_name(&board, PREV_BOARD_NUMBER);
        let player_to_move = match board.player_to_move {
            0 => Player::White,
            1 => Player::Black,
            a => {info!("player to move is {}", a); Player::White}
        };

        board_height =  board.height;

        // because we're looking one turn ahead,
        // we look for when we find a board where that player makes the next move to know that we've found all their moves


        let prev_board = match get_boards_where_field_has_value(game_boards, BOARD_NUMBER, prev_board_num) {
            Some(a) => a,
            None => {info!("field named {} with value {} not found", BOARD_NUMBER, prev_board_num); 
                    return turns; }
        };

        if player == player_to_move {
            let turn = Turn {
                moves: moves,
                player,
            };
            moves = Vec::new();
            info!("turn: {:?}", turn);
            info!("turn notation: {}", turn.to_notation(Some(&prev_board)));
            turns.push(turn);
            info!("\n");
            player = match player {
                Player::White => Player::Black,
                Player::Black => Player::White,
            }
        }

        let prev_board =
            match get_boards_where_field_has_value(game_boards, BOARD_NUMBER, prev_board_num) {
                Some(a) => a,
                None => {
                    println!(
                        "field named {} with value {} not found",
                        BOARD_NUMBER, prev_board_num
                    );
                    return turns;
                }
            };

        // let mut moves = Vec::new();
        let prev_move = move_for_board(&prev_board, game_boards);
        moves.push(prev_move);

        {
        // a jump creates two boards, the one where the piece left that board, and the one where the piece arrived
        // so we increment one extra time to account for this
            if prev_move.is_jump {
                board_num += 1;
            }
        }
        prev_boardOption = Some(prev_board);
        board_num += 1;
    }


    let turn = Turn {moves: moves, player};
    info!("turn: {:?}", turn);
    let prev= prev_boardOption.as_ref();
    info!("turn notation: {}", turn.to_notation(prev));
    turns.push(turn);
    info!("\n");
    return turns;
}

pub fn get_turns_string(turns: &Vec<Turn>) -> String {
    let mut turn_num = 1;
    let mut to_print = "".to_string();
    for t in turns {
        if t.player == Player::White {
            to_print.push_str(&format!("{}. {}/ ", turn_num, t.to_notation(None)));
        }
        else {
            // println!("{}{}", to_print , t.to_notation());
            to_print.push_str(&format!("{}\n", t.to_notation(None)));
            turn_num += 1;
        }
    }
    return to_print;
}
// for when you need to be really bad and turn an arbirary pointer into a u8 slice
unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}
use log::{info, warn, error};
use retry::delay::Fixed;
use retry::retry;
extern crate retry;
mod cli;
use self::cli::*;
#[cfg(windows)]
fn main() -> std::io::Result<()>  {
    let mut args = parser();
    let out : &mut Box<dyn std::io::Write> = &mut args.outfile;

    info!("{}{}", "test", "test");

/// this function is definetly a mess
/// a lot of that mess is attempts at writing to the game, that are generally unsuccesful
/// some of it is just debug stuff, like printing out information at various points
/// if you don't care about writing, you can 100% just yeet it
    // We need to make sure that we get a handle to a process, in this case, ourselves
    use process_memory::*;
    let pid = interface::get_pid("5dchesswithmultiversetimetravel.exe");
    let offset = match interface::get_offset(pid) {
        Ok(v) => v,
        Err(e) => panic!("Could not find 5dchesswithmultiversetimetravel.exe: {}" ,e ),
    };
    // println!("{}", print_process_name_and_id(pid));
    // println!("pid is {}", pid);
    if !args.polling {
        return do_print(&read_turns(pid, offset).unwrap());
    }
    use retry::OperationResult;
    let _result = retry(Fixed::from_millis(500), || {
        if false {
            return OperationResult::Ok(());
        }
        match read_turns(pid, offset) {
            Ok(r) => {do_print(&r).unwrap() ; OperationResult::Retry("Continue")},
            Err(e) => {error!("{}", e.to_string()); OperationResult::Err("Error")}
        }

    }).unwrap();
    Ok(())
}

    fn read_turns(pid: u32, offset: usize) -> std::io::Result<String>  {
        use process_memory::*;

        let process_handle = pid.try_into_process_handle()?;
        // println!("process handle exists {:?}", process_handle);

        let num_boards = DataMember::<u32>::new_offset(process_handle, vec![offset + 0x14bab0]).read().unwrap();

        let game_board_data_length = 0x40u64;
        const START_OF_GAME_BOARD_DATA: usize =  0x14BA80; // Give names to constants 
        info!("About to read into {}" , (offset + START_OF_GAME_BOARD_DATA) );
        let _game_board_data : std::vec::Vec<u8>;
        let result = retry(Fixed::from_millis(100).take(2), || {
        return reading::read_bytes(process_handle, (offset + START_OF_GAME_BOARD_DATA) as u64, game_board_data_length);
        });
        
        match result {
            Ok(_) => info!("oke"),
            Err(e) => error!("error : {:?}", e),
        }
        
        const BOARD_ADDRES_OFFSET: usize = 0x14bab8;
        // let mut game_board_data = DataMember::<u64>::new_offset(process_handle, vec![offset + 0x14BA80]).read().unwrap();
        let mut current_board_address = DataMember::<u64>::new_offset(process_handle, vec![offset + BOARD_ADDRES_OFFSET]).read().unwrap();
        
        let board_memory_length = 0xe4u64;

        let board_data = reading::read_bytes(process_handle, current_board_address, (num_boards as u64) * board_memory_length);
        
        

        // wait for keyboard input so that we can do stuff on the other end
        // use std::io;
        // let mut guess = String::new();
        // io::stdin().read_line(&mut guess).expect("Failed to read line");

        // println!("board data length is {}", board_data.len());
        // // write the saved data back to the process and get the pointer to that data
        // let new_board_ptr = alloc_and_write_memory(pid, board_data.len(), &board_data).unwrap();
        // println!("new board ptr {:?}", new_board_ptr);
        // write_bytes(process_handle, (offset + 0x14BA80) as u64, game_board_data_length, game_board_data);
        // // put the address of the new data in the current board address location
        // DataMember::<u64>::new_offset(process_handle, vec![offset + 0x14bab8]).write(&(new_board_ptr as u64)).unwrap();
    

        current_board_address = DataMember::<u64>::new_offset(process_handle, vec![offset + BOARD_ADDRES_OFFSET]).read().unwrap();
        if current_board_address == 0 {
            error!("current_board_address is 0");
        }

        info!("current_board_address: {}, ",current_board_address);
        // DataMember::<u32>::new_offset(process_handle, vec![offset + 0x14bab0]).write(&num_boards).unwrap();
        // write_board_bytes(process_handle, current_board_address, (num_boards as u64) * board_memory_length, board_data);

        
        let mut address =  current_board_address;
        let mut board_vec = Vec::new();
        for _ in 0..num_boards {
            board_vec.push(address);
            address += board_memory_length;
        }
    let boards = reading::read_boards(board_vec, process_handle, offset);
    print_board(&boards[0]);
    let turns = generate_turns(&boards);

        let turns = generate_turns(&boards);

        Ok(get_turns_string(&turns))
    }
    // });
    // for board in &boards {
    //     print_board(board);
    // }

    // println!("offset : {}", board_width.get_offset()?);

    // println!("board_width: {}", board_width);
    // println!("num_boards: {}", num_boards);
    // println!("current_board_address: {:#x}", current_board_address);
    #[macro_use]
    extern crate lazy_static;
    use std::sync::Mutex;
    lazy_static! {
        /// This is an example for using doc comment attributes
        static  ref ALREADYPRINTED : Mutex<String> =  Mutex::new("".to_string());
    }
    /**
     * Prints to console
     */
    fn do_print(my_str : &str) -> std::io::Result<()> {
        let mut the_str = ALREADYPRINTED.lock().unwrap();
        let length = the_str.len();
        for (i, c) in my_str.chars().enumerate() {
            if i > length {
                print!("{}", c);
                the_str.push_str(&c.to_string());
            }
        }
        Ok(())
    }
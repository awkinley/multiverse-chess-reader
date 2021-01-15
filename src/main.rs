extern crate process_memory;

#[cfg(windows)]
extern crate winapi;

#[cfg(not(windows))]
fn main() {
    println!("FastyBoy can only be run on systems supporting the game Mirror's Edge Catalyst, which as of writing is only Windows.")
}
/// A helper function to get a Pid from the name of a process
#[cfg(windows)]
pub fn get_pid(process_name: &str) -> process_memory::Pid {
    /// A helper function to turn a c_char array to a String
    fn utf8_to_string(bytes: &[i8]) -> String {
        use std::ffi::CStr;
        unsafe {
            CStr::from_ptr(bytes.as_ptr())
                .to_string_lossy()
                .into_owned()
        }
    }
    let mut entry = winapi::um::tlhelp32::PROCESSENTRY32 {
        dwSize: std::mem::size_of::<winapi::um::tlhelp32::PROCESSENTRY32>() as u32,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0; winapi::shared::minwindef::MAX_PATH],
    };
    let snapshot: winapi::um::winnt::HANDLE;
    unsafe {
        snapshot = winapi::um::tlhelp32::CreateToolhelp32Snapshot(
            winapi::um::tlhelp32::TH32CS_SNAPPROCESS,
            0,
        );
        if winapi::um::tlhelp32::Process32First(snapshot, &mut entry)
            == winapi::shared::minwindef::TRUE
        {
            while winapi::um::tlhelp32::Process32Next(snapshot, &mut entry)
                == winapi::shared::minwindef::TRUE
            {
                if utf8_to_string(&entry.szExeFile) == process_name {
                    return entry.th32ProcessID;
                }
            }
        }
    }
    0
}

#[cfg(windows)]
use winapi::ctypes::{c_void};
use winapi::shared::minwindef::{LPDWORD, HMODULE};
use winapi::um::winnt::*;
use winapi::um::processthreadsapi::*;
use winapi::um::psapi::*;
use std::ptr::*;
pub fn list_modules(process_id: winapi::shared::minwindef::DWORD) -> std::io::Result<()> {
    unsafe {
        let hProcess: HANDLE =
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
        if hProcess != null_mut() {
            let mut hMod: HMODULE = null_mut();
            let mut modules: [HMODULE; 1024] = [0 as HMODULE; 1024];
            let mut cb_needed: u32 = 0u32;
            if EnumProcessModules(
                hProcess,
                modules.as_ptr() as *mut HMODULE,
                std::mem::size_of::<HMODULE>() as u32 * 1024,
                &mut cb_needed as *mut _ as *mut u32,
            ) == 0
            {
                println!("enum process modules failed {}", std::io::Error::last_os_error());
                return Ok(());
            }
            println!("cb_needed = {:?}", cb_needed);
            let mut i: usize = 0;
            while (i as u32) < cb_needed {

                let module = modules[i];

                let mut szProcessName: Vec<u16> = Vec::new();
                let mut count = 0;
                while count < 40 {
                    szProcessName.push(0u16);
                    count += 1;
                }
                GetModuleBaseNameW(hProcess, module, szProcessName.as_ptr() as *mut u16, 40);

                count = 0;

                while szProcessName[count as usize] != 0 {
                    count += 1;
                }
                let mut process_name: String = String::new();
                process_name = String::from_utf16(&szProcessName[..count as usize]).unwrap();
                println!("module = {:?}, process name = {}", module, process_name);
                i += 1;
            }
        }
    }
    Ok(())
}

pub fn get_offset(process_id: winapi::shared::minwindef::DWORD) -> std::result::Result<usize, String> {
    unsafe {
        let hProcess: HANDLE =
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
        if hProcess != null_mut() {
            let mut hMod: HMODULE = null_mut();
            let mut cb_needed: u32 = 0u32;
            if EnumProcessModules(
                hProcess,
                &mut hMod,
                std::mem::size_of::<HMODULE>() as u32,
                &mut cb_needed as *mut _ as *mut u32,
            ) == 0
            {
                return Err(String::from("failed"))
            }
            return Ok(hMod as usize);
        }
    }
    Err(String::from("failed"))
}

pub fn alloc_and_write_memory(process_id: winapi::shared::minwindef::DWORD, num_bytes: usize, bytes: &Vec<u8>) -> std::result::Result<*mut c_void, String> {
    unsafe {
        let hProcess: HANDLE =
            OpenProcess(PROCESS_ALL_ACCESS, 0, process_id);
        if hProcess != null_mut() {
            let mem_ptr = winapi::um::memoryapi::VirtualAllocEx(hProcess, null_mut(), num_bytes, winapi::um::winnt::MEM_COMMIT, winapi::um::winnt::PAGE_READWRITE);
            println!("mem_ptr is {:?}", mem_ptr);
            let mut bytes_written: winapi::shared::basetsd::SIZE_T = 0;
            let mut bytes_written_sum = 0;

            let mut bytes_left = num_bytes;
            const buffer_size: usize = 1024;
            while bytes_left > buffer_size {

                let mut buffer: [u8; buffer_size] = [0 as u8; buffer_size];

                for i in bytes_written_sum..(bytes_written_sum + buffer_size) {
                    buffer[i-bytes_written_sum] = bytes[i];
                }

                let address = (mem_ptr as u64 + bytes_written_sum as u64) as *mut c_void;
                println!("address is {:?}", address);
                let result = winapi::um::memoryapi::WriteProcessMemory(hProcess, address, buffer.as_ptr() as *mut winapi::ctypes::c_void, buffer_size,  
                                                                        &mut bytes_written as *mut winapi::shared::basetsd::SIZE_T);
                if result == 0 {
                    println!("write in loop failed {}", std::io::Error::last_os_error());
                    return Err(String::from("failed write in loop"));
                }
                bytes_written_sum += bytes_written;
                bytes_written = 0;
                bytes_left -= buffer_size;
            }

            let mut buffer: [u8; buffer_size] = [0 as u8; buffer_size];

            for i in bytes_written_sum..(num_bytes) {
                buffer[i - bytes_written_sum] = bytes[i];
            }

            let address = (mem_ptr as u64 + bytes_written_sum as u64) as *mut c_void;
            println!("address is {:?}", address);
            let result = winapi::um::memoryapi::WriteProcessMemory(hProcess, address, buffer.as_ptr() as *mut winapi::ctypes::c_void, bytes_left, &mut bytes_written as *mut winapi::shared::basetsd::SIZE_T);
            if result == 0 {
                println!("WriteProcessMemory failed {}", std::io::Error::last_os_error());
                return Err(String::from("failed write after loop"));
            }
            bytes_written_sum += bytes_written;
            println!("wrote {} bytes" , bytes_written_sum);

            return Ok(mem_ptr);
        }
    }

    Err(String::from("failed on the outside"))

}


#[cfg(windows)]
use winapi::um::winnt::*;
use winapi::um::processthreadsapi::*;
use winapi::um::psapi::*;
// use winapi::ctypes::{c_void};
// use winapi::shared::minwindef::{LPDWORD, HMODULE};
// use std::ptr;
pub fn print_process_name_and_id(processID: u32) -> String {
    unsafe {
        let mut process_name: String = String::new();
        let hProcess: HANDLE =
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, processID);
        if hProcess != null_mut() {
            let mut hMod: HMODULE = null_mut();
            let mut cb_needed: u32 = 0u32;
            if EnumProcessModules(
                hProcess,
                &mut hMod,
                std::mem::size_of::<HMODULE>() as u32,
                &mut cb_needed as *mut _ as *mut u32,
            ) == 0
            {
                return String::new();
            }
            let mut szProcessName: Vec<u16> = Vec::new();
            let mut count = 0;
            while count < 40 {
                szProcessName.push(0u16);
                count += 1;
            }
            GetModuleBaseNameW(hProcess, hMod, szProcessName.as_ptr() as *mut u16, 40);

            count = 0;

            while szProcessName[count as usize] != 0 {
                count += 1;
            }
            process_name = String::from_utf16(&szProcessName[..count as usize]).unwrap();
        }
        return process_name;
    }
}

mod game_data;
use self::game_data::*;

pub fn char_for_piece_type(piece_type: PieceType) -> char {
    match piece_type {
        PieceType::Pawn => 'p',
        PieceType::Knight => 'k',
        PieceType::Bishop => 'b',
        PieceType::Rook => 'r',
        PieceType::Queen => 'Q',
        PieceType::King => 'K',
        PieceType::Unicorn => 'u',
        PieceType::Dragon => 'd',
        PieceType::Brawn => 'B',
        PieceType::FourPointQueen => 'q',
        PieceType::RoyalQueen => 'Q',
        PieceType::CommongKing => 'K',
        _ => ' ',
    }
}

pub fn char_for_piece_owner(piece_owner: PieceOwner) -> char {
    match piece_owner {
        PieceOwner::White => 'w',
        PieceOwner::Black => 'b',
        _ => ' ',
    }
}

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
        println!("{} = {}", named_field.name, named_field.value);
    }
    println!("player_to_move = {:?}", board.player_to_move);
    // println!("last move start (row, col) = ({}, {}), last move end (row, col) = ({}, {})", board.last_move_start_row, board.last_move_start_col, board.last_move_end_row, board.last_move_end_col);
    for row in (0..board.height).rev() {
        for col in 0..board.width {
            print!("|");
            let piece = pieces[row as usize][col as usize];
            let type_char = char_for_piece_type(piece.piece_type);
            let owner_char = char_for_piece_owner(piece.owner);
            print!("{}{}", owner_char,type_char);

            if col == board.width - 1 {
                println!("|");
            }
        }
    }
}

pub fn read_bytes(process_handle: (*mut winapi::ctypes::c_void, process_memory::Architecture), starting_addresss: u64, length_bytes: u64) -> Vec<u8> {
    use process_memory::*;
    
    let mut bytes = Vec::<u8>::new();
    for i in 0..length_bytes {
        let value = DataMember::<u8>::new_offset(process_handle, vec![(starting_addresss + i) as usize]).read().unwrap();
        bytes.push(value);
    }
    return bytes;
}

pub fn write_bytes(process_handle: (*mut winapi::ctypes::c_void, process_memory::Architecture), starting_addresss: u64, length_bytes: u64, bytes: Vec<u8>) {
    use process_memory::*;
    println!("writting values");
    for i in 0..length_bytes {
        let value = bytes[i as usize];
        DataMember::<u8>::new_offset(process_handle, vec![(starting_addresss + i) as usize]).write(&value).unwrap();
    }

}

pub fn read_boards(board_vec: Vec<u64>, process_handle: (*mut winapi::ctypes::c_void, process_memory::Architecture), process_offset: usize) -> Vec<GameBoard> {
    use process_memory::*;

    let board_width = DataMember::<u32>::new_offset(process_handle, vec![process_offset + 0x14bb60]).read().unwrap();
    let board_height = DataMember::<u32>::new_offset(process_handle, vec![process_offset + 0x14bb64]).read().unwrap();


    let nothing_piece = GamePiece {piece_type: PieceType::Nothing, owner: PieceOwner::NoOwner};
    let mut real_boards = Vec::<GameBoard>::new();
    for board in &board_vec {
        let empty_row = vec![nothing_piece; board_width as usize];
        let mut pieces = vec![empty_row; board_width as usize];

        let player_to_move = DataMember::<u32>::new_offset(process_handle, vec![(board + 12) as usize]).read().unwrap();
        
        for col in 0..board_width {
            for row in 0..board_height {

                let offset: u64 = (16 * col + 2 * row + 16).into();

                let piece_type = match DataMember::<u8>::new_offset(process_handle, vec![(board + offset) as usize]).read().unwrap() {
                    1 => PieceType::Pawn,
                    2 => PieceType::Knight,
                    3 => PieceType::Bishop,
                    4 => PieceType::Rook,
                    5 => PieceType::Queen,
                    6 => PieceType::King,
                    7 => PieceType::Unicorn,
                    8 => PieceType::Dragon,
                    10 => PieceType::Brawn,
                    11 => PieceType::FourPointQueen,
                    12 => PieceType::RoyalQueen,
                    13 => PieceType::CommongKing,
                    _ => PieceType::Nothing,
                };
                let owner = match DataMember::<u8>::new_offset(process_handle, vec![(board + offset + 1) as usize]).read().unwrap() {
                    1 => PieceOwner::White,
                    2 => PieceOwner::Black,
                    _ => PieceOwner::NoOwner,
                };
                // println!("piece_type is {:?}, piece_owner is {:?}", piece_type, owner);

                let piece = GamePiece {piece_type, owner};
                pieces[row as usize][col as usize] = piece;

            }
        }

        let mut new_board_fields = Vec::new();
        for field in &BOARD_FIELDS {
            let value = DataMember::<i32>::new_offset(process_handle, vec![(board + field.offset) as usize]).read().unwrap();
            new_board_fields.push(FieldNameWithValue{name: field.name, value});
        }

        let new_board = GameBoard {player_to_move, width: board_width, height: board_height, pieces, fields: new_board_fields};
        real_boards.push(new_board);
        
    }

    return real_boards;

}

pub fn get_boards_where_field_has_value(game_boards: &Vec<GameBoard>, field_name: &str, value: i32) -> Option<GameBoard> {
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
pub fn game_piece_to_notation_piece(p: GamePiece) -> notation::Piece {
    match p.piece_type {
        PieceType::Nothing => Piece::Nothing,
        PieceType::Pawn => Piece::Pawn,
        PieceType::Knight => Piece::Knight,
        PieceType::Bishop => Piece::Bishop,
        PieceType::Rook => Piece::Rook,
        PieceType::Queen => Piece::Queen,
        PieceType::King => Piece::King,
        PieceType::Unicorn => Piece::Unicorn,
        PieceType::Dragon => Piece::Dragon,
        PieceType::Brawn => Piece::Brawn,
        PieceType::FourPointQueen => Piece::FourPointQueen,
        PieceType::RoyalQueen => Piece::RoyalQueen,
        PieceType::CommongKing => Piece::CommongKing,
        _ => Piece::Nothing,
    }
}

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
    let notation_piece: Piece = game_piece_to_notation_piece(moving_piece);

    let start_loc = Location {universe: board_timeline, time: board_time, row: next_move_start_row, col: next_move_start_col};
    let end_loc = Location {universe: next_move_dest_universe, time: next_move_dest_time, row: next_move_dest_row, col: next_move_dest_col};

    let is_jump: bool = start_loc.universe != end_loc.universe || start_loc.time != end_loc.time;
    let mut is_branching = false;
    let mut does_capture = false;
    if is_jump {
        
        let created_board = match get_boards_where_field_has_value(game_boards, BOARD_NUMBER, created_board_number) {
            Some(a) => a,
            None => panic!("field named {} with value {} not found", BOARD_NUMBER, created_board_number),
        };

        let created_board_timeline = get_field_value_in_board_by_name(&created_board, TIMELINE_NUMBER);
        let created_board_prev_board_num = get_field_value_in_board_by_name(&created_board, PREV_BOARD_NUMBER);
        let created_board_prev_board = match get_boards_where_field_has_value(game_boards, BOARD_NUMBER, created_board_prev_board_num) {
            Some(a) => a,
            None => panic!("field named {} with value {} not found", BOARD_NUMBER, created_board_prev_board_num),
        };
        let created_board_prev_timeline = get_field_value_in_board_by_name(&created_board_prev_board, TIMELINE_NUMBER);

        // the board created by the jump has a previous board, which is the start of the purple arrow that points to the
        // left side of that board on the UI
        // if the timeline that that previous board is in is different from the timeline that the board that was created by the jump
        // then the board branches, otherwise it does not
        if created_board_timeline != created_board_prev_timeline {
            is_branching = true;
        }

        let potentially_captured_pieces = created_board_prev_board.pieces;
        let potentially_captured_piece = potentially_captured_pieces[end_loc.row as usize][end_loc.col as usize];

        // check if we landed on a piece
        does_capture = match potentially_captured_piece.piece_type {
            PieceType::Nothing => false,
            _ => true,
        };
    }


    let piece_move = Move {start_loc, end_loc, start_piece: notation_piece, end_piece: notation_piece, is_jump, is_branching, does_capture, moves_present: false};
    
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
    while board_num < game_boards.len() as i32 {
        println!("board index: {}", board_num);
        let board = match get_boards_where_field_has_value(game_boards, BOARD_NUMBER, board_num) {
            Some(a) => a,
            None => {println!("field named {} with value {} not found", BOARD_NUMBER, board_num); 
                    return turns; }
        };
        
        print_board(&board);

        let prev_board_num = get_field_value_in_board_by_name(&board, PREV_BOARD_NUMBER);
        let player_to_move = match board.player_to_move{
            0 => Player::White,
            1 => Player::Black,
            a => {println!("player to move is {}", a); Player::White}
        };

        // because we're looking one turn ahead,
        // we look for when we find a board where that player makes the next move to know that we've found all their moves
        if player == player_to_move {
            let turn = Turn {moves: moves, player};
            moves = Vec::new();
            println!("turn: {:?}", turn);
            println!("turn notation: {}", turn.to_notation());
            turns.push(turn);
            println!("\n");
            player = match player {
                Player::White => Player::Black,
                Player::Black => Player::White,
            }
        }

        let prev_board = match get_boards_where_field_has_value(game_boards, BOARD_NUMBER, prev_board_num) {
            Some(a) => a,
            None => {println!("field named {} with value {} not found", BOARD_NUMBER, prev_board_num); 
                    return turns; }
        };

        

        // let mut moves = Vec::new();
        let prev_move = move_for_board(&prev_board, game_boards);
        moves.push(prev_move);

        // a jump creates two boards, the one where the piece left that board, and the one where the piece arrived
        // so we increment one extra time to account for this
        if prev_move.is_jump {
            board_num += 1;
        }

        board_num += 1;
    }

    let turn = Turn {moves: moves, player};
    println!("turn: {:?}", turn);
    println!("turn notation: {}", turn.to_notation());
    turns.push(turn);
    println!("\n");
    return turns;
}

pub fn print_turns(turns: &Vec<Turn>) {
    let mut turn_num = 1;
    for t in turns {
        if t.player == Player::White {
            print!("{}. {}/ ", turn_num, t.to_notation());
        }
        else {
            println!("{}", t.to_notation());
            turn_num += 1;
        }
        
    }
}

fn main() -> std::io::Result<()>  {

    // We need to make sure that we get a handle to a process, in this case, ourselves
    // let handle = (std::process::id() as Pid).try_into_process_handle().unwrap();
    use process_memory::*;
    let pid = get_pid("5dchesswithmultiversetimetravel.exe");
    let offset = match get_offset(pid) {
        Ok(v) => v,
        Err(e) => panic!(e),
    };
    // println!("{}", print_process_name_and_id(pid));
    // println!("pid is {}", pid);
    let process_handle = pid.try_into_process_handle()?;
    // println!("process handle exists {:?}", process_handle);

    let num_boards = DataMember::<u32>::new_offset(process_handle, vec![offset + 0x14bab0]).read().unwrap();

    let game_board_data_length = 0x40u64;
    let game_board_data = read_bytes(process_handle, (offset + 0x14BA80) as u64, game_board_data_length);
    // let mut game_board_data = DataMember::<u64>::new_offset(process_handle, vec![offset + 0x14BA80]).read().unwrap();
    let mut current_board_address = DataMember::<u64>::new_offset(process_handle, vec![offset + 0x14bab8]).read().unwrap();
    
    let board_memory_length = 0xe4u64;

    let board_data = read_bytes(process_handle, current_board_address, (num_boards as u64) * board_memory_length);
    
    

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
    
   
   
    current_board_address = DataMember::<u64>::new_offset(process_handle, vec![offset + 0x14bab8]).read().unwrap();
    // DataMember::<u32>::new_offset(process_handle, vec![offset + 0x14bab0]).write(&num_boards).unwrap();
    // write_board_bytes(process_handle, current_board_address, (num_boards as u64) * board_memory_length, board_data);

    
    let mut address =  current_board_address;
    let mut board_vec = Vec::new();
    for _ in 0..num_boards {
        board_vec.push(address);
        address += board_memory_length;

    }

    let boards = read_boards(board_vec, process_handle, offset);

    let turns = generate_turns(&boards);

    print_turns(&turns);

    // for board in &boards {
    //     print_board(board);
    // }

    // println!("offset : {}", board_width.get_offset()?);

    // println!("board_width: {}", board_width);
    // println!("num_boards: {}", num_boards);
    // println!("current_board_address: {:#x}", current_board_address);

    Ok(())
}

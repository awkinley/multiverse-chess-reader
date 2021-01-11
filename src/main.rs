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
                let result = winapi::um::memoryapi::WriteProcessMemory(hProcess, address, buffer.as_ptr() as *mut winapi::ctypes::c_void, buffer_size,  &mut bytes_written as *mut winapi::shared::basetsd::SIZE_T);
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

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Nothing, 
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Unicorn,
    Dragon,
    Brawn,
    FourPointQueen,
    RoyalQueen,
    CommongKing,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceOwner {
    NoOwner,
    White,
    Black,
}

extern crate num_enum;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum PlayerToMove {
    White = 0,
    Black = 1,
    SomethingElse,
}


#[derive(Debug, Clone, Copy)]
pub struct Piece {
    piece_type: PieceType,
    owner: PieceOwner,
}

#[derive(Debug)]
pub struct Field {
    name: &'static str,
    offset: u64,
}

#[derive(Debug)]
pub struct FieldNameWithValue {
    name: &'static str,
    value: i32,
}

static BOARD_FIELDS: [Field; 24] =      [Field {name: "board number", offset: 0}, 
                                        Field {name: "timeline number", offset: 4}, 
                                        Field {name: "time position", offset: 8},
                                        Field {name: "board id", offset: 144},
                                        Field {name: "is active", offset: 148},
                                        Field {name: "timeline number 2", offset: 152}, 
                                        Field {name: "time position 2", offset: 156},
                                        Field {name: "player to move 2", offset: 160},
                                        Field {name: "next move start row", offset: 164},
                                        Field {name: "next move start col", offset: 168},
                                        Field {name: "next move dest universe", offset: 172},
                                        Field {name: "next move dest time", offset: 176},
                                        Field {name: "next move piece owner", offset: 180},
                                        Field {name: "next move end row", offset: 184},
                                        Field {name: "next move end col", offset: 188},
                                        Field {name: "board id of previous move start", offset: 192},
                                        Field {name: "board number of next board", offset: 196},
                                        Field {name: "board number of previous board", offset: 200},
                                        Field {name: "board number of any new boards created by this boards move", offset: 204},
                                        Field {name: "board number of board that a piece left to create this board", offset: 208},
                                        Field {name: "last move start row", offset: 212},
                                        Field {name: "last move start col", offset: 216},
                                        Field {name: "last move end row", offset: 220},
                                        Field {name: "last move end col", offset: 224},
                                        ];


#[derive(Debug)]
pub struct Board {
    player_to_move: u32,
    width: u32,
    height: u32,
    pieces: Vec<Vec<Piece>>,
    fields: Vec<FieldNameWithValue>,
}

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

pub fn print_board(board: &Board) {
    let pieces = &board.pieces;
    for named_field in &board.fields {
        println!("{} = {}", named_field.name, named_field.value);
    }
    // println!("board_number = {} timeline_number = {} time_position = {} player_to_move = {:?}", board.board_number, board.timeline_number, board.time_position, board.player_to_move);
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

pub fn read_boards(board_vec: Vec<u64>, process_handle: (*mut winapi::ctypes::c_void, process_memory::Architecture), process_offset: usize) -> Vec<Board> {
    use process_memory::*;

    let board_width = DataMember::<u32>::new_offset(process_handle, vec![process_offset + 0x14bb60]).read().unwrap();
    let board_height = DataMember::<u32>::new_offset(process_handle, vec![process_offset + 0x14bb64]).read().unwrap();


    let nothing_piece = Piece {piece_type: PieceType::Nothing, owner: PieceOwner::NoOwner};
    let mut real_boards = Vec::<Board>::new();
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

                let piece = Piece {piece_type, owner};
                pieces[row as usize][col as usize] = piece;

            }
        }

        let mut new_board_fields = Vec::new();
        for field in &BOARD_FIELDS {
            let value = DataMember::<i32>::new_offset(process_handle, vec![(board + field.offset) as usize]).read().unwrap();
            new_board_fields.push(FieldNameWithValue{name: field.name, value});
        }

        let new_board = Board {player_to_move, width: board_width, height: board_height, pieces, fields: new_board_fields};
        real_boards.push(new_board);
        
    }

    return real_boards;

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
    use std::io;
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("board data length is {}", board_data.len());
    // write the saved data back to the process and get the pointer to that data
    let new_board_ptr = alloc_and_write_memory(pid, board_data.len(), &board_data).unwrap();
    println!("new board ptr {:?}", new_board_ptr);
    write_bytes(process_handle, (offset + 0x14BA80) as u64, game_board_data_length, game_board_data);
    // put the address of the new data in the current board address location
    DataMember::<u64>::new_offset(process_handle, vec![offset + 0x14bab8]).write(&(new_board_ptr as u64)).unwrap();
    
   
   
    current_board_address = DataMember::<u64>::new_offset(process_handle, vec![offset + 0x14bab8]).read().unwrap();
    DataMember::<u32>::new_offset(process_handle, vec![offset + 0x14bab0]).write(&num_boards).unwrap();
    // write_board_bytes(process_handle, current_board_address, (num_boards as u64) * board_memory_length, board_data);

    
    let mut address =  current_board_address;
    let mut board_vec = Vec::new();
    for _ in 0..num_boards {
        board_vec.push(address);
        address += board_memory_length;

    }

    let boards = read_boards(board_vec, process_handle, offset);

    for board in &boards {
        // print_board(board);
    }

    // println!("offset : {}", board_width.get_offset()?);

    // println!("board_width: {}", board_width);
    // println!("num_boards: {}", num_boards);
    // println!("current_board_address: {:#x}", current_board_address);

    Ok(())
}

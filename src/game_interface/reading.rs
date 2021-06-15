use crate::game_data::*;
pub fn read_bytes(process_handle: (*mut winapi::ctypes::c_void, process_memory::Architecture), starting_addresss: u64, length_bytes: u64) -> Result<Vec<u8>, std::io::Error> {
    use process_memory::*;
    
    let mut bytes = Vec::<u8>::new();
    for i in 0..length_bytes {
        let r = DataMember::<u8>::new_offset(process_handle, vec![(starting_addresss + i) as usize]).read();
        let value = match r {
            Ok(result) => result,
            Err(er) => return Err(er)
        };
        bytes.push(value);
    }
    return Ok(bytes);
}

// lazy fix to merge two different code bases
pub fn read_bytes2(
    process_handle: (*mut winapi::ctypes::c_void, process_memory::Architecture),
    starting_addresss: u64,
    length_bytes: u64,
) -> Vec<u8> {
    use process_memory::*;
    let mut bytes = Vec::<u8>::new();
    for i in 0..length_bytes {
        let value =
            DataMember::<u8>::new_offset(process_handle, vec![(starting_addresss + i) as usize])
                .read()
                .unwrap();
        bytes.push(value);
    }
    return bytes;
}

pub fn read_boards(
    board_vec: Vec<u64>,
    process_handle: (*mut winapi::ctypes::c_void, process_memory::Architecture),
    process_offset: usize,
) -> Vec<GameBoard> {
    use process_memory::*;

    let board_height =
        DataMember::<u32>::new_offset(process_handle, vec![process_offset + 0x14bb60])
            .read()
            .unwrap();
    let board_width =
        DataMember::<u32>::new_offset(process_handle, vec![process_offset + 0x14bb64])
            .read()
            .unwrap();

    let nothing_piece = GamePiece {
        piece_type: PieceType::Nothing,
        owner: PieceOwner::NoOwner,
    };
    let mut real_boards = Vec::<GameBoard>::new();
    for board in &board_vec {
        let empty_row = vec![nothing_piece; board_width as usize];
        let mut pieces = vec![empty_row; board_width as usize];

        let player_to_move =
            DataMember::<u32>::new_offset(process_handle, vec![(board + 12) as usize])
                .read()
                .unwrap();
        for col in 0..board_width {
            for row in 0..board_height {
                let offset: u64 = (16 * col + 2 * row + 16).into();

                let piece_type = match DataMember::<u8>::new_offset(
                    process_handle,
                    vec![(board + offset) as usize],
                )
                .read()
                .unwrap()
                {
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
                let owner = match DataMember::<u8>::new_offset(
                    process_handle,
                    vec![(board + offset + 1) as usize],
                )
                .read()
                .unwrap()
                {
                    1 => PieceOwner::White,
                    2 => PieceOwner::Black,
                    _ => PieceOwner::NoOwner,
                };
                // println!("piece_type is {:?}, piece_owner is {:?}", piece_type, owner);

                let piece = GamePiece { piece_type, owner };
                pieces[row as usize][col as usize] = piece;
            }
        }

        let mut new_board_fields = Vec::new();
        for field in &BOARD_FIELDS {
            let value = DataMember::<i32>::new_offset(
                process_handle,
                vec![(board + field.offset) as usize],
            )
            .read()
            .unwrap();
            new_board_fields.push(FieldNameWithValue {
                name: field.name,
                value,
            });
        }

        let new_board = GameBoard {
            player_to_move,
            width: board_width,
            height: board_height,
            pieces,
            fields: new_board_fields,
        };
        real_boards.push(new_board);
    }

    return real_boards;
}

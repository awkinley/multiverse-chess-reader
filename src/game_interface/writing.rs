use crate::game_data;
use crate::game_interface::*;

fn write_array(
    process_handle: (*mut winapi::ctypes::c_void, process_memory::Architecture),
    array_offset: u64,
    array: &game_data::ArrayWithCap,
) {
    // new_game_data.board_array.array_ptr = ptr as u64;

    // new_game_data.array_40.array_len = saved_game_data.array_40.array_len;
    // new_game_data.array_40.array_cap = saved_game_data.array_40.array_cap;
    // let ptr = interface::alloc_and_write_memory(
    // process_id,
    // saved_game_data.array_40.bytes.len(),
    // &saved_game_data.array_40.bytes,
    // )
    // .unwrap();
}
fn write_game(
    process_id: winapi::shared::minwindef::DWORD,
    saved_game_data: &game_data::SavedGameData,
    game_data_address: u64,
    process_handle: (*mut winapi::ctypes::c_void, process_memory::Architecture),
) -> game_data::GameData {
    unsafe {
        let game_board_data_length = std::mem::size_of::<game_data::GameData>() as u64;
        let current_board_data =
            reading::read_bytes2(process_handle, game_data_address, game_board_data_length);
        let (_, the_game_data, _) = current_board_data.align_to::<game_data::GameData>();

        let mut new_game_data = game_data::GameData::default();
        new_game_data.unknown1 = saved_game_data.unknown1;
        new_game_data.unknown2 = saved_game_data.unknown2;
        new_game_data.non_neg_universes = saved_game_data.non_neg_universes;
        new_game_data.neg_universes = saved_game_data.neg_universes;
        new_game_data.num_boards = saved_game_data.num_boards;
        new_game_data.unknown3 = saved_game_data.unknown3;
        new_game_data.unknown4 = saved_game_data.unknown4;
        new_game_data.unknown5 = saved_game_data.unknown5;
        new_game_data.unknown6 = saved_game_data.unknown6;
        new_game_data.unknown7 = saved_game_data.unknown7;
        new_game_data.unknown8 = saved_game_data.unknown8;
        new_game_data.unknown9 = saved_game_data.unknown9;
        new_game_data.board_width = saved_game_data.board_width;
        new_game_data.board_height = saved_game_data.board_height;
        new_game_data.unknown10 = saved_game_data.unknown10;
        new_game_data.unknown11 = saved_game_data.unknown11;

        // let mut current_arrays = CURRENT_ARRAYS.lock().unwrap();
        new_game_data.board_array.array_len = saved_game_data.board_array.array_len;
        new_game_data.board_array.array_cap = saved_game_data.board_array.array_cap;
        let ptr = interface::alloc_and_write_memory(
            process_id,
            saved_game_data.board_array.bytes.len(),
            &saved_game_data.board_array.bytes,
        )
        .unwrap();

        new_game_data.board_array.array_ptr = ptr as u64;

        new_game_data.array_40.array_len = saved_game_data.array_40.array_len;
        new_game_data.array_40.array_cap = saved_game_data.array_40.array_cap;
        let ptr = interface::alloc_and_write_memory(
            process_id,
            saved_game_data.array_40.bytes.len(),
            &saved_game_data.array_40.bytes,
        )
        .unwrap();

        new_game_data.array_40.array_ptr = ptr as u64;

        new_game_data.array_50.array_len = saved_game_data.array_50.array_len;
        new_game_data.array_50.array_cap = saved_game_data.array_50.array_cap;
        let ptr = interface::alloc_and_write_memory(
            process_id,
            saved_game_data.array_50.bytes.len(),
            &saved_game_data.array_50.bytes,
        )
        .unwrap();
        new_game_data.array_50.array_ptr = ptr as u64;

        new_game_data.array_60.array_len = saved_game_data.array_60.array_len;
        new_game_data.array_60.array_cap = saved_game_data.array_60.array_cap;
        let ptr = interface::alloc_and_write_memory(
            process_id,
            saved_game_data.array_60.bytes.len(),
            &saved_game_data.array_60.bytes,
        )
        .unwrap();
        new_game_data.array_60.array_ptr = ptr as u64;

        new_game_data.array_70.array_len = saved_game_data.array_70.array_len;
        new_game_data.array_70.array_cap = saved_game_data.array_70.array_cap;
        let ptr = interface::alloc_and_write_memory(
            process_id,
            saved_game_data.array_70.bytes.len(),
            &saved_game_data.array_70.bytes,
        )
        .unwrap();
        new_game_data.array_70.array_ptr = ptr as u64;

        new_game_data.array_80.array_len = saved_game_data.array_80.array_len;
        new_game_data.array_80.array_cap = saved_game_data.array_80.array_cap;
        let ptr = interface::alloc_and_write_memory(
            process_id,
            saved_game_data.array_80.bytes.len(),
            &saved_game_data.array_80.bytes,
        )
        .unwrap();
        new_game_data.array_80.array_ptr = ptr as u64;

        new_game_data.array_90.array_len = saved_game_data.array_90.array_len;
        new_game_data.array_90.array_cap = saved_game_data.array_90.array_cap;
        let ptr = interface::alloc_and_write_memory(
            process_id,
            saved_game_data.array_90.bytes.len(),
            &saved_game_data.array_90.bytes,
        )
        .unwrap();
        new_game_data.array_90.array_ptr = ptr as u64;

        new_game_data.array_a0.array_len = saved_game_data.array_a0.array_len;
        new_game_data.array_a0.array_cap = saved_game_data.array_a0.array_cap;
        let ptr = interface::alloc_and_write_memory(
            process_id,
            saved_game_data.array_a0.bytes.len(),
            &saved_game_data.array_a0.bytes,
        )
        .unwrap();
        new_game_data.array_a0.array_ptr = ptr as u64;

        new_game_data.array_b0.array_len = saved_game_data.array_b0.array_len;
        new_game_data.array_b0.array_cap = saved_game_data.array_b0.array_cap;
        let ptr = interface::alloc_and_write_memory(
            process_id,
            saved_game_data.array_b0.bytes.len(),
            &saved_game_data.array_b0.bytes,
        )
        .unwrap();
        new_game_data.array_b0.array_ptr = ptr as u64;

        new_game_data.array_c0.array_len = saved_game_data.array_c0.array_len;
        new_game_data.array_c0.array_cap = saved_game_data.array_c0.array_cap;
        let ptr = interface::alloc_and_write_memory(
            process_id,
            saved_game_data.array_c0.bytes.len(),
            &saved_game_data.array_c0.bytes,
        )
        .unwrap();
        new_game_data.array_c0.array_ptr = ptr as u64;

        new_game_data.array_d0.array_len = saved_game_data.array_d0.array_len;
        new_game_data.array_d0.array_cap = saved_game_data.array_d0.array_cap;
        let ptr = interface::alloc_and_write_memory(
            process_id,
            saved_game_data.array_d0.bytes.len(),
            &saved_game_data.array_d0.bytes,
        )
        .unwrap();
        new_game_data.array_d0.array_ptr = ptr as u64;

        // let mut current_game_data_ptr = CURRENT_GAME_DATA.lock().unwrap();
        // *current_game_data_ptr = *game_data_ptr;
        return new_game_data;
    }
}

pub fn write_bytes(
    process_handle: (*mut winapi::ctypes::c_void, process_memory::Architecture),
    starting_addresss: u64,
    length_bytes: u64,
    bytes: Vec<u8>,
) {
    use process_memory::*;
    println!("writting values");
    for i in 0..length_bytes {
        let value = bytes[i as usize];
        DataMember::<u8>::new_offset(process_handle, vec![(starting_addresss + i) as usize])
            .write(&value)
            .unwrap();
    }
}


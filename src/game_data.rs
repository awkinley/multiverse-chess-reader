use enum_map::Enum;


#[derive(Debug, Clone, Enum, Copy)]
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

impl From<PieceType> for char {
    fn from(piece_type: PieceType) -> Self {
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
}




#[derive(Debug, Clone, Copy)]
pub enum PieceOwner {
    NoOwner,
    White,
    Black,
}

impl From<PieceOwner> for char {
    fn from(piece_owner: PieceOwner) -> Self {
        match piece_owner {
            PieceOwner::White => 'w',
            PieceOwner::Black => 'b',
            _ => ' ',
        }
    }
}
extern crate num_enum;
use num_enum::TryFromPrimitive;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum PlayerToMove {
    White = 0,
    Black = 1,
    SomethingElse,
}


#[derive(Debug, Clone, Copy)]
pub struct GamePiece {
    pub piece_type: PieceType,
    pub owner: PieceOwner,
}
use crate::notation::Piece;
impl From<GamePiece> for Piece {
    fn from(p: GamePiece) -> Self {
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
}

#[derive(Debug)]
pub struct Field {
    pub name: &'static str,
    pub offset: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct FieldNameWithValue {
    pub name: &'static str,
    pub value: i32,
}

pub const BOARD_NUMBER: &str = "board number";
pub const TIMELINE_NUMBER: &str = "timeline number";
pub const TIME_POSITION: &str = "time position";

pub const NEXT_MOVE_START_ROW: &str = "next move start row";
pub const NEXT_MOVE_START_COL: &str = "next move start col";

pub const NEXT_MOVE_DEST_ROW: &str = "next move dest row";
pub const NEXT_MOVE_DEST_COL: &str = "next move dest col";
pub const NEXT_MOVE_DEST_UNIVERSE: &str = "next move dest universe";
pub const NEXT_MOVE_DEST_TIME: &str = "next move dest time";

pub const NEXT_BOARD_NUMBER: &str = "board number of next board";
pub const CREATED_BOARD_NUMBER: &str = "board number of any new boards created by this boards move";
pub const PREV_BOARD_NUMBER: &str = "board number of previous board";
pub const PLAYER_TO_MOVE: &str = "player to move";
pub const BOARD_FIELDS: [Field; 24] =      [Field {name: BOARD_NUMBER, offset: 0}, 
                                        Field {name: TIMELINE_NUMBER, offset: 4}, 
                                        Field {name: TIME_POSITION, offset: 8},
                                        Field {name: "board id", offset: 144},
                                        Field {name: "is active", offset: 148},
                                        Field {name: "timeline number 2", offset: 152}, 
                                        Field {name: "time position 2", offset: 156},
                                        Field {name: PLAYER_TO_MOVE, offset: 160},
                                        Field {name: NEXT_MOVE_START_ROW, offset: 164},
                                        Field {name: NEXT_MOVE_START_COL, offset: 168},
                                        Field {name: NEXT_MOVE_DEST_UNIVERSE, offset: 172},
                                        Field {name: NEXT_MOVE_DEST_TIME, offset: 176},
                                        Field {name: "next move piece owner", offset: 180},
                                        Field {name: NEXT_MOVE_DEST_ROW, offset: 184},
                                        Field {name: NEXT_MOVE_DEST_COL, offset: 188},
                                        Field {name: "board id of previous move start", offset: 192},
                                        Field {name: NEXT_BOARD_NUMBER, offset: 196},
                                        Field {name: PREV_BOARD_NUMBER, offset: 200},
                                        Field {name: CREATED_BOARD_NUMBER, offset: 204},
                                        Field {name: "board number of board that a piece left to create this board", offset: 208},
                                        Field {name: "last move start row", offset: 212},
                                        Field {name: "last move start col", offset: 216},
                                        Field {name: "last move end row", offset: 220},
                                        Field {name: "last move end col", offset: 224},
                                        ];


#[derive(Debug)]
pub struct GameBoard {
    pub player_to_move: u32,
    pub width: u32,
    pub height: u32,
    pub pieces: Vec<Vec<GamePiece>>, // pieces[row][col]
    pub fields: Vec<FieldNameWithValue>,
}

impl Clone for GameBoard {
    fn clone(&self) -> GameBoard {
        let mut new_pieces = Vec::new();


        for piece_row in &self.pieces {
            let mut row = Vec::new();
            for piece in piece_row {
                let new_piece = piece.clone();
                row.push(new_piece);
            }
            new_pieces.push(row);
        }

        let mut new_fields = Vec::new();

        for field in &self.fields {
            let field_name = field.name;
            let field_value = field.value;
            new_fields.push(FieldNameWithValue{name: field_name, value: field_value});
        }

        GameBoard { player_to_move: self.player_to_move, 
                width: self.width, height: self.height, 
                pieces: new_pieces, fields: new_fields}
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
pub struct WorldPosition {
    pub universe: u32,
    pub time: u32,
    pub player_to_move: u32,
    pub row: u32,
    pub col: u32,
}
#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
pub struct GameData {
    pub unknown1: u32, //80
    pub unknown2: u32, //84
    pub non_neg_universes: u32, //88
    pub neg_universes: u32, //8c
    pub num_boards: u32, //90
    pub unknown3: u32, //94
    pub unknown4: u32, //98
    pub unknown5: u32, //9c
    pub unknown6: u32, //a0 - this seems to be some indication of like what the starting board is or something, 84 means puzzle rook tactics I 
    pub unknown7: u32,
    pub unknown8: u32,
    pub unknown9: u32,
    pub board_array: ArrayWithCap,
    pub array_40: ArrayWithCap,
    pub array_50: ArrayWithCap,
    pub array_60: ArrayWithCap,
    pub array_70: ArrayWithCap,
    pub array_80: ArrayWithCap,
    pub array_90: ArrayWithCap,
    pub array_a0: ArrayWithCap,
    pub array_b0: ArrayWithCap,
    pub array_c0: ArrayWithCap,
    pub array_d0: ArrayWithCap,
    pub board_height: u32,
    pub board_width: u32,
    pub unknown10: u32,
    pub unknown11: u32,
}

pub static GAME_DATA_OFFSET: usize = 0x14ba80;


pub static BOARD_ELEMENT_LENGTH: u32 = 0xe4;
pub static _40_ELEMENT_LENGTH: u32 = 0x4;
pub static _50_ELEMENT_LENGTH: u32 = 0x1;
pub static _60_ELEMENT_LENGTH: u32 = 0x1;
pub static _70_ELEMENT_LENGTH: u32 = 0x8;
pub static _80_ELEMENT_LENGTH: u32 = 0x8;
pub static _90_ELEMENT_LENGTH: u32 = 0x8;
pub static _A0_ELEMENT_LENGTH: u32 = 0x8;
pub static _B0_ELEMENT_LENGTH: u32 = 0x8;
pub static _C0_ELEMENT_LENGTH: u32 = 0x8;
pub static _D0_ELEMENT_LENGTH: u32 = 0x8;

#[derive(Clone, Copy, Default)]
#[repr(C, packed)]
pub struct ArrayWithCap {
    pub array_len: u32,
    pub array_cap: u32,
    pub array_ptr: u64,
}

use std::fmt;
impl fmt::Debug for ArrayWithCap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            write!(f, "{{ length: {}, capacity: {}, address: 0x{:x} }}", self.array_len, self.array_cap, self.array_ptr)
        }
    }
}

#[derive(Debug, Clone)]
pub struct SavedGameData {
    pub unknown1: u32,
    pub unknown2: u32,
    pub non_neg_universes: u32,
    pub neg_universes: u32,
    pub num_boards: u32,
    pub unknown3: u32,
    pub unknown4: u32,
    pub unknown5: u32,
    pub unknown6: u32,
    pub unknown7: u32,
    pub unknown8: u32,
    pub unknown9: u32,
    pub board_array: SavedArrayWithCap,
    pub array_40: SavedArrayWithCap,
    pub array_50: SavedArrayWithCap,
    pub array_60: SavedArrayWithCap,
    pub array_70: SavedArrayWithCap,
    pub array_80: SavedArrayWithCap,
    pub array_90: SavedArrayWithCap,
    pub array_a0: SavedArrayWithCap,
    pub array_b0: SavedArrayWithCap,
    pub array_c0: SavedArrayWithCap,
    pub array_d0: SavedArrayWithCap,
    pub board_height: u32,
    pub board_width: u32,
    pub unknown10: u32,
    pub unknown11: u32,
}

use bytes::Bytes;
#[derive(Debug, Clone)]
pub struct SavedArrayWithCap {
    pub array_len: u32,
    pub array_cap: u32,
    pub bytes: Vec<u8>,
    pub bbytes: Bytes,
}


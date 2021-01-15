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
pub struct GamePiece {
    pub piece_type: PieceType,
    pub owner: PieceOwner,
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
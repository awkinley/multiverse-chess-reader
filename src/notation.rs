use core::num;
use crate::game_data::PieceType;
use enum_map::{enum_map, Enum};
use std::convert::From;
#[derive(Debug, Clone, Copy, Enum, PartialEq)]
pub enum Piece {
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
fn compare_pieces(lhs: &Piece, rhs : &PieceType) -> bool {
    let mut map = enum_map!{
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
    };

    return  *lhs == map[*rhs];

}

impl From<Piece> for &str {
    fn from(piece: Piece) -> Self {
        match piece {
            Piece::Pawn => "P",
            Piece::Knight => "N",
            Piece::Bishop => "B",
            Piece::Rook => "R",
            Piece::Queen => "Q",
            Piece::King => "K",
            Piece::Unicorn => "U",
            Piece::Dragon => "D",
            Piece::Brawn => "BR",
            Piece::FourPointQueen => "S",
            Piece::RoyalQueen => "RQ",
            Piece::CommongKing => "CK",
            Piece::Nothing => "",
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub universe: i32,
    pub time: i32,
    pub row: i32,
    pub col: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub start_loc: Location,
    pub end_loc: Location,
    pub start_piece: Piece,
    pub end_piece: Piece,
    pub is_jump: bool,
    pub is_branching: bool,
    pub does_capture: bool,
    pub moves_present: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    White,
    Black,
}

#[derive(Debug)]
pub struct Turn {
    pub moves: Vec<Move>,
    pub player: Player,
    // pub board_height : u32,
}

fn col_to_string(row: i32) -> &'static str {
    match row {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => "",
    }
}

use super::game_data::GameBoard;
impl Turn {
    pub fn to_notation(&self, prev_board: Option<&GameBoard>) -> String {
        let mut s = String::new();

        for single_move in &self.moves {
            let start_loc = single_move.start_loc;
            let end_loc = single_move.end_loc;

            if single_move.is_jump {
                let start_superphysical_coords = format!("({}T{})", start_loc.universe, start_loc.time);
                let end_superphysical_coords = format!("({}T{})", end_loc.universe, end_loc.time);

                let start_piece = single_move.start_piece;
                let piece_s: &str = start_piece.into();

                let start_loc_s = format!("{}{}", col_to_string(start_loc.col), start_loc.row + 1);
                let end_loc_s = format!("{}{}", col_to_string(end_loc.col), end_loc.row + 1);
                let mut jump_s = ">";
                if single_move.is_branching {
                    jump_s = ">>";
                }

                let mut takes_s = "";
                if single_move.does_capture {
                    takes_s = "x";
                }

                let mut moves_present_s = "";
                if single_move.moves_present {
                    moves_present_s = "~";
                }

                let move_s = format!("{}{}{}{}{}{}{}{}", start_superphysical_coords, 
                                                    piece_s, start_loc_s, jump_s, 
                                                    takes_s,
                                                    end_superphysical_coords,
                                                    end_loc_s, moves_present_s);
                s.push_str(move_s.as_str());
                s.push(' ');
            }
            else {
                let superphysical_coords = format!("({}T{})", start_loc.universe, start_loc.time);
                let start_piece = single_move.start_piece;
                let piece_s: &str = start_piece.into();
                
                let start_loc_s = match self.is_unique_move(prev_board, &single_move) {
                    true => format!("{}{}", col_to_string(start_loc.col), start_loc.row + 1),
                    false => "".to_string(),
                };
                let end_loc = single_move.end_loc;
                let end_loc_s = format!("{}{}", col_to_string(end_loc.col), end_loc.row + 1);
                let mut move_s: String = format!("{}{}{}{}", superphysical_coords, piece_s, start_loc_s, end_loc_s);
                
                let number = (start_piece == Piece::King) as i32 * (end_loc.col - single_move.start_loc.col);
                let castle: &str =  match number {
                    2 => "0-0", //kingside castle
                    -2 => "0-0-0",
                    _ => ""
                };
                if castle != "" {
                    move_s = format!("{}{}", superphysical_coords, castle);
                }
                
    
                s.push_str(move_s.as_str());
                s.push(' ');
            }
           
        }

        return s;
    }

    /** Determine if a move is unique 
     */
    fn is_unique_move(&self, prev_board: Option<&GameBoard>, the_move : &Move) -> bool {
        if the_move.is_jump  || prev_board.is_none() {
            return false
        }
        let start_piece = the_move.start_piece;
        
        let board = prev_board.unwrap();
        let ret = true;
        for piece in board.pieces.iter().flatten() {
            if compare_pieces(&start_piece, &piece.piece_type) {
                return false;
            }
        }
        return ret;
    }

}


#[derive(Clone,Copy,PartialEq,Eq)]
pub struct Move {
    pub from : (usize,usize),
    pub to : (usize,usize)
}

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum Piece_Type {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}


#[derive(Clone,Copy,PartialEq,Eq)]
pub enum Player {
    White,
    Black,
}

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum Piece {P(Player,Piece_Type)}


pub type Board = [[Option<Piece> ; 8] ; 8];

use Player::*;
use Piece_Type::*;
use Piece::*;



const fn SP(p : Player,pic : Piece_Type) -> Option<Piece> { Some(Piece::P(p,pic)) }

// inital_board
pub const inital_board : Board = 
    [[SP(White,Rook),SP(White,Knight),SP(White,Bishop),SP(White,King),SP(White,Queen),SP(White,Bishop),SP(White,Knight),SP(White,Rook)]
    ,[SP(White,Pawn); 8]
    ,[None; 8]
    ,[None; 8]
    ,[None; 8]
    ,[None; 8]
    ,[SP(Black,Pawn); 8]
    ,[SP(Black,Rook),SP(Black,Knight),SP(Black,Bishop),SP(Black,King),SP(Black,Queen),SP(Black,Bishop),SP(Black,Knight),SP(Black,Rook)]];

pub trait Show {
    fn show(&self) -> String;
}

pub trait Read<T> {
    fn read(&self) -> Option<T>;
}


impl Read<Piece_Type> for String {
    fn read(&self) -> Option<Piece_Type> {
        let mut s = self.clone();
        s.make_ascii_lowercase();
        match s.as_str().trim() {
           "pawn"   => {Some(Pawn)}
           "bishop" => {Some(Bishop)}
           "knight" => {Some(Knight)}
           "rook"   => {Some(Rook)}
           "queen"  => {Some(Queen)}
           "king"   => {Some(King)}
           _ => {None}
        }
    }
}

impl Show for Piece_Type {
    fn show(&self) -> String {
        String::from(
            match self {
                Pawn   => {"pawn"}
                Bishop => {"bishop"}
                Knight => {"knight"}
                Rook   => {"rook"}
                Queen  => {"queen"}
                King   => {"king"}
            }
        )
    }
}

impl Show for Player {
    fn show(&self) -> String {
        match self {
            White => {String::from("White")}
            Black => {String::from("Black")}
        }
    }
}
impl Show for Piece {
    fn show(&self) -> String {
        let mut s : String = String::new();
        s.push(match self {
            //black 
            P(Black,King)   =>  {'♔'}
            P(Black,Queen)  =>  {'♕'}
            P(Black,Rook)   =>  {'♖'}
            P(Black,Bishop) =>  {'♗'}
            P(Black,Knight) =>  {'♘'}
            P(Black,Pawn)   =>  {'♙'}
            //white
            P(White,King)   =>  {'♚'}
            P(White,Queen)  =>  {'♛'}
            P(White,Rook)   =>  {'♜'}
            P(White,Bishop) =>  {'♝'}
            P(White,Knight) =>  {'♞'}
            P(White,Pawn)   =>  {'♟'}
        });
        s
    }
}

pub fn switch(p : &mut Player) -> () {
    *p = match &p {
        Black => { White }
        White => { Black }
    };
}
// pub fn at(board : &Board, i : u8, j : u8) -> Option<Piece> {
//     board[usize::from(i)][usize::from(j)]
// }

pub fn execute(board : &mut Board, m:Move) {
    let (old_i,old_j) = m.from;
    let (old_i,old_j) = (usize::from(old_i),usize::from(old_j));
    let (new_i,new_j) = m.to;
    let (new_i,new_j) = (usize::from(new_i),usize::from(new_j));
    let piece = board[old_i][old_j];
    board[old_i][old_j] = None;
    board[new_i][new_j] = piece;
}
pub fn print_board(b : Board) -> () {
    let mut s : String;
//    let black_tile = Colour::White.on(Colour::Black);
//    let white_tile = Colour::Black.on(Colour::White);
    // first line
    println!("  1 2 3 4 5 6 7 8");
    let line_letter : [char ; 8] = ['a','b','c','d','e','f','g','h'];
    for i in 0..8 {
        s = String::from("");
        s.push(line_letter[i]);
        s.push(' ');
        for j in 0..8 {
            match b[i][j] {
                Some(piece) => {
                    s.push_str(&piece.show());
                    s.push(' ');
                }
                None => {
                    s.push_str("  ");
                }
            }

        }
        s.push(line_letter[i]);
        println!("{}",s);
    }
    // last line
    println!("  1 2 3 4 5 6 7 8");
}

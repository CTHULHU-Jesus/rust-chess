mod data_types;

use data_types::*;
use Player::*;
use PieceType::*;
use Piece::*;
use std::io;

fn not(x:bool) -> bool {
    if x { false } else { true }
}

fn get_player(board : &Board,i: isize, j : isize) -> Option<Player> {
        match board[i as usize][j as usize] {
            None => None,
            Some(P(p_player,_)) => Some(p_player),
        }
}

fn valid_move(board : &Board, player : Player, piece : PieceType, act : &Move) -> bool {
    
    let (old_i,old_j) = act.from;
    let (old_i,old_j) = (old_i as isize,old_j as isize);
    let (new_i,new_j) = act.to;
    let (new_i,new_j) = (new_i as isize,new_j as isize);
    // makes shure you wont over write your friend
    let landing_check = 
        match get_player(board,new_i,new_j) {
            None => true,
            Some(p_player) if p_player != player => true,
            _ => false,
        };

    match piece {
        Knight => {
            // knights go +-2 in one direction and +-1 in another
            let (a,b) = ((old_i-new_i).abs(),(old_j-new_j).abs());
            match (a,b) {
                (1,2) | (2,1) => landing_check,
                _ => false,
            }
        }

        Rook => {
            //fid out direction Rook is going
            let (a,b) = ((old_i-new_i).abs(),(old_j-new_j).abs());
            match (a,b) {
                (0,x) if x != 0 => {
                    // check path to spot
                    let mut in_way = false;
                    for j in (old_j+1)..new_j {
                        if in_way {
                            break;
                        } else {
                            in_way = board[old_i as usize][j as usize] != None;
                        }
                    };
                    not(in_way)&&landing_check
                }
                (x,0) if x != 0 => {
                    // check path to spot
                    let mut in_way = false;
                    for i in (old_i+1)..new_i {
                        if in_way {
                            break;
                        } else {
                            in_way = board[i as usize][old_j as usize] != None;
                        }
                    };
                    not(in_way)&&landing_check
                }

                _ => false,
            }
        }

            King => {
                let (a,b) = ((old_i-new_i).abs(),(old_j-new_j).abs());
                if (a == 0 || a == 1) && (b == 0 || a == 1) {
                    landing_check
                } else {
                    // TODO check for castleing
                    false
                }
            }

            Queen => valid_move(board,player,Bishop,act)||valid_move(board,player,Rook,act),
            
            Bishop => {
                let (a,b) = ((old_i-new_i),(old_j-new_j));
                if a.abs() == b.abs() {
                    match (a,b) {
                        (a,b) if a > 0 && b > 0 => {
                            // check for people in the way
                            let mut in_way = false;
                            for x in 1..a { 
                                if in_way {
                                    break;
                                } else {
                                    in_way = board[(old_i+x) as usize][(old_j+x) as usize] !=None;
                                }
                            };
                            not(in_way)&&landing_check
                        }
                        (a,b) if a < 0 && b > 0 => {
                            // check for people in the way
                            let mut in_way = false;
                            for x in 1..a { 
                                if in_way {
                                    break;
                                } else {
                                    in_way = board[(old_i-x) as usize][(old_j+x) as usize] !=None;
                                }
                            };
                            not(in_way)&&landing_check
                        }
                        (a,b) if a > 0 && b < 0 =>  {
                            // check for people in the way
                            let mut in_way = false;
                            for x in 1..a { 
                                if in_way {
                                    break;
                                } else {
                                    in_way = board[(old_i+x) as usize][(old_j-x) as usize]!=None;
                                }
                            };
                            not(in_way)&&landing_check
                        }
                        (a,b) if a < 0 && b < 0 =>  {
                            // check for people in the way
                            let mut in_way = false;
                            for x in 1..a { 
                                if in_way {
                                    break;
                                } else {
                                    in_way = board[(old_i-x) as usize][(old_j-x) as usize] !=None;
                                }
                            };
                            not(in_way)&&landing_check
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            }

            Pawn => {
                let inital_pos;
                let direction;
                match player {
                    // white pawns can only move down
                    White => {
                       inital_pos=1;
                       direction=1;
                    }
                    // Black pawns can only move up
                    Black => {
                        inital_pos=6;
                        direction=-1;
                    }
                };
                // check if pawn is in inital position
                let diff_i = direction*(new_i-old_i);
                let diff_j = new_j-old_j;
                // is it a normal move?
                if diff_i==inital_pos||(diff_i==2&&old_i==1){
                    // check side to side movement
                    if diff_j == 0 {
                        landing_check
                    } // are we takeing another piece
                    else if diff_j.abs() ==1 {
                        match board[new_i as usize][new_j as usize] {
                            Some(P(p_player,_)) if p_player != player => { true }
                            _ => { false }
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
    }
}
// parse things like 'a2 to b2'
fn parse_move(board :&Board, input : &String,player : Player) -> Result<Move,&'static str> {
    const PARSE_ERR : &str = "parse error";
    const SELECT_ERR : &str = "no select";
    const MOVE_ERR : &str = "bad move";
    fn find(c: char) -> Result<usize,&'static str> {
        match c{
            'a' | '1' => {Ok(0)}
            'b' | '2' => {Ok(1)}
            'c' | '3' => {Ok(2)}
            'd' | '4' => {Ok(3)}
            'e' | '5' => {Ok(4)}
            'f' | '6' => {Ok(5)}
            'g' | '7' => {Ok(6)}
            'h' | '8' => {Ok(7)}
            _ => {Err(PARSE_ERR)}
        }
    };
    fn parse_point(word : String) -> Result<(usize,usize),&'static str> {
        let mut chars = word.chars();
        let x : usize = find(chars.next().ok_or(PARSE_ERR)?)?;
        let y : usize = find(chars.next().ok_or(PARSE_ERR)?)?;
        if chars.next() == None {
            Ok((x,y))
        } else {
            Err(PARSE_ERR)
        }
    };
    let mut input = (*input).clone();
    input = String::from(input.trim());
    let mut input = input.split_whitespace();
    let word1 = input.next().ok_or(PARSE_ERR)?;
    let word2 = input.next().ok_or(PARSE_ERR)?;
    let word3 = input.next().ok_or(PARSE_ERR)?;
    let from : (usize,usize) = parse_point(word1.to_string())?;
    let to : (usize,usize) = parse_point(word3.to_string())?;
    let act = Ok(Move {from:from,to:to});

    if word2 != "to" {
        Err(PARSE_ERR)
    } else
    {
        let (old_i,old_j) = from;
        match board[old_i][old_j]{
            None => { Err(SELECT_ERR) }
            Some(P(p_player,piece)) =>
                {
                    // println!("selected {} at ({},{})", P(p_player,piece).show(),old_i,old_j);
                    if p_player == player {
                        let action = act.expect("");
                       if valid_move(board,player,piece,&action) {
                           act
                       } else {
                           Err(MOVE_ERR)
                       }
                    } else
                    {
                        Err(SELECT_ERR)
                    }
                }
        }
    }
}

// returns the player that won or None if there was no winner
fn who_won(board : &Board) -> Option<Player> {
    let mut black_king_exists = false; 
    let mut white_king_exists = false;
    for i in 0..8 {
        for j in 0..8 {
            match board[i][j] {
               Some(P(player,King)) => {
                   match player {
                       White => {white_king_exists = true;}
                       Black => {black_king_exists = true;}
                   }
               }
                _ => ()
            }
        }
    };
    if not(black_king_exists) {
        Some(White)
    } else if not(white_king_exists) {
        Some(Black)
    } else {
        None
    }
}

fn main() {
    let mut board : Board = INITAL_BOARD;
    let mut winner : Option<Player> = None;
    let mut turn : Player = White; // white goes First
    let mut input;
    while winner == None {
        print_board(board);
        println!("{} player, what is your move?", turn.show());
        input = String::from(""); // reset input
        io::stdin().read_line(&mut input).expect("could not get input");
        match parse_move(&board, &input, turn) {
            Ok(action) => {
                execute(&mut board,action);
                switch(&mut turn);
                winner = who_won(&board)
            }
            Err("parse error") => {
                println!("\"{}\" is not valed input. try something like \"a2 to b2\"",input);
            }
            Err("no select") => {
               println!("You did not select a valid piece"); 
            }
            Err("bad move") => {
                println!("That does not move like that")
            }
            Err(_) => {
                println!("Unknown error")
            }
        }
    }
    match winner {
        Some(White) => {
            println!("Congrats White Player");
        }
        Some(Black) => {
            println!("Congrats Black Player");
        }
        None => {
            panic!("There was no winner");
        }
    }
}

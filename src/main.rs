use std::io;
use regex::Regex;

mod util;

#[derive(Copy, Clone, PartialEq)]
enum MoveResult {
    InvalidMove,
    CorrectMove,
    Score,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Cross,
    CrossKing,
    Circle,
    CircleKing,
    Empty,
}

/// Keep track of total pieces captured.
static mut X_SCORE : i32 = 0;
static mut O_SCORE : i32 = 0;

/// Returns an initialized board.
fn initialize_board() -> [[Tile;8];8] {
    let mut board = [[Tile::Empty;8];8];

    for _y in 0..3 {
        for _x in 0..8 {
            if _y % 2 == 1 && _x % 2 == 0 {
                board[_y][_x] = Tile::Cross;
            } else if _y % 2 == 0 && _x % 2 == 1 {
                board[_y][_x] = Tile::Cross;
            }
        }
    }

    for _y in 5..8 {
        for _x in 0..8 {
            if _y % 2 == 1 && _x % 2 == 0 {
                board[_y][_x] = Tile::Circle;
            } else if _y % 2 == 0 && _x % 2 == 1 {
                board[_y][_x] = Tile::Circle;
            }
        }
    }

    return board;
}

/// Clear the terminal and print the score and board.
fn draw_board(board : [[Tile;8];8]) -> [[Tile;8];8] {
    print!("{}[2J", 27 as char);
    println!("===============================");
    println!("          PIETER X BIT");
    println!("          ------------");
    unsafe {
        println!("  Score X: {} <=> Score O: {}", X_SCORE, O_SCORE);
    }
    println!("===============================");
    println!("   0  1  2  3  4  5  6  7");
    for _y in 0..8 {
        let mut line = (util::LETTER_MAP[_y]).to_string() + " ";
        for _x in 0..8 {
            let symbol = tile_to_string(board[_y][_x]);
            line += &("[".to_owned() + &symbol + "]");
        }
        println!("{}", line);
    }
    board
}

/// Return the String representation of
/// a tile.
fn tile_to_string(tile : Tile) -> String {
    match tile {
        Tile::Cross => String::from("X"),
        Tile::CrossKing => String::from("X̂"),
        Tile::Circle => String::from("0"),
        Tile::CircleKing => String::from("Ō"),
        Tile::Empty => String::from(" "),
    }
}

/// Move a tile from one place to the other on the board.
fn move_piece(mut board : [[Tile;8];8], from : (i32,i32), to : (i32, i32)) -> [[Tile;8];8] {
    let mut moving_piece = board[from.0 as usize][from.1 as usize];
    if moving_piece == Tile::Cross && to.0 == 7 {
        moving_piece = Tile::CrossKing;
    } else if moving_piece == Tile::Circle && to.0 == 0 {
        moving_piece = Tile::CircleKing;
    }
    board[to.0 as usize][to.1 as usize] = moving_piece;
    board[from.0 as usize][from.1 as usize] = Tile::Empty;
    return board;
}

/// Returns true if the point is out of bounds.
fn out_of_bounds(point : (i32, i32)) -> bool {
    !(0 <= point.1 && point.1 < 8 && 0 <= point.0 && point.0 < 8)
}

/// Return the opponents peasant tile type.
fn opposite_tile(player : Tile) -> Tile {
    match player {
        Tile::Cross => Tile::Circle,
        Tile::CrossKing => Tile::Circle,
        Tile::Circle => Tile::Cross,
        Tile::CircleKing => Tile::Cross,
        Tile::Empty => Tile::Empty,
    }
}

/// Check if a piece can be captured.
/// If the locations are not out of bounds
/// and the piece to be captured is of the opposite side,
/// and the final location is empty,
/// return true.
fn can_make_move(board : &mut [[Tile;8];8], target : Tile, oppo : (i32, i32), final_loc : (i32, i32)) -> bool {
    return !out_of_bounds(final_loc) && !out_of_bounds(oppo) && board[oppo.0 as usize][oppo.1 as usize] == target
        && board[final_loc.0 as usize][final_loc.1 as usize] == Tile::Empty;
}

/// Move a point to a valid position.
/// If invalid position, return the original point.
fn move_point(point : (i32, i32), y : i32, x: i32) -> (i32, i32) {
    let new_p = (point.0 + y, point.1 + x);
    if out_of_bounds(new_p) {
        return point;
    }
    return new_p;
}

/// Return true if the player can capture an opponents piece.
fn can_score(board : &mut [[Tile;8];8], player : Tile) -> bool {
    let mut score = false;
    let opposite = opposite_tile(player);
    for _y in 0..8 {
        for _x in 0..8 {
            let tile = board[_y][_x];
            let is_match = tile == player;
            let top_left = move_point((_y as i32, _x as i32), -1, -1);
            let top_right = move_point((_y as i32, _x as i32), -1, 1);
            let top_left_2 = move_point((_y as i32, _x as i32), -2, -2);
            let top_right_2 = move_point((_y as i32, _x as i32), -2, 2);
            let bot_left = move_point((_y as i32, _x as i32), 1, -1);
            let bot_right = move_point((_y as i32, _x as i32), 1, 1);
            let bot_left_2 = move_point((_y as i32, _x as i32), 2, -2);
            let bot_right_2 = move_point((_y as i32, _x as i32), 2, 2);

            if is_match {
                score = score || can_make_move(board, opposite, top_left, top_left_2);
                score = score || can_make_move(board, opposite, top_right, top_right_2);
                score = score || can_make_move(board, opposite, bot_left, bot_left_2);
                score = score || can_make_move(board, opposite, bot_right, bot_right_2);
            }
            if score {
                return score;
            }
        }
    }
    return score;
}

/// Increment the score of a player.
fn add_score(player : Tile) {
    unsafe {
        match player {
            Tile::Cross => X_SCORE += 1,
            Tile::Circle => O_SCORE += 1,
            Tile::CrossKing => return,
            Tile::CircleKing => return,
            Tile::Empty => return,
        }
    }
}

/// Return the king version of a tile type.
fn king_version(player : Tile) -> Tile {
    match player {
        Tile::Cross => Tile::CrossKing,
        Tile::Circle => Tile::CircleKing,
        Tile::CrossKing => Tile::CrossKing,
        Tile::CircleKing => Tile::CircleKing,
        Tile::Empty => Tile::Empty,
    }
}

/// Check if a move is valid according to the rules of the game.
/// Returns true if the move is valid.
fn is_valid_move(board : &mut [[Tile;8];8], from : (i32, i32), to : (i32, i32), player : Tile) -> MoveResult {
    let mut dir = 1;
    if player == Tile::Cross {
        dir = -1;
    }

    let (y1, x1) = from;
    let (y2, x2) = to;

    if from == to || out_of_bounds(to) {
        println!("Out of bounds or same location!");
        return MoveResult::InvalidMove;
    }

    if board[y2 as usize][x2 as usize] != Tile::Empty {
        println!("Spot already taken!");
        return MoveResult::InvalidMove;
    }

    let is_king = board[y1 as usize][x1 as usize] == Tile::CrossKing ||
                board[y1 as usize][x1 as usize] == Tile::CircleKing;

    // There are 6 possible valid positions.
    let left_top_1 = move_point(from, dir * -1, dir * -1);
    let left_top_2 = move_point(from, dir * -2, dir * -2);
    let right_top_1 = move_point(from, dir * -1, dir * 1);
    let right_top_2 = move_point(from, dir * -2, dir * 2);
    let left_bot_1 = move_point(from, dir * 1, dir * -1);
    let right_bot_1 = move_point(from, dir * 1, dir * 1);
    let left_bot_2 = move_point(from, dir * 2, dir * -2);
    let right_bot_2 = move_point(from, dir * 2, dir * 2);

    let right_top_spot = king_version(board[(right_top_1.0 as usize)][(right_top_1.1 as usize)]);
    let left_top_spot = king_version(board[left_top_1.0 as usize][left_top_1.1 as usize]);
    let left_bot_spot = king_version(board[left_bot_1.0 as usize][left_bot_1.1 as usize]);
    let right_bot_spot = king_version(board[right_bot_1.0 as usize][right_bot_1.1 as usize]);

    let opposite_king = king_version(opposite_tile(player));

    if to == right_top_2 && right_top_spot == opposite_king {
        board[right_top_1.0 as usize][right_top_1.1 as usize] = Tile::Empty;
        add_score(player);
        return MoveResult::Score;
    } else if to == left_top_2 && left_top_spot == opposite_king {
        board[left_top_1.0 as usize][left_top_1.1 as usize] = Tile::Empty;
        add_score(player);
        return MoveResult::Score;
    } else if to == left_bot_2 && left_bot_spot == opposite_king {
        board[left_bot_1.0 as usize][left_bot_1.1 as usize] = Tile::Empty;
        add_score(player);
        return MoveResult::Score;
    } else if to == right_bot_2 && right_bot_spot == opposite_king {
        board[right_bot_1.0 as usize][right_bot_1.1 as usize] = Tile::Empty;
        add_score(player);
        return MoveResult::Score;
    } else if (to == right_top_1 || to == left_top_1) || (is_king && (to == left_bot_1 || to == right_bot_1)){
        return MoveResult::CorrectMove;
    }

    return MoveResult::InvalidMove;
}

/// Prompt the user to input the piece to move and where to move it to.
fn ask_move(whose_turn : Tile, board : &mut [[Tile;8];8]) -> ((i32, i32), (i32, i32)) {
    println!("================");
    println!("[{}] is playing!", tile_to_string(whose_turn));

    let mut from_coords;
    let mut to_coords;

    let re = Regex::new(r"[a-hA-H][0-8]").unwrap();

    loop {
        println!("Please enter your next piece to move");
        let mut from_piece = String::new();
        io::stdin().read_line(&mut from_piece).expect("Did not enter a correct string");
        if from_piece.chars().count() < 2 || !re.is_match(&from_piece[0..2]) {
            println!("Invalid format; example: A6, a2 etc. try again..");
            continue;
        }

        from_coords = util::coords_from_string(from_piece);
        if !out_of_bounds(from_coords) &&
                    king_version(board[from_coords.0 as usize][from_coords.1 as usize]) == king_version(whose_turn) {
            break;
        } else {
            println!("Invalid tile, try again..");
        }
    }

    loop {
        println!("Where do you want to move it?");
        let mut to_piece = String::new();
        io::stdin().read_line(&mut to_piece).expect("Did not enter a correct string");
        if to_piece.chars().count() < 2 || !re.is_match(&to_piece[0..2]) {
            println!("Invalid format; example: A6, a2 etc. try again..");
            continue;
        }

        to_coords = util::coords_from_string(to_piece);
        if !out_of_bounds(to_coords) &&
                    board[to_coords.0 as usize][to_coords.1 as usize] == Tile::Empty {
            break;
        } else {
            println!("Invalid tile, try again..")
        }
    }

    return (from_coords, to_coords);
}

/// Show the final result (end of game).
fn show_result(winner : Tile) {
    print!("{}[2J", 27 as char);
    println!("===============================");
    println!("          PIETER X BIT");
    println!("          ------------");
    println!("{} WINS THE GAME! Congratulations!", tile_to_string(winner));
    unsafe {
        println!("  Final Score: X: {} <=>  O: {}", X_SCORE, O_SCORE);
    }
    println!("===============================");
    util::prompt_keypress();
}

/// Main function, code starts here.
fn main() {

    let mut whose_turn = Tile::Circle; // circle is upwards, cross downwards
    let mut board = initialize_board();

    loop {
        board = draw_board(board);
        let (from, to) = ask_move(whose_turn, &mut board);
        let move_result = is_valid_move(&mut board, from, to, whose_turn);
        let score_possible = can_score(&mut board, whose_turn);
        if move_result == MoveResult::InvalidMove {
            println!("========================");
            println!("Invalid move! Try again.");
            println!("========================");
            util::prompt_keypress();
            continue;
        } else if move_result == MoveResult::CorrectMove {
            if score_possible {
                println!("Remember: You must score if possible!");
                util::prompt_keypress();
                continue;
            }
            whose_turn = opposite_tile(whose_turn);
        } else {
            unsafe{
                if &X_SCORE >= &12 {
                    show_result(Tile::Cross);
                    break;
                } else if &O_SCORE >= &12{
                    show_result(Tile::Circle);
                    break;
                }
            }
        }
        board = move_piece(board, from, to);
    }
}

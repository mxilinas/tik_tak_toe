#![allow(dead_code, unused)]
use std::f32::INFINITY;
use std::{io, thread, time};
mod tests;
use draw::*;
use render_tree::*;

// CONSTANTS

const SIDE_LEN: u32 = 256;

// DATA DEFINITIONS

/// A square on a tik-tak-toe board.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Square {
    E,
    X,
    O,
}
const E: Square = Square::E; // An empty square
const X: Square = Square::X;
const O: Square = Square::O;
// Template
fn fn_for_square(s: Square) {
    match s {
        E => return,
        X => return,
        O => return,
    }
}

/// A tik-tak-toe board.
type Board = [Square; 9];
const BD0: Board = [E, O, O, E, X, E, X, O, E]; // An empty board.
const BD1: Board = [X, X, X, X, X, X, X, X, X];
const BD2: Board = [X, E, X, X, X, E, X, X, X];
const BD3: Board = [E, E, O, E, O, E, O, E, E];
const BD4: Board = [E, E, E, X, X, X, E, E, E];
// Template
fn fn_for_bd(bd: Board) {
    for sqr in bd {
        fn_for_square(sqr);
    }
}

// FUNCTIONS

/// Produce the best possible move from a given board state
fn best_move(bd: Board) -> Option<Board> {
    let mut highest_value = -INFINITY;
    let mut best_move = None;

    for bd in next_boards(bd, X) {
        let value = minimax(bd);

        if value > highest_value {
            highest_value = value;
            best_move = Some(bd);
        }
    }

    if highest_value == -1.0 {
        return None;
    }
    return best_move;
}

/// Produce a drawing of a single board
fn draw_board(bd: Board, bg: RGB) -> Drawing {
    let line_width = 10;

    // Outline
    let mut background = Drawing::new()
        .with_shape(Shape::Rectangle {
            width: SIDE_LEN,
            height: SIDE_LEN,
        })
        .with_style(Style::filled((bg)));

    // Lines
    for i in 0..9 {
        let mut color = RGB::new(0, 0, 0);
        let sqr = bd[i];
        if sqr == O {
            color = RGB::new(0, 255, 0);
        } else if sqr == X {
            color = RGB::new(255, 0, 0);
        } else {
            color = RGB::new(255, 255, 255);
        }

        let pos = index_to_pos(i, 3);
        let x = SIDE_LEN as f32 / 3.0 * pos.x + line_width as f32 / 2.0;
        let y = SIDE_LEN as f32 / 3.0 * pos.y + line_width as f32 / 2.0;

        let side_len = SIDE_LEN / 3 - line_width;
        let tile = Drawing::new()
            .with_shape(Shape::Rectangle {
                width: side_len,
                height: side_len,
            })
            .with_style(Style::filled(color))
            .with_xy(x, y);

        background.display_list.add(tile);
    }

    return background;
}

/// Green circles are Os
/// Red circles are Xs
fn draw_boards(bd: Board) -> Drawing {
    fn fn_for_board(bd: Board, team: Square) -> Drawing {
        let y_offset = 1024.0;

        let mut b = draw_board(bd, Color::black());
        if solved(bd, X) {
            return b;
        }
        if solved(bd, O) {
            return b;
        }
        if tied(bd) {
            return b;
        }

        if team == X {
            let subs = next_boards(bd, O);
            let mut depth = subs.len() as u8;
            if depth > 0 {
                depth -= 1;
            }
            let tree = fn_for_lobd(subs, O);
            return above(tree, b, y_offset, depth);
        }

        let subs = next_boards(bd, X);
        let mut depth = subs.len() as u8;
        if depth > 0 {
            depth -= 1;
        }
        let tree = fn_for_lobd(subs, X);
        return above(tree, b, y_offset, depth);
    }

    /// Produce the value of the board with the highest evaluation
    fn fn_for_lobd(lobd: Vec<Board>, team: Square) -> Drawing {
        let x_offset = 256.0;
        let mut d = Drawing::new();
        for bd in lobd {
            d = beside_align_top(d, fn_for_board(bd, team), x_offset);
        }

        return d;
    }

    return fn_for_board(bd, X);
}

/// Produce the value of the given board
fn minimax(bd: Board) -> f32 {
    /// Produce the value of the given board
    fn evaluate(bd: Board, team: Square) -> f32 {
        if solved(bd, X) {
            return 1.0;
        }
        if solved(bd, O) {
            return -1.0;
        }
        if tied(bd) {
            return 0.0;
        }

        if team == X {
            return minimize(next_boards(bd, O), O);
        }

        return maximize(next_boards(bd, X), X);
    }

    /// Produce the value of the board with the highest evaluation
    fn maximize(lobd: Vec<Board>, team: Square) -> f32 {
        let mut value = -INFINITY;
        for bd in lobd {
            value = value.max(evaluate(bd, team));
        }

        return value;
    }

    /// Produce the value of the board with the lowest evaluation
    fn minimize(lobd: Vec<Board>, team: Square) -> f32 {
        let mut value = INFINITY;
        for bd in lobd {
            value = value.min(evaluate(bd, team));
        }

        return value;
    }

    return evaluate(bd, X);
}

/// Return the opposite of the given team
fn switch(team: Square) -> Square {
    match team {
        O => return X,
        X => return O,
        E => return E,
    }
}
/// Return true if the game is tied
fn tied(bd: Board) -> bool {
    for sqr in bd {
        if sqr == E {
            return false;
        }
    }

    true
}

/// Return the winning type if solved, else return None.
fn solved(bd: Board, team: Square) -> bool {
    // True if there are 3 diagonal squares of the same type on the board
    fn solved_diagonals(bd: &Board, team: Square) -> bool {
        fn solved_diagonal(bd: &Board, indices: [usize; 3], team: Square) -> bool {
            let mut count = 0;
            for i in indices {
                if bd[i] == team {
                    count += 1;
                }
                if count == 3 {
                    return true;
                }
            }

            return false;
        }

        solved_diagonal(bd, [2, 4, 6], team) || solved_diagonal(bd, [0, 4, 8], team)
    }

    /// True if there are 3 squares of the same type in a vertical or horizontal line
    fn solved_lines(bd: &Board, team: Square) -> bool {
        for column in 0..3 {
            let mut row_count = 0;
            let mut column_count = 0;

            for row in 0..3 {
                let column_value = bd[pos_to_index(row, column, 3)];
                let row_value = bd[pos_to_index(column, row, 3)];

                if column_value == team {
                    column_count += 1;
                }
                if column_count == 3 {
                    return true;
                }

                if row_value == team {
                    row_count += 1;
                }
                if row_count == 3 {
                    return true;
                }
            }
        }

        return false;
    }

    solved_diagonals(&bd, team) || solved_lines(&bd, team)
}

/// Convert an x,y coordinate to an array index
/// sqrt is the square root of the length of the array
fn pos_to_index(x: usize, y: usize, sqrt: usize) -> usize {
    y * sqrt + x
}

/// Convert an x,y coordinate to an array index
/// sqrt is the square root of the length of the array
fn index_to_pos(index: usize, sqrt: usize) -> Point {
    let mut x = 0;
    let mut y = 0;

    for i in 0..index {
        x += 1;
        if x == sqrt {
            y += 1;
            x = 0;
        }
    }

    return Point {
        x: x as f32,
        y: y as f32,
    };
}

/// Team must be either X or O
/// Return the next possible boards
fn next_boards(bd: Board, team: Square) -> Vec<Board> {
    let mut next_boards: Vec<Board> = vec![];

    for i in 0..9 {
        if bd[i] == E {
            let mut new_board = bd.clone();
            if team == X {
                new_board[i] = X;
            }
            if team == O {
                new_board[i] = O;
            }
            next_boards.push(new_board);
        }
    }

    return next_boards;
}

/// Print a board to the console
fn print_board(bd: Board) {
    let mut row = 0;
    for s in bd {
        print!(" {:?}", s);
        row += 1;
        if row == 3 {
            println!("");
            row = 0;
        }
    }
}

/// Play tik-tak-toe against the computer!
fn play() {
    fn main_loop(bd: Board) {
        fn get_input() -> usize {
            println!("Enter Index: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            match input.trim().parse() {
                Ok(n) => {
                    let result = valid_input(n);
                    match result {
                        Ok(i) => return i,
                        Err(e) => return get_input(),
                    }
                }
                Err(e) => {
                    return get_input();
                }
            };
        }

        fn valid_input(input: usize) -> Result<usize, String> {
            if input > 8 {
                return Err("Index must be in [0, 8]".to_string());
            }

            return Ok(input);
        }

        let index = get_input();
        let mut usr_move = bd.clone();

        usr_move[index] = O;

        println!();
        print_board(usr_move);
        println!();

        let comp_mv = best_move(usr_move);

        if let Some(mut comp_move) = comp_mv {
            let time = time::Duration::from_secs(1);
            thread::sleep(time);
            print_board(comp_move);

            return main_loop(comp_move);
        } else {
            println!("Game over!");
            return;
        }
    }

    println!();
    print_board(BD0);
    main_loop(BD0);
}

/// Produce an image of the move tree of a given board.
fn draw_tree(bd: Board) {
    let mut boards = draw_boards(bd);
    let bounds = get_bounds(&boards);

    let width = (bounds.right + 100.0) - (bounds.left - 100.0);
    let height = (bounds.bottom + 150.0) - (bounds.top - 100.0);

    boards = center(boards, width, height, 0.0);
    let mut lines = draw_lines(&boards, 0.0);

    let mut canvas = Canvas::new(width as u32, height as u32);
    canvas.display_list.add(lines);
    canvas.display_list.add(boards);

    render::save(&canvas, "./output.svg", SvgRenderer::new()).expect("Failed to save");
}

fn main() {
    draw_tree(BD0);
}

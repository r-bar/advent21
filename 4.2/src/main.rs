use std::{fs, env};
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;

type Cell = u8;

type Board = Vec<Vec<Cell>>;

struct Game {
    call_pointer: usize,
    calls: Vec<Cell>,
    boards: Vec<Board>,
}

struct Win<'a> {
    board: &'a Board,
    calls: &'a [Cell],
    winning_number: &'a Cell,
    index: usize,
    score: usize,
}

impl Game {
    fn from_file(path: &str) -> anyhow::Result<Self> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(&file);
        let mut lines = reader.lines().map(|result| result.map(|l| l.trim().to_string()));
        let calls: Vec<Cell> = lines.next().unwrap()?.split(',').map(|i| Cell::from_str_radix(i, 10).unwrap()).collect();
        // discard empty line after call line
        lines.next();
        let mut board = Vec::new();
        let mut boards = Vec::new();
        for line in lines {
            let line = line?;
            if line == "" {
                boards.push(board);
                board = Vec::new();
                continue
            }
            let row: Vec<Cell> = line.split_ascii_whitespace().map(|i| Cell::from_str_radix(i, 10).unwrap()).collect();
            board.push(row)
        }
        if board.len() > 0 {
            boards.push(board);
        }
        Ok(Game { calls, boards, call_pointer: 0 })
    }

    fn called<'a>(&'a self) -> &'a [Cell] {
        &self.calls[0..self.call_pointer]
    }

    // plays all boards returning the calls and winning board
    fn play<'a>(&'a mut self) -> Option<Win<'a>> {
        while self.call_pointer < self.calls.len() {
            for (i, board) in self.boards.iter().enumerate() {
                if let Some(winning_numbers) = win(self.called(), &board) {
                    let calls = self.called();
                    return Some(Win {
                        board,
                        calls,
                        winning_number: &calls[self.call_pointer - 1],
                        index: i,
                        score: score(calls, &board),
                    })
                }
            }
            self.call_pointer += 1;
        }
        None
    }
}

/// Determines if the board wins with the given calls. Returns the winning row or column numbers.
fn win(calls: &[Cell], board: &Board) -> Option<Vec<Cell>> {
    let calls: HashSet<Cell> = HashSet::from_iter(calls.iter().map(|i| *i));
    for row in board {
        let cells: HashSet<Cell> = HashSet::from_iter(row.iter().map(|i| *i));
        if calls.intersection(&cells).count() == cells.len() {
            return Some(row.clone())
        }
    }

    for col in cols(&board) {
        let cells: HashSet<Cell> = HashSet::from_iter(col.iter().map(|i| *i));
        if calls.intersection(&cells).count() == cells.len() {
            return Some(col.clone())
        }
    }
    None
}

/// Swaps row and columns for the given board
fn cols(board: &Board) -> Board {
    let width = board[0].len();
    let mut cols = Vec::new();
    for row in board {
        for (i, cell) in row.iter().enumerate() {
            if i == cols.len() {
                cols.push(Vec::new());
            }
            cols[i].push(*cell);
        }
    }
    cols
}

fn score(calls: &[Cell], board: &Board) -> usize {
    let winning_numbers = win(calls, board).unwrap();
    let last_call = calls[calls.len() - 1] as usize;
    let calls: HashSet<Cell> = HashSet::from_iter(calls.iter().map(|i| *i));
    let unused_cells = board.iter().flatten().filter(|i| !calls.contains(i));
    unused_cells.map(|i| *i as usize).sum::<usize>() * last_call
}

fn fmt_cells(cells: &[Cell]) -> String {
    let mut output = String::new();
    for i in cells {
        output.push_str(&format!("{:>2} ", &i));
    }
    output.pop();
    output
}

fn fmt_board(board: &Board) -> String {
    let mut output = String::new();
    for row in board {
        output.push_str(&fmt_cells(&row));
        output.push('\n');
    }
    output.pop();
    output
}

fn main() {
    let path = env::args().nth(1).expect("First argument must be an input file");
    let mut game = Game::from_file(&path).expect("Unable to load game from file");
    while game.boards.len() > 1 {
        let mut index = 0;
        {
            let win = game.play().unwrap();
            index = win.index;
        }
        game.boards.remove(index);
    }
    let last_win = game.play().unwrap();
    println!("last score: {}", last_win.score);
}

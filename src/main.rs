use clap::Parser;
use std::{collections::VecDeque, io};

mod board;
use board::*;

/// Sudoku solver.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Sudoku board digits in row major order. Format can be anything as long as there is exactly
    /// the right amount of digits and blanks to fill the whole sudoku board.
    /// Empty slot can be marked with either '_' or '0'.
    /// Cannot be used in combination with --stdin.
    /// EXAMPLES:
    /// simple
    /// -b "_____321_|___654___|_387_____|_6___935_|_7_468_9_|_295___8_|_____596_|___941___|_572_____"
    /// hard
    /// -b "___7__2_1|8_39_____|5________|_4__6____|_7____1__|___5___8_|____42__6|9________|_______3_"
    #[arg(short = 'b', long)]
    board: String,

    /// If this argument is given expects the board input come from STDIN as one line. See
    /// the documentation for --board for format notes.
    #[arg(short = 't', long)]
    stdin: bool,

    /// Sets the solver to return a non-zero error code for everything else
    /// except a sudoku with single possible solution.
    #[arg(short = 's', long)]
    single: bool,

    /// Prints sudoku board in human readable format. Otherwise prints as one line.
    #[arg(short = 'f', long = "format-human")]
    human: bool,
}

fn main() {
    let args = Args::parse();
    let mut input_b = String::new();

    if args.stdin {
        // TODO: Error mgmt; use something like io::Result<()> as main result type
        let stdin = io::stdin();
        // stdin.read_line(&mut buffer)?;
        stdin.read_line(&mut input_b).unwrap();
    } else {
        input_b = args.board;
    }

    let b: Board = input_b.parse().unwrap();
    println!("Parsed board:\n\n{}", b);

    if let Some(solved) = solve(b) {
        // TODO: Track multiple solutions
        println!("Solved:\n\n{}", solved);
    } else {
        // TODO: Write to STDERR and return error code
        println!("No solution found");
    }
}

fn solve(b: Board) -> Option<Board> {
    let mut stack = VecDeque::new();
    stack.push_front((0, b));

    while let Some((mut i, nb)) = stack.pop_front() {
        // Find the next cell we can change. Empty cell without any options means
        // this branch did not result in solved sudoku and we can end this branch.
        while nb.get(i / 9, i % 9).is_some_and(|d| *d != EMPTY) {
            i += 1;

            if i == 81 {
                // Found a solution
                // TODO: Collect all solutions in case there are more
                return Some(nb);
            }
        }

        // Found an empty cell. Branch out from here
        for val in nb.possible_inputs(i / 9, i % 9) {
            let mut clone = nb.clone();
            clone.set(i / 9, i % 9, val);
            stack.push_back((i, clone));
        }
    }

    None
}

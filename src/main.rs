use std::io::{self, Write};
use std::{fmt, collections, error};
use dialoguer::{Input, theme::ColorfulTheme, Select};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    Human,
    COMP,
    None(u8)
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::Human => write!(f, "o"),
            Player::COMP => write!(f, "x"),
            Player::None(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Debug)]
struct BoardPlacementError {
    message: String
}

impl fmt::Display for BoardPlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BoardPlacementError: {}", self.message)
    }
}

impl error::Error for BoardPlacementError {
    fn description(&self) -> &str {
        "Description for BoardPlacementError"
    }
}

#[derive(Clone)]
struct Board {
    positions: collections::HashMap<u8, Player>,
    available_positions: collections::HashMap<u8, Player>
}

impl Board {
    fn new() -> Self {
        let mut positions = collections::HashMap::new();
        for i in 1..=9 {
            positions.insert(i, Player::None(i));
        }
        let available_positions = positions.clone();
        Board {
            positions,
            available_positions
        }
    }
    fn draw(&self) -> Result<(), io::Error> {
        let stdout = io::stdout();
        let mut handle = io::BufWriter::new(stdout.lock());
        writeln!(handle)?;
        writeln!(handle, "{} | {} | {}", self.positions.get(&1).unwrap(), 
            self.positions.get(&2).unwrap(), self.positions.get(&3).unwrap())?;
        writeln!(handle, "{} {} {} {} {}", "-", "+", "-", "+", "-")?;
        writeln!(handle, "{} | {} | {}", self.positions.get(&4).unwrap(), 
            self.positions.get(&5).unwrap(), self.positions.get(&6).unwrap())?;
        writeln!(handle, "{} {} {} {} {}", "-", "+", "-", "+", "-")?;
        writeln!(handle, "{} | {} | {}", self.positions.get(&7).unwrap(), 
            self.positions.get(&8).unwrap(), self.positions.get(&9).unwrap())?;
        writeln!(handle)?;
        Ok(())
    }

    fn place_move(&mut self, at: u8, player: Player) -> Result<(), BoardPlacementError> {
        match self.positions.get(&at) {
            Some(position) => {
                match position {
                    Player::None(_) => {
                        *self.positions.get_mut(&at).unwrap() = player;
                        self.available_positions.remove(&at);
                        Ok(())
                    },
                    _ => {
                        Err(BoardPlacementError {
                            message: String::from("Invalid move! Position is already taken.")
                        })
                    }
                }
            },
            _ => {
                Err(BoardPlacementError {
                    message: String::from("Invalid position! Choose between 1 and 9.")
                })
            }
        }
    }

    fn evaluate(&self) -> i8 {
        for (combo, step) in [([1, 4, 7], 1), ([1, 2, 3], 3), ([1, 5, 9], 0), ([3, 5, 7], 0)].iter() {
            for i in 0..3 {
                let step = (step*i) as u8;
                let val1 = self.positions.get(&(combo[0]+step)).unwrap();
                let val2 = self.positions.get(&(combo[1]+step)).unwrap();
                let val3 = self.positions.get(&(combo[2]+step)).unwrap();
                if val1 == val2 && val2 == val3 {
                    if val1 == &Player::COMP {
                        return 1;
                    } else {
                        return -1;
                    }
                }
            }
        }
        0
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut board = Board::new();
    println!("\nXO: Tic-tac-toe\n");
    let selections = [
        "Yes",
        "No"
    ];
    let selection = Select::with_theme(&ColorfulTheme::default()).with_prompt("Do you want to go first?")
        .default(0).items(&selections[..]).interact()?;
    let order;
    if selections[selection] == "Yes" {
        order = 1;
    } else {
        order = 0;
    }

    board.draw()?;

    for i in (1..=9).rev() {
        if i % 2 == order {
            loop {
                let choice = Input::<u8>::new().with_prompt("Your move").interact()?;
                match board.place_move(choice, Player::Human) {
                    Ok(_) => {
                        break;
                    },
                    Err(e) => println!("{}", e)
                }
            }
        } else {
            let optimal_move;
            // Hardcoded strategic placements for higher depths to save computation.
            if i == 9 {
                optimal_move = OptimalMove { position: 1, score: 0 };
            } else if i == 7 {
                if !board.available_positions.contains_key(&5) {
                    optimal_move = OptimalMove { position: 9, score: 0 };
                } else {
                    optimal_move = minimax(&board, i, Player::COMP);
                }
            } else {
                optimal_move = minimax(&board, i, Player::COMP);
            }
            println!("Computer's move: {}", optimal_move.position);
            board.place_move(optimal_move.position as u8, Player::COMP)?;
        }
        board.draw()?;
        let evaluation = board.evaluate();
        if evaluation == 1 {
            if i % 2 == order {
                println!("* You win! *\n");
                break;
            } else {
                println!("* Computer wins! *\n");
                break;
            }
        } else if evaluation == 0 && i == 1 {
            println!("* Tie *\n");
            break;
        }
    }
    Ok(())
}

struct OptimalMove {
    position: i8,
    score: i8
}

fn minimax(board: &Board, depth: i8, player: Player) -> OptimalMove {
    let opponent;
    let mut optimal;
    if player == Player::COMP {
        opponent = Player::Human;
        optimal = OptimalMove { position: -1, score: -50 };
    } else {
        opponent = Player::COMP;
        optimal = OptimalMove { position: -1, score: 50 };
    }

    let score = board.evaluate();
    if depth == 0 || score == 1 || score == -1 {
        return OptimalMove { position: -1, score };
    }

    for (k, _) in board.available_positions.iter() {
        let mut new_board = board.clone();
        let _ = new_board.place_move(*k, player);
        let mut current_optimal = minimax(&new_board, depth-1, opponent);
        current_optimal.position = *k as i8;

        if player == Player::COMP {
            if current_optimal.score > optimal.score {
                optimal = current_optimal;
            }
        } else {
            if current_optimal.score < optimal.score {
                optimal = current_optimal;
            }
        }
    }
    optimal
}

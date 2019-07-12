use std::{fmt, error};

#[derive(Debug)]
pub struct BoardPlacementError {
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

pub struct OptimalMove {
    pub position: i8,
    pub score: i8
}

pub mod components {
    use std::io::{self, Write};
    use std::{fmt, collections};

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Player {
        Human,
        Computer,
        None(u8)
    }

    impl fmt::Display for Player {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Player::Human => write!(f, "o"),
                Player::Computer => write!(f, "x"),
                Player::None(x) => write!(f, "{}", x),
            }
        }
    }

    #[derive(Clone)]
    pub struct Board {
        pub positions: collections::HashMap<u8, Player>,
        pub available_positions: collections::HashMap<u8, Player>
    }

    impl Board {
        pub fn new() -> Self {
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

        pub fn draw(&self) -> Result<(), io::Error> {
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

        pub fn place_move(&mut self, at: u8, player: Player) -> Result<(), super::BoardPlacementError> {
            match self.positions.get(&at) {
                Some(position) => {
                    match position {
                        Player::None(_) => {
                            *self.positions.get_mut(&at).unwrap() = player;
                            self.available_positions.remove(&at);
                            Ok(())
                        },
                        _ => {
                            Err(super::BoardPlacementError {
                                message: String::from("Invalid move! Position is already taken.")
                            })
                        }
                    }
                },
                _ => {
                    Err(super::BoardPlacementError {
                        message: String::from("Invalid position! Choose between 1 and 9.")
                    })
                }
            }
        }

        pub fn evaluate(&self) -> Player {
            for (combo, step) in [([1, 4, 7], 1), ([1, 2, 3], 3), ([1, 5, 9], 0), ([3, 5, 7], 0)].iter() {
                for i in 0..3 {
                    let step = (step*i) as u8;
                    let val1 = self.positions.get(&(combo[0]+step)).unwrap();
                    let val2 = self.positions.get(&(combo[1]+step)).unwrap();
                    let val3 = self.positions.get(&(combo[2]+step)).unwrap();
                    if val1 == val2 && val2 == val3 {
                        return *val1
                    }
                }
            }
            Player::None(0)
        }
    }
}

pub mod algorithms {
    use super::components;

    pub fn minimax(board: &components::Board, depth: i8, player: components::Player) -> super::OptimalMove {
        let opponent;
        let mut optimal;
        if player == components::Player::Computer {
            opponent = components::Player::Human;
            optimal = super::OptimalMove { position: -1, score: -50 };
        } else {
            opponent = components::Player::Computer;
            optimal = super::OptimalMove { position: -1, score: 50 };
        }

        let evaluation = board.evaluate();
        let score;
        match evaluation {
            components::Player::Computer => score = 1,
            components::Player::Human => score = -1,
            _ => score = 0
        }
        if depth == 0 || score == 1 || score == -1 {
            return super::OptimalMove { position: -1, score };
        }

        for (k, _) in board.available_positions.iter() {
            let mut new_board = board.clone();
            let _ = new_board.place_move(*k, player);
            let mut current_optimal = minimax(&new_board, depth-1, opponent);
            current_optimal.position = *k as i8;

            if player == components::Player::Computer {
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

    pub fn minimax_alpha_beta(board: &components::Board, depth: i8, player: components::Player, alpha: i8, beta: i8) -> super::OptimalMove {
        let opponent;
        let mut optimal;
        if player == components::Player::Computer {
            opponent = components::Player::Human;
            optimal = super::OptimalMove { position: -1, score: -50 };
        } else {
            opponent = components::Player::Computer;
            optimal = super::OptimalMove { position: -1, score: 50 };
        }

        let evaluation = board.evaluate();
        let score;
        match evaluation {
            components::Player::Computer => score = 1,
            components::Player::Human => score = -1,
            _ => score = 0
        }
        if depth == 0 || score == 1 || score == -1 {
            return super::OptimalMove { position: -1, score };
        }

        for (k, _) in board.available_positions.iter() {
            let mut new_board = board.clone();
            let _ = new_board.place_move(*k, player);
            let mut new_alpha = alpha;
            let mut new_beta = beta;
            let mut current_optimal = minimax_alpha_beta(&new_board, depth-1, opponent, new_alpha, new_beta);
            current_optimal.position = *k as i8;

            if player == components::Player::Computer {
                if current_optimal.score > optimal.score {
                    optimal = current_optimal;
                }
                if new_alpha > optimal.score {
                    new_alpha = optimal.score
                }
                if new_beta <= new_alpha {
                    break;
                }
            } else {
                if current_optimal.score < optimal.score {
                    optimal = current_optimal;
                }
                if new_beta < optimal.score {
                    new_beta = optimal.score;
                }
                if new_beta <= new_alpha {
                    break;
                }
            }
        }
        optimal
    }
}

pub mod utils {
    use std::error;
    use dialoguer::{theme::ColorfulTheme, Select};

    pub fn prompt_should_user_go_first() -> Result<bool, Box<dyn error::Error>> {
        let selections = ["Yes", "No"];
        let selection = Select::with_theme(&ColorfulTheme::default()).with_prompt("Do you want to go first?")
            .default(0).items(&selections[..]).interact()?;
        
        Ok(selections[selection] == "Yes")
    }
}
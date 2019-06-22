use std::io::{self, Write};
use std::{fmt, collections};
use dialoguer::Input;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    Human,
    AI,
    None(u8)
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::Human => write!(f, "o"),
            Player::AI => write!(f, "x"),
            Player::None(x) => write!(f, "{}", x),
        }
    }
}

struct Board {
    positions: [[Player; 3]; 3],
    position_alias: collections::HashMap<u8, (usize, usize)>
}

impl Board {
    fn new() -> Self {
        let positions: [[Player; 3]; 3] = 
            [[Player::None(1), Player::None(4), Player::None(7)], 
            [Player::None(2), Player::None(5), Player::None(8)], 
            [Player::None(3), Player::None(6), Player::None(9)]];

        let mut position_alias = collections::HashMap::new();
        position_alias.insert(1, (0, 0));
        position_alias.insert(2, (1, 0));
        position_alias.insert(3, (2, 0));
        position_alias.insert(4, (0, 1));
        position_alias.insert(5, (1, 1));
        position_alias.insert(6, (2, 1));
        position_alias.insert(7, (0, 2));
        position_alias.insert(8, (1, 2));
        position_alias.insert(9, (2, 2));
        
        Board {
            positions,
            position_alias
        }
    }

    fn visualize(&self, stdout: &io::Stdout) -> Result<(), io::Error> {
        let mut handle = io::BufWriter::new(stdout.lock());

        writeln!(handle)?;
        writeln!(handle, "{} | {} | {}", self.positions[0][0], self.positions[1][0], self.positions[2][0])?;
        writeln!(handle, "{} {} {} {} {}", "-", "+", "-", "+", "-")?;
        writeln!(handle, "{} | {} | {}", self.positions[0][1], self.positions[1][1], self.positions[2][1])?;
        writeln!(handle, "{} {} {} {} {}", "-", "+", "-", "+", "-")?;
        writeln!(handle, "{} | {} | {}", self.positions[0][2], self.positions[1][2], self.positions[2][2])?;
        writeln!(handle)?;

        Ok(())
    }

    fn place_move_with_alias(&mut self, alias: u8, player: Player) -> Option<Player> {
        match self.position_alias.get(&alias) {
            Some(&(row, col)) => {
                self.place_move(row, col, player)
            },
            _ => None
        }
    }

    // Credits to "Hardwareguy" for the algorithm
    // Source: https://stackoverflow.com/a/1056352
    fn place_move(&mut self, x: usize, y: usize, player: Player) -> Option<Player> {
        match self.positions[x][y] {
            Player::None(_) => self.positions[x][y] = player,
            _ => panic!("Invalid move: the spot on the board is already taken.")
        }
        
        let n = 3;
        // Check columns
        for i in 0..n {
            if self.positions[x][i] != player {
                break;
            }
            if i == n-1 {
                return Some(player)
            }
        }

        // Check rows
        for i in 0..n {
            if self.positions[i][y] != player {
                break;
            }
            if i == n-1 {
                return Some(player)
            }
        }

        // Check diagonal
        if x == y {
            for i in 0..n {
                if self.positions[i][i] != player {
                    break;
                }
                if i == n-1 {
                    return Some(player)
                }
            }
        }

        // Check anti-diagonal
        if x + y == n-1 {
            for i in 0..n {
                if self.positions[i][(n-1)-i] != player {
                    break;
                }
                if i == n-1 {
                    return Some(player)
                }
            }
        }

        None
    }
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let mut board = Board::new();
    let mut outcome: Option<Player>;

    println!("\nWelcome to XO");
    board.visualize(&stdout)?;
    for i in 0..9 {
        if i % 2 == 0 {
            let choice = Input::<u8>::new().with_prompt("Your turn").interact()?;
            outcome = board.place_move_with_alias(choice, Player::Human);
        } else {
            let choice = Input::<u8>::new().with_prompt("AI's turn").interact()?;
            outcome = board.place_move_with_alias(choice, Player::AI);
        }

        match outcome {
            Some(player) => {
                match player {
                    Player::AI => {
                        board.visualize(&stdout)?;
                        println!("* AI wins! *\n", );
                        break;
                    },
                    Player::Human => {
                        board.visualize(&stdout)?;
                        println!("* You win! *\n", );
                        break;
                    },
                    _ => ()
                }
            }
            _ => {
                println!("\n- Turn {} -", i+1);
                board.visualize(&stdout)?;
                if i == 8 {
                    println!("Draw");
                }
            }
        }
    }

    Ok(())
}


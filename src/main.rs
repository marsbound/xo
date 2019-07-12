use std::error;

use xo::{self, components, algorithms, utils};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("\nXO: Tic-tac-toe\n");

    let mut board = components::Board::new();
    let mut order = 0;
    let should_user_go_first = utils::prompt_should_user_go_first()?;
    let depth_iterations = (1..=9).rev(); // 9 -> 1 to represent depth

    if should_user_go_first {
        order = 1;
    }

    board.draw()?;
    
    for depth in depth_iterations {
        let is_user_turn = depth % 2 == order;

        if is_user_turn {
            loop {
                let choice = dialoguer::Input::<u8>::new().with_prompt("Your move").interact()?;
                match board.place_move(choice, components::Player::Human) {
                    Ok(_) => {
                        break;
                    },
                    Err(e) => println!("{}", e)
                }
            }
        } else {
            let optimal_move;
            // Hardcoded strategic placements for higher depths to save computation.
            if depth == 9 {
                optimal_move = xo::OptimalMove { position: 1, score: 0 };
            } else if depth == 7 {
                if board.available_positions.contains_key(&5) {
                    optimal_move = algorithms::minimax_alpha_beta(&board, depth, components::Player::Computer, -50, 50);
                } else {
                    optimal_move = xo::OptimalMove { position: 9, score: 0 };
                }
            } else if depth == 8 {
                if board.available_positions.contains_key(&5) {
                    optimal_move = xo::OptimalMove { position: 5, score: 0 };
                } else {
                    optimal_move = xo::OptimalMove { position: 1, score: 0 };
                }
            } else {
                optimal_move = algorithms::minimax_alpha_beta(&board, depth, components::Player::Computer, -50, 50);
            }
            println!("Computer's move: {}", optimal_move.position);
            board.place_move(optimal_move.position as u8, components::Player::Computer)?;
        }

        board.draw()?;

        let evaluation = board.evaluate();

        match evaluation {
            components::Player::Computer => {
                println!("* Computer wins! *\n");
                break;
            },
            components::Player::Human => {
                println!("* You win! *\n");
                break;
            },
            components::Player::None(_) => {
                let is_last_turn = depth == 1;
                if is_last_turn {
                    println!("* Tie *\n");
                    break;
                }
            }
        }
    }
    Ok(())
}
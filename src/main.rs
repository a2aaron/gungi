#![allow(dead_code)]
// TEMPORARY: this is just to make rust shut up about dead code
// (It's dead because I haven't written the rest of it!!!)

// GUNGI RULES
// https://mmmmalo.tumblr.com/post/74510568781/rules-of-gungi

mod tests;
mod pieces;
mod board;

use pieces::*;
use board::*;

fn main() {
    let special = MoveSpecial::Forward(Color::Black);
    let bow_map = vec![(0, 1), (0, -1), (-2, 2), (2, 2)];
    let (mut start_i, mut start_j) = (0, 0);
    loop {
        println!("WHITE SIDE");
        for j in (0..9).rev() {
            // Want 0th row to be at the bottom
            for i in 0..9 {
                if start_i == i && start_j == j {
                    print!("o");
                } else if board::check_move_special(special, start_i, start_j, i, j) {
                    // board::check_move_map(&bow_map, start_i, start_j, i, j) {
                    print!("Y");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
        println!("BLACK SIDE");
        start_i = (start_i + 1) % 9;
        start_j = (start_j + 1) % 9;
    }
}

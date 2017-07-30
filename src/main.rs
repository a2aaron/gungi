#![allow(dead_code)]
// TEMPORARY: this is just to make rust shut up about dead code
// (It's dead because I haven't written the rest of it!!!)

// GUNGI RULES
// https://mmmmalo.tumblr.com/post/74510568781/rules-of-gungi

mod tests;
mod pieces;
mod board;

use std::iter::Map;
use pieces::*;

fn main() {
    println!("Hello World!");
    let mut board: [[Option<Piece>; 9]; 9] = [[None; 9]; 9];

    let player1 = &Player::new_blank() as *const Player;
    let player2 = &Player::new_blank() as *const Player;
    println!("{}", player1 == player2);
}

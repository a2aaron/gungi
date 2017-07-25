#![allow(dead_code)] // TEMPORARY: this is just to make rust shut up about dead code (It's dead because I haven't written the rest of it!!!)

// GUNGI RULES
//https://mmmmalo.tumblr.com/post/74510568781/rules-of-gungi

mod tests;
mod pieces;

use std::iter::Map;
use pieces::*;

fn main() {
    let player = Player::new_blank();
    let piece_bottom = Piece::new(PieceCombination::PawnGold, &player);
    let piece_middle = Piece::new(PieceCombination::BowArrow, &player);
    let piece_top = Piece::new(PieceCombination::ProdigyPhoenix, &player);

    // let mut tower = Tower::new(Some(piece_bottom), Some(piece_middle), Some(piece_top)).unwrap();
    println!("{}", piece_top);
}
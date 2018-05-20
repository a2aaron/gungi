#![allow(dead_code)]
// TEMPORARY: this is just to make rust shut up about dead code
// (It's dead because I haven't written the rest of it!!!)

// GUNGI RULES
// https://mmmmalo.tumblr.com/post/74510568781/rules-of-gungi

mod pieces;
mod tests;

use pieces::*;
use std::iter::Map;

fn main() {
    let player = Color::Black;
    let piece_bottom = Piece::new(PieceCombination::PawnBronze, player);
    let piece_middle = Piece::new(PieceCombination::BowArrow, player);
    let piece_top = Piece::new(PieceCombination::PawnSilver, player);

    let tower = Tower::Triple(piece_bottom, piece_middle, piece_top);
    println!("{}", piece_top);
    println!("{}", piece_middle);
    println!("{}", piece_bottom);
    println!("{}", tower);
}

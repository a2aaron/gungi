// GUNGI RULES
//https://mmmmalo.tumblr.com/post/74510568781/rules-of-gungi
fn main() {
    println!("Hello World!");
}

/// A piece has two sides, called "Front" and "Back." Pieces initially
/// start out as their Front side but will flip to Back if they are captured.
/// The only piece that does not have this is the Commander, which is similar
/// to the king in chess.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece<'a> {
    // This should be either front_side or back_side. May change when piece is captured
    pub current_side: PieceType,
    front_side: PieceType,
    back_side: PieceType,
    pub belongs_to: &'a Player,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player {
    // TODO
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
    // Front Pieces
    Commander, // Command has no back piece!
    Captain,
    Samurai,
    Spy,
    Catapult,
    Fortress,
    HiddenDragon,
    Prodigy,
    Bow,
    Pawn,
    // Back Pieces
    Pistol,
    Pike,
    Clandestinite,
    Lance,
    DragonKing,
    Phoenix,
    Arrow,
    Bronze,
    Silver,
    Gold,
}
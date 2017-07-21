//TODO: Move the tests into their own file

// GUNGI RULES
//https://mmmmalo.tumblr.com/post/74510568781/rules-of-gungi
fn main() {
    println!("Hello World!");

    let player1 = &Player::new_blank() as *const Player;
    let player2 = &Player::new_blank() as *const Player;
    println!("{}", player1 == player2);
}

/// A tower consists of zero to three pieces. Towers may contain pieces from 
/// both players. Only the top piece on a tower can move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Tower<'a> {
    bottom: Option<Piece<'a>>,
    mid: Option<Piece<'a>>,
    top: Option<Piece<'a>>,
}

/// Returns true if both pieces have the same PieceType and belong to the same player
fn same_type_and_player(piece_1: Piece, piece_2: Piece) -> bool {
    // Compare that the *pointers* are equal, NOT the contents of the pointers
    // This ensures that the pieces definitely belong to the same player and not just
    // different players that happen to look like each other.
    let same_player = piece_1.belongs_to as *const Player == piece_2.belongs_to as *const Player;
    let same_type = piece_1.current_type() == piece_2.current_type();
    return same_player && same_type
           
}

#[test]
fn test_same_type_and_player() {
    let player1 = Player::new_blank();
    let player2 = Player::new_blank();

    // Same piece types but one is on the back (true)
    let piece_1 = Piece {
                    current_side: SideType::Front,
                    front_side: PieceType::Pawn,
                    back_side: PieceType::Gold,
                    belongs_to: &player1
                    };

    let piece_2 = Piece {
                    current_side: SideType::Back,
                    front_side: PieceType::Silver,
                    back_side: PieceType::Pawn,
                    belongs_to: &player1
                    };
    assert!(same_type_and_player(piece_1, piece_2));

    // Same pieces but different current sides (false)
    let piece_3 = Piece {
                    current_side: SideType::Front,
                    front_side: PieceType::Pawn,
                    back_side: PieceType::Gold,
                    belongs_to: &player1
                    };

    let piece_4 = Piece {
                    current_side: SideType::Back,
                    front_side: PieceType::Pawn,
                    back_side: PieceType::Gold,
                    belongs_to: &player1
                    };
    assert!(!same_type_and_player(piece_3, piece_4));

    // Same piece types but different players (false)
    let piece_5 = Piece {
                    current_side: SideType::Front,
                    front_side: PieceType::Pawn,
                    back_side: PieceType::Gold,
                    belongs_to: &player1
                    };

    let piece_6 = Piece {
                    current_side: SideType::Front,
                    front_side: PieceType::Pawn,
                    back_side: PieceType::Gold,
                    belongs_to: &player2
                    };
    assert!(!same_type_and_player(piece_5, piece_6));
}

/// A piece has two sides, called "Front" and "Back." Pieces initially
/// start out as their Front side but will flip to Back if they are captured.
/// The only piece that does not have this is the Commander, which is similar
/// to the king in chess.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece<'a> {
    // This should be either front_side or back_side.
    // May change when piece is captured
    pub current_side: SideType,
    front_side: PieceType,
    back_side: PieceType,
    // We use a pointer here because the player owns the piece, not 
    // the other way around.
    pub belongs_to: &'a Player,
}

impl<'a> Piece<'a> {
    fn current_type(&self) -> PieceType {
        use SideType::*;
        match self.current_side {
            Front => self.front_side,
            Back => self.back_side
        }
    }
}

#[test]
fn test_current_type() {
    let front_piece = Piece {
                    current_side: SideType::Front,
                    front_side: PieceType::Pawn,
                    back_side: PieceType::Gold,
                    belongs_to: &Player::new_blank()
                    };
    assert_eq!(PieceType::Pawn, front_piece.current_type());

    let back_piece = Piece {
                    current_side: SideType::Back,
                    front_side: PieceType::Pawn,
                    back_side: PieceType::Gold,
                    belongs_to: &Player::new_blank()
                    };
    assert_eq!(PieceType::Gold, back_piece.current_type());
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player {
    // TODO
}

impl Player {
    // Stub for the Player struct
    fn new_blank() -> Player {
        return Player {

        }
    }
}

/// Return what side a PieceType is
/// For example, `get_sidet_type(Spy)` returns `Front`
fn get_side_type(piece_type: PieceType) -> SideType {
    use PieceType::*;
    use SideType::*;
    match piece_type {
        Commander => Front,
        Captain => Front,
        Samurai => Front,
        Spy => Front,
        Catapult => Front,
        Fortress => Front,
        HiddenDragon => Front,
        Prodigy => Front,
        Bow => Front,
        Pawn => Front,
        Pistol => Back,
        Pike => Back,
        Clandestinite => Back,
        Lance => Back,
        DragonKing => Back,
        Phoenix => Back,
        Arrow => Back,
        Bronze => Back,
        Silver => Back,
        Gold => Back,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SideType {
    Front, Back
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
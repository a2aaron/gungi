/// A tower consists of zero to three pieces. Towers may contain pieces from 
/// both players. Only the top piece on a tower can move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tower<'a> {
    pub bottom: Option<Piece<'a>>,
    pub mid: Option<Piece<'a>>,
    pub top: Option<Piece<'a>>,
}

/// A tower is valid as long as no two pieces from the same player
/// of the same type are in it
///    For example, (Your) Pawn, (Your) Gold, (Your) Gold is disallowed
///    but (Your) Pawn, (Your) Gold, (Enemy) Gold is fine
pub fn is_valid(tower: Tower) -> bool {
    match (tower.bottom, tower.mid, tower.top) {
        // Empty towers are obviously fine!
        (None, None, None) => true,
        // Cases where the tower clearly isn't a tower (ie: bottom pieces missing)
        (None, _, _) => false,
        (Some(_), None, Some(_)) => false,
        // Towers of just one piece can never have two pieces of the same type
        (Some(_), None, None) => true,
        (Some(bottom), Some(middle), None) => {
        // Towers of two mustn't have the pieces be the same type and from same player
            return !same_type_and_player(bottom, middle)
        }
        // Same idea for towers of three but it applies to all piece combinations
        (Some(bottom), Some(middle), Some(top)) => {
            return !(same_type_and_player(bottom, middle) ||
                   same_type_and_player(bottom, top)    || 
                   same_type_and_player(middle, top))
        }
    }
}

/// Returns true if both pieces have the same PieceType and belong to the same player.
pub fn same_type_and_player(piece_1: Piece, piece_2: Piece) -> bool {
    // Compare that the *pointers* are equal, NOT the contents of the pointers
    // This ensures that the pieces definitely belong to the same player and not just
    // different players that happen to look like each other.
    use std::ptr::eq;
    let same_player = eq(piece_1.belongs_to, piece_2.belongs_to);
    let same_type = piece_1.current_type() == piece_2.current_type();
    return same_player && same_type
           
}

/// A piece has two sides, called "Front" and "Back." Pieces initially
/// start out as their Front side but will flip to Back if they are captured.
/// The only piece that does not have this is the Commander, which is similar
/// to the king in chess. Note that the Commander piece has the Commander PieceType for the front and back
/// (this was done because having Option<PieceType> for just a single case would be dumb)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece<'a> {
    // This should be either front_side or back_side.
    // May change when piece is captured
    pub current_side: SideType,
    pub front_side: PieceType,
    pub back_side: PieceType,
    // We use a pointer here because the player owns the piece, not 
    // the other way around.
    pub belongs_to: &'a Player,
}

impl<'a> Piece<'a> {
    pub fn new(piece_combination: PieceCombination, initial_side: SideType, player: &'a Player) -> Piece<'a> {
        use pieces::PieceType::*;
        use pieces::PieceCombination::*;
        let (front_side, back_side) = match piece_combination {
            PieceCombination::Commander => (PieceType::Commander, PieceType::Commander),
            CaptainPistol => (Captain, Pistol),
            SamuraiPike => (Samurai, Pike),
            SpyCladestinite => (Spy, Clandestinite),
            CatapultLance => (Catapult, Lance),
            FortressLance => (Fortress, Lance),
            HiddenDragonKing => (HiddenDragon, DragonKing),
            ProdigyPhoenix => (Prodigy, Phoenix),
            BowArrow => (Bow, Arrow),
            PawnBronze => (Pawn, Bronze),
            PawnSilver => (Pawn, Silver),
            PawnGold => (Pawn, Bronze),
        };

        return Piece {
            current_side: initial_side,
            front_side: front_side,
            back_side: back_side,
            belongs_to: player
        }
    }

    pub fn current_type(&self) -> PieceType {
        use SideType::*;
        match self.current_side {
            Front => self.front_side,
            Back => self.back_side
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player {
    // TODO
}

impl Player {
    // Stub for the Player struct
    pub fn new_blank() -> Player {
        return Player {

        }
    }
}

/// Return what side a PieceType is.
/// For example, `get_sidet_type(Spy)` returns `Front`
pub fn get_side_type(piece_type: PieceType) -> SideType {
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

/// A Piece has both a Front and a Back side, which determines what
/// PieceType it wil lact as.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SideType {
    Front, Back
}

/// All of the avaliable Gungi pieces 
/// This enum is useful as a shorthand for creating new Piece structs
/// these variants listed are the only combinations of Front and Back
/// pieces found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceCombination {
    Commander, // This is probably not a good name...
    CaptainPistol,
    SamuraiPike,
    SpyCladestinite,
    CatapultLance,
    FortressLance,
    HiddenDragonKing,
    ProdigyPhoenix,
    BowArrow,
    PawnBronze,
    PawnSilver,
    PawnGold,
}

/// All the individal piece types.
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
/// A tower consists of zero to three pieces. Towers may contain pieces from 
/// both players. Only the top piece on a tower can move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tower<'a> {
    Empty,
    Single(Piece<'a>),
    Double(Piece<'a>, Piece<'a>),
    Triple(Piece<'a>, Piece<'a>, Piece<'a>)
}

/// A convient enum for refering to the height of a tower.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TowerHeight{ Top, Middle, Bottom, Empty }

impl<'a> Tower<'a> {
    /// Returns the top most piece and a tower that has its top piece removed
    /// Returns Err if the tower is empty
    /// This function does not modify the original tower
    pub fn lift_piece(&self) -> Result<(Tower, Piece<'a>), &'static str> {
        use pieces::Tower::*;
        match *self {
            Empty => Err("Cannot lift a piece off an empty tower!"),
            Single(bottom) => Ok((Empty, bottom)),
            Double(bottom, middle) => Ok((Single(bottom), middle)),
            Triple(bottom, middle, top) => Ok((Double(bottom, middle), top))
        }
    }

    /// Returns a tower that has a piece added to the topmost position on this tower
    /// Returns Err if the tower is full does not modify Tower state when this happens
    /// This function does not modify the original tower.
    pub fn drop_piece(&self, piece: Piece<'a>) -> Result<Tower<'a>, &'static str> {
        use pieces::Tower::*;
        match *self {
            Empty => Ok(Single(piece)),
            Single(bottom) => Ok(Double(bottom, piece)),
            Double(bottom, middle) => Ok(Triple(bottom, middle, piece)),
            Triple(_, _, _) => Err("Tower is full."),
        }
    }

    pub fn height(&self) -> TowerHeight {
        use pieces::Tower::*;
        match *self {
            Empty => TowerHeight::Empty,
            Single(_) => TowerHeight::Bottom,
            Double(_, _) => TowerHeight::Middle,
            Triple(_, _, _) => TowerHeight::Top
        }
    }

    /// A tower is valid as long as no two pieces from the same player
    /// of the same type are in it
    /// For example, (Your) Pawn, (Your) Pawn is disallowed
    /// but (Your) Pawn, (Enemy) Pawn is fine
    /// ```
    /// let player1 = Player::new_blank();
    /// let player2 = Player::new_blank();
    /// let pawn_gold = Piece::new(PieceCombination::PawnGold, &player1);
    /// let pawn_silver_1 = Piece::new(PieceCombination::PawnSilver, &player1);
    /// let pawn_silver_2 = Piece::new(PieceCombination::PawnSilver, &player2);
    /// let bad_tower = Tower::Double(pawn_gold, pawn_silver_1);
    /// let good_tower = Tower::Double(pawn_gold, pawn_silver_2);
    /// assert!(!bad_tower.is_valid());
    /// assert!(good_tower.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        use pieces::Tower::*;
        match *self {
            // Empty towers are obviously fine!
            Empty => true,
            // Towers of just one piece can never have two pieces of the same type
            Single(_) => true,
            // Towers of two mustn't have the pieces be the same type and from same player
            Double(bottom, middle) => !same_type_and_player(bottom, middle),
            // Same idea for towers of three but it applies to all piece combinations
            Triple(bottom, middle, top) => !(same_type_and_player(bottom, middle) ||
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

/// Returns the initial number of pieces a player has at the begining of the game
/// The number of pieces at the start of the game is as follows:
/// Commander        x1
/// CaptainPistol    x2
/// SamuraiPike      x2
/// SpyCladestinite  x3
/// CatapultLance    x1
/// FortressLance    x1
/// HiddenDragonKing x1
/// ProdigyPhoenix   x1
/// BowArrow         x2
/// PawnBronze       x7
/// PawnSilver       x1
/// PawnGold         x1
pub fn initial_hand<'a>() -> Vec<PieceCombination> {
    use PieceCombination::*;
    // There are probably better ways of doing this but I am lazy and do not care
    let vec = [Commander,
               CaptainPistol, CaptainPistol,
               SamuraiPike, SamuraiPike,
               SpyCladestinite, SpyCladestinite, SpyCladestinite,
               CatapultLance, FortressLance, HiddenDragonKing, ProdigyPhoenix,
               BowArrow, BowArrow,
               PawnBronze, PawnBronze, PawnBronze, PawnBronze, PawnBronze, PawnBronze, PawnBronze, PawnSilver, PawnGold].to_vec();
    return vec;
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
    pub belongs_to: &'a Player<'a>,
}

impl<'a> Piece<'a> {
    pub fn new(piece_combination: PieceCombination, player: &'a Player) -> Piece<'a> {
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
            current_side: SideType::Front,
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

#[derive(Debug, PartialEq, Eq)]
pub struct Player<'a> {
    pub hand: Vec<Piece<'a>>,
}

impl<'a> Player<'a> {
    // Stub for the Player struct
    pub fn new_blank() -> Player<'a> {
        return Player {
            hand: vec!()
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
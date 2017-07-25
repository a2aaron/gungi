use std::fmt;


/// A tower consists of zero to three pieces. Towers may contain pieces from 
/// both players. Only the top piece on a tower can move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tower<'a> {
    pub bottom: Option<Piece<'a>>,
    pub mid: Option<Piece<'a>>,
    pub top: Option<Piece<'a>>,
}

/// A convient enum for refering to the height of a tower.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TowerHeight{ Top, Middle, Bottom, Empty }

impl<'a> Tower<'a> {
    pub fn new(bottom: Option<Piece<'a>>, mid: Option<Piece<'a>>, top: Option<Piece<'a>>) -> Result<Tower<'a>, &'static str> {
        let tower = Tower {bottom: bottom, mid: mid, top:top };
        match tower.is_valid() {
            true => Ok(tower),
            false => Err("Invalid tower")
        }
    }

    pub fn get(&self, position: TowerHeight) -> Option<Piece<'a>> {
        use pieces::TowerHeight::*;
        match position {
            Top => self.top,
            Middle => self.mid,
            Bottom => self.bottom,
            Empty => None,
        }
    }
    // Set the given position to the given piece
    // Panics on invalid tower setting or when trying to set a piece to the Empty height
    fn set(&mut self, piece: Option<Piece<'a>>, position: TowerHeight) {
        use pieces::TowerHeight::*;
        match position {
            Top => self.top = piece,
            Middle => self.mid = piece,
            Bottom => self.bottom = piece,
            Empty => panic!("Cannot set a piece at TowerHeight::Empty")
        }

        if !self.is_valid() {
            panic!("Attempt to set Tower to an invalid state: {:?}", self);
        }
    }

    /// Removes and returns the top most piece from the tower
    /// Panics if the tower is empty
    pub fn pop(&mut self) -> Piece<'a> {
        let height = self.height();

        if height == TowerHeight::Empty {
            panic!("Cannot pop an empty tower!")
        }
        // This unwrap is safe because the tower is non-empty
        let top_piece = self.get(height).unwrap();
        self.set(None, height);
        return top_piece
    }

    /// Adds a piece to the top most position on the tower
    /// Panics if the tower is full
    pub fn drop_piece(&mut self, piece: Piece<'a>) {
        let height = self.height();
        use pieces::TowerHeight::*;
        match height {
            Top => panic!("Tower is full."),
            Middle => self.set(Some(piece), Top),
            Bottom => self.set(Some(piece), Middle),
            Empty =>self.set(Some(piece), Bottom),
        }
    }

    pub fn height(&self) -> TowerHeight {
        if let Some(_) = self.top {
            return TowerHeight::Top;
        } else if let Some(_) = self.mid {
            return TowerHeight::Middle;
        } else if let Some(_) = self.bottom {
            return TowerHeight::Bottom;
        } else {
            return TowerHeight::Empty;
        }
    }

    /// A tower is valid as long as no two pieces from the same player
    /// of the same type are in it
    ///    For example, (Your) Pawn, (Your) Gold, (Your) Gold is disallowed
    ///    but (Your) Pawn, (Your) Gold, (Enemy) Gold is fine
    pub fn is_valid(&self) -> bool {
        match (self.bottom, self.mid, self.top) {
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

impl<'a> fmt::Display for Piece<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.current_side {
            Front => write!(f, "{}", self.front_side),
            Back => write!(f, "{}", self.back_side)
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

impl<'a> fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use pieces::PieceType::*;
        match *self {
            Commander => write!(f, "Commander"),
            Captain => write!(f, "Captain"),
            Samurai => write!(f, "Samurai"),
            Spy => write!(f, "Spy"),
            Catapult => write!(f, "Catapult"),
            Fortress => write!(f, "Fortress"),
            HiddenDragon => write!(f, "HiddenDragon"),
            Prodigy => write!(f, "Prodigy"),
            Bow => write!(f, "Bow"),
            Pawn => write!(f, "Pawn"),
            Pistol => write!(f, "Pistol"),
            Pike => write!(f, "Pike"),
            Clandestinite => write!(f, "Clandestinite"),
            Lance => write!(f, "Lance"),
            DragonKing => write!(f, "DragonKing"),
            Phoenix => write!(f, "Phoenix"),
            Arrow => write!(f, "Arrow"),
            Bronze => write!(f, "Bronze"),
            Silver => write!(f, "Silver"),
            Gold => write!(f, "Gold"),
        }
    }
}
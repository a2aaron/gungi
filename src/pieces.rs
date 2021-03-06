use std::fmt;


/// A tower consists of zero to three pieces. Towers may contain pieces from
/// both players. Only the top piece on a tower can move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tower {
    Empty,
    Single(Piece),
    Double(Piece, Piece),
    Triple(Piece, Piece, Piece),
}

impl Tower {
    /// Returns the top most piece and a tower that has its top piece removed
    /// Returns Err if the tower is empty
    pub fn lift_piece(&self) -> Result<(Tower, Piece), &'static str> {
        use pieces::Tower::*;
        match *self {
            Empty => Err("Cannot lift a piece off an empty tower!"),
            Single(bottom) => Ok((Empty, bottom)),
            Double(bottom, middle) => Ok((Single(bottom), middle)),
            Triple(bottom, middle, top) => Ok((Double(bottom, middle), top)),
        }
    }

    /// Returns a tower that has a piece added to the topmost position on this tower
    /// Returns Err if the tower is full does not modify Tower state when this happens
    pub fn drop_piece(&self, piece: Piece) -> Result<Tower, &'static str> {
        use pieces::Tower::*;
        match *self {
            Empty => Ok(Single(piece)),
            Single(bottom) => Ok(Double(bottom, piece)),
            Double(bottom, middle) => Ok(Triple(bottom, middle, piece)),
            Triple(_, _, _) => Err("Tower is full."),
        }
    }

    /// A tower is valid as long as no two pieces from the same player
    /// of the same type are in it
    /// For example, (Your) Pawn, (Your) Pawn is disallowed
    /// but (Your) Pawn, (Enemy) Pawn is fine
    /// ```
    /// # let player1 = Player::new_blank();
    /// # let player2 = Player::new_blank();
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
            Double(bottom, middle) => bottom != middle,
            // Same idea for towers of three but it applies to all piece combinations
            Triple(bottom, middle, top) => !(bottom == middle || bottom == top || middle == top),
        }
    }
}

impl fmt::Display for Tower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use pieces::Tower::*;
        match *self {
            Empty => write!(f, "[]"),
            Single(piece) => write!(f, "[{}]", piece),
            Double(bottom, mid) => write!(f, "[{}, {}]", bottom, mid),
            Triple(bottom, mid, top) => write!(f, "[{}, {}, {}]", bottom, mid, top),
        }
    }
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
pub fn initial_hand() -> Vec<PieceCombination> {
    use PieceCombination::*;
    // There are probably better ways of doing this but I am lazy and do not care
    vec![
        Commander,
        CaptainPistol,
        CaptainPistol,
        SamuraiPike,
        SamuraiPike,
        SpyCladestinite,
        SpyCladestinite,
        SpyCladestinite,
        CatapultLance,
        FortressLance,
        HiddenDragonKing,
        ProdigyPhoenix,
        BowArrow,
        BowArrow,
        PawnBronze,
        PawnBronze,
        PawnBronze,
        PawnBronze,
        PawnBronze,
        PawnBronze,
        PawnBronze,
        PawnSilver,
        PawnGold,
    ]
}

/// A piece has two sides, called "Front" and "Back." Pieces initially
/// start out as their Front side but will flip to Back if they are captured.
/// The only piece that does not have this is the Commander, which is similar
/// to the king in chess. Note that the Commander piece has the Commander PieceType
// for the front and back. This was done because having Option<PieceType> for just
// a single case would be dumb.
#[derive(Clone, Copy, Debug)]
pub struct Piece {
    // This should be either front_side or back_side.
    // May change when piece is captured
    pub current_side: SideType,
    pub front_side: PieceType,
    pub back_side: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(piece_combination: PieceCombination, color: Color) -> Piece {
        use pieces::PieceCombination::*;
        use pieces::PieceType::*;
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
            PawnGold => (Pawn, Gold),
        };

        return Piece {
            current_side: SideType::Front,
            front_side: front_side,
            back_side: back_side,
            color: color,
        };
    }

    pub fn current_type(&self) -> PieceType {
        use SideType::*;
        match self.current_side {
            Front => self.front_side,
            Back => self.back_side,
        }
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Piece) -> bool {
        let same_player = self.color == other.color;
        let same_type = self.current_type() == other.current_type();
        same_player && same_type
    }
}

impl Eq for Piece {}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.current_side {
            SideType::Front => write!(f, "{} {} ({})", self.color, self.front_side, self.back_side),
            SideType::Back => write!(f, "{} {} ({})", self.color, self.back_side, self.front_side),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    pub hand: Vec<Piece>,
}

/// The color of a piece determines which player owns the piece
/// Note that Black always moves first
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Color::Black => write!(f, "Black"),
            Color::White => write!(f, "White"),
        }
    }
}

impl Player {
    // Stub for the Player struct
    pub fn new_blank() -> Player {
        Player { hand: vec![] }
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
    Front,
    Back,
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
            HiddenDragon => write!(f, "Hidden Dragon"),
            Prodigy => write!(f, "Prodigy"),
            Bow => write!(f, "Bow"),
            Pawn => write!(f, "Pawn"),
            Pistol => write!(f, "Pistol"),
            Pike => write!(f, "Pike"),
            Clandestinite => write!(f, "Clandestinite"),
            Lance => write!(f, "Lance"),
            DragonKing => write!(f, "Dragon King"),
            Phoenix => write!(f, "Phoenix"),
            Arrow => write!(f, "Arrow"),
            Bronze => write!(f, "Bronze"),
            Silver => write!(f, "Silver"),
            Gold => write!(f, "Gold"),
        }
    }
}

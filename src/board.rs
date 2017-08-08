use pieces::*;

/// This file describes how a the board should work in Gungi.
/// A Gungi board is a 9x9 board, similar to a Shogi board.
/// For the purposes of this file, we will define the board positions in terms
/// of (x, y) coordinates, with (0, 0) located at the *bottom left* corner.
/// Black is at (0, _) while white is at (8, _).
/// Visualized below:
/// WHITE SIDE
/// 8#########
/// 7#########
/// 6#########
/// 5#########
/// 4#########
/// 3#########
/// 2#########
/// 1#########
/// 0#########
///  012345678
/// BLACK SIDE
/// Note that this essentially means that the board resides in quadrant 1
/// of a Cartesian plane.
/// Note that, for our purposes, we are using a [[Tower; 9] 9] to represent
/// a board.

/// Returns the board but with the top piece moved to the goal location.
/// Returns Err if the initial tower selected is empty (and thus cannot pop)
fn move_piece(
    mut board: [[Tower; 9]; 9],
    initial_i: usize,
    initial_j: usize,
    end_i: usize,
    end_j: usize,
) -> Result<[[Tower; 9]; 9], &'static str> {
    let start_tower = board[initial_i][initial_j];
    let end_tower = board[end_i][end_j];
    // First check that we can actually remove a piece from the starting tower
    // and place a piece onto the ending tower.
    if let Ok((lifted_tower, piece)) = start_tower.lift_piece() {
        if let Ok(dropped_tower) = end_tower.drop_piece(piece) {
            // Actually do the moving
            board[initial_i][initial_j] = lifted_tower;
            board[end_i][end_j] = dropped_tower;
            return Ok(board);
        } else {
            return Err("Couldn't drop piece successfully");
        }
    } else {
        return Err("Couldn't pop piece successfully");
    }
}

/// A Move Map indicates how a piece may move.
/// A Piece typically can move to another spot within a 5x5 area. For example,
/// a Tier 1 Arrow may move forwards, backwards, back left and back right.
/// Some pieces may also move like a Rook, Bishop, or Rook (only forward).
/// Note that Move Descriptions may differ between Tiers significantly.
/// A standard Move Description contains a Vector of tuples indicating the positions
/// relative to the piece that it may move.
/// For example, since the Tier 2 Bow may move forward, backwards, or
/// up 2 spaces and left or right 2 spaces, it's move list is
/// [(0, 1), (0, -1), (-2, 2), (2, 2)]
/// Visualized below. P indicates the T2 Bow, # indicates a square the piece
/// move to, and - indicates a non-reachable square.
/// IMPORTANT: These move maps are all described for pieces belonging to Black!
///
pub type MoveMap = Vec<(i32, i32)>;

/// Some pieces may also move in a special way, such as like a Rook or Bishop
/// For example, Hidden Dragon (Tier 1) and Dragon King (Tier 1) move like Rooks,
/// Prodigy (Tier 1) and Phoenix (Tier 1) move like bishops
/// and the Lance (Tier 1) can move forward like a Rook. The Fortress's Mobile
/// Range Expansion Effect also applies to all pieces in front of it (like a Rook)
/// Note that a piece with the `Forward` variant always moves towards the enemy player.
/// For example, if Black has the 0th row and White has the 8th row then Forward
/// would let a Black piece move towards the 8th row and a White piece towards the 0th
/// row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveSpecial {
    Normal,
    Rook,
    Bishop,
    Forward(Color),
}

/// Returns true if the end coordinates can be reached by the start coordinates.
/// Note: Always returns false if MoveSpecial::Normal.
pub fn check_move_special(
    move_special: MoveSpecial,
    start_i: usize,
    start_j: usize,
    end_i: usize,
    end_j: usize,
) -> bool {
    use board::MoveSpecial::*;
    match move_special {
        // No special move, so false by default
        Normal => false,
        // Same file or rank
        Rook => start_i == end_i || start_j == end_j,
        // Move equally vertically and horizontally
        Bishop => abs_diff(start_i, end_i) == abs_diff(start_j, end_j),
        // For consistency, we will say that Black's side is at row 0
        // while White's side is at row 8
        Forward(color) => {
            match color {
                Color::Black => start_i == end_i && start_j < end_j,
                Color::White => start_i == end_i && start_j > end_j,
            }
        }
    }
}

fn abs_diff(x: usize, y: usize) -> usize {
    if x > y { x - y } else { y - x }
}

/// Returns true if the end coordinates can be reached by atleast one move map
/// entry from the starting coordinates.
pub fn check_move_map(
    move_map: &MoveMap,
    start_i: usize,
    start_j: usize,
    end_i: usize,
    end_j: usize,
) -> bool {
    for deltas in move_map {
        let (delta_i, delta_j) = *deltas;
        let result_i = add(start_i, delta_i);
        let result_j = add(start_j, delta_j);
        // I would combine these two if statements but I can't figure out how
        // so nested ifs will have to do for now
        if let (Ok(i), Ok(j)) = (result_i, result_j) {
            if i == end_i && j == end_j {
                return true;
            }
        }
    }

    false
}

// Used to flip MoveMaps for white side.
fn vertical_flip(move_map: &MoveMap) -> MoveMap {
    return move_map.iter().map(|x| (x.0, x.1 * -1)).collect();
}

/// A Tier refers to the location that a Piece is in a Tower. Tier One pieces are
/// at the bottom, Tier Two are in the middle, and Tier Three pieces are at the
/// top. Pieces with the same PieceType but different Tiers move in different ways.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
    One,
    Two,
    Three,
}

/// Returns the MoveSpecial that a PieceType has. Note that `color` is only relevant
/// if the PieceType is a Lance (Tier 1) as the direction the Lance moves depends
/// on its color. All Tier 2 and Tier 3 pieces move normally.
/// only Hidden Dragon and Dragon King(both Tier 1) move as Rook,
/// and only Prodigy and Pheonix (both Tier 1) move as Bishop.
pub fn get_move_special(piece_type: PieceType, color: Color, tier: Tier) -> MoveSpecial {
    use MoveSpecial::*;
    use PieceType::*;
    match tier {
        Tier::One => {
            match piece_type {
                HiddenDragon | DragonKing => Rook,
                Prodigy | Phoenix => Bishop,
                Lance => Forward(color),
                _ => Normal,
            }
        }
        _ => Normal,
    }
}


// This function attempts to add the two arguments. If the i32 is negatative
// such that the resulting value would be negative, this funtion returns Err.
fn add(x: usize, y: i32) -> Result<usize, &'static str> {
    let num: i32 = x as i32 + y;
    if num < 0 {
        Err("Result was negative")
    } else {
        Ok(num as usize)
    }
}


#[cfg(test)]
mod tests {
    use pieces::*;
    use board::*;

    #[test]
    fn test_get_move_special() {
        use board::MoveSpecial::*;
        use pieces::PieceType::*;
        assert_eq!(
            get_move_special(HiddenDragon, Color::Black, Tier::One),
            Rook
        );
        assert_eq!(get_move_special(DragonKing, Color::Black, Tier::One), Rook);
        assert_eq!(get_move_special(Prodigy, Color::Black, Tier::One), Bishop);
        assert_eq!(get_move_special(Phoenix, Color::Black, Tier::One), Bishop);

        assert_eq!(
            get_move_special(HiddenDragon, Color::Black, Tier::Two),
            Normal
        );

        assert_eq!(
            get_move_special(Lance, Color::Black, Tier::One),
            Forward(Color::Black)
        );
        assert_eq!(
            get_move_special(Lance, Color::White, Tier::One),
            Forward(Color::White)
        );
    }

    #[test]
    fn test_move_special() {
        use board::MoveSpecial::*;
        // Normal variant always false.
        assert!(!check_move_special(Normal, 0, 0, 0, 0));
        assert!(!check_move_special(Normal, 0, 1, 0, 0));
        assert!(!check_move_special(Normal, 6, 2, 3, 5));
        assert!(!check_move_special(Normal, 1, 0, 2, 3));
        assert!(!check_move_special(Normal, 1, 3, 5, 7));

        assert!(check_move_special(Rook, 5, 3, 5, 4));
        assert!(check_move_special(Rook, 5, 3, 5, 1));
        assert!(check_move_special(Rook, 5, 3, 5, 2));

        assert!(check_move_special(Rook, 5, 3, 2, 3));
        assert!(check_move_special(Rook, 5, 3, 5, 3));
        assert!(check_move_special(Rook, 5, 3, 7, 3));

        assert!(!check_move_special(Rook, 5, 3, 2, 0));
        assert!(!check_move_special(Rook, 5, 3, 7, 4));
        assert!(!check_move_special(Rook, 5, 3, 6, 2));

        assert!(check_move_special(Bishop, 5, 5, 6, 6));
        assert!(check_move_special(Bishop, 5, 5, 7, 7));
        assert!(check_move_special(Bishop, 5, 5, 5, 5));
        assert!(check_move_special(Bishop, 5, 5, 4, 4));
        assert!(check_move_special(Bishop, 5, 5, 6, 4));
        assert!(check_move_special(Bishop, 5, 5, 7, 3));
        assert!(check_move_special(Bishop, 5, 5, 4, 6));
        assert!(check_move_special(Bishop, 5, 5, 3, 7));

        assert!(!check_move_special(Bishop, 5, 5, 6, 5));
        assert!(!check_move_special(Bishop, 5, 5, 7, 6));
        assert!(!check_move_special(Bishop, 5, 5, 2, 3));

        assert!(check_move_special(Forward(Color::Black), 1, 0, 1, 1));
        assert!(check_move_special(Forward(Color::Black), 1, 0, 1, 7));
        assert!(check_move_special(Forward(Color::Black), 1, 2, 1, 5));
        assert!(!check_move_special(Forward(Color::Black), 0, 5, 1, 5));

        assert!(check_move_special(Forward(Color::White), 0, 8, 0, 7));
        assert!(check_move_special(Forward(Color::White), 0, 8, 0, 5));
        assert!(check_move_special(Forward(Color::White), 0, 6, 0, 3));

        assert!(
            !check_move_special(Forward(Color::Black), 0, 5, 0, 2),
            "Can't move backwards when black!"
        );
        assert!(
            !check_move_special(Forward(Color::White), 0, 5, 0, 7),
            "Can't move forwards when white!"
        );
    }

    #[test]
    fn test_abs_diff() {
        assert_eq!(abs_diff(5, 5), 0);
        assert_eq!(abs_diff(0, 0), 0);
        assert_eq!(abs_diff(0, 1), 1);
        assert_eq!(abs_diff(7, 10), 3);
        assert_eq!(abs_diff(7, 10), abs_diff(10, 7));
    }

    #[test]
    fn test_usize_i32_add() {
        assert!(add(2, 3).is_ok());
        assert_eq!(add(2, 3).unwrap(), 5);
        assert!(add(1, -1).is_ok());
        assert_eq!(add(1, -1).unwrap(), 0);
        assert!(add(5, -3).is_ok());
        assert_eq!(add(5, -3).unwrap(), 2);
        assert!(add(0, -1).is_err());
        assert!(add(5, -7).is_err());
    }

    #[test]
    fn test_check_move_map() {
        let trival_move_map = vec![(0, 0)];
        assert!(check_move_map(&trival_move_map, 0, 0, 0, 0));
        assert!(check_move_map(&trival_move_map, 1, 1, 1, 1));
        assert!(!check_move_map(&trival_move_map, 0, 0, 1, 0));
        assert!(!check_move_map(&trival_move_map, 1, 6, 2, 7));

        let simple_move_map = vec![(1, 1)];
        assert!(check_move_map(&simple_move_map, 0, 0, 1, 1));
        assert!(check_move_map(&simple_move_map, 3, 4, 4, 5));
        assert!(!check_move_map(&simple_move_map, 3, 1, 4, 5));
        assert!(!check_move_map(&simple_move_map, 8, 8, 7, 7));

        let bow_move_map = vec![(0, 1), (0, -1), (-2, 2), (2, 2)];
        assert!(check_move_map(&bow_move_map, 5, 5, 5, 6));
        assert!(check_move_map(&bow_move_map, 5, 5, 5, 4));
        assert!(check_move_map(&bow_move_map, 5, 5, 3, 7));
        assert!(check_move_map(&bow_move_map, 5, 5, 7, 7));
        assert!(!check_move_map(&bow_move_map, 5, 5, 6, 6));
        assert!(!check_move_map(&bow_move_map, 5, 5, 5, 5));
    }

    #[test]
    fn test_vertical_flip() {
        let move_map = vec![(0, 0), (0, 1), (0, -1), (-2, 2), (2, 2)];
        let flipped_map = vec![(0, 0), (0, -1), (0, 1), (-2, -2), (2, -2)];
        assert_eq!(vertical_flip(&move_map), flipped_map);
        assert_eq!(vertical_flip(&flipped_map), move_map);
    }

    #[test]
    fn test_move_piece_succeeds() {
        let player = Color::Black;
        let piece_bottom = Piece::new(PieceCombination::PawnGold, player);
        let piece_middle = Piece::new(PieceCombination::BowArrow, player);
        let piece_top = Piece::new(PieceCombination::ProdigyPhoenix, player);

        let mut board = [[Tower::Empty; 9]; 9];

        board[5][5] = Tower::Triple(piece_bottom, piece_middle, piece_top);
        board[6][6] = Tower::Empty;
        let mut clone = board.clone();
        let result = move_piece(board, 5, 5, 6, 6);
        assert!(result.is_ok());

        board = result.unwrap();
        assert_eq!(
            board[5][5].height(),
            TowerHeight::Middle,
            "Expected {:#?} to have Middle height but was {:#?}",
            board[5][5],
            board[5][5].height()
        );
        assert_eq!(board[5][5], Tower::Double(piece_bottom, piece_middle));
        assert_eq!(
            board[6][6].height(),
            TowerHeight::Bottom,
            "Expected {:#?} to have Bottom height but was {:#?}",
            board[6][6],
            board[6][6].height()
        );
        assert_eq!(board[6][6], Tower::Single(piece_top));
        assert!(board != clone);
    }

    #[test]
    fn test_move_piece_fails() {
        let player = Color::Black;
        let piece_bottom = Piece::new(PieceCombination::PawnGold, player);
        let piece_middle = Piece::new(PieceCombination::BowArrow, player);
        let piece_top = Piece::new(PieceCombination::ProdigyPhoenix, player);

        let mut board = [[Tower::Empty; 9]; 9];

        board[0][1] = Tower::Double(piece_bottom, piece_middle);
        board[1][0] = Tower::Single(piece_bottom);
        board[1][1] = Tower::Triple(piece_bottom, piece_middle, piece_top);

        let mut clone = board.clone();

        // Shouldn't be able to move piece from empty tower
        let no_move_empty = move_piece(board, 0, 0, 0, 1);
        assert!(no_move_empty.is_err());
        assert_eq!(
            board,
            clone,
            "Board was {:#?} and should be unchange but is now {:#?}",
            clone,
            board
        );
        // Shouldn't be able to add pieces onto a full tower
        let no_move_full = move_piece(board, 1, 0, 1, 1);
        assert!(no_move_full.is_err());
        assert_eq!(
            board,
            clone,
            "Board was {:#?} and should be unchange but is now {:#?}",
            clone,
            board
        );
    }
}
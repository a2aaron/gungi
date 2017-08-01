// A board is a [[Tower; 9]; 9]

use pieces::*;


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

/// A Move Description indicates how a piece may move
/// A Piece typically can move to another spot within a 5x5 area. For example,
/// a Tier 1 Arrow may move forwards, backwards, back left and back right.
/// Some pieces may also move like a Rook, Bishop, or Rook (only forward).
/// Note that Move Descriptions may differ between Tiers significantly.
/// A standard Move Description contains a Vector of tuples indicating the positions
/// relative to the piece that it may move.
/// For example, since the Tier 2 Bow may move forward, backwards, or
/// up 2 spaces and left or right 2 spaces, it's move list is
/// [(0, 1), (0, -1), (-2, 2), (2, 2)]
#[derive(Clone, Debug, PartialEq, Eq)]
enum MoveDescription {
    Normal(Vec<(i32, i32)>),
    LikeRook(Vec<(i32, i32)>), // Applies to Hidden Dragon (Tier 1), Dragon King (Tier 1)
    LikeBishop(Vec<(i32, i32)>), // Applies to Prodige (Tier 1), Phoenix (Tier 1),
    LikeForward(Vec<(i32, i32)>), // Applies to Lance (Tier 1),
                                  // and Fortress's Mobile Range Expansion Effect
}

/// Returns true if the end coordinates can be reached by atleast one move map
/// entry from the starting coordinates.
fn check_move_map(
    move_map: &Vec<(i32, i32)>,
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

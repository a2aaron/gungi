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

#[cfg(test)]
mod tests {
    use pieces::*;
    use board::*;

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

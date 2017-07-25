// A board is a [[Tower; 9]; 9]

use pieces::*;


/// Returns the board but with the top piece moved to the goal location.
/// Returns Err if the initial tower selected is empty (and thus cannot pop)
fn move_piece(mut board: [[Tower; 9]; 9], initial_i: usize, initial_j: usize, end_i: usize, end_j: usize) -> Result<[[Tower; 9]; 9], &'static str> {
    let piece = board[initial_i][initial_j].pop();
    if let Ok(piece) = piece {
        if let Ok(_) =  board[end_i][end_j].drop_piece(piece) {
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
        let player = Player::new_blank();
        let piece_bottom = Piece::new(PieceCombination::PawnGold, &player);
        let piece_middle = Piece::new(PieceCombination::BowArrow, &player);
        let piece_top = Piece::new(PieceCombination::ProdigyPhoenix, &player);

        let mut board = [[Tower::new(None, None, None).unwrap(); 9]; 9];

        board[5][5] = Tower::new(Some(piece_bottom), Some(piece_middle), Some(piece_top)).unwrap();
        board[6][6] = Tower::new(None, None, None).unwrap();
        let mut clone = board.clone();
        let result = move_piece(board, 5, 5, 6, 6);
        assert!(result.is_ok());

        board = result.unwrap();
        assert_eq!(board[5][5].height(), TowerHeight::Middle,
            "Expected {:#?} to have Middle height but was {:#?}", board[5][5], board[5][5].height());
        assert_eq!(board[5][5].get(TowerHeight::Middle).unwrap(), piece_middle);
        assert_eq!(board[6][6].height(), TowerHeight::Bottom,
            "Expected {:#?} to have Bottom height but was {:#?}", board[6][6], board[6][6].height());
        assert_eq!(board[6][6].get(TowerHeight::Bottom).unwrap(), piece_top);
        assert!(board != clone);
    }
    
    #[test]
    fn test_move_piece_fails() {
        let player = Player::new_blank();
        let piece_bottom = Piece::new(PieceCombination::PawnGold, &player);
        let piece_middle = Piece::new(PieceCombination::BowArrow, &player);
        let piece_top = Piece::new(PieceCombination::ProdigyPhoenix, &player);

        let mut board = [[Tower::new(None, None, None).unwrap(); 9]; 9];
        
        board[0][1] = Tower::new(Some(piece_bottom), Some(piece_middle), None).unwrap();
        board[1][0] = Tower::new(Some(piece_bottom), None, None).unwrap();
        board[1][1] = Tower::new(Some(piece_bottom), Some(piece_middle), Some(piece_top)).unwrap();

        let mut clone = board.clone(); 

        // Shouldn't be able to move piece from empty tower
        let no_move_empty = move_piece(board, 0, 0, 0, 1);
        assert!(no_move_empty.is_err());
        assert_eq!(board, clone, "Board was {:#?} and should be unchange but is now {:#?}", clone, board);
        // Shouldn't be able to add pieces onto a full tower
        let no_move_full = move_piece(board, 1, 0, 1, 1);
        assert!(no_move_full.is_err());
        assert_eq!(board, clone, "Board was {:#?} and should be unchange but is now {:#?}", clone, board);
    }
}
#[cfg(test)]
mod tests {
    use pieces::*;
    // Yes, use static here instead of const because we do want 
    // these pieces to refer to entirely different objects
    // (Player 1 should not have the same memory location as Player 2!)
    static PLAYER_1: Player = Player {};
    static PLAYER_2: Player = Player {};

    // Some example pieces for testing
    // Front Pawn, Back Gold, Player 1
    static PIECE_1A: Piece = Piece {
                    current_side: SideType::Front,
                    front_side: PieceType::Pawn,
                    back_side: PieceType::Gold,
                    belongs_to: &PLAYER_1
                    };
    // Front Bow, Back Silver, Player 1
    static PIECE_1B: Piece = Piece {
                    current_side: SideType::Front,
                    front_side: PieceType::Bow,
                    back_side: PieceType::Silver,
                    belongs_to: &PLAYER_1
                    };

    // Front Pawn, Back Bronze, Player 2
    static PIECE_2A: Piece = Piece {
                    current_side: SideType::Front,
                    front_side: PieceType::Pawn,
                    back_side: PieceType::Bronze,
                    belongs_to: &PLAYER_2
                    };

    // Front Bow, Back Arrow, Player 2
    static PIECE_2B: Piece = Piece {
                    current_side: SideType::Front,
                    front_side: PieceType::Bow,
                    back_side: PieceType::Arrow,
                    belongs_to: &PLAYER_2
                    };

    #[test]
    fn test_commander_has_both_front_and_back() {
        let commander = Piece::new(PieceCombination::Commander, SideType::Front, &PLAYER_1);
        assert_eq!(commander.front_side, PieceType::Commander);
        assert_eq!(commander.back_side, PieceType::Commander);
    }

    #[test]
    fn test_valid_towers() {
        let empty_tower = Tower {bottom: None, mid: None, top: None};
        assert!(is_valid(empty_tower));


        let single_tower = Tower {bottom: Some(PIECE_1A), mid: None, top: None};
        assert!(is_valid(empty_tower));


        let double_tower = Tower {bottom: Some(PIECE_1A), mid: Some(PIECE_1B), top: None};
        assert!(is_valid(double_tower));

        let double_same_type_diff_player_tower = Tower {bottom: Some(PIECE_1A), mid: Some(PIECE_2A), top: None};
        assert!(is_valid(double_same_type_diff_player_tower));

        let triple_tower = Tower {bottom: Some(PIECE_1A), mid: Some(PIECE_1B), top: Some(PIECE_2A)};
        assert!(is_valid(triple_tower));
    }

    #[test]
    fn test_invalid_towers() {
        // Towers can't have any holes in them
        let bad_tower_mid = Tower {bottom: None, mid: Some(PIECE_1A), top: None};
        assert!(!is_valid(bad_tower_mid));

        let bad_tower_top = Tower {bottom: None, mid: None, top: Some(PIECE_1A)};
        assert!(!is_valid(bad_tower_top));

        let bad_tower_mid_top = Tower {bottom: None, mid: Some(PIECE_1A), top: Some(PIECE_1A)};
        assert!(!is_valid(bad_tower_mid_top));

        let bad_tower_bot_top = Tower {bottom: Some(PIECE_1A), mid: None, top: Some(PIECE_1A)};
        assert!(!is_valid(bad_tower_bot_top));
    }

    #[test]
    fn test_same_type_and_player() {
        // Same piece types but one is on the back (true)
        let piece_1 = Piece {
                        current_side: SideType::Front,
                        front_side: PieceType::Pawn,
                        back_side: PieceType::Gold,
                        belongs_to: &PLAYER_1
                        };

        let piece_2 = Piece {
                        current_side: SideType::Back,
                        front_side: PieceType::Silver,
                        back_side: PieceType::Pawn,
                        belongs_to: &PLAYER_1
                        };
        assert!(same_type_and_player(piece_1, piece_2), "Expected the types to be the same even though the current side is different.");

        // Same pieces but different current sides (false)
        let piece_3 = Piece {
                        current_side: SideType::Front,
                        front_side: PieceType::Pawn,
                        back_side: PieceType::Gold,
                        belongs_to: &PLAYER_1
                        };

        let piece_4 = Piece {
                        current_side: SideType::Back,
                        front_side: PieceType::Pawn,
                        back_side: PieceType::Gold,
                        belongs_to: &PLAYER_1
                        };
        assert!(!same_type_and_player(piece_3, piece_4), "Expected the types to be different even though the sides are the same");

        // Same piece types but different players (false)
        let piece_5 = Piece {
                        current_side: SideType::Front,
                        front_side: PieceType::Pawn,
                        back_side: PieceType::Gold,
                        belongs_to: &PLAYER_1
                        };

        let piece_6 = Piece {
                        current_side: SideType::Front,
                        front_side: PieceType::Pawn,
                        back_side: PieceType::Gold,
                        belongs_to: &PLAYER_2
                        };
        assert!(!same_type_and_player(piece_5, piece_6), "Expected the players to be different even though the sides and type are the same");
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
}
#[cfg(test)]
mod tests {
    use pieces::*;

    #[test]
    fn test_commander_has_both_front_and_back() {
        let player = Player::new_blank();
        let commander = Piece::new(PieceCombination::Commander, &player);
        assert_eq!(commander.front_side, PieceType::Commander);
        assert_eq!(commander.back_side, PieceType::Commander);
    }

    #[test]
    fn test_valid_towers() {
        let player1 = Player::new_blank();
        let player2 = Player::new_blank();
        let pawn_gold = Piece::new(PieceCombination::PawnGold, &player1);
        let bow_arrow = Piece::new(PieceCombination::BowArrow, &player1);
        let pawn_gold_2 = Piece::new(PieceCombination::PawnGold, &player2);

        let empty_tower = Tower {bottom: None, mid: None, top: None};
        assert!(empty_tower.is_valid());

        let single_tower = Tower {bottom: Some(pawn_gold), mid: None, top: None};
        assert!(single_tower.is_valid());

        let double_tower = Tower {bottom: Some(pawn_gold), mid: Some(bow_arrow), top: None};
        assert!(double_tower.is_valid());

        let double_same_type_diff_player_tower = Tower {bottom: Some(pawn_gold), mid: Some(pawn_gold_2), top: None};
        assert!(double_same_type_diff_player_tower.is_valid());

        let triple_tower = Tower {bottom: Some(pawn_gold), mid: Some(bow_arrow), top: Some(pawn_gold_2)};
        assert!(triple_tower.is_valid());
    }

    #[test]
    fn test_invalid_towers() {
        let player1 = Player::new_blank();
        let player2 = Player::new_blank();
        let pawn_gold = Piece::new(PieceCombination::PawnGold, &player1);
        let pawn_silver = Piece::new(PieceCombination::PawnSilver, &player1);
        let bow_arrow = Piece::new(PieceCombination::BowArrow, &player1);
        let pawn_gold_2 = Piece::new(PieceCombination::PawnGold, &player2);

        // Towers can't have any holes in them
        let only_mid = Tower {bottom: None, mid: Some(pawn_gold), top: None};
        assert!(!only_mid.is_valid());

        let only_top = Tower {bottom: None, mid: None, top: Some(pawn_gold)};
        assert!(!only_top.is_valid());

        let only_mid_top = Tower {bottom: None, mid: Some(pawn_gold), top: Some(bow_arrow)};
        assert!(!only_mid_top.is_valid());

        let only_bot_top = Tower {bottom: Some(pawn_gold), mid: None, top: Some(bow_arrow)};
        assert!(!only_bot_top.is_valid());

        // Towers can't have two of the same piece in them
        let double_same_tower = Tower {bottom: Some(pawn_gold), mid: Some(pawn_silver), top: None};
        assert!(!double_same_tower.is_valid());

        let triple_same_tower = Tower {bottom: Some(pawn_gold), mid: Some(pawn_silver), top: Some(pawn_gold_2)};
        assert!(!triple_same_tower.is_valid());       
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
        assert!(same_type_and_player(piece_1, piece_2), "Expected the types to be the same even though the current side is different.");

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
        assert!(!same_type_and_player(piece_3, piece_4), "Expected the types to be different even though the sides are the same");

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

    #[test]
    fn test_top_most() {
        let player = Player::new_blank();
        let piece_1 = Some(Piece::new(PieceCombination::PawnGold, &player));
        let piece_2 = Some(Piece::new(PieceCombination::BowArrow, &player));
        let piece_3 = Some(Piece::new(PieceCombination::ProdigyPhoenix, &player));

        let empty = Tower::new(None, None, None).unwrap();
        assert_eq!(empty.height(), TowerHeight::Empty);

        let bottom = Tower::new(piece_1, None, None).unwrap();
        assert_eq!(bottom.height(), TowerHeight::Bottom);

        let middle = Tower::new(piece_1, piece_2, None).unwrap();
        assert_eq!(middle.height(), TowerHeight::Middle);

        let top = Tower::new(piece_1, piece_2, piece_3).unwrap();
        assert_eq!(top.height(), TowerHeight::Top);
    }
}
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

        let empty_tower = Tower::Empty;
        assert!(empty_tower.is_valid());

        let single_tower = Tower::Single(pawn_gold);
        assert!(single_tower.is_valid());

        let double_tower = Tower::Double(pawn_gold, bow_arrow);
        assert!(double_tower.is_valid());

        let double_same_type_diff_player_tower = Tower::Double(pawn_gold, pawn_gold_2);
        assert!(double_same_type_diff_player_tower.is_valid());

        let triple_tower = Tower::Triple(pawn_gold, bow_arrow, pawn_gold_2);
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

        // Towers can't have two of the same piece in them
        let double_same_tower = Tower::Double(pawn_gold, pawn_silver);
        assert!(!double_same_tower.is_valid());

        let triple_same_tower = Tower::Triple(pawn_gold, pawn_silver, pawn_gold_2);
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
            belongs_to: &player1,
        };

        let piece_2 = Piece {
            current_side: SideType::Back,
            front_side: PieceType::Silver,
            back_side: PieceType::Pawn,
            belongs_to: &player1,
        };
        assert!(
            same_type_and_player(piece_1, piece_2),
            "Expected the types to be the same even though the current sides are different."
        );

        // Same pieces but different current sides (false)
        let piece_3 = Piece {
            current_side: SideType::Front,
            front_side: PieceType::Pawn,
            back_side: PieceType::Gold,
            belongs_to: &player1,
        };

        let piece_4 = Piece {
            current_side: SideType::Back,
            front_side: PieceType::Pawn,
            back_side: PieceType::Gold,
            belongs_to: &player1,
        };
        assert!(
            !same_type_and_player(piece_3, piece_4),
            "Expected the types to be different even though the sides are the same"
        );

        // Same piece types but different players (false)
        let piece_5 = Piece {
            current_side: SideType::Front,
            front_side: PieceType::Pawn,
            back_side: PieceType::Gold,
            belongs_to: &player1,
        };

        let piece_6 = Piece {
            current_side: SideType::Front,
            front_side: PieceType::Pawn,
            back_side: PieceType::Gold,
            belongs_to: &player2,
        };
        assert!(
            !same_type_and_player(piece_5, piece_6),
            "Expected the players to be different even though the sides and type are the same"
        );
    }

    #[test]
    fn test_current_type() {
        let front_piece = Piece {
            current_side: SideType::Front,
            front_side: PieceType::Pawn,
            back_side: PieceType::Gold,
            belongs_to: &Player::new_blank(),
        };
        assert_eq!(PieceType::Pawn, front_piece.current_type());

        let back_piece = Piece {
            current_side: SideType::Back,
            front_side: PieceType::Pawn,
            back_side: PieceType::Gold,
            belongs_to: &Player::new_blank(),
        };
        assert_eq!(PieceType::Gold, back_piece.current_type());
    }

    #[test]
    fn test_height() {
        let player = Player::new_blank();
        let piece_1 = Piece::new(PieceCombination::PawnGold, &player);
        let piece_2 = Piece::new(PieceCombination::BowArrow, &player);
        let piece_3 = Piece::new(PieceCombination::ProdigyPhoenix, &player);

        let empty = Tower::Empty;
        assert_eq!(empty.height(), TowerHeight::Empty);

        let bottom = Tower::Single(piece_1);
        assert_eq!(bottom.height(), TowerHeight::Bottom);

        let middle = Tower::Double(piece_1, piece_2);
        assert_eq!(middle.height(), TowerHeight::Middle);

        let top = Tower::Triple(piece_1, piece_2, piece_3);
        assert_eq!(top.height(), TowerHeight::Top);
    }

    #[test]
    fn test_lift_piece() {
        let player = Player::new_blank();
        let piece_bottom = Piece::new(PieceCombination::PawnGold, &player);
        let piece_middle = Piece::new(PieceCombination::BowArrow, &player);
        let piece_top = Piece::new(PieceCombination::ProdigyPhoenix, &player);

        let mut tower = Tower::Triple(piece_bottom, piece_middle, piece_top);

        let (tower, piece_top_lift_piece) = tower.lift_piece().unwrap();
        assert_eq!(piece_top_lift_piece, piece_top);
        assert_eq!(tower.height(), TowerHeight::Middle);

        let (tower, piece_middle_lift_piece) = tower.lift_piece().unwrap();
        assert_eq!(piece_middle_lift_piece, piece_middle);
        assert_eq!(tower.height(), TowerHeight::Bottom);

        let (tower, piece_bottom_lift_piece) = tower.lift_piece().unwrap();
        assert_eq!(piece_bottom_lift_piece, piece_bottom);
        assert_eq!(tower.height(), TowerHeight::Empty);

        let mut empty = Tower::Empty;
        assert!(empty.lift_piece().is_err());
    }

    #[test]
    fn test_drop() {
        use pieces::TowerHeight::*;
        let player = Player::new_blank();
        let piece_bottom = Piece::new(PieceCombination::PawnGold, &player);
        let piece_middle = Piece::new(PieceCombination::BowArrow, &player);
        let piece_top = Piece::new(PieceCombination::ProdigyPhoenix, &player);

        let mut tower = Tower::Empty;

        tower = tower.drop_piece(piece_bottom).unwrap();
        assert_eq!(tower.height(), Bottom);
        assert_eq!(tower, Tower::Single(piece_bottom));

        tower = tower.drop_piece(piece_middle).unwrap();
        assert_eq!(tower.height(), Middle);
        assert_eq!(tower, Tower::Double(piece_bottom, piece_middle));

        tower = tower.drop_piece(piece_top).unwrap();
        assert_eq!(tower.height(), Top);
        assert_eq!(tower, Tower::Triple(piece_bottom, piece_middle, piece_top));

        let piece = Piece::new(PieceCombination::Commander, &player);
        let mut full = Tower::Triple(piece_bottom, piece_middle, piece_top);
        assert!(full.drop_piece(piece).is_err());
    }
}

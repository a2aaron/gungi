# Gungi Board Game

This is an implementation of the Gungi board game found in Hunter X Hunter.
The rules are based off of this link: https://mmmmalo.tumblr.com/post/74510568781/rules-of-gungi

A quick summary of the rules is as follows:

# The Board
The game board consists of a 9x9 square, similar to a chess board. 
There are two players, Black and White. Black moves first. You set up the board by taking turns placing pieces within the first three ranks (rows) of your side. Note that at least one pawn must be placed in every file at the end of the set up.

# The Pieces and Towers
There are many pieces available, each one moving in their own unique way. The pieces can
stack on top of each other. This modifies how they may move. A stack of pieces is called a **tower**, and there can be up to three pieces in a tower. Note that towers may consist of both your and enemy pieces. Also note that two of the same piece type that are of the same color may not be in a tower. Pieces in higher towers are in higher "Tiers," and most pieces move differently depending on which tier they are in. Only the top piece of a tower may move. Pieces on top of an enemy piece move like a Gold reguardless of their actual piece type.

Pieces consist of two sides, called **Front** and **Back**. Initially pieces are played on their Front side, but can be captured by either landing on that piece with one of your one like in Chess (called a **mobile strike**) or by attacking an enemy piece in a tower that you also occupy (called an **immobile strike**).

Upon capturing a piece, you take the enemy piece and add it to your **hand**. These pieces become yours and can be placed on upon any open square on the board. This is called a **drop**. Note that if you capture an enemy Front piece, you drop the Back piece of it instead and vice versa. Note that you may not drop on another piece (including yours), however this has some execptions nor can you capture another piece using a drop. You can achieve a checkmate using a drop.

Note that you can only choose to do one of the following things per turn: move a piece (and possibly capture another), attack a piece from within a tower, or drop a piece.

# Checkmate

The piece you need to capture is the **Commander**, which is essentially the same as the King in Chess. Like Chess, if the Commander is threatened by a mobile or immobile strike, it is in **check**. The King may not move into check.

# Special Moves and Effects
## Earth Link

Earth Link, also called Land Link pieces may be dropped onto despite the fact that this usually isn't possible. These pieces are Spy, Clandestininte, and Fortress. Note that only Back pieces can be dropped on the Spy and only Front pieces can be dropped on the Clandestinite.

## Mobile Range Expansion Effect (MREE)

The Mobile Range Expansion Effect upgrades the Tier of a piece by one. Also, enemy pieces cannot jump over other pieces when effected by MREE. MREE only applies to your pieces. Fortress and Catapult are the only ones which have the MREE.

The following pieces do not experience MREE
- Commander
- Hidden Dragon
- Prodigy
- Dragon King
- Phoenix

## 1-3 Tier Exchange Effect (13TEE)

The 1-3 Tier Exchange Effect describes the effect of two pieces. A Tier 1 Captian is allowed to exchange places with any ally Tier 3 piece (except for the Commander). Additionally, a Tier 3 Commander can exchange with any ally Tier 1 piece (except for the Fortress and Catapult). You cannot 13TEE on the same tower twice in a row.

## Substitution Effect

The Substitution Effect applies to the Samurai. Non-Towered Samurai may exchange places with the Commander if the Commander is in check and the Samruai is directly adjecent with the Commander (diagonally adjecent does not count).

## Betrayal Effect

The Betrayal Effect applies to the Bronze. If a Bronze captures the top piece on a tower and there are any enemy pieces within that tower below it, those pieces automatically become ally pieces. Note that, just like if you had dropped them, the enemy Front pieces become ally Back pieces and enemy Back pieces become ally Front pieces.

## Forced Recovery Effect

The Forced Recovery Effect applies to Pawns, Spies, and Lances, which can only move in one direction. If these pieces are in a position where they are unable to move next turn, you may return those pieces to your hand. You can not use this effect on the same turn you capture an enemy piece.

## Forced Rearrangement Effect

If you capture a Lance, you gain a Fortress or Catapult piece. This new piece must be placed within your territory immediately.

## Commander
- Towers cannot be built on Commanders
- Does not experience MREE

## Piece Jumping
The following pieces may jump over other pieces.
- Spy
- Bow
- Clandestinite

## Catapult and Fortress
- Cannot be added to hand. See Forced Rearrangement Effect.
- Cannot move (and thus cannot mobile strike)
- Cannot immobile strike

# Foul Play

The following are prohibitied. If you do so and your opponent catches you doing one of these actions, you automatically lose the match.

## Repetition
If the same board layout occurs for four turns in a row, the match ends in a draw

## Double Bronze
You may not have two or more Bronze pieces in the same file. This includes both dropping and moving a Bronze piece into the file. Note that you may use this Foul Play even if you have already resigned if you do notice your opponent committing it.

## Double Pawn Drop
You may not *drop* two more Pawns in the same file. This does not apply to simply *moving* a Pawn into another file. You must call out the Foul Play immediately upon seeing your opponent do it, before you take your turn. If you notice afterwards, the game will continue normally.

## Bronze Checkmate
You may not checkmate the Commander using a Bronze piece. This applies to both drop checkmates and regular checkmates.

## Pawn Drop Checkmate
You may not checkmate the Commander by *dropping* a Pawn piece. It is okay to checkmate by moving a Pawn however.


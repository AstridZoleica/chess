Scheme for encoding moves.
Moves each possess an ID, which is constructed as follows:
HV1234cmjnfolmMOVEIDrFI1234srFI1234t1234perFI1234mMOVEID!

When the answer to a question is no, or false, a 0 replaces the character.
Let's consider this in four sections:
HV1234cmjnfolmMOVEID, which is the most basic section relevant to all pieces.
rfI1234, which could handle moves that can only be performed conditionally based on another piece's position.
srFI1234t1234p, which handles moves like castling. Any "castling-like" move cannot be performed in check,
    may be performed once,
    and not if another "castling-like" move has been performed.
    Furthermore, it is conditional requiring a target piece at a certain location and which may or may not have moved.
erfI1234mMOVEID, which handles moves like en passant. Any "en passant-like" move requires a target piece at a certain location.
!, a final exclamation point to denote the end of a moveID. Had to add this in to parse potentially recursive moveIDs.

Basic Notation
HV1234cmjnfolmMOVEID
H -> Horizontal, positive integer or 0, how many spaces to the right the piece moves.
V -> Vertical, positive integer or 0, how many spaces up the piece moves.
1234 -> Reflections. Whether or not the move is reflected into the other quadrants in a Cartesian grid centered on the piece.
    If there is no reflection in this quadrant, place a zero. For example, a move found only in the 3rd quadrant would be 0030.
    1 -> 1st quadrant. +H, +V. If this excluded, it is a rather unusual case. Observe that this occurs for en passant's specification.
    2 -> 2nd quadrant. -H, +V
    3 -> 3rd quadrant. -H, -V
    4 -> 4th quadrant. +H, -V
c -> Capture, can this move be performed with an enemy piece on the target square?
m -> Move, can this move be performed without an enemy piece on the target square?
j -> Jump, is this a jumping move? Will it be stopped by pieces along the way?
    Checking for blocking occurs differently for pieces depending on the next entry, "n".
n -> aNy multiple, can this move occur in any scalar multiples of [H V]?
    If so, then block checking (0 for j) occurs by testing the squares along scalar multiples of [H V].
    If not, then block checking occurs first by checking all squares H steps then V steps,
    and second by checking all squares V steps then H steps. Note that this latter option is far more restrictive.
f -> First, can this only occur on the piece's first move?
o -> Once, can this move only occur once?
lmMOVEID -> Must the piece make a particular move before this one? If so, paste in its moveid where lm is. If not, lm0 will suffice.

Conditional Notation
rFI1234
If the move in question has no conditions, then r000000 will suffice.
r is never replaced by 0. It breaks the move ID up.
F -> Friendly, is the piece in the required position on the same or different team?
    0 for doesn't matter, upper case F for must be friendly, lower case f for must be enemy.
I -> ID of the piece in the position.
    You can use the lower or uppercase, doesn't matter provided that you gave the piece an ID with a lower or uppercase.
    (Rust's to_uppercase function returns the uppercase version of a character or the character of there is no uppercase.)
1 -> spaces up from the moving piece.
2 -> spaces down from the moving piece.
3 -> spaces left of the moving piece.
4 -> spaces right of the moving piece.

Castling Notation
srFI1234t1234p
If the move in question is not a castling move, then sr000000t00000 will suffice.
This notation is for moves similar to castling. The King must not have moved, and castling cannot occur in check.
As you can observe, this is much the same as the notation for conditionals.
sr is never replaced by 00. Once more serves to divide the move ID to keep it from being an endless string of 0s.
FI1234 are the same as in the conditional notation.
Note, however, that they pertain specifically to the other "target" (non-king) piece which is moving with the king during a castling move.
t is never replaced by 0.
t -> Target, the next four integers provide the movement of the aforementioned target piece in the castling move.
1234, up down left right.
p -> Previous moves, is this move illegal if the target piece has moved before?
    p for yes (standard rule where castling requires the rook hasn't moved before), 0 for no (let's you castle if the other piece has already moved).

En Passant Notation
erFI1234mMOVEID
If the move in question is not an en passant move, then er000000m0 will suffice.
This notation is for moves similar to en passant. En passant requires a certain target piece and for said target piece to have just made a particular move.
er is never replaced by 00.
F1234 are the usual, they are the target piece's position relative to the moving piece.
m is never replaced by 0.
MOVEID. Simply paste on the MOVEID of the move that the target piece must make in order to perform en passant.
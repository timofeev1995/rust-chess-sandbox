#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum Color {
    Light,
    Dark,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum PieceKind {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
    pub made_n_moves: u16,
    pub last_moved_at_move_number: u16,
    pub has_pawn_made_leap: bool,
}

impl Piece {
    pub fn new(color: Color, kind: PieceKind) -> Self {
        Self {
            color,
            kind,
            made_n_moves: 0,
            last_moved_at_move_number: 0,
            has_pawn_made_leap: false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Board {
    pub state: [[Option<Piece>; 8]; 8],
    pub is_reversed: bool,
    pub turn: Color,
    pub color_of_king_under_attack: Option<Color>,
    pub number_of_moves: u16,
}

mod datamodel;
mod moves;
pub use datamodel::{Board, Color, Piece, PieceKind};
use moves::{
    can_move_bishop, can_move_king, can_move_knight, can_move_pawn, can_move_queen, can_move_rook,
    is_en_passant_move,
};

impl Board {
    pub fn new(player_side: Color) -> Board {
        // TODO: fix king/queen for dark player side
        let other_side: Color;
        let is_reversed: bool;
        match player_side {
            Color::Dark => {
                other_side = Color::Light;
                is_reversed = true;
            }
            Color::Light => {
                other_side = Color::Dark;
                is_reversed = false;
            }
        }

        let mut initial_board_state = [
            [
                Some(Piece::new(other_side, PieceKind::Rook)),
                Some(Piece::new(other_side, PieceKind::Knight)),
                Some(Piece::new(other_side, PieceKind::Bishop)),
                if player_side == Color::Light {
                    Some(Piece::new(other_side, PieceKind::Queen))
                } else {
                    Some(Piece::new(other_side, PieceKind::King))
                },
                if player_side == Color::Light {
                    Some(Piece::new(other_side, PieceKind::King))
                } else {
                    Some(Piece::new(other_side, PieceKind::Queen))
                },
                Some(Piece::new(other_side, PieceKind::Bishop)),
                Some(Piece::new(other_side, PieceKind::Knight)),
                Some(Piece::new(other_side, PieceKind::Rook)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some(Piece::new(player_side, PieceKind::Rook)),
                Some(Piece::new(player_side, PieceKind::Knight)),
                Some(Piece::new(player_side, PieceKind::Bishop)),
                if player_side == Color::Light {
                    Some(Piece::new(player_side, PieceKind::Queen))
                } else {
                    Some(Piece::new(player_side, PieceKind::King))
                },
                if player_side == Color::Light {
                    Some(Piece::new(player_side, PieceKind::King))
                } else {
                    Some(Piece::new(player_side, PieceKind::Queen))
                },
                Some(Piece::new(player_side, PieceKind::Bishop)),
                Some(Piece::new(player_side, PieceKind::Knight)),
                Some(Piece::new(player_side, PieceKind::Rook)),
            ],
        ];
        for x_pos in 0..8 {
            initial_board_state[1][x_pos] = Some(Piece::new(other_side, PieceKind::Pawn));
            initial_board_state[6][x_pos] = Some(Piece::new(player_side, PieceKind::Pawn));
        }
        Board {
            state: initial_board_state,
            is_reversed,
            turn: Color::Light,
            color_of_king_under_attack: None,
            number_of_moves: 0,
        }
    }
    pub fn clear_cell(&mut self, cell_position: &(usize, usize)) {
        self.state[7 - cell_position.1][cell_position.0] = None;
    }
    pub fn put_piece_at_cell(&mut self, cell_position: &(usize, usize), piece: Piece) {
        self.state[7 - cell_position.1][cell_position.0] = Some(piece);
    }
    pub fn make_a_move(
        &mut self,
        from_cell_position: &(usize, usize),
        to_cell_position: &(usize, usize),
    ) {
        if self.state[7 - to_cell_position.1][to_cell_position.0].is_some() {
            Self::clear_cell(self, to_cell_position);
        }

        let mut piece_to_move_in = self.state[7 - from_cell_position.1][from_cell_position.0]
            .unwrap()
            .clone();
        piece_to_move_in.made_n_moves += 1;
        piece_to_move_in.last_moved_at_move_number = self.number_of_moves;

        if piece_to_move_in.kind == PieceKind::Pawn {
            let vertical_diff = (from_cell_position.1 as i32 - to_cell_position.1 as i32).abs();
            if vertical_diff == 2 {
                piece_to_move_in.has_pawn_made_leap = true;
            }
            let is_en_passant_case = is_en_passant_move(self, from_cell_position, to_cell_position);
            if is_en_passant_case {
                Self::clear_cell(self, &(to_cell_position.0, from_cell_position.1));
            }
        }
        Self::put_piece_at_cell(self, to_cell_position, piece_to_move_in);
        Self::clear_cell(self, from_cell_position);

        self.number_of_moves += 1;
        self.turn = match self.turn {
            Color::Light => Color::Dark,
            Color::Dark => Color::Light,
        };
    }
    pub fn get_king_position(&self, color: Color) -> (usize, usize) {
        // Assumes there is always king exists
        for x_pos in 0..8 {
            for y_pos in 0..8 {
                let piece = self.state[7 - y_pos][x_pos];
                match piece {
                    Some(piece) => {
                        if piece.color == color && piece.kind == PieceKind::King {
                            return (x_pos, y_pos);
                        }
                    }
                    None => (),
                }
            }
        }
        (0, 0)
    }
}

pub fn can_move_piece(
    board: &Board,
    from_position: &(usize, usize),
    to_position: &(usize, usize),
) -> bool {
    let piece = board.state[7 - from_position.1][from_position.0];
    let piece_to_take = board.state[7 - to_position.1][to_position.0];

    if piece.is_none() {
        return false;
    }
    let piece = piece.unwrap();

    match piece_to_take {
        Some(piece_to_take) => {
            if piece_to_take.color == piece.color {
                return false;
            }
        }
        None => (),
    }

    let is_reversed = (board.is_reversed & (piece.color == Color::Light))
        || (!board.is_reversed & (piece.color == Color::Dark));

    let mut vertical_diff = to_position.1 as i8 - from_position.1 as i8;
    let horizontal_diff = (to_position.0 as i8 - from_position.0 as i8).abs();
    if is_reversed {
        vertical_diff *= -1;
    }

    let is_valid_move = match piece.kind {
        PieceKind::Pawn => can_move_pawn(
            board,
            vertical_diff,
            horizontal_diff,
            from_position,
            to_position,
        ),
        PieceKind::Rook => can_move_rook(
            board,
            vertical_diff,
            horizontal_diff,
            from_position,
            to_position,
        ),
        PieceKind::Bishop => can_move_bishop(
            board,
            vertical_diff,
            horizontal_diff,
            from_position,
            to_position,
        ),
        PieceKind::Knight => can_move_knight(vertical_diff, horizontal_diff),
        PieceKind::Queen => can_move_queen(
            board,
            vertical_diff,
            horizontal_diff,
            from_position,
            to_position,
        ),
        PieceKind::King => can_move_king(vertical_diff, horizontal_diff),
    };
    is_valid_move
}

pub fn is_any_piece_attacks_position(
    board: &Board,
    color: Color,
    position: &(usize, usize),
) -> bool {
    for x_pos in 0..8 {
        for y_pos in 0..8 {
            let piece = board.state[7 - y_pos][x_pos];
            if piece.is_some() {
                if piece.unwrap().color == color {
                    if can_move_piece(board, &(x_pos, y_pos), position) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn encode_valid_cells_to_integers(valid_cells_to_move: &[[bool; 8]; 8]) -> (i32, i32) {
    let mut result_0: i32 = 0;
    let mut result_1: i32 = 0;
    for x_pos in 0..8 {
        for y_pos in 0..4 {
            if valid_cells_to_move[x_pos][y_pos] {
                result_0 += 1;
            }
            if x_pos != 7 || y_pos != 3 {
                result_0 <<= 1;
            }
        }

        for y_pos in 4..8 {
            if valid_cells_to_move[x_pos][y_pos] {
                result_1 += 1;
            }
            if x_pos != 7 || y_pos != 7 {
                result_1 <<= 1;
            }
        }
    }
    (result_0, result_1)
}

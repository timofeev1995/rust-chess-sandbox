use super::{is_any_piece_attacks_position, Board, Color, PieceKind};

fn _is_king_move_leads_to_check(
    board: &Board,
    from_position: &(usize, usize),
    to_position: &(usize, usize),
) -> bool {
    let king_color = board.state[7 - from_position.1][from_position.0]
        .unwrap()
        .color;
    let oppenent_color = match king_color {
        Color::Light => Color::Dark,
        Color::Dark => Color::Light,
    };
    is_any_piece_attacks_position(board, oppenent_color, to_position)
}

pub fn can_move_pawn(
    board: &Board,
    vertical_diff: i8,
    horizontal_diff: i8,
    from_position: &(usize, usize),
    to_position: &(usize, usize),
) -> bool {
    let piece_to_take = board.state[7 - to_position.1][to_position.0];
    if vertical_diff == 1 {
        if horizontal_diff == 0 {
            piece_to_take.is_none()
        } else if horizontal_diff == 1 {
            let is_regular_taking = piece_to_take.is_some()
                && board.state[7 - from_position.1][from_position.0]
                    .unwrap()
                    .color
                    != piece_to_take.unwrap().color;
            let is_en_passant = is_en_passant_move(board, from_position, to_position);
            is_regular_taking || is_en_passant
        } else {
            false
        }
    } else if vertical_diff == 2 && horizontal_diff == 0 {
        if (from_position.1 == 1) | (from_position.1 == 6) {
            let intermediate_vertical_pos = (to_position.1 + from_position.1) / 2;
            let piece_at_intermediate_pos =
                board.state[7 - intermediate_vertical_pos][to_position.0];
            piece_at_intermediate_pos.is_none()
                && board.state[7 - to_position.1][to_position.0].is_none()
        } else {
            false
        }
    } else {
        false
    }
}

pub fn can_move_rook(
    board: &Board,
    vertical_diff: i8,
    horizontal_diff: i8,
    from_position: &(usize, usize),
    to_position: &(usize, usize),
) -> bool {
    if vertical_diff != 0 && horizontal_diff != 0 {
        false
    } else {
        let mut is_valid = true;
        if vertical_diff == 0 {
            let _from = std::cmp::min(from_position.0, to_position.0);
            let _to = std::cmp::max(from_position.0, to_position.0);
            for x_pos in (_from + 1).._to {
                if board.state[7 - from_position.1][x_pos].is_some() {
                    is_valid = false;
                    break;
                }
            }
        } else {
            let _from = std::cmp::min(from_position.1, to_position.1);
            let _to = std::cmp::max(from_position.1, to_position.1);
            for y_pos in (_from + 1).._to {
                if board.state[7 - y_pos][from_position.0].is_some() {
                    is_valid = false;
                    break;
                }
            }
        }
        is_valid
    }
}

pub fn can_move_knight(vertical_diff: i8, horizontal_diff: i8) -> bool {
    vertical_diff.abs() == 1 && horizontal_diff == 2
        || vertical_diff.abs() == 2 && horizontal_diff == 1
}

pub fn can_move_bishop(
    board: &Board,
    vertical_diff: i8,
    horizontal_diff: i8,
    from_position: &(usize, usize),
    to_position: &(usize, usize),
) -> bool {
    if vertical_diff.abs() != horizontal_diff {
        false
    } else {
        let mut is_valid = true;
        let _x_diff = to_position.0 as i8 - from_position.0 as i8;
        let _y_diff = to_position.1 as i8 - from_position.1 as i8;
        let _n_steps = _x_diff.abs() - 1;

        let mut current_x_pos = from_position.0 as i8 + _x_diff.signum();
        let mut current_y_pos = from_position.1 as i8 + _y_diff.signum();
        for _ in 0.._n_steps {
            if board.state[(7 - current_y_pos) as usize][current_x_pos as usize].is_some() {
                is_valid = false;
                break;
            } else {
                current_x_pos += _x_diff.signum();
                current_y_pos += _y_diff.signum();
            }
        }
        is_valid
    }
}

pub fn can_move_queen(
    board: &Board,
    vertical_diff: i8,
    horizontal_diff: i8,
    from_position: &(usize, usize),
    to_position: &(usize, usize),
) -> bool {
    can_move_bishop(
        board,
        vertical_diff,
        horizontal_diff,
        from_position,
        to_position,
    ) || can_move_rook(
        board,
        vertical_diff,
        horizontal_diff,
        from_position,
        to_position,
    )
}

pub fn can_move_king(vertical_diff: i8, horizontal_diff: i8) -> bool {
    vertical_diff < 2 && horizontal_diff < 2
}

pub fn is_en_passant_move(
    board: &Board,
    from_position: &(usize, usize),
    to_position: &(usize, usize),
) -> bool {
    let pawn_to_move = board.state[7 - from_position.1][from_position.0].unwrap();
    let piece_at_destination = board.state[7 - to_position.1][to_position.0];
    let piece_or_empty_at_ep_pos = board.state[7 - from_position.1][to_position.0];
    match piece_or_empty_at_ep_pos {
        Some(piece) => {
            piece.kind == PieceKind::Pawn
                && piece.color != pawn_to_move.color
                && piece.made_n_moves == 1
                && piece.last_moved_at_move_number == board.number_of_moves - 1
                && piece.has_pawn_made_leap
                && piece_at_destination.is_none()
        }
        None => false,
    }
}

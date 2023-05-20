mod backend;
mod ui;

use backend::{Board, Color};
use glium::{glutin::event::ElementState, Surface};
use ui::shaders::primitives::Vertex;
use ui::shaders::{get_board_shader, get_piece_shader, get_textures};
use ui::ui_support::{init, AppSettings};

pub fn run() -> () {
    let app_settings = AppSettings {
        window_height: 640f64,
        window_width: 640f64,
        window_name: "Chess".to_owned(),
        max_fps: 60.0,
    };

    let system = init(&app_settings);
    let start = std::time::Instant::now();
    let empty_texture = glium::texture::SrgbTexture2d::empty(&system.display, 1, 1).unwrap();
    let pieces_textures = get_textures(&system.display);
    let board_shader: ui::shaders::Shader = get_board_shader(&system.display);
    let pawn_shader: ui::shaders::Shader = get_piece_shader(&system.display);

    let mut board = Board::new(Color::Light);
    let mut valid_cells_to_move: [[bool; 8]; 8] = [[false; 8]; 8];
    let mut valid_cell_to_move_encoded: (i32, i32) = (0, 0);

    let mut previous_mouse_state = &ElementState::Released;
    let mut taken_piece_cell_position: Option<(usize, usize)> = None;

    system.main_loop(move |_run, display, cursor_position, mouse_input_state| {
        let screenspace_cursor_position: (f64, f64) = (cursor_position.0, cursor_position.1);
        // -1. -> 1.
        let raw_ndc_cursor_position: (f64, f64) = (
            (screenspace_cursor_position.0 - app_settings.window_width) / app_settings.window_width,
            -1.0 * (screenspace_cursor_position.1 - app_settings.window_height)
                / app_settings.window_height,
        );
        let ndc_cursor_position: (f64, f64) = (
            (raw_ndc_cursor_position.0 as f32).clamp(-1.0, 1.0) as f64,
            (raw_ndc_cursor_position.1 as f32).clamp(-1.0, 1.0) as f64,
        );

        if mouse_input_state == &ElementState::Pressed
            && previous_mouse_state == &ElementState::Released
        {
            // take piece
            let cursor_cell_position =
                ui::ndc_cursor_position_to_cell_position(&ndc_cursor_position);
            let taken_piece = board.state[7 - cursor_cell_position.1][cursor_cell_position.0];

            if taken_piece.is_some() && taken_piece.unwrap().color == board.turn {
                taken_piece_cell_position = Some(cursor_cell_position);
                // calculate appropriate moves for the piece
                for pos_x in 0..8 {
                    for pos_y in 0..8 {
                        let mut is_appropriate_move = backend::can_move_piece(
                            &board,
                            &taken_piece_cell_position.unwrap(),
                            &(pos_x, pos_y),
                        );

                        if is_appropriate_move {
                            let piece = taken_piece.unwrap();
                            let oppenent_color = match piece.color {
                                Color::Light => Color::Dark,
                                Color::Dark => Color::Light,
                            };
                            // Copy the board and make move to check if there is a check after
                            let mut tmp_board = board.clone();

                            let from_position = taken_piece_cell_position.unwrap();
                            tmp_board.put_piece_at_cell(&(pos_x, pos_y), piece);
                            tmp_board.clear_cell(&from_position);
                            let this_side_king_position = tmp_board.get_king_position(piece.color);
                            let lead_to_check = backend::is_any_piece_attacks_position(
                                &tmp_board,
                                oppenent_color,
                                &this_side_king_position,
                            );
                            is_appropriate_move = is_appropriate_move && !lead_to_check;
                        }
                        valid_cells_to_move[pos_x][pos_y] = is_appropriate_move;
                    }
                }
                valid_cell_to_move_encoded =
                    backend::encode_valid_cells_to_integers(&valid_cells_to_move);
            }
            previous_mouse_state = &ElementState::Pressed;
        } else if mouse_input_state == &ElementState::Released
            && previous_mouse_state == &ElementState::Pressed
        {
            if taken_piece_cell_position.is_some() {
                // drop: finish moving or eat opponent piece
                let destination_cell_position =
                    ui::ndc_cursor_position_to_cell_position(&ndc_cursor_position);
                let initial_cell_position = taken_piece_cell_position.unwrap();
                if destination_cell_position != initial_cell_position
                    && valid_cells_to_move[destination_cell_position.0][destination_cell_position.1]
                {
                    board.make_a_move(&initial_cell_position, &destination_cell_position);
                }
                taken_piece_cell_position = None;
            }
            previous_mouse_state = &ElementState::Released;
            valid_cell_to_move_encoded = (0, 0);
        }

        let time: f32 = start.elapsed().as_secs_f32();

        let mut target = display.draw();
        target.clear_color_srgb(0.52, 0.41, 0.22, 1.0);

        let board_shape = vec![
            Vertex::from_position([-1.0, -1.0]),
            Vertex::from_position([1.0, -1.0]),
            Vertex::from_position([-1.0, 1.0]),
            Vertex::from_position([1.0, 1.0]),
        ];
        let vertex_buffer = glium::VertexBuffer::new(display, &board_shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
        board_shader.draw(
            &mut target,
            vertex_buffer,
            indices,
            &ndc_cursor_position,
            &empty_texture,
            time,
            valid_cell_to_move_encoded,
        );

        // Draw pieces
        let cell_size: f32 = 0.25;
        for pos_x in 0..8 {
            for pos_y in 0..8 {
                // Do not draw taken piece in the loop:
                if taken_piece_cell_position.is_some() {
                    let cell_position = taken_piece_cell_position.unwrap();
                    if pos_x == cell_position.0 && pos_y == cell_position.1 {
                        continue;
                    }
                }

                let pos_y = 7 - pos_y;
                let lower_left_position: [f32; 2] = [
                    -1.0 + pos_x as f32 * cell_size,
                    -1.0 + (7 - pos_y) as f32 * cell_size,
                ];
                let piece = &board.state[pos_y][pos_x];
                match piece {
                    Some(piece) => {
                        let shape = vec![
                            ui::shaders::primitives::Vertex::from_position_and_texture_coordinates(
                                lower_left_position,
                                [0.0, 0.0],
                            ),
                            ui::shaders::primitives::Vertex::from_position_and_texture_coordinates(
                                [lower_left_position[0], lower_left_position[1] + cell_size],
                                [0.0, 1.0],
                            ),
                            ui::shaders::primitives::Vertex::from_position_and_texture_coordinates(
                                [lower_left_position[0] + cell_size, lower_left_position[1]],
                                [1.0, 0.0],
                            ),
                            ui::shaders::primitives::Vertex::from_position_and_texture_coordinates(
                                [
                                    lower_left_position[0] + cell_size,
                                    lower_left_position[1] + cell_size,
                                ],
                                [1.0, 1.0],
                            ),
                        ];

                        let texture_ = pieces_textures.get(&(piece.kind, piece.color));
                        let piece_texture = match texture_ {
                            Some(texture) => texture,
                            None => &empty_texture,
                        };

                        let indices =
                            glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
                        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
                        pawn_shader.draw(
                            &mut target,
                            vertex_buffer,
                            indices,
                            &ndc_cursor_position,
                            piece_texture,
                            time,
                            (0, 0),
                        );
                    }
                    None => (),
                }
            }
        }

        // Draw taken piece:
        if taken_piece_cell_position.is_some() {
            let piece_cell = taken_piece_cell_position.unwrap();
            let taken_piece = &board.state[7 - piece_cell.1][piece_cell.0];
            match taken_piece {
                Some(piece) => {
                    let cell_size = 0.32;
                    let lower_left_position: [f32; 2] = [
                        ndc_cursor_position.0 as f32 - (cell_size / 2.0),
                        ndc_cursor_position.1 as f32 - (cell_size / 2.0),
                    ];
                    let shape = vec![
                        ui::shaders::primitives::Vertex::from_position_and_texture_coordinates(
                            lower_left_position,
                            [0.0, 0.0],
                        ),
                        ui::shaders::primitives::Vertex::from_position_and_texture_coordinates(
                            [lower_left_position[0], lower_left_position[1] + cell_size],
                            [0.0, 1.0],
                        ),
                        ui::shaders::primitives::Vertex::from_position_and_texture_coordinates(
                            [lower_left_position[0] + cell_size, lower_left_position[1]],
                            [1.0, 0.0],
                        ),
                        ui::shaders::primitives::Vertex::from_position_and_texture_coordinates(
                            [
                                lower_left_position[0] + cell_size,
                                lower_left_position[1] + cell_size,
                            ],
                            [1.0, 1.0],
                        ),
                    ];
                    let texture_ = pieces_textures.get(&(piece.kind, piece.color));
                    let piece_texture = match texture_ {
                        Some(texture) => texture,
                        None => &empty_texture,
                    };
                    let indices =
                        glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
                    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
                    pawn_shader.draw(
                        &mut target,
                        vertex_buffer,
                        indices,
                        &ndc_cursor_position,
                        piece_texture,
                        time,
                        (0, 0),
                    );
                }
                _ => (),
            }
        }

        target.finish().expect("Failed to swap buffers");
    });
}

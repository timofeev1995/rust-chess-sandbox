use std::collections::HashMap;
use std::io::Cursor;

use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::{index::*, uniform, VertexBuffer};
use glium::{Display, Frame, Surface};

use crate::backend::{Color, PieceKind};

pub mod primitives;
pub mod utils;

pub struct Shader {
    program: glium::Program,
}

impl Shader {
    // TODO: move vertex_buffer and shape, index outside:
    // shader is a program, shape and indices are inputs
    pub fn compile(
        vertex_shader_src_code: String,
        fragment_shader_src_code: String,
        display: &Display,
    ) -> Shader {
        let program = glium::Program::from_source(
            display,
            &vertex_shader_src_code,
            &fragment_shader_src_code,
            None,
        )
        .unwrap();
        Self { program }
    }

    pub fn draw(
        &self,
        target: &mut Frame,
        vertex_buffer: VertexBuffer<primitives::Vertex>,
        indices: NoIndices,
        ndc_cursor_position: &(f64, f64),
        texture: &glium::texture::SrgbTexture2d,
        time: f32,
        cells_to_highlight: (i32, i32),
    ) -> () {
        let uniforms = uniform! {
            ndc_cursor_x: ndc_cursor_position.0 as f32,
            ndc_cursor_y: ndc_cursor_position.1 as f32,
            time: time,
            tex: texture,
            cells_to_highlight: cells_to_highlight
        };
        target
            .draw(
                &vertex_buffer,
                indices,
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}

pub fn get_board_shader(display: &Display) -> Shader {
    Shader::compile(
        utils::read_text_from_file("src/ui/shaders/data/board.vert"),
        utils::read_text_from_file("src/ui/shaders/data/board.frag"),
        display,
    )
}

pub fn get_piece_shader(display: &Display) -> Shader {
    Shader::compile(
        utils::read_text_from_file("src/ui/shaders/data/piece.vert"),
        utils::read_text_from_file("src/ui/shaders/data/piece.frag"),
        display,
    )
}

pub fn get_texture(image: RawImage2d<u8>, display: &Display) -> glium::texture::SrgbTexture2d {
    glium::texture::SrgbTexture2d::new(display, image).unwrap()
}

pub fn get_textures(display: &Display) -> HashMap<(PieceKind, Color), SrgbTexture2d> {
    let mut mapping: HashMap<(PieceKind, Color), SrgbTexture2d> = HashMap::new();
    // TODO image::open("src/ui/resources/pieces/light/king.png")

    let image = image::load(
        Cursor::new(&include_bytes!(
            "/Users/egortimofeev/Documents/rust/chess/src/ui/resources/pieces/light/pawn.png"
        )),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    mapping.insert((PieceKind::Pawn, Color::Light), get_texture(image, display));

    let image = image::load(
        Cursor::new(&include_bytes!(
            "/Users/egortimofeev/Documents/rust/chess/src/ui/resources/pieces/dark/pawn.png"
        )),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    mapping.insert((PieceKind::Pawn, Color::Dark), get_texture(image, display));

    let image = image::load(
        Cursor::new(&include_bytes!(
            "/Users/egortimofeev/Documents/rust/chess/src/ui/resources/pieces/light/rook.png"
        )),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    mapping.insert((PieceKind::Rook, Color::Light), get_texture(image, display));

    let image = image::load(
        Cursor::new(&include_bytes!(
            "/Users/egortimofeev/Documents/rust/chess/src/ui/resources/pieces/dark/rook.png"
        )),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    mapping.insert((PieceKind::Rook, Color::Dark), get_texture(image, display));

    let image = image::load(
        Cursor::new(&include_bytes!(
            "/Users/egortimofeev/Documents/rust/chess/src/ui/resources/pieces/light/knight.png"
        )),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    mapping.insert(
        (PieceKind::Knight, Color::Light),
        get_texture(image, display),
    );

    let image = image::load(
        Cursor::new(&include_bytes!(
            "/Users/egortimofeev/Documents/rust/chess/src/ui/resources/pieces/dark/knight.png"
        )),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    mapping.insert(
        (PieceKind::Knight, Color::Dark),
        get_texture(image, display),
    );

    let image = image::load(
        Cursor::new(&include_bytes!(
            "/Users/egortimofeev/Documents/rust/chess/src/ui/resources/pieces/light/bishop.png"
        )),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    mapping.insert(
        (PieceKind::Bishop, Color::Light),
        get_texture(image, display),
    );

    let image = image::load(
        Cursor::new(&include_bytes!(
            "/Users/egortimofeev/Documents/rust/chess/src/ui/resources/pieces/dark/bishop.png"
        )),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    mapping.insert(
        (PieceKind::Bishop, Color::Dark),
        get_texture(image, display),
    );

    let image = image::load(
        Cursor::new(&include_bytes!(
            "/Users/egortimofeev/Documents/rust/chess/src/ui/resources/pieces/light/queen.png"
        )),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    mapping.insert(
        (PieceKind::Queen, Color::Light),
        get_texture(image, display),
    );

    let image = image::load(
        Cursor::new(&include_bytes!(
            "/Users/egortimofeev/Documents/rust/chess/src/ui/resources/pieces/dark/queen.png"
        )),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    mapping.insert((PieceKind::Queen, Color::Dark), get_texture(image, display));

    let image = image::load(
        Cursor::new(&include_bytes!(
            "/Users/egortimofeev/Documents/rust/chess/src/ui/resources/pieces/light/king.png"
        )),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    mapping.insert((PieceKind::King, Color::Light), get_texture(image, display));

    let image = image::load(
        Cursor::new(&include_bytes!(
            "/Users/egortimofeev/Documents/rust/chess/src/ui/resources/pieces/dark/king.png"
        )),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    mapping.insert((PieceKind::King, Color::Dark), get_texture(image, display));

    mapping
}

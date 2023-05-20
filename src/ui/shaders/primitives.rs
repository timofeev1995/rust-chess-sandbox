use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub texture_coords: [f32; 2],
}

impl Vertex {
    pub fn from_position(position: [f32; 2]) -> Vertex {
        Vertex {
            position: position,
            texture_coords: [0.0, 0.0],
        }
    }

    pub fn from_position_and_texture_coordinates(
        position: [f32; 2],
        texture_coords: [f32; 2],
    ) -> Vertex {
        Vertex {
            position: position,
            texture_coords: texture_coords,
        }
    }
}

implement_vertex!(Vertex, position, texture_coords);

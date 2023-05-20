#version 140

in vec2 position;
out vec2 world_position;
out vec2 uv_position;

void main() {

    // Narrow case: our uv positions exacply match uv_position with [-1, 1] ???
    world_position = position;
    uv_position = (world_position + 1.0) / 2.0;
    gl_Position = vec4(world_position, 1.0, 1.0);
}

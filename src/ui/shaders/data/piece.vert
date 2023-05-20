#version 140

in vec2 position;
in vec2 texture_coords;
uniform float ndc_cursor_x;
uniform float ndc_cursor_y;
uniform float time;
out vec2 world_position;
out vec2 v_texture_coords;

void main() {

    // Narrow case: our uv positions exacply match uv_position with [-1, 1] ???
    // its actually because our world == [-1, 1]
    world_position = position;
    // 0 - 1, uv == texture coordinates
    v_texture_coords = texture_coords;

    // float cursor_offset_x = ndc_cursor_x - 1.0 + 0.125;
    // float cursor_offset_y = ndc_cursor_y - 1.0 + 0.125;
    // gl_Position = vec4(world_position.x + cursor_offset_x, world_position.y + cursor_offset_y, 0.0, 1.0);
    gl_Position = vec4(world_position.x, world_position.y, 0.0, 1.0);
}

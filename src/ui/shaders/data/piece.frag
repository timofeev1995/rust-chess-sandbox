#version 140

in vec2 v_texture_coords;

uniform sampler2D tex;

out vec4 color;

void main() {
    vec4 texture_color = texture(tex, v_texture_coords);
    if (texture_color.a < 0.01) {
        discard;
    }
    color = texture_color;
}

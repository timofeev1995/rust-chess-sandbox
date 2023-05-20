#version 140

in vec2 uv_position;
in vec2 world_position;

uniform float time;
uniform ivec2 cells_to_highlight;

out vec4 color;

float getNthBit(int v, int bit) {
    int shifted = v >> bit;
    return float(shifted & 1);
    // if (mod(shifted, 2) <= 0) {
    //     return 0.0;
    // } else {
    //     return 1.0;
    // }
}

void main() {

    vec3 color_;
    float border_width = 0.003;
    float to_highlight = 1.0;
    if (uv_position.x < border_width || uv_position.x > 1.0 - border_width) {
        color_ = vec3(0.10, 0.05, 0.05); 
    } else if (uv_position.y < border_width || uv_position.y > 1.0 - border_width) {
        color_ = vec3(0.10, 0.05, 0.05);
    } else {
        
        vec2 cell = floor(uv_position * 8.0);
        int cell_x = int(cell.x);
        int cell_y = int(cell.y);

        bool cell_to_highlight = false;
        int to_check;
        if (cell_y > 3) {
            cell_y -= 4;
            to_check = cells_to_highlight.y;
        } else {
            to_check = cells_to_highlight.x;
        }
        int bit = 31 - cell_x * 4 - cell_y;
        to_highlight = getNthBit(to_check, bit);
        if (to_highlight > 0.0) {
            cell_to_highlight = true;
        }

        vec3 cell_color;
        
        vec2 grid = fract(uv_position * 4.0) - 0.5;
        float sign_ = sign(grid.x * grid.y);
        if (sign_ > 0.0) {
            cell_color = vec3(0.14, 0.086, 0.034);
        } else {
            cell_color = vec3(0.76, 0.69, 0.55);
        };

        vec3 highlight_color = cell_color;
        if (cell_to_highlight == true) {
            vec2 distances_to_cell_border = (0.25 - abs(abs(grid) - 0.25)) * 4.0;
            float distance_to_border = min(distances_to_cell_border.x, distances_to_cell_border.y);
            if (distance_to_border < 0.3 * abs(sin(time * 2.0))) {
                highlight_color = vec3(0.0, 1.0 - pow(distance_to_border + 0.5, 4.0), 0.0); 
            }      
        }

        color_ = (cell_color + highlight_color) / 2.0;


    };
    color = vec4(color_, 1.0);
}

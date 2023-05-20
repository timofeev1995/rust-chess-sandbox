pub mod shaders;
pub mod ui_support;

pub fn ndc_cursor_position_to_cell_position(ndc_cursor_position: &(f64, f64)) -> (usize, usize) {
    let cell_x = ((ndc_cursor_position.0 + 0.9999) * 4.0).floor() as usize;
    let cell_y = ((ndc_cursor_position.1 + 0.9999) * 4.0).floor() as usize;
    (cell_x, cell_y)
}

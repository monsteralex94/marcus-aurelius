use macroquad::prelude::*;

pub fn get_centered_text_x(text: &str, font_size: f32) -> f32 {
    let dims = measure_text(text, None, font_size as u16, 1.0);
    (screen_width() - dims.width) / 2.0
}

pub fn draw_text_outline(
    text: &str,
    x: f32,
    y: f32,
    font_size: f32,
    outline_thickness: f32,
    text_color: Color,
    outline_color: Color,
) {
    draw_text(text, -outline_thickness + x, y, font_size, outline_color);
    draw_text(text, outline_thickness + x, y, font_size, outline_color);
    draw_text(text, x, y - outline_thickness, font_size, outline_color);
    draw_text(text, x, y + outline_thickness, font_size, outline_color);

    draw_text(text, x, y, font_size, text_color);
}

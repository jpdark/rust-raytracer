//! For saving canvas to .ppm file
//!
//!

use crate::{canvas::Canvas, color::Color};

/// String formatted as PPM
type PPMString = String;

fn pixel_row_to_string(row: &[Color<f32>]) -> String {
    let mut lines = vec![];
    let mut current_line = String::new();
    for pixel in row {
        for p in pixel.as_rgb8().as_array().iter() {
            let new_str = format!("{} ", p);
            // There is software out there that requires .ppm file lines to
            // be shorter than 70 characters. Thus, if the length _will_ be
            // longer than 70, add newline.
            if (current_line.len() + new_str.len()) > 70 {
                lines.push(current_line);
                current_line = String::new();
            }
            current_line.push_str(new_str.as_str())
        }
    }
    lines.push(current_line);
    lines.join("\n")
}

/// Transform the pixels to continuous string for PPM file.
fn canvas_pixels_to_string(canvas: &Canvas) -> String {
    let mut pixels_string = String::new();
    for row in canvas.pixels.chunks(canvas.height) {
        let row_string = pixel_row_to_string(row);
        pixels_string.push_str(format!("{}\n", row_string).as_str());
    }
    pixels_string.trim_end().to_string()
}

/// Reads canvas pixels and writes to PPM string.
pub fn ppm_from_canvas(canvas: &Canvas) -> PPMString {
    format!(
        "\
        P3\n\
        {height} {width}\n\
        255\n\
        {pixels}\n
        ",
        height = canvas.height,
        width = canvas.width,
        pixels = canvas_pixels_to_string(canvas)
    )
}

#[cfg(test)]
mod test_ppm {
    use crate::{canvas::Canvas, color::Color};

    use super::{ppm_from_canvas, PPMString};

    #[test]
    fn test_ppm_from_canvas_single_pixel() {
        let canvas: Canvas = Canvas::new(1, 1);
        let result: PPMString = ppm_from_canvas(&canvas);
        let expected_result = "\
            P3\n\
            1 1\n\
            255\n\
            0 0 0\n
        ";
        assert!(result.eq(expected_result))
    }

    #[test]
    fn test_ppm_from_canvas_multiple_pixels() {
        let canvas: Canvas = Canvas::new(5, 3);
        let result: PPMString = ppm_from_canvas(&canvas);
        let expected_result = "\
            P3\n\
            5 3\n\
            255\n\
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n\
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n\
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n
        ";
        assert!(result.eq(expected_result))
    }

    #[test]
    fn test_ppm_from_edited_canvas() {
        let mut canvas: Canvas = Canvas::new(5, 3);
        let color1 = Color::new(1.5, 0.0, 0.0);
        let color2 = Color::new(0.0, 0.5, 0.0);
        let color3 = Color::new(-0.5, 0.0, 1.0);
        canvas[(0, 0)] = color1;
        canvas[(2, 1)] = color2;
        canvas[(4, 2)] = color3;
        let result: PPMString = ppm_from_canvas(&canvas);
        let expected_result = "\
            P3\n\
            5 3\n\
            255\n\
            255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n\
            0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 \n\
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n
        ";
        assert!(result.eq(expected_result))
    }

    #[test]
    fn test_ppm_multiline() {
        let mut canvas: Canvas = Canvas::new(10, 2);
        canvas.fill(Color::new(1.0, 0.8, 0.6));
        let result: PPMString = ppm_from_canvas(&canvas);
        let expected_result = "\
            P3\n\
            10 2\n\
            255\n\
            255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 \n\
            153 255 204 153 255 204 153 255 204 153 255 204 153 \n\
            255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 \n\
            153 255 204 153 255 204 153 255 204 153 255 204 153\n
        ";
        assert!(result.eq(expected_result))
    }
}

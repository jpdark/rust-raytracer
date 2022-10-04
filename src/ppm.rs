//! For saving canvas to .ppm file
//!
//!

use crate::canvas::Canvas;

/// String formatted as PPM
type PPMString = String;

/// Transform the pixels to continuous string for PPM file.
/// TODO: Refactor this into smaller units. Use Vec::chunks for instance...
fn canvas_pixels_to_string(canvas: &Canvas) -> String {
    let mut pixels_string = String::new();
    let mut current_length: usize = 0;
    let mut pixel_counter: usize = 0;
    for pixel in &canvas.pixels {
        for p in pixel.as_rgb8().as_array().iter() {
            let new_str = format!("{} ", p);
            current_length = current_length + new_str.len();
            if current_length > 68 {
                pixels_string.push_str("\n");
                current_length = 0;
            }
            pixels_string.push_str(new_str.as_str())
        }
        pixel_counter += 1;
        if pixel_counter == canvas.height {
            pixels_string.push_str("\n");
            current_length = 0;
            pixel_counter = 0;
        }
    }
    return pixels_string.trim_end().to_string();
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
        println!("{}", result);
        println!("{}", expected_result);
        assert!(result.eq(expected_result))
    }
}

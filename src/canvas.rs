//! Two dimensional grid of pixels of configurable size; a canvas.
//!
//!

use std::ops::{IndexMut, Index};

use crate::color::Color;

/// Represents a 2d Canvas
#[derive(Default)]
pub struct Canvas {
    /// Height
    pub height: usize,
    /// Width
    pub width: usize,
    /// Pixels
    pub pixels: Vec<Color<f32>>
}

impl Canvas {
    /// Construct a new instance of Canvas
    pub fn new(height: usize, width: usize) -> Self {
        let size: usize = width * height;
        let pixels: Vec<Color<f32>> = vec![Color(0.0, 0.0, 0.0); size];
        Self { height, width, pixels }
    }

    /// Get value at given row and column if index is in bounds.
    pub fn get(&self, row: usize, column: usize) -> Option<&Color<f32>> {
        self.get_index(row, column)
            .map(|index| &self.pixels[index])
    }

    /// Get mutable value at given row and column if index is in bounds.
    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut Color<f32>> {
        self.get_index(row, column)
            .map(move |index| &mut self.pixels[index])
    }

    /// Get index for row & column
    fn get_index(&self, row: usize, column: usize) -> Option<usize> {
        if row < self.height && column < self.width {
            Some(row * self.width + column)
        } else {
            None
        }
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = Color<f32>;

    fn index(&self, indices: (usize, usize)) -> &Self::Output {
        let (row, column) = indices;
        self.get(row, column).unwrap()
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, indices: (usize, usize)) -> &mut Self::Output {
        let (row, column) = indices;
        self.get_mut(row, column).unwrap()
    }
}

#[cfg(test)]
mod test_canvas {
    use crate::color::Color;

    use super::Canvas;

    #[test]
    fn create_canvas() {
        let canvas: Canvas = Canvas::new(10, 20);
        assert!(canvas.height == 10);
        assert!(canvas.width == 20);
        assert!(canvas.pixels.len() == 200);
        // Assert all pixels are zero
        for pix in canvas.pixels.iter() {
            assert!(*pix == Color(0.0, 0.0, 0.0))
        }
    }

    #[test]
    fn write_pixels_to_canvas() {
        let mut canvas: Canvas = Canvas::new(10, 20);
        let red: Color<f32> = Color(1.0, 0.0, 0.0);
        canvas[(2, 3)] = red;
    }
}

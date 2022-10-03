//! Color implemented as simple tuple.
//!
//!
use num::clamp;
use num::traits::{Float, Num};
use std::fmt;
use std::ops::{Add, Mul, Sub};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};

/// Represents a color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color<T> {
    /// Red
    pub r: T,
    /// Green
    pub g: T,
    /// Blue
    pub b: T,
}

impl<T> Color<T> {
    /// Convenience function for creating a new color.
    pub const fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }

    /// Return the r, g and b values as array.
    pub fn as_array(self) -> [T; 3] {
        return [self.r, self.g, self.b];
    }
}

impl Color<f32> {
    /// Return Color with rgb values as 8-bit integer.
    pub fn as_rgb8(self) -> Color<u8> {
        return Color::<u8>::new(
            clamp((self.r * 255.0) as u8, 0, 255),
            clamp((self.g * 255.0) as u8, 0, 255),
            clamp((self.b * 255.0) as u8, 0, 255),
        );
    }
}

impl<T> IntoIterator for Color<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new([self.r, self.b, self.g])
    }
}

impl<T: fmt::Display + Num + fmt::Debug> fmt::Display for Color<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.r, self.g, self.b)
    }
}

impl<T: AbsDiffEq + Num> AbsDiffEq for Color<T>
where
    T::Epsilon: Copy,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, rhs: &Self, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(&self.r, &rhs.r, epsilon)
            && T::abs_diff_eq(&self.g, &rhs.g, epsilon)
            && T::abs_diff_eq(&self.b, &rhs.b, epsilon)
    }
}

impl<T: RelativeEq + Num> RelativeEq for Color<T>
where
    T::Epsilon: Copy,
{
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(&self, rhs: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
        T::relative_eq(&self.r, &rhs.r, epsilon, max_relative)
            && T::relative_eq(&self.g, &rhs.g, epsilon, max_relative)
            && T::relative_eq(&self.b, &rhs.b, epsilon, max_relative)
    }
}

impl<T: UlpsEq + Num> UlpsEq for Color<T>
where
    T::Epsilon: Copy,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, rhs: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.r, &rhs.r, epsilon, max_ulps)
            && T::ulps_eq(&self.g, &rhs.g, epsilon, max_ulps)
            && T::ulps_eq(&self.b, &rhs.b, epsilon, max_ulps)
    }
}

impl<T: Num> Add<Color<T>> for Color<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl<T: Num> Sub<Color<T>> for Color<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl<T: Num + Copy> Mul<T> for Color<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}

impl<T: Num + Copy> Mul<Color<T>> for Color<T> {
    type Output = Self;

    fn mul(self, rhs: Color<T>) -> Self {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

#[cfg(test)]
mod test_color {
    use super::Color;

    #[test]
    fn add_colors() {
        let col1: Color<f32> = Color::new(0.9, 0.6, 0.75);
        let col2: Color<f32> = Color::new(0.7, 0.1, 0.25);
        let result: Color<f32> = col1 + col2;
        let expected_result: Color<f32> = Color::new(1.6, 0.7, 1.0);
        assert_relative_eq!(result, expected_result);
    }
    #[test]
    fn subtract_colors() {
        let col1: Color<f32> = Color::new(0.9, 0.6, 0.75);
        let col2: Color<f32> = Color::new(0.7, 0.1, 0.25);
        let result: Color<f32> = col1 - col2;
        let expected_result: Color<f32> = Color::new(0.2, 0.5, 0.5);
        assert_relative_eq!(result, expected_result);
    }

    #[test]
    fn multiply_color_by_scalar() {
        let col: Color<f32> = Color::new(0.2, 0.3, 0.4);
        let result: Color<f32> = col * 2.0;
        let expected_result: Color<f32> = Color::new(0.4, 0.6, 0.8);
        assert_relative_eq!(result, expected_result);
    }

    #[test]
    fn multiply_colors() {
        let col1: Color<f32> = Color::new(1.0, 0.2, 0.4);
        let col2: Color<f32> = Color::new(0.9, 1.0, 0.1);
        let result: Color<f32> = col1 * col2;
        let expected_result: Color<f32> = Color::new(0.9, 0.2, 0.04);
        assert_relative_eq!(result, expected_result)
    }
}

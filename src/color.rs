//! Color implemented as simple tuple.
//!
//!
use std::fmt;
use std::ops::{Add, Sub, Mul};
use num::traits::{Num};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};

/// Represents a color
#[derive(Debug, Clone, PartialEq)]
pub struct Color<T: Num>(pub T, pub T, pub T);

impl <T: fmt::Display + Num + fmt::Debug> fmt::Display for Color<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.0, self.1, self.2)
    }
}

impl <T: AbsDiffEq + Num> AbsDiffEq for Color<T> where T::Epsilon: Copy {
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, rhs: &Self, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(&self.0, &rhs.0, epsilon) &&
        T::abs_diff_eq(&self.1, &rhs.1, epsilon) &&
        T::abs_diff_eq(&self.2, &rhs.2, epsilon)
    }
}

impl <T: RelativeEq + Num> RelativeEq for Color<T> where T::Epsilon: Copy {
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(&self, rhs: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
        T::relative_eq(&self.0, &rhs.0, epsilon, max_relative) &&
        T::relative_eq(&self.1, &rhs.1, epsilon, max_relative) &&
        T::relative_eq(&self.2, &rhs.2, epsilon, max_relative)
    }
}

impl <T: UlpsEq + Num> UlpsEq for Color<T> where T::Epsilon: Copy {
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, rhs: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.0, &rhs.0, epsilon, max_ulps) &&
        T::ulps_eq(&self.1, &rhs.1, epsilon, max_ulps) &&
        T::ulps_eq(&self.2, &rhs.2, epsilon, max_ulps)
    }
}

impl <T: Num>Add<Color<T>> for Color<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl <T: Num> Sub<Color<T>> for Color<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl <T: Num + Copy> Mul<T> for Color<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self (
            self.0 * scalar,
            self.1 * scalar,
            self.2 * scalar
        )
    }
}

impl <T: Num + Copy> Mul<Color<T>> for Color<T> {
    type Output = Self;

    fn mul(self, rhs: Color<T>) -> Self {
        Self (
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2
        )
    }
}


#[cfg(test)]
mod test_color {
    use super::Color;

    #[test]
    fn add_colors() {
        let col1: Color<f32> = Color(0.9, 0.6, 0.75);
        let col2: Color<f32> = Color(0.7, 0.1, 0.25);
        let result: Color<f32> = col1 + col2;
        let expected_result: Color<f32> = Color(1.6, 0.7, 1.0);
        assert_relative_eq!(result, expected_result);
    }
    #[test]
    fn subtract_colors() {
        let col1: Color<f32> = Color(0.9, 0.6, 0.75);
        let col2: Color<f32> = Color(0.7, 0.1, 0.25);
        let result: Color<f32> = col1 - col2;
        let expected_result: Color<f32> = Color(0.2, 0.5, 0.5);
        assert_relative_eq!(result, expected_result);
    }

    #[test]
    fn multiply_color_by_scalar() {
        let col: Color<f32> = Color(0.2, 0.3, 0.4);
        let result: Color<f32> = col * 2.0;
        let expected_result: Color<f32> = Color(0.4, 0.6, 0.8);
        assert_relative_eq!(result, expected_result);
    }

    #[test]
    fn multiply_colors() {
        let col1: Color<f32> = Color(1.0, 0.2, 0.4);
        let col2: Color<f32> = Color(0.9, 1.0, 0.1);
        let result: Color<f32> = col1 * col2;
        let expected_result: Color<f32> = Color(0.9, 0.2, 0.04);
        assert_relative_eq!(result, expected_result)
    }
}

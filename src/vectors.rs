use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
use num::traits::{Float, Num};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};

#[derive(Debug, PartialEq)]
pub struct Vec3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: Float + Num> Vec3<T> {
    pub fn magnitude(&self) -> T {
        return self.norm().sqrt();
    }

    pub fn norm(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag
        }
    }
}

impl<T: fmt::Display + Num + fmt::Debug> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.x, self.y, self.z)
    }
}

impl<T: AbsDiffEq + Num> AbsDiffEq for Vec3<T> where T::Epsilon: Copy {
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, rhs: &Self, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(&self.x, &rhs.x, epsilon) &&
        T::abs_diff_eq(&self.y, &rhs.y, epsilon) &&
        T::abs_diff_eq(&self.z, &rhs.z, epsilon)
    }
}

impl<T: RelativeEq + Num> RelativeEq for Vec3<T> where T::Epsilon: Copy {
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(&self, rhs: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
        T::relative_eq(&self.x, &rhs.x, epsilon, max_relative) &&
        T::relative_eq(&self.y, &rhs.y, epsilon, max_relative) &&
        T::relative_eq(&self.z, &rhs.z, epsilon, max_relative)
    }
}

impl<T: UlpsEq + Num> UlpsEq for Vec3<T> where T::Epsilon: Copy {
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, rhs: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.x, &rhs.x, epsilon, max_ulps) &&
        T::ulps_eq(&self.y, &rhs.y, epsilon, max_ulps) &&
        T::ulps_eq(&self.z, &rhs.z, epsilon, max_ulps)
    }
}

impl<T: Num> Add<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl<T: Num> Sub<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<T: Num + Neg + Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl<T: Num + Copy> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

impl<T: Num + Copy> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, scalar: T) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_equality() {
        let vec1 = Vec3::<f64>{x:1.0, y:2.0, z:3.0};
        assert!(vec1 == vec1);
    }

    #[test]
    fn vector_inequality() {
        let vec1 = Vec3::<f64>{x:1.0, y:2.0, z:3.0};
        let vec2 = Vec3::<f64>{x:1.1, y:2.0, z:3.0};
        let vec3 = Vec3::<f64>{x:1.1, y:2.0, z:3.3};
        assert!(vec1 != vec2);
        assert!(vec1 != vec3);
    }

    #[test]
    fn add_vectors() {
        let vec1 = Vec3::<f64>{x:1.0, y:2.0, z:3.0};
        let vec2 = Vec3::<f64>{x:2.0, y:3.0, z:4.0};
        let mut result = vec1 + vec2;
        let expected_result = Vec3::<f64>{x:3.0, y:5.0, z:7.0};
        assert_relative_eq!(result, expected_result);

        let vec3 = Vec3::<f64>{x:1.0, y:2.0, z:3.0};
        let vec4 = Vec3::<f64>{x:3.0, y:4.0, z:5.0};
        result = vec3 + vec4;
        let expected_result = Vec3::<f64>{x:4.0, y:6.0, z:8.0};
        assert_relative_eq!(result, expected_result);
    }

    #[test]
    fn subtract_vectors() {
        let vec1 = Vec3::<f64>{x:1.0, y:2.0, z:3.0};
        let vec2 = Vec3::<f64>{x:2.0, y:3.0, z:4.0};
        let mut result = vec1 - vec2;
        let expected_result = Vec3::<f64>{x:-1.0, y:-1.0, z:-1.0};
        assert_relative_eq!(result, expected_result);

        let vec3 = Vec3::<f64>{x:0.0, y:0.0, z:0.0};
        let vec4 = Vec3::<f64>{x:1.0, y:-2.0, z:3.0};
        result = vec3 - vec4;
        let expected_result = Vec3::<f64>{x:-1.0, y:2.0, z:-3.0};
        assert_relative_eq!(result, expected_result);
    }

    #[test]
    fn negate_vector() {
        let vec1 = Vec3::<f64>{x:1.0, y: -2.0, z: 3.0};
        let result = -vec1;
        let expected_result = Vec3::<f64>{x:-1.0, y:2.0, z:-3.0};
        assert_relative_eq!(result, expected_result);
    }

    #[test]
    fn multiply_vector_by_scalar() {
        let vec1 = Vec3::<f64>{x:1.0, y:-2.0, z:3.0};
        let result = vec1 * 2.6;
        let expected_result = Vec3::<f64>{x:2.6, y:-5.2, z:7.8};
        assert_relative_eq!(result, expected_result)
    }

    #[test]
    fn multiply_vector_by_fraction() {
        let vec1 = Vec3::<f64>{x:1.0, y:-2.0, z:3.0};
        let result = vec1 * 0.5;
        let expected_result = Vec3::<f64>{x:0.5, y:-1.0, z:1.5};
        assert_relative_eq!(result, expected_result);
    }

    #[test]
    fn divide_vector_by_scalar() {
        let vec1 = Vec3::<f64>{x:1.0, y:-2.0, z:3.0};
        let result = vec1 / 2.0;
        let expected_result = Vec3::<f64>{x:0.5, y:-1.0, z:1.5};
        assert_relative_eq!(result, expected_result);
    }

    #[test]
    fn compute_vector_norm() {
        let vec1 = Vec3::<f64>{x:1.0, y:2.0, z:0.0};
        let expected_result = 5.0;
        let result = vec1.norm();
        assert_relative_eq!(result, expected_result);
    }

    #[test]
    fn compute_vector_magnitude() {
        let vec1 = Vec3::<f64>{x:1.0, y:0.0, z:0.0};
        assert_relative_eq!(vec1.magnitude(), 1.0);
        let vec2 = Vec3::<f64>{x:0.0, y:1.0, z:0.0};
        assert_relative_eq!(vec2.magnitude(), 1.0);
        let vec3 = Vec3::<f64>{x:0.0, y:0.0, z:1.0};
        assert_relative_eq!(vec3.magnitude(), 1.0);
        let vec4 = Vec3::<f64>{x:1.0, y:2.0, z:3.0};
        assert_relative_eq!(vec4.magnitude(), 14.0.sqrt());
        let vec5 = Vec3::<f64>{x:-1.0, y:-2.0, z:-3.0};
        assert_relative_eq!(vec5.magnitude(), 14.0.sqrt());
    }

    #[test]
    fn normalize_vector() {
        let vec1 = Vec3::<f64>{x:4.0, y:0.0, z:0.0};
        let expected_result = Vec3::<f64>{x:1.0, y:0.0, z:0.0};
        assert_relative_eq!(vec1.normalize(), expected_result);

        let vec1 = Vec3::<f64>{x:1.0, y:2.0, z:3.0};
        let expected_result = Vec3::<f64>{x:1.0/14.0.sqrt(), y:2.0/14.0.sqrt(), z:3.0/14.0.sqrt()};
        assert_relative_eq!(vec1.normalize(), expected_result);
    }
}

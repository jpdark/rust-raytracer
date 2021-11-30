use float_cmp::ApproxEq;
use float_cmp::F64Margin;

pub fn point(x: f64, y: f64, z: f64) -> (f64, f64, f64, f64) {
    return (x, y, z, 1.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> (f64, f64, f64, f64) {
    return (x, y, z, 0.0)
}

pub fn is_vector(tup: (f64, f64, f64, f64)) -> bool {
    let (_x, _y, _z, w) = tup;
    return w.approx_eq(0.0, F64Margin::default());
}

pub fn is_point(tup: (f64, f64, f64, f64)) -> bool {
    let (_x, _y, _z, w) = tup;
    return w.approx_eq(1.0, F64Margin::default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_vector() {
        let vec = (1.0, 2.0, 3.0, 0.0);
        assert!(is_vector(vec));
        assert!(!is_point(vec));
    }

    #[test]
    fn tuple_is_point() {
        let pt: (f64, f64, f64, f64) = (1.0, 2.0, 3.0, 1.0);
        assert!(!is_vector(pt));
        assert!(is_point(pt));
    }

    #[test]
    fn create_vector_tuple() {
        assert_eq!(vector(4.0, -4.0, 3.0), (4.0, -4.0, 3.0, 0.0))
    }

    #[test]
    fn create_point_tuple() {
        assert_eq!(point(4.0, -4.0, 3.0), (4.0, -4.0, 3.0, 1.0))
    }
}

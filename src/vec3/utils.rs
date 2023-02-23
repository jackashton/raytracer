use crate::vec3::Point3;
use rand::Rng;

pub fn random_in_unit_sphere() -> Point3<f64> {
    let mut rng = rand::thread_rng();
    let (u, x, y, z): (f64, f64, f64, f64) = (
        rng.gen(),
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
    );
    let mag = (x * x + y * y + z * z).sqrt();
    (Point3::new(x, y, z) * u.cbrt()) / mag
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_in_unit_sphere_in_expected_range() {
        let p = random_in_unit_sphere();
        let range = -1.0..1.0;
        assert!(range.contains(&p.x));
        assert!(range.contains(&p.y));
        assert!(range.contains(&p.z));
    }
}

use nalgebra::Vector3;

pub trait Vector3ToBasis: Sized {
    fn orthonormal_basis(&self) -> (Self, Self, Self);
}

impl Vector3ToBasis for Vector3<f32> {
    fn orthonormal_basis(&self) -> (Self, Self, Self) {
        let b3 = self.clone();
        let cross_dir = if b3[0] < 0.5 {
            Vector3::new(1.0, 0.0, 0.0)
        } else {
            Vector3::new(0.0, 1.0, 0.0)
        };
        let b1 = b3.cross(&cross_dir).normalize();
        let b2 = b3.cross(&b1);
        (b1, b2, b3)
    }
}

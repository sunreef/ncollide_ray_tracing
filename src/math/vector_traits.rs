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

pub trait ToLocal: Sized {
    fn to_local(&self, normal: &Self) -> Self;
}

impl ToLocal for Vector3<f32> {
    fn to_local(&self, normal: &Self) -> Self {
        let (b1, b2, b3) = normal.orthonormal_basis();
        Vector3::new(self.dot(&b1), self.dot(&b2), self.dot(&b3))
    }
}

pub trait ToGlobal: Sized {
    fn to_global(&self, normal: &Self) -> Self;
}

impl ToGlobal for Vector3<f32> {
    fn to_global(&self, normal: &Self) -> Self {
        let (b1, b2, b3) = normal.orthonormal_basis();
        self[0] * b1 + self[1] * b2 + self[2] * b3
    }
}

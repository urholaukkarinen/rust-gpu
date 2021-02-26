/// Abstract trait representing a SPIR-V vector type.
pub trait Vector<T: crate::scalar::Scalar>:
    crate::sealed::Sealed + Default
{
}

impl Vector<f32> for glam::Vec2 {}
impl Vector<f32> for glam::Vec3 {}
impl Vector<f32> for glam::Vec3A {}
impl Vector<f32> for glam::Vec4 {}

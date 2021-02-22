/// Abstract trait representing a SPIR-V floating point type.
pub trait Float: num_traits::Float + crate::sealed::Sealed + Default {
    const WIDTH: usize;
}

impl Float for f32 {
    const WIDTH: usize = 32;
}

impl Float for f64 {
    const WIDTH: usize = 64;
}

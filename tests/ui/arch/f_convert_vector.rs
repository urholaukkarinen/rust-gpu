// build-pass

#[spirv(fragment)]
pub fn main() {
    let vec = glam::Vec2::new(1.0, 2.0);
    let dvec = glam::DVec2::new(1.0f64, 2.0);

    unsafe {
        assert!(dvec == spirv_std::arch::f_convert_vector(vec));
    }
}

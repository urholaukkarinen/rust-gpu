// build-pass

#[spirv(fragment)]
pub fn main() {
    let value = 5.0f32;
    let dvec = glam::DVec2::new(1.0, 2.0);
    let vec = glam::Vec2::new(1.0, 2.0);
    let too_big = glam::Vec2::new(f32::MAX, f32::MAX);
    let inf = glam::Vec2::new(f32::INFINITY, f32::INFINITY);

    assert!(value == spirv_std::arch::quantize_to_f16(value));
    assert!(vec == spirv_std::arch::quantize_to_f16_vector(vec));
    //assert!(vec == spirv_std::arch::quantize_to_f16_vector(dvec));
    assert!(inf == spirv_std::arch::quantize_to_f16_vector(too_big));
}

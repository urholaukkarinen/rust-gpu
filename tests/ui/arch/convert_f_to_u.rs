// build-pass

#[spirv(fragment)]
pub fn main() {
    let vec = glam::Vec2::new(1.0, 2.0);
    let uvec = glam::UVec2::new(1, 2);

    unsafe {
        assert!(uvec == spirv_std::arch::convert_f_to_u_vector(vec));
    }
}

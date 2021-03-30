// build-pass

#[spirv(fragment)]
pub fn main() {
    let vec = glam::Vec2::new(1.0, 2.0);
    let ivec = glam::IVec2::new(1, 2);

    unsafe {
        assert!(ivec == spirv_std::arch::convert_f_to_s_vector(vec));
    }
}

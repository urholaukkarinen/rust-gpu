// build-pass

#[spirv(ray_generation_khr)]
pub fn main(#[spirv(ray_payload_khr)] payload: &mut glam::Vec3) {
    unsafe {
        let handle = spirv_std::arch::convert_u_to_acceleration_structure_khr(0xffff_ffff);
        asm!(r#"OpExtension "SPV_KHR_ray_tracing""#);
        asm!("OpCapability RayTracingKHR");
        spirv_std::arch::trace_ray_khr(
            handle,
            0,
            0,
            0,
            0,
            0,
            glam::vec3(1.0, 2.0, 3.0),
            0.5,
            glam::vec3(3.0, 2.0, 1.0),
            1.0,
            payload,
        );
    }
}

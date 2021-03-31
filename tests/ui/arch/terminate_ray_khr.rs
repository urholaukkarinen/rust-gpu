// build-pass

#[spirv(any_hit_khr)]
pub fn main() {
    unsafe {
        asm!(r#"OpExtension "SPV_KHR_ray_tracing""#);
        asm!("OpCapability RayTracingKHR");
        spirv_std::arch::terminate_ray_khr();
    }
}

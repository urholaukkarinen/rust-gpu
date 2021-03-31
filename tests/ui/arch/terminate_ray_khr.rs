// build-pass

use spirv_std as _;

#[spirv(any_hit_khr)]
pub fn main() {
    unsafe {
        asm!(r#"OpExtension "SPV_KHR_ray_tracing""#);
        asm!("OpCapability RayTracingKHR", "OpTerminateRayKHR");
    }
}


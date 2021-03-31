use crate::{ray_tracing::*, vector::Vector};

/// Converts a 64-bit integer into an [`AccelerationStructureKHR`].
/// # Safety
/// The 64-bit integer must point to a valid acceleration structure.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpConvertUToAccelerationStructureKHR")]
#[inline]
pub unsafe fn convert_u_to_acceleration_structure_khr(id: u64) -> AccelerationStructureKhr {
    let mut result = AccelerationStructureKhr { _private: () };

    asm! {
        "%ret = OpTypeAccelerationStructureKHR",
        "%result = OpConvertUToAccelerationStructureKHR %ret {id}",
        "OpStore {result} %result",
        id = in(reg) id,
        result = in(reg) &mut result,
    }

    result
}

/// Trace a ray into the acceleration structure.
///
/// - `structure` is the descriptor for the acceleration structure to trace into.
/// - `ray_flags` contains one or more of the Ray Flag values.
/// - `cull_mask` is the mask to test against the instance mask. Only the 8
///   least-significant bits of are used by this instruction - other bits
///   are ignored.
/// - `sbt_offset` and `sbt_stride` control indexing into the SBT (Shader
///   Binding Table) for hit shaders called from this trace. Only the 4
///   least-significant bits of `sbt_offset` and `sbt_stride` are used by this
///   instruction - other bits are ignored.
/// - `miss_index` is the index of the miss shader to be called from this
///   trace call. Only the 16 least-significant bits are used by this
///   instruction - other bits are ignored.
/// - `ray_origin`, `ray_tmin`, `ray_direction`, and `ray_tmax` control the
///   basic parameters of the ray to be traced.
///
/// - `payload` is a pointer to the ray payload structure to use for this trace.
///   `payload` must have a storage class of `ray_payload_khr`
///   or `incoming_ray_payload_khr`.
///
/// This instruction is allowed only in `ray_generation_khr`, `closest_hit_khr` and
/// `miss_khr` execution models.
///
/// This instruction is a shader call instruction which may invoke shaders with
/// the `intersection_khr`, `any_hit_khr`, `closest_hit_khr`, and `miss_khr`
/// execution models.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpTraceRayKHR")]
#[inline]
pub unsafe fn trace_ray_khr<T>(
    acceleration_structure: AccelerationStructureKhr,
    ray_flags: i32,
    cull_mask: i32,
    sbt_offset: i32,
    sbt_stride: i32,
    miss_index: i32,
    ray_origin: impl Vector<f32, 3>,
    ray_tmin: f32,
    ray_direction: impl Vector<f32, 3>,
    ray_tmax: f32,
    payload: &mut T,
) {
    asm! {
        "OpTraceRayKHR \
            {acceleration_structure} \
            {ray_flags} \
            {cull_mask} \
            {sbt_offset} \
            {sbt_stride} \
            {miss_index} \
            {ray_origin} \
            {ray_tmin} \
            {ray_direction} \
            {ray_tmax} \
            {payload}",
        acceleration_structure = in(reg) &acceleration_structure,
        ray_flags = in(reg) ray_flags,
        cull_mask = in(reg) cull_mask,
        sbt_offset = in(reg) sbt_offset,
        sbt_stride = in(reg) sbt_stride,
        miss_index = in(reg) miss_index,
        ray_origin = in(reg) &ray_origin,
        ray_tmin = in(reg) ray_tmin,
        ray_direction = in(reg) &ray_direction,
        ray_tmax = in(reg) ray_tmax,
        payload = in(reg) payload,
    }
}

/// Reports an intersection back to the traversal infrastructure.
///
/// If the intersection occurred within the current ray interval, the
/// intersection confirmation is performed (see the API specification for more
/// details). If the value of Hit falls outside the current ray interval, the
/// hit is rejected.
///
/// Returns True if the hit was accepted by the ray interval and the intersection was confirmed. Returns False otherwise.
///
/// - `hit` is the floating point parametric value along ray for the intersection.
/// - `hit_kind` is the integer hit kind reported back to other shaders and
///   accessible by the `hit kind` builtin.
///
/// This instruction is allowed only in IntersectionKHR execution model.
///
/// This instruction is a shader call instruction which may invoke shaders with
/// the `any_hit_khr` execution model.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpReportIntersectionKHR")]
#[inline]
pub unsafe fn report_intersection_khr(hit: f32, hit_kind: u32) -> bool {
    let mut result: u8 = 0;

    asm! {
        "%bool = OpTypeBool",
        "%u8 = OpTypeInt 8 0",
        "%result = OpReportIntersectionKHR %u8 {hit} {hit_kind}",
        "OpStore {result} %result",
        result = in(reg) &mut result,
        hit = in(reg) hit,
        hit_kind = in(reg) hit_kind,
    };

    result != 0
}

/// Ignores the current potential intersection, terminating the invocation that
/// executes it, and continues the ray traversal.  This instruction is allowed
/// only in `any_hit_khr` execution model.  This instruction must be the last
/// instruction in a block.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpIgnoreIntersectionKHR")]
#[inline]
pub unsafe fn ignore_intersection_khr() {
    asm!("OpIgnoreIntersectionKHR", "%unused = OpLabel")
}

/// Terminates the invocation that executes it, stops the ray traversal, accepts
/// the current hit, and invokes the `closest_hit_khr` execution model
/// (if active).  This instruction is allowed only in the `any_hit_khr`
/// execution model.  This instruction must be the last instruction in a block.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpTerminateRayKHR")]
#[inline]
pub unsafe fn terminate_ray_khr() {
    asm!("OpTerminateRayKHR", "%unused = OpLabel")
}

/// Invoke a callable shader.
///
/// - `INDEX` is the index into the SBT table to select callable shader
///   to execute.
/// - `data` is a pointer to the callable data to pass into the called shader.
///   `data` must have a storage class of `callable_data_khr`
///   or `incoming_callable_data_khr`.
///
/// This instruction is allowed only in `ray_generation_khr`, `closest_hit_khr`,
/// `miss_khr` and `callable_khr` execution models.
///
/// This instruction is a shader call instruction which will invoke a shader
/// with the `callable_khr` execution model.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpExecuteCallableKHR")]
#[inline]
pub unsafe fn execute_callable_khr<T, const ID: usize>(data: &T) {
    asm! {
        "OpExecuteCallableKHR {id} {data}",
        id = const ID,
        data = in(reg) data,
    };
}

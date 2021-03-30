use crate::{
    float::Float,
    integer::{SignedInteger, UnsignedInteger},
    vector::Vector,
};

/// Convert value numerically from floating point to a unsigned integer, with
/// rounding towards 0.0.
/// # Safety
/// Behavior is undefined if Result Type is not wide enough to hold the converted value.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpConvertFToU")]
#[inline]
pub unsafe fn convert_f_to_u_vector<F, U, V, const N: usize>(value: impl Vector<F, N>) -> V
where
    F: Float,
    U: UnsignedInteger,
    V: Vector<U, N>,
{
    let mut result = V::default();

    asm! {
        "%int_type = OpTypeInt {width} 0",
        "%vec_type = OpTypeVector %int_type {length}",
        "%value = OpLoad _ {value}",
        "%result = OpConvertFToU %vec_type %value",
        "OpStore {result} %result",
        value = in(reg) &value,
        width = const U::WIDTH,
        length = const N,
        result = in(reg) &mut result,
    }

    result
}

/// Convert value numerically from floating point to a signed integer, with
/// rounding towards 0.0.
/// # Safety
/// Behavior is undefined if Result Type is not wide enough to hold the converted value.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpConvertFToS")]
#[inline]
pub unsafe fn convert_f_to_s_vector<F, S, V, const N: usize>(value: impl Vector<F, N>) -> V
where
    F: Float,
    S: SignedInteger,
    V: Vector<S, N>,
{
    let mut result = V::default();

    asm! {
        "%int_type = OpTypeInt {width} 1",
        "%vec_type = OpTypeVector %int_type {length}",
        "%value = OpLoad _ {value}",
        "%result = OpConvertFToS %vec_type %value",
        "OpStore {result} %result",
        value = in(reg) &value,
        width = const S::WIDTH,
        length = const N,
        result = in(reg) &mut result,
    }

    result
}

/// Convert value numerically from a signed integer to a floating point number.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpConvertSToF")]
#[inline]
pub fn convert_s_to_f_vector<F, S, V, const N: usize>(value: impl Vector<S, N>) -> V
where
    F: Float,
    S: SignedInteger,
    V: Vector<F, N>,
{
    let mut result = V::default();

    unsafe {
        asm! {
            "%float_type = OpTypeFloat {width}",
            "%vec_type = OpTypeVector %float_type {length}",
            "%value = OpLoad _ {value}",
            "%result = OpConvertSToF %vec_type %value",
            "OpStore {result} %result",
            value = in(reg) &value,
            width = const S::WIDTH,
            length = const N,
            result = in(reg) &mut result,
        }
    }

    result
}

/// Convert value numerically from a unsigned integer to a floating point number.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpConvertUToF")]
#[inline]
pub fn convert_u_to_f_vector<F, U, V, const N: usize>(value: impl Vector<U, N>) -> V
where
    F: Float,
    U: UnsignedInteger,
    V: Vector<F, N>,
{
    let mut result = V::default();

    unsafe {
        asm! {
            "%float_type = OpTypeFloat {width}",
            "%vec_type = OpTypeVector %float_type {length}",
            "%value = OpLoad _ {value}",
            "%result = OpConvertUToF %vec_type %value",
            "OpStore {result} %result",
            value = in(reg) &value,
            width = const F::WIDTH,
            length = const N,
            result = in(reg) &mut result,
        }
    }

    result
}

/// Convert value numerically from one floating-point width to another width.
/// The width of the new floating point type must not match the original.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpFConvert")]
#[inline]
pub fn f_convert_vector<F, F2, V, const N: usize>(value: impl Vector<F, N>) -> V
where
    F: Float,
    F2: Float,
    V: Vector<F2, N>,
{
    let mut result = V::default();

    unsafe {
        asm! {
            "%float_type = OpTypeFloat {width}",
            "%vec_type = OpTypeVector %float_type {length}",
            "%value = OpLoad _ {value}",
            "%result = OpFConvert %vec_type %value",
            "OpStore {result} %result",
            value = in(reg) &value,
            width = const F2::WIDTH,
            length = const N,
            result = in(reg) &mut result,
        }
    }

    result
}

/// Quantize a floating-point value to what is expressible by a 16-bit
/// floating-point value.
///
/// - If `component` is infinity, the result is the same infinity.
/// - If `component` is a NaN, the result is a NaN, but not necessarily the
///   same NaN.
/// - If `component` is positive with a magnitude too large to represent as a
///   16-bit floating-point value, the result is positive infinity.
/// - If `component` is negative with a magnitude too large to represent as a
///   16-bit floating-point value, the result is negative infinity.
/// - If the magnitude of `component` is too small to represent as a normalized
///   16-bit floating-point value, the result may be either +0 or -0.
///
/// The `RelaxedPrecision` Decoration has no effect on this instruction.
#[spirv_std_macros::vectorized]
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpQuantizeToF16")]
#[inline]
pub fn quantize_to_f16<F>(component: F) -> f32
where
    F: Float,
{
    let mut result = Default::default();

    unsafe {
        asm! {
            "%value = OpLoad _ {value}",
            "%result = OpQuantizeToF16 _ %value",
            "OpStore {result} %result",
            value = in(reg) &component,
            result = in(reg) &mut result,
        }
    }

    result
}

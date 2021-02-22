//! SPIR-V Instrinics
//!
//! This module is intended as a low level abstraction over SPIR-V instructions.
//! These functions will typically map to a single instruction, and will perform
//! no additional safety checks beyond type-checking.
use crate::{float::Float, scalar::Scalar, vector::Vector};

/// Result is true if any component of `vector` is true, otherwise result is
/// false.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpAny")]
#[inline]
pub fn any<V: Vector<bool, N>, const N: usize>(vector: V) -> bool {
    let mut result = false;

    unsafe {
        asm! {
            // Types & Constants
            "%bool = OpTypeBool",
            "%uchar = OpTypeInt 8 0",
            "%uchar_0 = OpConstant %uchar 0",
            "%uchar_1 = OpConstant %uchar 1",
            "%glam_vec_type = OpTypeVector %uchar {len}",
            "%bool_vec_type = OpTypeVector %bool {len}",
            "%false_vec = OpConstantNull %glam_vec_type",
            // Code
            "%vector = OpLoad %glam_vec_type {vector}",
            "%bool_vec = OpINotEqual %bool_vec_type %vector %false_vec",
            "%result = OpAny %bool %bool_vec",
            "%boolean = OpSelect %uchar %result %uchar_1 %uchar_0",
            "OpStore {result} %boolean",
            vector = in(reg) &vector,
            len = const N,
            result = in(reg) &mut result
        }
    }

    result
}

/// Result is true if all components of `vector` is true, otherwise result is
/// false.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpAll")]
#[inline]
pub fn all<V: Vector<bool, N>, const N: usize>(vector: V) -> bool {
    let mut result = false;

    unsafe {
        asm! {
            // Types & Constants
            "%bool = OpTypeBool",
            "%uchar = OpTypeInt 8 0",
            "%uchar_0 = OpConstant %uchar 0",
            "%uchar_1 = OpConstant %uchar 1",
            "%glam_vec_type = OpTypeVector %uchar {len}",
            "%bool_vec_type = OpTypeVector %bool {len}",
            "%false_vec = OpConstantNull %glam_vec_type",
            // Code
            "%vector = OpLoad %glam_vec_type {vector}",
            "%bool_vec = OpINotEqual %bool_vec_type %vector %false_vec",
            "%result = OpAll %bool %bool_vec",
            "%boolean = OpSelect %uchar %result %uchar_1 %uchar_0",
            "OpStore {element} %boolean",
            vector = in(reg) &vector,
            len = const N,
            element = in(reg) &mut result
        }
    }

    result
}

/// Result is true if `component` is an IEEE `NaN`, otherwise result is false.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpIsNan")]
pub fn is_nan<F: Float>(component: F) -> bool {
    let mut result = false;
    unsafe {
        asm! {
            // Types and Constants
            "%float = OpTypeFloat {width}",
            "%bool = OpTypeBool",
            "%uchar = OpTypeInt 8 0",
            "%uchar_0 = OpConstant %uchar 0",
            "%uchar_1 = OpConstant %uchar 1",
            // Code
            "%component = OpLoad %float {component}",
            "%result = OpIsNan %bool %component",
            "%boolean = OpSelect %uchar %result  %uchar_1 %uchar_0",
            "OpStore {result} %boolean",
            component = in(reg) &component,
            width = const F::WIDTH,
            result = in(reg) &mut result
        }

        result
    }
}

/// Result is true if `vector` is an IEEE `NaN`, otherwise result is false.
/// Results are computed per component.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpIsNan")]
#[inline]
pub fn is_nan_vector<F, V, V2, const N: usize>(vector: V) -> V2
where
    F: Float,
    V: Vector<F, N>,
    V2: Vector<bool, N>,
{
    let mut result = V2::default();

    unsafe {
        asm! {
            // Types and Constants
            "%float = OpTypeFloat {width}",
            "%bool = OpTypeBool",
            "%uchar = OpTypeInt 8 0",
            "%uchar_0 = OpConstant %uchar 0",
            "%uchar_1 = OpConstant %uchar 1",
            "%bool_vec_type = OpTypeVector %bool {len}",
            "%glam_vec_type = OpTypeVector %uchar {len}",
            "%vector_type = OpTypeVector %float {len}",
            // Code
            "%vector = OpLoad %vector_type {vector}",
            "%result = OpIsNan %bool_vec_type %vector",
            "OpStore {result} %result",
            vector = in(reg) &vector,
            width = const F::WIDTH,
            len = const N,
            result = in(reg) &mut result
        }
    }

    result
}

/// Result is true if `component` is an IEEE `Inf`, otherwise result is false.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpIsInf")]
#[inline]
pub fn is_inf<F: Float>(component: F) -> bool {
    let mut result = false;

    unsafe {
        asm! {
            "%component = OpLoad _ {component}",
            "%float = OpTypeFloat {len}",
            "%result = OpIsInf %float %component",
            "OpStore {element} %result",
            component = in(reg) &component,
            len = const F::WIDTH,
            element = in(reg) &mut result
        }
    }

    result
}

/// Result is true if `vector` is an IEEE `Inf`, otherwise result is false.
/// Results are computed per component.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpIsInf")]
#[inline]
pub fn is_inf_vector<F, V, V2, const N: usize>(vector: V) -> V2
where
    F: Float,
    V: Vector<F, N>,
    V2: Vector<bool, N>,
{
    let mut result = V2::default();

    unsafe {
        asm! {
            "%vector = OpLoad _ {vector}",
            "%bool = OpTypeBool",
            "%vector_type = OpTypeVector %bool {len}",
            "%result = OpIsInf %vector_type %vector",
            "OpStore {element} %result",
            vector = in(reg) &vector,
            len = const N,
            element = in(reg) &mut result
        }
    }

    result
}

/// Result is true if `component` is an IEEE finite number, otherwise result
/// is false. Requires `Kernel` capabilities.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpIsFinite")]
#[inline]
pub fn is_finite<F: Float>(component: F) -> bool {
    let mut result = false;

    unsafe {
        asm! {
            "%component = OpLoad _ {component}",
            "%float = OpTypeFloat {len}",
            "%result = OpIsFinite %float %component",
            "OpStore {element} %result",
            component = in(reg) &component,
            len = const F::WIDTH,
            element = in(reg) &mut result
        }
    }

    result
}

/// Result is true if `component` is an IEEE finite number, otherwise result
/// is false. Results are computed per component. Requires
/// `Kernel` capabilities.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpIsFinite")]
#[inline]
pub fn is_finite_vector<F, V, V2, const N: usize>(vector: V) -> V2
where
    F: Float,
    V: Vector<F, N>,
    V2: Vector<bool, N>,
{
    let mut result = V2::default();

    unsafe {
        asm! {
            "%vector = OpLoad _ {vector}",
            "%result = OpIsFinite typeof*{vector} %vector",
            "OpStore {element} %result",
            vector = in(reg) &vector,
            element = in(reg) &mut result
        }
    }

    result
}

/// Result is true if `component` is an IEEE normal number, otherwise result
/// is false. Requires `Kernel` capabilities.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpIsNormal")]
#[inline]
pub fn is_normal<F: Float>(component: F) -> bool {
    let mut result = false;

    unsafe {
        asm! {
            "%component = OpLoad _ {component}",
            "%float = OpTypeFloat {len}",
            "%result = OpIsNormal %float %component",
            "OpStore {element} %result",
            component = in(reg) &component,
            len = const F::WIDTH,
            element = in(reg) &mut result
        }
    }

    result
}

/// Result is true if `component` is an IEEE normal number, otherwise result
/// is false. Results are computed per component. Requires
/// `Kernel` capabilities.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpIsNormal")]
#[inline]
pub fn is_normal_vector<F, V, V2, const N: usize>(vector: V) -> V2
where
    F: Float,
    V: Vector<F, N>,
    V2: Vector<bool, N>,
{
    let mut result = V2::default();

    unsafe {
        asm! {
            "%vector = OpLoad _ {vector}",
            "%result = OpIsNormal typeof*{vector} %vector",
            "OpStore {element} %result",
            vector = in(reg) &vector,
            element = in(reg) &mut result
        }
    }

    result
}

/// Result is true if `component` has its sign bit set, otherwise result
/// is false. Requires `Kernel` capabilities.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpSignBitSet")]
#[inline]
pub fn sign_bit_set<F: Float>(component: F) -> bool {
    let mut result = false;

    unsafe {
        asm! {
            "%component = OpLoad _ {component}",
            "%result = OpSignBitSet _ %component",
            "OpStore {element} %result",
            component = in(reg) &component,
            element = in(reg) &mut result
        }
    }

    result
}

/// Result is true if component in `vector` has its sign bit set, otherwise
/// result is false. Results are computed per component. Requires
/// `Kernel` capabilities.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpSignBitSet")]
#[inline]
pub fn sign_bit_set_vector<F, V, V2, const N: usize>(vector: V) -> V2
where
    F: Float,
    V: Vector<F, N>,
    V2: Vector<bool, N>,
{
    let mut result = V2::default();

    unsafe {
        asm! {
            "%vector = OpLoad _ {vector}",
            "%result = OpSignBitSet _ %vector",
            "OpStore {element} %result",
            vector = in(reg) &vector,
            element = in(reg) &mut result
        }
    }

    result
}

/// Extract a single, dynamically selected, component of a vector.
///
/// # Safety
/// Behavior is undefined if `index`’s value is greater than or equal to the
/// number of components in `vector`.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpVectorExtractDynamic")]
#[inline]
pub unsafe fn vector_extract_dynamic<T: Scalar, V: Vector<T, N>, const N: usize>(
    vector: V,
    index: usize,
) -> T {
    let mut result = T::default();

    asm! {
        "%vector = OpLoad _ {vector}",
        "%element = OpVectorExtractDynamic _ %vector {index}",
        "OpStore {element} %element",
        vector = in(reg) &vector,
        index = in(reg) index,
        element = in(reg) &mut result
    }

    result
}

/// Make a copy of a vector, with a single, variably selected,
/// component modified.
///
/// # Safety
/// Behavior is undefined if `index`’s value is greater than or equal to the
/// number of components in `vector`.
#[spirv_std_macros::gpu_only]
#[doc(alias = "OpVectorInsertDynamic")]
#[inline]
pub unsafe fn vector_insert_dynamic<T: Scalar, V: Vector<T, N>, const N: usize>(
    vector: V,
    index: usize,
    element: T,
) -> V {
    let mut result = V::default();

    asm! {
        "%vector = OpLoad _ {vector}",
        "%element = OpLoad _ {element}",
        "%new_vector = OpVectorInsertDynamic _ %vector %element {index}",
        "OpStore {result} %new_vector",
        vector = in(reg) &vector,
        index = in(reg) index,
        element = in(reg) &element,
        result = in(reg) &mut result,
    }

    result
}

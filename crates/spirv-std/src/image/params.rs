use crate::{scalar::Scalar, vector::Vector};
use super::{Dimensionality, ImageFormat, SampledType};

/// Marker trait for arguments that accept single scalar values or vectors
/// of scalars.
pub trait SampleType<const FORMAT: ImageFormat> {}
impl SampleType<{ ImageFormat::Unknown(SampledType::Integer { signed: true, width: 8 }) }> for i8 {}
impl SampleType<{ ImageFormat::Unknown(SampledType::Integer { signed: true, width: 16 }) }> for i16 {}
impl SampleType<{ ImageFormat::Unknown(SampledType::Integer { signed: true, width: 32 }) }> for i32 {}
impl SampleType<{ ImageFormat::Unknown(SampledType::Integer { signed: false, width: 8 }) }> for u8 {}
impl SampleType<{ ImageFormat::Unknown(SampledType::Integer { signed: false, width: 16 }) }> for u16 {}
impl SampleType<{ ImageFormat::Unknown(SampledType::Integer { signed: false, width: 32 }) }> for u32 {}
impl SampleType<{ ImageFormat::Unknown(SampledType::Float { width: 32 }) }> for f32 {}
impl SampleType<{ ImageFormat::Rgba32i }> for i32 {}

/// Marker trait for arguments that accept single scalar values or vectors
/// of scalars.
pub trait ImageCoordinate<T, const DIM: Dimensionality> {}

impl<S: Scalar> ImageCoordinate<S, { Dimensionality::OneD }> for S {}
impl<S: Scalar> ImageCoordinate<S, { Dimensionality::Buffer }> for S {}
impl<V: Vector<S, 2>, S: Scalar> ImageCoordinate<S, { Dimensionality::TwoD }> for V {}
impl<V: Vector<S, 2>, S: Scalar> ImageCoordinate<S, { Dimensionality::Rect }> for V {}
impl<V: Vector<S, 2>, S: Scalar> ImageCoordinate<S, { Dimensionality::Cube }> for V {}
impl<V: Vector<S, 3>, S: Scalar> ImageCoordinate<S, { Dimensionality::ThreeD }> for V {}

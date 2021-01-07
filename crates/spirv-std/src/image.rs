//! Image types

mod params;

pub use params::{
    AccessQualifier, Arrayed, Dimensionality, ImageCoordinate, ImageDepth, ImageFormat,
    Multisampled, Sampled,
};

use crate::{scalar::Scalar, vector::Vector};

macro_rules! basic_image_type {
    ($($(#[$($meta:meta)+])* $dim:path => $name:ident),+ $(,)?) => {
        $(
            $(#[$($meta)+])*
            pub type $name = Image<
            f32,
            { $dim },
            { ImageDepth::Unknown },
            { Arrayed::False },
            { Multisampled::False },
            { Sampled::Unknown },
            { ImageFormat::Unknown },
            { None }
            >;
        )+
    }
}

basic_image_type! {
    /// A convenience type alias for a one dimensional image.
    Dimensionality::OneD => Image1d,
    /// A convenience type alias for a two dimensional image.
    Dimensionality::TwoD => Image2d,
    /// A convenience type alias for a three dimensional image.
    Dimensionality::ThreeD => Image3d,
    /// A convenience type alias for a cube buffer image.
    Dimensionality::Cube => ImageCube,
    /// A convenience type alias for a rectangle buffer image.
    Dimensionality::Rect => ImageRect,
    /// A convenience type alias for a buffer image.
    Dimensionality::Buffer => ImageBuffer,
}

/// Types which can be sampled from an image.
pub trait SampledType: crate::sealed::Sealed {}
impl<I: crate::number::Number> SampledType for I {}

macro_rules! shared_methods {
    (impl Image<$(Sampled::$typ:ident),+> { $($tree:tt)* }) => {
        shared_methods! {
            @impl<{
                TYPE: SampledType + Scalar,
                const DIM: Dimensionality,
                const DEPTH: ImageDepth,
                const ARRAYED: Arrayed,
                const MULTISAMPLED: Multisampled,
                const FORMAT: ImageFormat,
                const ACCESS_QUALIFIER: Option<AccessQualifier>,
        }> $(Image<TYPE, DIM, DEPTH, ARRAYED, MULTISAMPLED, { Sampled::$typ }, FORMAT, ACCESS_QUALIFIER>),+
            { { $($tree)* } }
        }
    };

    (impl $($types:ty),+ { $($tree:tt)* }) => {
        shared_methods! { @impl $($types),+ { { $($tree)* } } }
    };

    (@impl<{ $bounds:tt }> $($types:ty),+ {$tree:tt}) => {
        $(impl<$bounds> $types $tree)+
    };
}

/// An opaque image type. Corresponds to `OpTypeImage`.
#[allow(unused_attributes)]
#[spirv(image)]
#[derive(Copy, Clone)]
pub struct Image<
    TYPE: SampledType,
    const DIM: Dimensionality,
    const DEPTH: ImageDepth,
    const ARRAYED: Arrayed,
    const MULTISAMPLED: Multisampled,
    const SAMPLED: Sampled,
    const FORMAT: ImageFormat,
    const ACCESS_QUALIFIER: Option<AccessQualifier>,
> {
    _x: u32,
    _marker: core::marker::PhantomData<TYPE>,
}

impl<
        TYPE: SampledType + Scalar,
        const DIM: Dimensionality,
        const DEPTH: ImageDepth,
        const ARRAYED: Arrayed,
        const MULTISAMPLED: Multisampled,
        const SAMPLED: Sampled,
        const FORMAT: ImageFormat,
        const ACCESS_QUALIFIER: Option<AccessQualifier>,
    > Image<TYPE, DIM, DEPTH, ARRAYED, MULTISAMPLED, SAMPLED, FORMAT, ACCESS_QUALIFIER>
{
    #[spirv_std_macros::gpu_only]
    pub fn sample<V: Vector<TYPE, 4>>(&self, sampler: Sampler, coord: impl ImageCoordinate<TYPE, { DIM }>) -> V {
        unsafe {
            let mut result = Default::default();
            asm!(
                "%typeSampledImage = OpTypeSampledImage typeof*{1}",
                "%image = OpLoad typeof*{1} {1}",
                "%sampler = OpLoad typeof*{2} {2}",
                "%coord = OpLoad typeof*{3} {3}",
                "%sampledImage = OpSampledImage %typeSampledImage %image %sampler",
                "%result = OpImageSampleImplicitLod typeof*{0} %sampledImage %coord",
                "OpStore {0} %result",
                in(reg) &mut result,
                in(reg) self,
                in(reg) &sampler,
                in(reg) &coord
            );
            result
        }
    }
}


shared_methods! {
    impl Image<Sampled::Unknown, Sampled::No> {
        #[spirv_std_macros::gpu_only]
        #[cfg(feature = "const-generics")]
        pub fn read<I, V, const N: usize>(&self, coordinate: impl Vector<I, 2>) -> V
        where
            I: Integer,
            V: Vector<TYPE, N>,
        {
            let mut result = V::default();

            unsafe {
                asm! {
                    "%image = OpLoad _ {this}",
                    "%coordinate = OpLoad _ {coordinate}",
                    "%result = OpImageRead typeof*{result} %image %coordinate",
                    "OpStore {result} %result",
                    this = in(reg) self,
                    coordinate = in(reg) &coordinate,
                    result = in(reg) &mut result,
                }
            }

            result
        }
    }
}

/// An opaque reference to settings that describe how to access, filter, or
/// sample an image.
#[allow(unused_attributes)]
#[spirv(sampler)]
#[derive(Copy, Clone)]
pub struct Sampler {
    _x: u32,
}

/// An image combined with a sampler, enabling filtered accesses of the
/// image’s contents.
#[allow(unused_attributes)]
#[spirv(sampled_image)]
#[derive(Copy, Clone)]
pub struct SampledImage<I> {
    _image: I,
}

impl<
        TYPE: SampledType + Scalar,
        const DIM: Dimensionality,
        const DEPTH: ImageDepth,
        const ARRAYED: Arrayed,
        const MULTISAMPLED: Multisampled,
        const SAMPLED: Sampled,
        const FORMAT: ImageFormat,
        const ACCESS_QUALIFIER: Option<AccessQualifier>,
    > SampledImage<Image<TYPE, DIM, DEPTH, ARRAYED, MULTISAMPLED, SAMPLED, FORMAT, ACCESS_QUALIFIER>>
{
    #[spirv_std_macros::gpu_only]
    pub unsafe fn sample<V: Vector<TYPE, 4>>(&self, coord: impl ImageCoordinate<TYPE, { DIM }>) -> V {
        let mut result = Default::default();
        asm!(
            "%sampledImage = OpLoad typeof*{1} {1}",
            "%coord = OpLoad typeof*{2} {2}",
            "%result = OpImageSampleImplicitLod typeof*{0} %sampledImage %coord",
            "OpStore {0} %result",
            in(reg) &mut result,
            in(reg) self,
            in(reg) &coord
        );
        result
    }
}



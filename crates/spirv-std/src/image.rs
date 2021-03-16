//! Image types

mod params;

pub use spirv_std_shared::image_params::{
    AccessQualifier, Arrayed, Dimensionality, ImageDepth, ImageFormat,
    Multisampled, Sampled, SampledType,
};
pub use spirv_std_macros::Image;
pub use self::params::{ImageCoordinate, SampleType};

use crate::{scalar::Scalar, vector::Vector};

pub type Image1d = Image!(1D, format=unknown(f32), __crate_root=crate);
pub type Image2d = Image!(2D, format=unknown(f32), __crate_root=crate);
pub type Image3d = Image!(3D, format=unknown(f32), __crate_root=crate);
pub type ImageCube = Image!(cube, format=unknown(f32), __crate_root=crate);
pub type ImageRect = Image!(rect, format=unknown(f32), __crate_root=crate);
pub type ImageBuffer = Image!(buffer, format=unknown(f32), __crate_root=crate);

//macro_rules! shared_methods {
//    (impl Image<$(Sampled::$typ:ident),+> { $($tree:tt)* }) => {
//        shared_methods! {
//            @impl<{
//                TYPE: SampledType + Scalar,
//                const DIM: Dimensionality,
//                const DEPTH: ImageDepth,
//                const ARRAYED: Arrayed,
//                const MULTISAMPLED: Multisampled,
//                const FORMAT: ImageFormat,
//                const ACCESS_QUALIFIER: Option<AccessQualifier>,
//        }> $(Image<TYPE, DIM, DEPTH, ARRAYED, MULTISAMPLED, { Sampled::$typ }, FORMAT, ACCESS_QUALIFIER>),+
//            { { $($tree)* } }
//        }
//    };
//
//    (impl $($types:ty),+ { $($tree:tt)* }) => {
//        shared_methods! { @impl $($types),+ { { $($tree)* } } }
//    };
//
//    (@impl<{ $bounds:tt }> $($types:ty),+ {$tree:tt}) => {
//        $(impl<$bounds> $types $tree)+
//    };
//}

/// An opaque image type. Corresponds to `OpTypeImage`.
#[spirv(image)]
#[derive(Copy, Clone)]
pub struct Image<
    const DIM: Dimensionality,
    const DEPTH: ImageDepth,
    const ARRAYED: Arrayed,
    const MULTISAMPLED: Multisampled,
    const SAMPLED: Sampled,
    const FORMAT: ImageFormat,
    const ACCESS_QUALIFIER: Option<AccessQualifier>,
> {
    _x: u32,
}

impl<
        const DIM: Dimensionality,
        const DEPTH: ImageDepth,
        const ARRAYED: Arrayed,
        const MULTISAMPLED: Multisampled,
        const SAMPLED: Sampled,
        const FORMAT: ImageFormat,
        const ACCESS_QUALIFIER: Option<AccessQualifier>,
    > Image<DIM, DEPTH, ARRAYED, MULTISAMPLED, SAMPLED, FORMAT, ACCESS_QUALIFIER>
{
    /// Sample texels at `coord` from the image using `sampler`.
    #[spirv_std_macros::gpu_only]
    pub fn sample<S, V>(&self, sampler: Sampler, coord: impl ImageCoordinate<S, { DIM }>) -> V
        where S: SampleType<{ FORMAT }> + Scalar,
              V: Vector<S, 4>,
    {
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


// shared_methods! {
//     impl Image<Sampled::Unknown, Sampled::No> {
//         #[spirv_std_macros::gpu_only]
//         #[cfg(feature = "const-generics")]
//         pub fn read<I, V, const N: usize>(&self, coordinate: impl Vector<I, 2>) -> V
//         where
//             I: Integer,
//             V: Vector<TYPE, N>,
//         {
//             let mut result = V::default();
//
//             unsafe {
//                 asm! {
//                     "%image = OpLoad _ {this}",
//                     "%coordinate = OpLoad _ {coordinate}",
//                     "%result = OpImageRead typeof*{result} %image %coordinate",
//                     "OpStore {result} %result",
//                     this = in(reg) self,
//                     coordinate = in(reg) &coordinate,
//                     result = in(reg) &mut result,
//                 }
//             }
//
//             result
//         }
//     }
// }

/// An opaque reference to settings that describe how to access, filter, or
/// sample an image.
#[spirv(sampler)]
#[derive(Copy, Clone)]
pub struct Sampler {
    _x: u32,
}

/// An image combined with a sampler, enabling filtered accesses of the
/// image’s contents.
#[spirv(sampled_image)]
#[derive(Copy, Clone)]
pub struct SampledImage<I> {
    _image: I,
}

impl<
        const DIM: Dimensionality,
        const DEPTH: ImageDepth,
        const ARRAYED: Arrayed,
        const MULTISAMPLED: Multisampled,
        const SAMPLED: Sampled,
        const FORMAT: ImageFormat,
        const ACCESS_QUALIFIER: Option<AccessQualifier>,
    > SampledImage<Image<DIM, DEPTH, ARRAYED, MULTISAMPLED, SAMPLED, FORMAT, ACCESS_QUALIFIER>>
{
    /// Sample texels at `coord` from the sampled image.
    #[spirv_std_macros::gpu_only]
    pub unsafe fn sample<S, V>(&self, sampler: Sampler, coord: impl ImageCoordinate<S, { DIM }>) -> V
        where S: SampleType<{ FORMAT }> + Scalar,
              V: Vector<S, 4>,
    {
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



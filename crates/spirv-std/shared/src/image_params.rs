use core::str::FromStr;

#[cfg(feature = "macros")]
use quote::{quote, TokenStreamExt};

/// The access permissions for the image.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AccessQualifier {
    /// A read only image.
    ReadOnly = 0,
    /// A write only image.
    WriteOnly = 1,
    /// A readable and writable image.
    ReadWrite = 2,
}

#[cfg(feature = "macros")]
impl quote::ToTokens for AccessQualifier {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        stream.append_all(match self {
            Self::ReadOnly => quote!(AccessQualifier::ReadOnly),
            Self::WriteOnly => quote!(AccessQualifier::WriteOnly),
            Self::ReadWrite => quote!(AccessQualifier::ReadWrite),
        });
    }
}

impl FromStr for AccessQualifier {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "read" => Ok(Self::ReadOnly),
            "write" => Ok(Self::WriteOnly),
            "read_write" => Ok(Self::ReadWrite),
            _ => Err("Invalid access qualifier."),
        }
    }
}

/// Whether the image uses arrayed content.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Arrayed {
    /// The image uses not arrayed content.
    False = 0,
    /// The image uses arrayed content.
    True = 1,
}

impl From<bool> for Arrayed {
    fn from(val: bool) -> Self {
        if val { Self::True } else { Self::False }
    }
}

#[cfg(feature = "macros")]
impl quote::ToTokens for Arrayed {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        stream.append_all(match self {
            Self::True => quote!(Arrayed::True),
            Self::False => quote!(Arrayed::False),
        });
    }
}

/// The dimension of the image.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Dimensionality {
    /// 1D
    OneD = 0,
    /// 2D
    TwoD = 1,
    /// 3D
    ThreeD = 2,
    /// 2D Cubemap texture
    Cube = 3,
    /// 2D Rectangle texture
    Rect = 4,
    /// 1D Buffer texture
    Buffer = 5,
    /// Vulkan subpass buffer
    SubpassData = 6,
}

#[cfg(feature = "macros")]
impl quote::ToTokens for Dimensionality {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        stream.append_all(match self {
            Self::OneD => quote!(Dimensionality::OneD),
            Self::TwoD => quote!(Dimensionality::TwoD),
            Self::ThreeD => quote!(Dimensionality::ThreeD),
            Self::Rect => quote!(Dimensionality::Rect),
            Self::Cube => quote!(Dimensionality::Cube),
            Self::Buffer => quote!(Dimensionality::Buffer),
            Self::SubpassData => quote!(Dimensionality::SubpassData),
        });
    }
}

/// Whether a given image contains [depth] information. **Note** Whether or not
/// to perform depth comparisons is a property of the sampling code, not of this
/// type.
///
/// [depth]: https://en.wikipedia.org/wiki/Depth_map
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ImageDepth {
    /// Indicates that the image does not contain depth information.
    False = 0,
    /// Indicates that the image contains depth information.
    True = 1,
    /// Indicates that is not known ahead of time whether the image has depth
    /// information or not.
    Unknown = 2,
}

impl From<Option<bool>> for ImageDepth {
    fn from(val: Option<bool>) -> Self {
        match val {
            Some(true) => Self::True,
            Some(false) => Self::False,
            None => Self::Unknown,
        }
    }
}

#[cfg(feature = "macros")]
impl quote::ToTokens for ImageDepth {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        stream.append_all(match self {
            Self::True => quote!(ImageDepth::True),
            Self::False => quote!(ImageDepth::False),
            Self::Unknown => quote!(ImageDepth::Unknown),
        });
    }
}

/// The sampled type of an unknown image format.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SampledType {
    Float { width: u8 },
    Integer { signed: bool, width: u8 },
    Void,
}

/// The underlying internal representation of the image.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ImageFormat {
    /// Representation not known at compile time.
    Unknown(SampledType) = 0,
    /// RGBA channels, 32 bit floating point integer.
    Rgba32f = 1,
    /// RGBA channels, 16 bit floating point integer.
    Rgba16f = 2,
    /// RGBA channels, 16 bit floating point integer.
    R32f = 3,
    /// RGBA channels, 8 bit floating point integer.
    Rgba8 = 4,
    /// RGBA channels, 8 bit signed normalized integer.
    Rgba8Snorm = 5,
    /// Red+Green channels, 32 bit floating point integer.
    Rg32f = 6,
    /// Red+Green channels, 16 bit floating point integer.
    Rg16f = 7,
    /// 32 bit unsigned integer containing two 11 bit floating point integers
    /// for the Red and Green channels, and a 10 bit floating point integer for
    /// the Blue channel.
    R11fG11fB10f = 8,
    /// Red channel, 16 bit floating point.
    R16f = 9,
    /// RGBA channel, 16 bit floating point.
    Rgba16 = 10,
    /// 32 bit unsigned integer containing three 10 bit unsigned normalized
    /// integers for the Red, Green, and Blue channels; with a 2 unsigned
    /// normalized integer for the Alpha channel.
    Rgb10A2 = 11,
    /// Red+Green channels, 16 bit floating point integer.
    Rg16 = 12,
    /// Red+Green channels, 8 bit floating point integer.
    Rg8 = 13,
    /// Red+Green channels, 16 bit floating point integer.
    R16 = 14,
    /// Red channel, 8 bit floating point integer.
    R8 = 15,
    /// RGBA channels, 16 bit signed normalized integer.
    Rgba16Snorm = 16,
    /// RGB channels, 16 bit signed normalized integer.
    Rg16Snorm = 17,
    /// Red+Green channels, 8 bit signed normalized integer.
    Rg8Snorm = 18,
    /// Red channel, 16 bit signed normalized integer.
    R16Snorm = 19,
    /// Red channel, 16 bit signed normalized integer.
    R8Snorm = 20,
    /// RGBA channels, 32 bit signed integer.
    Rgba32i = 21,
    /// RGBA channels, 16 bit signed integer.
    Rgba16i = 22,
    /// RGBA channels, 8 bit signed integer.
    Rgba8i = 23,
    /// Red channel, 32 bit signed integer.
    R32i = 24,
    /// Red+Green channels, 32 bit signed integer.
    Rg32i = 25,
    /// Red+Green channels, 16 bit signed integer.
    Rg16i = 26,
    /// Red+Green channels, 8 bit signed integer.
    Rg8i = 27,
    /// Red channel, 16 bit signed integer.
    R16i = 28,
    /// Red channel, 8 bit signed integer.
    R8i = 29,
    /// RGBA channels, 32 bit unsigned integer.
    Rgba32ui = 30,
    /// RGBA channels, 16 bit unsigned integer.
    Rgba16ui = 31,
    /// RGBA channels, 8 bit unsigned integer.
    Rgba8ui = 32,
    /// Red channel, 32 bit unsigned integer.
    R32ui = 33,
    /// 32 bit unsigned integer containing three 10 bit unsigned integers for
    /// the Red, Green, and Blue channels, and a 2 bit unsigned integer for the
    /// Alpha channel.
    Rgb10A2ui = 34,
    /// Red+Green channels, 32 bit unsigned integer.
    Rg32ui = 35,
    /// Red+Green channels, 16 bit unsigned integer.
    Rg16ui = 36,
    /// Red+Green channels, 8 bit unsigned integer.
    Rg8ui = 37,
    /// Red channel, 16 bit unsigned integer.
    R16ui = 38,
    /// Red channel, 8 bit unsigned integer.
    R8ui = 39,
}

#[cfg(feature = "macros")]
impl ImageFormat {
    pub fn to_tokens(&self, crate_root: &syn::Path) -> proc_macro2::TokenStream {
        let variant = if let Self::Unknown(sampled_type) = self {
            match sampled_type {
                SampledType::Void => {
                    quote::quote!(Unknown(#crate_root::image::SampledType::Void))
                }
                SampledType::Float { width } => {
                    quote::quote!(Unknown(#crate_root::image::SampledType::Float {
                        width: #width,
                    }))
                }
                SampledType::Integer { signed, width } => {
                    quote::quote!(Unknown(#crate_root::image::SampledType::Integer {
                        signed: #signed,
                        width: #width,
                    }))
                }
            }
        } else {
            let variant = match self {
                Self::Unknown(_) => unreachable!(),
                Self::Rgba32f => "Rgba32f",
                Self::Rgba16f => "Rgba16f",
                Self::R32f => "R32f",
                Self::Rgba8 => "Rgba8",
                Self::Rgba8Snorm => "Rgba8Snorm",
                Self::Rg32f => "Rg32f",
                Self::Rg16f => "Rg16f",
                Self::R11fG11fB10f => "R11fG11fB10f",
                Self::R16f => "R16f",
                Self::Rgba16 => "Rgba16",
                Self::Rgb10A2 => "Rgb10A2",
                Self::Rg16 => "Rg16",
                Self::Rg8 => "Rg8",
                Self::R16 => "R16",
                Self::R8 => "R8",
                Self::Rgba16Snorm => "Rgba16Snorm",
                Self::Rg16Snorm => "Rg16Snorm",
                Self::Rg8Snorm => "Rg8Snorm",
                Self::R16Snorm => "R16Snorm",
                Self::R8Snorm => "R8Snorm",
                Self::Rgba32i => "Rgba32i",
                Self::Rgba16i => "Rgba16i",
                Self::Rgba8i => "Rgba8i",
                Self::R32i => "R32i",
                Self::Rg32i => "Rg32i",
                Self::Rg16i => "Rg16i",
                Self::Rg8i => "Rg8i",
                Self::R16i => "R16i",
                Self::R8i => "R8i",
                Self::Rgba32ui => "Rgba32ui",
                Self::Rgba16ui => "Rgba16ui",
                Self::Rgba8ui => "Rgba8ui",
                Self::R32ui => "R32ui",
                Self::Rgb10A2ui => "Rgb10A2ui",
                Self::Rg32ui => "Rg32ui",
                Self::Rg16ui => "Rg16ui",
                Self::Rg8ui => "Rg8ui",
                Self::R16ui => "R16ui",
                Self::R8ui => "R8ui",
            };

            let variant = proc_macro2::Ident::new(variant, proc_macro2::Span::mixed_site());

            quote!(#variant)
        };

        quote!(#crate_root::image::ImageFormat::#variant)
    }

}

impl FromStr for ImageFormat {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "rgba32f" => Self::Rgba32f,
            "rgba16f" => Self::Rgba16f,
            "r32f" => Self::R32f,
            "rgba8" => Self::Rgba8,
            "rgba8_snorm" => Self::Rgba8Snorm,
            "rg32f" => Self::Rg32f,
            "rg16f" => Self::Rg16f,
            "r11f_g11f_b10f" => Self::R11fG11fB10f,
            "r16f" => Self::R16f,
            "rgba16" => Self::Rgba16,
            "rgb10_a2" => Self::Rgb10A2,
            "rg16" => Self::Rg16,
            "rg8" => Self::Rg8,
            "r16" => Self::R16,
            "r8" => Self::R8,
            "rgba16_snorm" => Self::Rgba16Snorm,
            "rg16_snorm" => Self::Rg16Snorm,
            "rg8_snorm" => Self::Rg8Snorm,
            "r16_snorm" => Self::R16Snorm,
            "r8_snorm" => Self::R8Snorm,
            "rgba32i" => Self::Rgba32i,
            "rgba16i" => Self::Rgba16i,
            "rgba8i" => Self::Rgba8i,
            "r32i" => Self::R32i,
            "rg32i" => Self::Rg32i,
            "rg16i" => Self::Rg16i,
            "rg8i" => Self::Rg8i,
            "r16i" => Self::R16i,
            "r8i" => Self::R8i,
            "rgba32ui" => Self::Rgba32ui,
            "rgba16ui" => Self::Rgba16ui,
            "rgba8ui" => Self::Rgba8ui,
            "r32ui" => Self::R32ui,
            "rgb10_a2ui" => Self::Rgb10A2ui,
            "rg32ui" => Self::Rg32ui,
            "rg16ui" => Self::Rg16ui,
            "rg8ui" => Self::Rg8ui,
            "r16ui" => Self::R16ui,
            "r8ui" => Self::R8ui,
            _ => return Err("Unknown specified image format. Use `unknown(<type>)` if this is intentional."),
        })
    }
}

/// Whether the image uses arrayed content.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Multisampled {
    /// The image contains single-sampled content.
    False = 0,
    /// The image contains multisampled content.
    True = 1,
}

impl From<bool> for Multisampled {
    fn from(val: bool) -> Self {
        if val { Self::True } else { Self::False }
    }
}

#[cfg(feature = "macros")]
impl quote::ToTokens for Multisampled {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        stream.append_all(match self {
            Self::True => quote!(Multisampled::True),
            Self::False => quote!(Multisampled::False),
        });
    }
}

/// Whether or not the image will be accessed in combination with a sampler.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Sampled {
    /// Indicates that it is not known ahead of time whether the image will use
    /// a sampler or not.
    Unknown = 0,
    /// The image will be used with a sampler.
    Yes = 1,
    /// The image will not be used with a sampler.
    No = 2,
}

impl From<Option<bool>> for Sampled {
    fn from(val: Option<bool>) -> Self {
        match val {
            Some(true) => Self::Yes,
            Some(false) => Self::No,
            None => Self::Unknown,
        }
    }
}

#[cfg(feature = "macros")]
impl quote::ToTokens for Sampled {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        stream.append_all(match self {
            Self::Yes => quote!(Sampled::Yes),
            Self::No => quote!(Sampled::No),
            Self::Unknown => quote!(Sampled::Unknown),
        });
    }
}

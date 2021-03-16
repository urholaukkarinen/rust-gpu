use proc_macro2::Ident;

use quote::{quote, TokenStreamExt};
use syn::{parse::{Parse, ParseStream}, Result, spanned::Spanned};
use spirv_std_shared::image_params::*;

mod kw {
    syn::custom_keyword!(u8);
    syn::custom_keyword!(u16);
    syn::custom_keyword!(u32);
    syn::custom_keyword!(i8);
    syn::custom_keyword!(i16);
    syn::custom_keyword!(i32);
    syn::custom_keyword!(f32);
}

/// Creates an `Image` type using the following syntax.
pub struct ImageType {
    access_qualifier: Option<AccessQualifier>,
    dimensionality: Dimensionality,
    arrayed: Arrayed,
    depth: ImageDepth,
    format: ImageFormat,
    multisampled: Multisampled,
    sampled: Sampled,
    crate_root: Option<syn::Path>,
}

impl Parse for ImageType {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut access_qualifier = None;
        let mut dimensionality = None;
        let mut arrayed = None;
        let mut depth: Option<ImageDepth> = None;
        let mut format = None;
        let mut multisampled = None;
        let mut sampled = None;
        let mut crate_root = None;

        let starting_span = input.span();

        macro_rules! set_unique {
            ($id:ident = $ex:expr) => {{
                if $id.replace($ex).is_some() {
                    return Err(
                        syn::Error::new(
                            input.span(),
                            concat!(
                                "Unexpected duplicate parameter for `",
                                stringify!($id),
                                "`"
                            )
                        )
                    )
                }
            }}
        }


        macro_rules! peek_and_eat_value {
            ($typ:ty) => {{
                if input.peek(syn::Token![=]) {
                    input.parse::<syn::Token![=]>()?;
                    Some(input.parse::<$typ>()?)
                } else {
                    None
                }
            }}
        }

        while !input.is_empty() {
            if input.peek(syn::LitInt) {
                let int = input.parse::<syn::LitInt>().unwrap();
                match (int.base10_digits(), int.suffix()) {
                    ("1", "D") | ("1", "d") => set_unique!(dimensionality = Dimensionality::OneD),
                    ("2", "D") | ("2", "d") => set_unique!(dimensionality = Dimensionality::TwoD),
                    ("3", "D") | ("3", "d") => set_unique!(dimensionality = Dimensionality::ThreeD),
                    _ => return Err(syn::Error::new(int.span(), "Unexpected integer")),
                }

            } else if input.peek(syn::Ident) {
                let ident = input.parse::<Ident>().unwrap();

                if ident == "access" {
                    let value = peek_and_eat_value!(syn::Ident)
                                    .as_ref()
                                    .map(|i| i.to_string().parse());

                    if value.is_none() {
                        return Err(syn::Error::new(ident.span(), "Expected argument for `access`."));
                    }

                    access_qualifier = value.unwrap().ok();
                } else if ident == "buffer" {
                    set_unique!(dimensionality = Dimensionality::Buffer);
                } else if ident == "cube" {
                    set_unique!(dimensionality = Dimensionality::Cube);
                } else if ident == "rect" {
                    set_unique!(dimensionality = Dimensionality::Rect);
                } else if ident == "subpass" {
                    set_unique!(dimensionality = Dimensionality::SubpassData);
                } else if ident == "arrayed" {
                    set_unique!(
                        arrayed = peek_and_eat_value!(syn::LitBool)
                                    .as_ref()
                                    .map(syn::LitBool::value)
                                    .map_or(Arrayed::False, From::from)
                    );
                } else if ident == "multisampled" {
                    set_unique!(
                        multisampled = peek_and_eat_value!(syn::LitBool)
                                    .as_ref()
                                    .map(syn::LitBool::value)
                                    .map_or(Multisampled::False, From::from)
                    );
                } else if ident == "sampled" {
                    set_unique!(
                        sampled = From::from(
                            peek_and_eat_value!(syn::LitBool)
                                    .as_ref()
                                    .map(syn::LitBool::value)
                            )
                    );
                } else if ident == "depth" {
                    set_unique!(
                        depth = From::from(peek_and_eat_value!(syn::LitBool)
                                    .as_ref()
                                    .map(syn::LitBool::value))
                    );
                } else if ident == "format" {
                    let value = peek_and_eat_value!(syn::Ident);

                    if value.is_none() {
                        return Err(syn::Error::new(ident.span(), "Expected argument for `format`."));
                    }

                    let value = value.unwrap();

                    if value == "unknown" {
                        let ty;
                        syn::parenthesized!(ty in input);
                        let sampled_type = if ty.peek(kw::u8) {
                            ty.parse::<kw::u8>()?;

                            SampledType::Integer { signed: false, width: 8 }
                        } else if ty.peek(kw::u16) {
                            ty.parse::<kw::u16>()?;

                            SampledType::Integer { signed: false, width: 16 }
                        } else if ty.peek(kw::u32) {
                            ty.parse::<kw::u32>()?;

                            SampledType::Integer { signed: false, width: 32 }
                        } else if ty.peek(kw::i8) {
                            ty.parse::<kw::i8>()?;

                            SampledType::Integer { signed: true, width: 8 }
                        } else if ty.peek(kw::i16) {
                            ty.parse::<kw::i16>()?;

                            SampledType::Integer { signed: true, width: 16 }
                        } else if ty.peek(kw::i32) {
                            ty.parse::<kw::i32>()?;

                            SampledType::Integer { signed: true, width: 32 }
                        } else if ty.peek(kw::f32) {
                            ty.parse::<kw::f32>()?;

                            SampledType::Float { width: 32 }
                        } else {
                            return Err(syn::Error::new(ty.span(), "Unknown value provided to `unknown(_)`."));
                        };

                        format = Some(ImageFormat::Unknown(sampled_type));
                    } else {
                        let value = value.to_string().parse::<ImageFormat>();

                        if let Err(err) = value {
                            return Err(syn::Error::new(ident.span(), err));
                        }

                        format = value.ok();

                    }
                } else if ident == "__crate_root" {
                    input.parse::<syn::Token![=]>()?;
                    crate_root = Some(input.parse::<syn::Path>()?);
                }
            }

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
                continue;
            } else {
                break;
            }
        }

        if !input.is_empty() {
            return Err(syn::Error::new(input.span(), "Unexpected trailing arguments."));
        }

        let dimensionality = dimensionality.ok_or(
            syn::Error::new(starting_span, "Expected either `1D`, `2D`, `3D`, `cube`, `rect`, `buffer`, or `subpass` to be present"))?;

        let format = format.ok_or(
            syn::Error::new(starting_span, "Expected an image `format` to be \
                specified. Use `format=unknown(<sampled_type>)`, if the format \
                isn't available in SPIR-V.")
        )?;

        let depth = depth.unwrap_or(ImageDepth::Unknown);
        let arrayed = arrayed.unwrap_or(Arrayed::False);
        let multisampled = multisampled.unwrap_or(Multisampled::False);
        let sampled = sampled.unwrap_or(Sampled::Unknown);

        Ok(Self {
            access_qualifier,
            dimensionality,
            arrayed,
            depth,
            format,
            multisampled,
            sampled,
            crate_root,
        })
    }
}

impl quote::ToTokens for ImageType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_root = self.crate_root.clone().unwrap_or_else(|| syn::Path {
            leading_colon: None,
            segments: {
                let mut punct = syn::punctuated::Punctuated::new();
                punct.push(Ident::new("spirv_std", proc_macro2::Span::mixed_site()).into());

                punct
            }
        });
        let access_qualifier = match self.access_qualifier {
                Some(aq) => quote!(Some(#crate_root::image::#aq)),
                None => quote!(None),
        };
        let dimensionality = &self.dimensionality;
        let arrayed = &self.arrayed;
        let depth = &self.depth;
        let format = self.format.to_tokens(&crate_root);
        let multisampled = &self.multisampled;
        let sampled = &self.sampled;

        tokens.append_all(quote::quote! {
            #crate_root::image::Image<
                { #dimensionality },
                { #depth },
                { #arrayed },
                { #multisampled },
                { #sampled },
                { #format },
                { #access_qualifier },
            >
        })
    }
}

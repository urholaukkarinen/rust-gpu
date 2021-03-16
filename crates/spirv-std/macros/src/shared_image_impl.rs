/// Creates multiple Image impl blocks based on what parameters provided. You
/// can provide 1 or more variants using an array literal syntax.
#[proc_macro_attribute]
#[doc(hidden)]
pub fn vectorized(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = syn::parse_macro_input!(item as syn::ItemFn);
    let vectored_function = match create_vectored_fn(function.clone()) {
        Ok(val) => val,
        Err(err) => return err.to_compile_error().into(),
    };

    let output = quote::quote!(
        #function

        #vectored_function
    );

    output.into()
}


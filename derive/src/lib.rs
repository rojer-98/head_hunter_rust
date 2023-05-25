use proc_macro::TokenStream;

use quote::quote;
use syn;

#[proc_macro_derive(Query)]
pub fn query_handler(input: TokenStream) -> TokenStream {
    impl_query_handler(&syn::parse(input).unwrap())
}

fn impl_query_handler(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    quote! {
        impl QueryHandler for #name {
                fn into_query_string(&self) -> Result<::std::string::String, crate::utils::HError> {
                    Ok(to_string(self)?)
                }
            }
    }
    .into()
}

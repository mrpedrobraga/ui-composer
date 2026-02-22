use {crate::transform::view_internal, proc_macro::TokenStream};

mod transform;

#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    view_internal(input)
}

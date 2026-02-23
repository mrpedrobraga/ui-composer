use {
    crate::transform::view_internal, proc_macro::TokenStream,
    proc_macro_error2::proc_macro_error,
};

mod transform;

#[proc_macro]
#[proc_macro_error]
pub fn view(input: TokenStream) -> TokenStream {
    view_internal(input)
}

use crate::tags::transform_tokens;
use proc_macro::TokenStream;

mod blocks;
mod config;
mod tags;

#[proc_macro]
pub fn uix(input: TokenStream) -> TokenStream {
    transform_tokens(input)
}

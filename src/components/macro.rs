extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn component(args: Option<TokenStream>, input: TokenStream) -> TokenStream {

}
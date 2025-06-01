extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn program(_args: TokenStream, input: TokenStream) -> TokenStream {
    // 1. Parse input as Program (raw ItemMod) even if we don’t inspect it yet
    let parsed = parse_macro_input!(input as geppetto_parser::Program);

    // 2. Grab the original module tokens, so we can re-emit them “as is” inside the generated code
    let original_mod: proc_macro2::TokenStream = parsed.0.into_token_stream();

    // 3. Emit a Pinocchio entrypoint wrapper around the original module
    let generated = quote! {
        // This macro comes from geppetto-core re-exporting pinocchio::lazy_program_entrypoint
        geppetto_core::lazy_program_entrypoint!(process_instruction);

        // User’s module goes here
        #original_mod
    };

    TokenStream::from(generated)
}
